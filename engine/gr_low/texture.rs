extern mod gl;

use std;
use std::cmp::Eq;
use std::hashmap::HashMap;
use std::{managed,ptr};
use std::to_str::ToStr;

use gr_low::{context,frame};


pub type Mode	= gl::types::GLenum;
#[deriving(Eq)]
pub struct Handle( gl::types::GLuint );
#[deriving(Clone,Eq,IterBytes)]
pub struct Target( gl::types::GLenum );

impl Drop for Handle	{
	fn drop( &mut self )	{
		unsafe	{
			gl::DeleteTextures( 1, ptr::to_unsafe_ptr(&**self) );
		}
	}
}

//#[deriving(Clone)]	//doesn't work in Rust-0.8
pub struct Sampler	{
	filter	: [gl::types::GLint, ..2],
	wrap	: [gl::types::GLint, ..3],
	compare	: Option<gl::types::GLenum>,
}

impl Clone for Sampler	{
	fn clone( &self )-> Sampler	{
		Sampler	{
			filter	: [self.filter[0],self.filter[1]],
			wrap	: [self.wrap[0],self.wrap[1],self.wrap[2]],
			compare	: self.compare,
		}
	}
}

impl Sampler	{
	pub fn new( filter : uint, wrap : int )-> Sampler	{
		assert!( filter>0u && filter<=3u );
		let min_filter =	if filter==3u	{gl::LINEAR_MIPMAP_LINEAR}
			else			if filter==2u	{gl::LINEAR}
			else							{gl::NEAREST}
			as gl::types::GLint;
		let mag_filter =	if filter>1u	{gl::LINEAR}
			else							{gl::NEAREST}
			as gl::types::GLint;
		let wr =	if wrap>0	{gl::REPEAT}
			else	if wrap<0	{gl::MIRRORED_REPEAT}
			else				{gl::CLAMP_TO_EDGE}
			as gl::types::GLint;
		Sampler{
			filter	: [min_filter,mag_filter],
			wrap	: [wr,wr,wr],
			compare	: None,
		}
	}
}

impl ToStr for Sampler	{
	fn to_str( &self )-> ~str	{
		let sf = match self.filter[0] as u32	{
			gl::LINEAR_MIPMAP_LINEAR	=> ~"trilinear",
			gl::LINEAR				=> ~"bilinear",
			gl::NEAREST				=> ~"point",
			_								=> ~"unknown"
		};
		let sw = match self.wrap[0] as u32	{
			gl::REPEAT			=> ~"repeat",
			gl::MIRRORED_REPEAT	=> ~"mirror_repeat",
			gl::CLAMP_TO_EDGE	=> ~"clamp",
			_							=> ~"unknown"
		};
		format!( "Sampler(filter={:s}, wrap={:s}, compare={:b})", sf, sw, !self.compare.is_none() )
	}
}


pub struct LevelInfo	{
	total	: uint,
	min		: uint,
	max		: uint,
}


pub struct Texture	{
	handle	: Handle,
	target	: Target,
	width		: uint,
	height		: uint,
	depth		: uint,
	samples		: uint,
	priv levels		: @mut LevelInfo,
	priv sampler	: @mut Sampler,
}

impl context::ProxyState for Texture	{
	fn sync_back( &mut self )->bool	{
		//FIXME
		true
	}
}

impl ToStr for Texture	{
	fn to_str( &self )-> ~str	{
		format!( "Texture(h={:i}, {:u}x{:u}x{:u}, samples={:u})", *self.handle as int,
			self.width, self.height, self.depth, self.samples )
	}
}

impl Texture	{
	pub fn get_num_levels( &self )-> uint	{
		self.levels.total
	}
	pub fn get_level_size( &self, lev : uint )-> (uint,uint)	{
		assert!( self.width>0u && self.height>0u );
		(std::uint::max(1u,self.width>>lev), std::uint::max(1u,self.height>>lev))
	}
	pub fn get_level_limits( &self )-> (uint,uint)	{
		(self.levels.min, self.levels.max)
	}
	pub fn count_levels( &self )-> uint	{
		let mut i = 0;
		while self.get_level_size(i) != (1u,1u)	{
			i += 1;
		}
		i
	}
	pub fn is_filtering_mipmap( &self )-> bool	{
		[gl::LINEAR_MIPMAP_LINEAR,gl::NEAREST_MIPMAP_NEAREST,
		gl::LINEAR_MIPMAP_NEAREST,gl::NEAREST_MIPMAP_LINEAR].
		contains(&(self.sampler.filter[1] as gl::types::GLenum))
	}
	pub fn can_sample( &self )-> bool	{
		self.samples==0u && (!self.is_filtering_mipmap() || self.levels.total==1u)
	}
}


pub fn map_int_format( s : &str )-> gl::types::GLint	{
	(match s	{
		"r8"		=> gl::R8,
		"rgba8"		=> gl::RGBA8,
		"rgba16f"	=> gl::RGBA16F,
		_	=> fail!("Can not recognize texture internal format {:s}", s)
	}) as gl::types::GLint
}

pub fn map_pix_format( s : &str )-> gl::types::GLenum	{
	match s	{
		"red"	=> gl::RED,
		"rg"	=> gl::RG,
		"rgb"	=> gl::RGB,
		"bgr"	=> gl::BGR,
		"rgba"	=> gl::RGBA,
		"bgra"	=> gl::BGRA,
		"depth"	=> gl::DEPTH_COMPONENT,
		"ds"	=> gl::DEPTH_STENCIL,
		_	=> fail!("Can not recognize texture pixel format {:s}", s)
	}
}


#[deriving(Clone,Eq,IterBytes)]
pub struct Slot	{
	unit	: uint,
	target	: Target
}

pub struct Binding	{
	unit	: uint,
	active	: HashMap<Slot,@Texture>,
}

impl context::ProxyState for Binding	{
	fn sync_back( &mut self )->bool	{
		let mut was_ok = true;
		let mut id = 0 as gl::types::GLint;
		unsafe{ gl::GetIntegerv( gl::ACTIVE_TEXTURE, ptr::to_mut_unsafe_ptr(&mut id) ); }
		let cur_unit = id as uint - (gl::TEXTURE0 as uint);
		if self.unit != cur_unit	{
			was_ok = false;
			self.unit = cur_unit;
		}
		// Rust wouldn't allow us to mutate while scanning
		let active = self.active.clone();
		for (slot,tex) in active.iter()	{
			let t = *slot.target;
			let query =	if t == gl::TEXTURE_2D	{gl::TEXTURE_BINDING_2D}
			else	{
				fail!( "Unkown binding {:i}", *slot.target as int )
			};
			self.switch( slot.unit );
			unsafe{ gl::GetIntegerv( query, ptr::to_mut_unsafe_ptr(&mut id) )};
			if *tex.handle != id as gl::types::GLuint	{
				was_ok = false;
				self.active.remove( slot );
			}
		}
		self.switch( cur_unit );
		was_ok
	}
}

impl Binding	{
	pub fn new()-> Binding	{
		Binding{
			unit	: 0u,
			active	: HashMap::new()
		}
	}

	pub fn bind( &mut self, t : @Texture )	{
		let slot = Slot{ unit:self.unit, target:t.target };
		if !self.active.contains_key(&slot) || !managed::ptr_eq(*self.active.get(&slot),t)	{
			self.active.insert( slot, t );
			gl::BindTexture( *t.target, *t.handle );
		}
	}
	
	pub fn switch( &mut self, unit : uint )	{
		if self.unit != unit	{
			self.unit = unit;
			gl::ActiveTexture( gl::TEXTURE0 + (unit as gl::types::GLenum) );
		}
	}

	pub fn bind_to( &mut self, unit: uint, t : @Texture )	{
		self.switch( unit );
		self.bind( t );
	}

	pub fn bind_sampler( &mut self, t : @Texture, s : &Sampler )	{
		self.bind( t );
		if t.samples != 0	{fail!(~"Unable to bound a sampler for MSAA texture")}
		let filter_modes = [gl::TEXTURE_MIN_FILTER,gl::TEXTURE_MAG_FILTER];
		for i in range(0,2)	{
			if t.sampler.filter[i] != s.filter[i]	{
				gl::TexParameteri( *t.target, filter_modes[i], s.filter[i] );
			}
		}
		let wrap_modes = [gl::TEXTURE_WRAP_S,gl::TEXTURE_WRAP_T,gl::TEXTURE_WRAP_R];
		for i in range(0,3)	{
			if t.sampler.wrap[i] != s.wrap[i]	{
				gl::TexParameteri( *t.target, wrap_modes[i], s.wrap[i] );
			}
		}
		if t.sampler.compare != s.compare	{
			let e_mode = gl::TEXTURE_COMPARE_MODE;
			let e_func = gl::TEXTURE_COMPARE_FUNC;
			match s.compare	{
				Some(mode)	=>	{
					gl::TexParameteri( *t.target, e_mode,
						gl::COMPARE_REF_TO_TEXTURE as gl::types::GLint );
					gl::TexParameteri( *t.target, e_func, mode as gl::types::GLint );
				},
				None	=>	{
					gl::TexParameteri( *t.target, e_mode,
						gl::NONE as gl::types::GLint );
				}
			}
		}
		*t.sampler = *s;
	}
	pub fn unbind( &mut self, target : Target )	{
		let slot = Slot{ unit:self.unit, target:target };
		if self.active.find(&slot).is_some()	{
			self.active.remove( &slot );
			gl::BindTexture( *target, 0 );
		}
	}

	pub fn get_bound( &self, target : Target )-> Option<@Texture>	{
		let slot = Slot{ unit:self.unit, target:target };
		match self.active.find( &slot )	{
			Some(t)	=> Some(*t),
			None	=> None
		}
	}

	pub fn is_bound( &self, t : @Texture )-> bool	{
		match self.get_bound( t.target )	{
			Some(tex)	=> managed::ptr_eq( tex, t ),
			None		=> false
		}
	}

	pub fn find( &self, t : @Texture )-> Option<uint>	{
		for (&slot,&tex) in self.active.iter()	{
			if managed::ptr_eq(t,tex)	{
				assert!( slot.target == t.target );
				return Some(slot.unit)
			}
		}
		None
	}

	pub fn init( &mut self, t : @Texture, num_levels : uint, int_format : gl::types::GLint, alpha : bool )	{
		self.bind( t );
		assert!( t.samples == 0u && (t.depth == 0u || num_levels == 1u) );
		let mut level = 0u;
		while level<num_levels	{
			let (w,h) = t.get_level_size( level );
			let (wi,hi,di) = ( w as gl::types::GLsizei, h as gl::types::GLsizei, t.depth as gl::types::GLsizei );
			let pix_format = if alpha {gl::RGBA} else {gl::RGB};
			let data_type = gl::UNSIGNED_BYTE;
			let li = level as gl::types::GLint;
			unsafe{
				if t.depth != 0u	{
					gl::TexImage3D( *t.target, li, int_format, wi, hi, di,
						0, pix_format, data_type, ptr::null() );
				}else if t.height != 0u	{
					gl::TexImage2D( *t.target, li, int_format, wi, hi,
						0, pix_format, data_type, ptr::null() );
				}else	{
					gl::TexImage1D( *t.target, li, int_format, wi,
						0, pix_format, data_type, ptr::null() );
				}
			}
			level += 1u;
		}
		t.levels.total = level;
		gl::GetError();	//debug
	}

	pub fn init_depth( &mut self, t : @Texture, stencil : bool )	{
		self.bind( t );
		assert!( t.samples == 0u && t.levels.total == 0u );
		let (wi,hi,di) = ( t.width as gl::types::GLsizei, t.height	as gl::types::GLsizei, t.depth as gl::types::GLsizei );
		let (ifm,pfm)	= if stencil { (gl::DEPTH24_STENCIL8, gl::DEPTH_STENCIL) }
			else { (gl::DEPTH_COMPONENT16, gl::DEPTH_COMPONENT) };
		let data_type = gl::UNSIGNED_BYTE;
		unsafe{
			if t.depth != 0u	{
				gl::TexImage3D( *t.target, 0, ifm as gl::types::GLint, wi, hi, di,
					0, pfm, data_type, ptr::null() );
			}else	{
				gl::TexImage2D( *t.target, 0, ifm as gl::types::GLint, wi, hi,
					0, pfm, data_type, ptr::null() );
			}
		}
		t.levels.total = 1u;
		gl::GetError();	//debug
	}

	#[cfg(multisample)]
	pub fn init_multi( &mut self, t : @Texture, int_format : gl::types::GLint, fixed_loc : bool )	{
		self.bind( t );
		assert!( t.samples != 0u && t.levels.total == 0u );
		let (wi,hi,di,si) = (
			t.width as gl::types::GLsizei, t.height	as gl::types::GLsizei,
			t.depth as gl::types::GLsizei, t.samples	as gl::types::GLsizei );
		let fixed = fixed_loc as gl::types::GLboolean;
		unsafe{
			if t.depth != 0u	{
				gl::TexImage3DMultisample( *t.target, si, int_format, wi, hi, di,	fixed );
			}else {
				gl::TexImage2DMultisample( *t.target, si, int_format, wi, hi,		fixed );
			}
		}
		t.levels.total = 1u;
		gl::GetError();	//debug
	}

	pub fn load_2D<T>( &mut self, t : @Texture, level : uint, int_format : gl::types::GLint,
			pix_format : gl::types::GLenum, pix_type : gl::types::GLenum, data : &[T])	{
		self.bind( t );
		gl::PixelStorei( gl::UNPACK_ALIGNMENT, 1 as gl::types::GLint );
		assert!( t.width>0 && t.height>0 && t.samples==0u );
		assert!( t.levels.total >= level );
		if t.levels.total==level	{ t.levels.total += 1; }
		let (w,h) = t.get_level_size( level );
		unsafe{
			let raw = std::vec::raw::to_ptr(data) as *gl::types::GLvoid;
			gl::TexImage2D( *t.target, level as gl::types::GLint, int_format,
				w as gl::types::GLint, h as gl::types::GLint, 0 as gl::types::GLint,
				pix_format, pix_type, raw );
		}
		gl::GetError();	//debug
	}

	pub fn load_sub_2D<T>( &mut self, t : @Texture, level : uint, r : &frame::Rect,
			pix_format : gl::types::GLenum, pix_type : gl::types::GLenum, data : &[T])	{
		self.bind( t );
		assert!( t.width>0 && t.height>0 && t.samples==0u && t.levels.total>level );
		assert!( r.w*r.h == data.len() );
		let (w,h) = t.get_level_size( level );
		assert!( frame::Rect::new(w,h).contains_rect( r ) );
		unsafe{
			let raw = std::vec::raw::to_ptr(data) as *gl::types::GLvoid;
			gl::TexSubImage2D( *t.target, level as gl::types::GLint,
				r.x as gl::types::GLint, r.y as gl::types::GLint,
				r.w as gl::types::GLsizei, r.h as gl::types::GLsizei,
				pix_format, pix_type, raw );
		}
		gl::GetError();	//debug
	}

	pub fn generate_levels( &self, t : @Texture )	{
		assert!( self.is_bound( t ) );
		assert!( t.samples == 0u && t.levels.total > 0u );
		gl::GenerateMipmap( *t.target );
		t.levels.total = t.count_levels();
	}

	pub fn limit_levels( &self, t : @Texture, base : uint, max : uint )	{
		assert!( self.is_bound( t ) );
		assert!( base <= max );
		if t.levels.min != base	{
			t.levels.min = base;
			gl::TexParameteri( *t.target, gl::TEXTURE_BASE_LEVEL, base as gl::types::GLint );
		}
		if t.levels.max != max	{
			t.levels.max = max;
			gl::TexParameteri( *t.target, gl::TEXTURE_MAX_LEVEL, max as gl::types::GLint );
		}
	}
}


pub fn map_target( s : &str )-> Target	{
	Target(match s	{
		&"1D"		=> gl::TEXTURE_1D,
		&"Rect"		=> gl::TEXTURE_RECTANGLE,
		&"2D"		=> gl::TEXTURE_2D,
		&"2DArray"	=> gl::TEXTURE_2D_ARRAY,
		//&"2DMS"		=> gl::TEXTURE_2D_MULTISAMPLE,	//TEMP!
		&"3D"		=> gl::TEXTURE_3D,
		_	=> fail!( "Unable to map texture target {:s}", s )
	})
}


impl context::Context	{
	pub fn create_texture( &self, st:&str, w:uint, h:uint, d:uint, s:uint )-> @Texture	{
		let mut hid = 0 as gl::types::GLuint;
		unsafe{ gl::GenTextures( 1, ptr::to_mut_unsafe_ptr(&mut hid) )};
		@Texture{ handle:Handle(hid), target:map_target(st),
			width:w, height:h, depth:d, samples:s,
			levels:@mut LevelInfo{total:0u,min:0u,max:1000u},
			sampler:@mut Sampler::new(3u,1) }
	}
}
