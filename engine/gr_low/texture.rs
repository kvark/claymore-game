extern mod gl;

use std;
use std::cmp::Eq;
use std::hashmap::HashMap;
use std::{borrow,cell,ptr,rc};
use std::to_str::ToStr;

use gr_low::{context,frame};


pub type Mode	= gl::types::GLenum;
#[deriving(Eq)]
pub struct Handle( gl::types::GLuint );
#[deriving(Clone,Eq,IterBytes)]
pub struct Target( gl::types::GLenum );

impl Drop for Handle	{
	fn drop( &mut self )	{
		let &Handle(ref h) = self;
		unsafe	{
			gl::DeleteTextures( 1, ptr::to_unsafe_ptr(h) );
		}
	}
}

pub type SamplerPtr = cell::RefCell<Sampler>;

//#[deriving(Clone)]	//doesn't work in Rust-0.9
pub struct Sampler	{
	filter	: [gl::types::GLint, ..2],
	wrap	: [gl::types::GLint, ..3],
	compare	: Option<gl::types::GLenum>,
}

impl Clone for Sampler {
	fn clone( &self )-> Sampler {
		Sampler {
			filter	: [self.filter[0],self.filter[1]],
			wrap	: [self.wrap[0],self.wrap[1],self.wrap[2]],
			compare	: self.compare,
		}
	}
}

impl Sampler	{
	pub fn new( filter: uint, wrap: int )-> Sampler	{
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


pub type LevelInfoPtr = cell::RefCell<LevelInfo>;

pub struct LevelInfo	{
	total	: uint,
	min		: uint,
	max		: uint,
}

pub type TexturePtr = rc::Rc<Texture>;

pub struct Texture	{
	handle	: Handle,
	target	: Target,
	width		: uint,
	height		: uint,
	depth		: uint,
	samples		: uint,
	priv levels		: LevelInfoPtr,
	priv sampler	: SamplerPtr,
}


impl context::ProxyState for Texture	{
	fn sync_back( &mut self )->bool	{
		//TODO
		true
	}
}

impl ToStr for Texture	{
	fn to_str( &self )-> ~str	{
		let Handle(h) = self.handle;
		format!( "Texture(h={}, {:u}x{:u}x{:u}, samples={:u})", h,
			self.width, self.height, self.depth, self.samples )
	}
}

impl Texture	{
	pub fn get_num_levels( &self )-> uint	{
		self.levels.borrow().get().total
	}
	pub fn get_level_size( &self, lev: uint )-> (uint,uint)	{
		assert!( self.width>0u && self.height>0u );
		(std::num::max(1u,self.width>>lev), std::num::max(1u,self.height>>lev))
	}
	pub fn get_level_limits( &self )-> (uint,uint)	{
		let lev = self.levels.borrow();
		(lev.get().min, lev.get().max)
	}
	pub fn count_levels( &self )-> uint	{
		let mut i = 0;
		while self.get_level_size(i) != (1u,1u)	{
			i += 1;
		}
		i
	}
	pub fn is_filtering_mipmap( &self )-> bool	{
		let sam = self.sampler.borrow();
		[gl::LINEAR_MIPMAP_LINEAR,gl::NEAREST_MIPMAP_NEAREST,
		gl::LINEAR_MIPMAP_NEAREST,gl::NEAREST_MIPMAP_LINEAR].
		contains(&(sam.get().filter[1] as gl::types::GLenum))
	}
	pub fn can_sample( &self )-> bool	{
		let lev = self.levels.borrow();
		self.samples==0u && (!self.is_filtering_mipmap() || lev.get().total==1u)
	}
}


pub fn map_int_format( s: &str )-> gl::types::GLint	{
	(match s	{
		"r8"		=> gl::R8,
		"rgba8"		=> gl::RGBA8,
		"rgba16f"	=> gl::RGBA16F,
		_	=> fail!("Can not recognize texture internal format {:s}", s)
	}) as gl::types::GLint
}

pub fn map_pix_format( s: &str )-> gl::types::GLenum	{
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
	active	: HashMap<Slot,TexturePtr>,
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
			let Target(t) = slot.target;
			let query =	match t	{
				gl::TEXTURE_2D	=> gl::TEXTURE_BINDING_2D,
				_	=> fail!("Unkown binding {}", t)
			};
			self.switch( slot.unit );
			unsafe{ gl::GetIntegerv( query, ptr::to_mut_unsafe_ptr(&mut id) )};
			if tex.borrow().handle != Handle(id as gl::types::GLuint)	{
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

	pub fn bind( &mut self, pt: &TexturePtr )	{
		if !self.is_bound(pt)	{
			let slot = Slot{ unit:self.unit, target:pt.borrow().target };
			let Target(tar) = pt.borrow().target;
			let Handle(han) = pt.borrow().handle;
			gl::BindTexture( tar, han );
			self.active.insert( slot, pt.clone() );
		}
	}
	
	pub fn switch( &mut self, unit: uint )	{
		if self.unit != unit	{
			self.unit = unit;
			gl::ActiveTexture( gl::TEXTURE0 + (unit as gl::types::GLenum) );
		}
	}

	pub fn bind_to( &mut self, unit: uint, pt: &TexturePtr )	{
		self.switch( unit );
		self.bind( pt );
	}

	pub fn bind_sampler( &mut self, pt: &TexturePtr, s: &Sampler )	{
		self.bind( pt );
		let t = pt.borrow();
		if t.samples != 0	{fail!(~"Unable to bind a sampler for MSAA texture")}
		let filter_modes = [gl::TEXTURE_MIN_FILTER,gl::TEXTURE_MAG_FILTER];
		let Target(tar) = t.target;
		let mut ts = t.sampler.borrow_mut();
		for i in range(0,2)	{
			if ts.get().filter[i] != s.filter[i]	{
				gl::TexParameteri( tar, filter_modes[i], s.filter[i] );
			}
		}
		let wrap_modes = [gl::TEXTURE_WRAP_S,gl::TEXTURE_WRAP_T,gl::TEXTURE_WRAP_R];
		for i in range(0,3)	{
			if ts.get().wrap[i] != s.wrap[i]	{
				gl::TexParameteri( tar, wrap_modes[i], s.wrap[i] );
			}
		}
		if ts.get().compare != s.compare	{
			let e_mode = gl::TEXTURE_COMPARE_MODE;
			let e_func = gl::TEXTURE_COMPARE_FUNC;
			match s.compare	{
				Some(mode)	=>	{
					gl::TexParameteri( tar, e_mode,
						gl::COMPARE_REF_TO_TEXTURE as gl::types::GLint );
					gl::TexParameteri( tar, e_func, mode as gl::types::GLint );
				},
				None	=>	{
					gl::TexParameteri( tar, e_mode,
						gl::NONE as gl::types::GLint );
				}
			}
		}
		*ts.get() = *s;
	}

	pub fn unbind( &mut self, target: Target )	{
		let slot = Slot{ unit:self.unit, target:target };
		if self.active.find(&slot).is_some()	{
			self.active.remove( &slot );
			let Target(tar) = target;
			gl::BindTexture( tar, 0 );
		}
	}

	pub fn get_bound( &self, target: Target )-> Option<TexturePtr>	{
		let slot = Slot{ unit:self.unit, target:target };
		self.active.find( &slot ).map(|t| t.clone())
	}

	pub fn is_bound( &self, pt: &TexturePtr )-> bool	{
		match self.get_bound( pt.borrow().target )	{
			Some(tex)	=> borrow::ref_eq(tex.borrow(), pt.borrow()),
			None		=> false
		}
	}

	pub fn find( &self, pt: &TexturePtr )-> Option<uint>	{
		for (&slot,ref tex) in self.active.iter()	{
			if borrow::ref_eq(tex.borrow(), pt.borrow())	{
				assert!( slot.target == pt.borrow().target );
				return Some(slot.unit)
			}
		}
		None
	}

	pub fn init( &mut self, pt: &TexturePtr, num_levels: uint, int_format: gl::types::GLint, alpha: bool )	{
		self.bind( pt );
		let t = pt.borrow();
		assert!( t.samples == 0u && (t.depth == 0u || num_levels == 1u) );
		let mut level = 0u;
		while level<num_levels	{
			let (w,h) = t.get_level_size( level );
			let (wi,hi,di) = ( w as gl::types::GLsizei, h as gl::types::GLsizei, t.depth as gl::types::GLsizei );
			let pix_format = if alpha {gl::RGBA} else {gl::RGB};
			let data_type = gl::UNSIGNED_BYTE;
			let li = level as gl::types::GLint;
			let Target(tar) = t.target;
			unsafe{
				if t.depth != 0u	{
					gl::TexImage3D( tar, li, int_format, wi, hi, di,
						0, pix_format, data_type, ptr::null() );
				}else if t.height != 0u	{
					gl::TexImage2D( tar, li, int_format, wi, hi,
						0, pix_format, data_type, ptr::null() );
				}else	{
					gl::TexImage1D( tar, li, int_format, wi,
						0, pix_format, data_type, ptr::null() );
				}
			}
			level += 1u;
		}
		t.levels.borrow_mut().get().total = level;
		gl::GetError();	//debug
	}

	pub fn init_depth( &mut self, pt: &TexturePtr, stencil: bool )	{
		self.bind( pt );
		let t = pt.borrow();
		assert!( t.samples == 0u && t.levels.borrow().get().total == 0u );
		let (wi,hi,di) = ( t.width as gl::types::GLsizei, t.height	as gl::types::GLsizei, t.depth as gl::types::GLsizei );
		let (ifm,pfm)	= if stencil { (gl::DEPTH24_STENCIL8, gl::DEPTH_STENCIL) }
			else { (gl::DEPTH_COMPONENT16, gl::DEPTH_COMPONENT) };
		let data_type = gl::UNSIGNED_BYTE;
		let Target(tar) = t.target;
		unsafe{
			if t.depth != 0u	{
				gl::TexImage3D( tar, 0, ifm as gl::types::GLint, wi, hi, di,
					0, pfm, data_type, ptr::null() );
			}else	{
				gl::TexImage2D( tar, 0, ifm as gl::types::GLint, wi, hi,
					0, pfm, data_type, ptr::null() );
			}
		}
		t.levels.borrow_mut().get().total = 1u;
		gl::GetError();	//debug
	}

	#[cfg(multisample)]
	pub fn init_multi( &mut self, pt: &TexturePtr, int_format: gl::types::GLint, fixed_loc: bool )	{
		self.bind( pt );
		let t = pt.borrow();
		assert!( t.samples != 0u && t.levels.borrow().get().total == 0u );
		let (wi,hi,di,si) = (
			t.width as gl::types::GLsizei, t.height	as gl::types::GLsizei,
			t.depth as gl::types::GLsizei, t.samples	as gl::types::GLsizei );
		let fixed = fixed_loc as gl::types::GLboolean;
		let Target(tar) = t.target;
		unsafe{
			if t.depth != 0u	{
				gl::TexImage3DMultisample( tar, si, int_format, wi, hi, di,	fixed );
			}else {
				gl::TexImage2DMultisample( tar, si, int_format, wi, hi,		fixed );
			}
		}
		t.levels.borrow_mut().get().total = 1u;
		gl::GetError();	//debug
	}

	pub fn load_2D<T>( &mut self, pt: &TexturePtr, level: uint, int_format: gl::types::GLint,
			pix_format: gl::types::GLenum, pix_type: gl::types::GLenum, data: &[T])	{
		self.bind( pt );
		let t = pt.borrow();
		gl::PixelStorei( gl::UNPACK_ALIGNMENT, 1 as gl::types::GLint );
		assert!( t.width>0 && t.height>0 && t.samples==0u );
		let mut levs = t.levels.borrow_mut();
		assert!( levs.get().total >= level );
		if levs.get().total==level	{ levs.get().total += 1; }
		let (w,h) = t.get_level_size( level );
		let Target(tar) = t.target;
		unsafe{
			let raw = data.as_ptr() as *gl::types::GLvoid;
			gl::TexImage2D( tar, level as gl::types::GLint, int_format,
				w as gl::types::GLint, h as gl::types::GLint, 0 as gl::types::GLint,
				pix_format, pix_type, raw );
		}
		gl::GetError();	//debug
	}

	pub fn load_sub_2D<T>( &mut self, pt: &TexturePtr, level: uint, r: &frame::Rect,
			pix_format: gl::types::GLenum, pix_type: gl::types::GLenum, data: &[T])	{
		self.bind( pt );
		let t = pt.borrow();
		assert!( t.width>0 && t.height>0 && t.samples==0u );
		assert!( t.levels.borrow().get().total>level );
		assert!( r.w*r.h == data.len() );
		let (w,h) = t.get_level_size( level );
		assert!( frame::Rect::new(w,h).contains_rect( r ) );
		let Target(tar) = t.target;
		unsafe{
			let raw = data.as_ptr() as *gl::types::GLvoid;
			gl::TexSubImage2D( tar, level as gl::types::GLint,
				r.x as gl::types::GLint, r.y as gl::types::GLint,
				r.w as gl::types::GLsizei, r.h as gl::types::GLsizei,
				pix_format, pix_type, raw );
		}
		gl::GetError();	//debug
	}

	pub fn generate_levels( &self, pt: &TexturePtr )	{
		assert!( self.is_bound( pt ) );
		let t = pt.borrow();
		let mut levs = t.levels.borrow_mut();
		let Target(tar) = t.target;
		assert!( t.samples == 0u && levs.get().total > 0u );
		gl::GenerateMipmap( tar );
		levs.get().total = t.count_levels();
	}

	pub fn limit_levels( &self, pt: &TexturePtr, base: uint, max: uint )	{
		assert!( self.is_bound( pt ) );
		assert!( base <= max );
		let t = pt.borrow();
		let mut levs = t.levels.borrow_mut();
		let Target(tar) = t.target;
		if levs.get().min != base	{
			levs.get().min = base;
			gl::TexParameteri( tar, gl::TEXTURE_BASE_LEVEL,	base as gl::types::GLint );
		}
		if levs.get().max != max	{
			levs.get().max = max;
			gl::TexParameteri( tar, gl::TEXTURE_MAX_LEVEL,	max as gl::types::GLint );
		}
	}
}


pub fn map_target( s: &str )-> Target	{
	Target(match s	{
		&"1D"		=> gl::TEXTURE_1D,
		&"Rect"		=> gl::TEXTURE_RECTANGLE,
		&"2D"		=> gl::TEXTURE_2D,
		&"2DArray"	=> gl::TEXTURE_2D_ARRAY,
		//#[cfg(multisample)]
		//&"2DMS"		=> gl::TEXTURE_2D_MULTISAMPLE,
		&"3D"		=> gl::TEXTURE_3D,
		_	=> fail!( "Unable to map texture target {:s}", s )
	})
}


impl context::Context	{
	pub fn create_texture( &self, st:&str, w:uint, h:uint, d:uint, s:uint )-> TexturePtr	{
		let mut hid = 0 as gl::types::GLuint;
		unsafe{ gl::GenTextures( 1, ptr::to_mut_unsafe_ptr(&mut hid) )};
		rc::Rc::new(Texture{
			handle:Handle(hid), target:map_target(st),
			width:w, height:h, depth:d, samples:s,
			levels	: cell::RefCell::new( LevelInfo{total:0u,min:0u,max:1000u} ),
			sampler	: cell::RefCell::new( Sampler::new(3u,1) )
		})
	}
}
