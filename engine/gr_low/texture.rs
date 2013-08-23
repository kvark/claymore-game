extern mod glcore;

use core::cmp::Eq;
use core::hashmap::linear::LinearMap;
use core::managed;
use core::to_bytes;
use core::to_str::ToStr;

use gr_low::{context,frame};


pub type Mode	= glcore::GLenum;
#[deriving(Eq)]
pub struct Handle( glcore::GLuint );
#[deriving(Eq)]
pub struct Target( glcore::GLenum );

impl Drop for Handle	{
	fn finalize( &self )	{
		glcore::glDeleteTextures( 1, ptr::addr_of(&**self) );
	}
}


pub struct Sampler	{
	filter	: [glcore::GLint, ..2],
	wrap	: [glcore::GLint, ..3],
	compare	: Option<glcore::GLenum>,
}

pub impl Sampler	{
	fn new( filter : uint, wrap : int )-> Sampler	{
		assert!( filter>0u && filter<=3u );
		let min_filter =	if filter==3u	{glcore::GL_LINEAR_MIPMAP_LINEAR}
			else			if filter==2u	{glcore::GL_LINEAR}
			else							{glcore::GL_NEAREST}
			as glcore::GLint;
		let mag_filter =	if filter>1u	{glcore::GL_LINEAR}
			else							{glcore::GL_NEAREST}
			as glcore::GLint;
		let wr =	if wrap>0	{glcore::GL_REPEAT}
			else	if wrap<0	{glcore::GL_MIRRORED_REPEAT}
			else				{glcore::GL_CLAMP_TO_EDGE}
			as glcore::GLint;
		Sampler{
			filter	: [min_filter,mag_filter],
			wrap	: [wr,wr,wr],
			compare	: None,
		}
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
		fmt!( "Texture(h=%d, %ux%ux%u, samples=%u)", *self.handle as int,
			self.width, self.height, self.depth, self.samples )
	}
}

pub impl Texture	{
	fn get_num_levels( &self )-> uint	{
		self.levels.total
	}
	fn get_level_size( &self, lev : uint )-> (uint,uint)	{
		assert!( self.width>0u && self.height>0u );
		(uint::max(1u,self.width>>lev),uint::max(1u,self.height>>lev))
	}
	fn get_level_limits( &self )-> (uint,uint)	{
		(self.levels.min, self.levels.max)
	}
	fn count_levels( &self )-> uint	{
		let mut i = 0;
		while self.get_level_size(i) != (1u,1u)	{
			i += 1;
		}
		i
	}
	fn is_filtering_mapmap( &self )-> bool	{
		[glcore::GL_LINEAR_MIPMAP_LINEAR,glcore::GL_NEAREST_MIPMAP_NEAREST,
		glcore::GL_LINEAR_MIPMAP_NEAREST,glcore::GL_NEAREST_MIPMAP_LINEAR].
		contains(&(self.sampler.filter[1] as glcore::GLenum))
	}
	fn can_sample( &self )-> bool	{
		self.samples==0u && (!self.is_filtering_mapmap() || self.levels.total==1u)
	}
}


pub fn map_int_format( s : ~str )-> glcore::GLint	{
	(match s	{
		~"r8"		=> glcore::GL_R8,
		~"rgba8"	=> glcore::GL_RGBA8,
		~"rgba16f"	=> glcore::GL_RGBA16F,
		_	=> fail!(fmt!( "Can not recognize texture internal format %s",s ))
	}) as glcore::GLint
}

pub fn map_pix_format( s : ~str )-> glcore::GLenum	{
	match s	{
		~"red"	=> glcore::GL_RED,
		~"rg"	=> glcore::GL_RG,
		~"rgb"	=> glcore::GL_RGB,
		~"bgr"	=> glcore::GL_BGR,
		~"rgba"	=> glcore::GL_RGBA,
		~"bgra"	=> glcore::GL_RGBA,
		~"depth"=> glcore::GL_DEPTH_COMPONENT,
		~"ds"	=> glcore::GL_DEPTH_STENCIL,
		_	=> fail!(fmt!( "Can not recognize texture pixel format %s",s ))
	}
}


//#[deriving(IterBytes)]
#[deriving(Eq)]
pub struct Slot	{
	unit	: uint,
	target	: Target
}

impl to_bytes::IterBytes for Slot	{
	fn iter_bytes( &self, lsb0 : bool, f : to_bytes::Cb)	{
		self.unit.iter_bytes( lsb0, f );
		(*self.target).iter_bytes( lsb0, f );
	}
}


pub struct Binding	{
	unit	: uint,
	active	: LinearMap<Slot,@Texture>,
}

impl context::ProxyState for Binding	{
	fn sync_back( &mut self )->bool	{
		let mut was_ok = true;
		let mut id = 0 as glcore::GLint;
		glcore::glGetIntegerv( glcore::GL_ACTIVE_TEXTURE, ptr::addr_of(&id) );
		let cur_unit = id as uint - (glcore::GL_TEXTURE0 as uint);
		if self.unit != cur_unit	{
			was_ok = false;
			self.unit = cur_unit;
		}
		// Rust wouldn't allow us to mutate while scanning
		for (copy self.active).each |&(slot,tex)|	{
			let t = *slot.target;
			let query =	if t == glcore::GL_TEXTURE_2D	{glcore::GL_TEXTURE_BINDING_2D}
			else	{
				fail!(fmt!( "Unkown binding %d", *slot.target as int ));
			};
			self.switch( slot.unit );
			glcore::glGetIntegerv( query, ptr::addr_of(&id) );
			if *tex.handle != id as glcore::GLuint	{
				io::println("bad2");
				was_ok = false;
				self.active.remove( slot );
			}
		}
		self.switch( cur_unit );
		was_ok
	}
}

pub impl Binding	{
	fn new()-> Binding	{
		Binding{
			unit	: 0u,
			active	: LinearMap::new()
		}
	}

	fn bind( &mut self, t : @Texture )	{
		let slot = Slot{ unit:self.unit, target:t.target };
		if !self.active.contains_key(&slot) || !managed::ptr_eq(*self.active.get(&slot),t)	{
			self.active.insert( slot, t );
			glcore::glBindTexture( *t.target, *t.handle );
		}
	}
	
	fn switch( &mut self, unit : uint )	{
		if self.unit != unit	{
			self.unit = unit;
			glcore::glActiveTexture( glcore::GL_TEXTURE0 + (unit as glcore::GLenum) );
		}
	}

	fn bind_to( &mut self, unit: uint, t : @Texture )	{
		self.switch( unit );
		self.bind( t );
	}

	fn bind_sampler( &mut self, t : @Texture, s : &Sampler )	{
		self.bind( t );
		if t.samples != 0	{return}	//TODO: error here?
		let filter_modes = [glcore::GL_TEXTURE_MIN_FILTER,glcore::GL_TEXTURE_MAG_FILTER];
		for [0,1].each() |i|	{
			if t.sampler.filter[*i] != s.filter[*i]	{
				glcore::glTexParameteri( *t.target, filter_modes[*i], s.filter[*i] );
			}
		}
		let wrap_modes = [glcore::GL_TEXTURE_WRAP_S,glcore::GL_TEXTURE_WRAP_T,glcore::GL_TEXTURE_WRAP_R];
		for [0,1,2].each() |i|	{
			if t.sampler.wrap[*i] != s.wrap[*i]	{
				glcore::glTexParameteri( *t.target, wrap_modes[*i], s.wrap[*i] );
			}
		}
		if t.sampler.compare != s.compare	{
			let e_mode = glcore::GL_TEXTURE_COMPARE_MODE;
			let e_func = glcore::GL_TEXTURE_COMPARE_FUNC;
			match s.compare	{
				Some(mode)	=>	{
					glcore::glTexParameteri( *t.target, e_mode,
						glcore::GL_COMPARE_REF_TO_TEXTURE as glcore::GLint );
					glcore::glTexParameteri( *t.target, e_func, mode as glcore::GLint );
				},
				None	=>	{
					glcore::glTexParameteri( *t.target, e_mode,
						glcore::GL_NONE as glcore::GLint );
				}
			}
		}
		*t.sampler = *s;
	}
	fn unbind( &mut self, target : Target )	{
		let slot = Slot{ unit:self.unit, target:target };
		if self.active.find(&slot).is_some()	{
			self.active.remove( &slot );
			glcore::glBindTexture( *target, 0 );
		}
	}

	fn get_bound( &self, target : Target )-> Option<@Texture>	{
		let slot = Slot{ unit:self.unit, target:target };
		match self.active.find( &slot )	{
			Some(t)	=> Some(*t),
			None	=> None
		}
	}

	fn is_bound( &self, t : @Texture )-> bool	{
		match self.get_bound( t.target )	{
			Some(tex)	=> managed::ptr_eq( tex, t ),
			None		=> false
		}
	}

	fn find( &self, t : @Texture )-> Option<uint>	{
		for self.active.each |&(slot,tex)|	{
			if managed::ptr_eq(t,*tex)	{
				assert!( slot.target == t.target );
				return Some(slot.unit)
			}
		}
		None
	}

	fn init( &mut self, t : @Texture, num_levels : uint, int_format : glcore::GLint, alpha : bool )	{
		self.bind( t );
		assert!( t.samples == 0u && (t.depth == 0u || num_levels == 1u) );
		let mut level = 0u;
		while level<num_levels	{
			let (w,h) = t.get_level_size( level );
			let (wi,hi,di) = ( w as glcore::GLsizei, h as glcore::GLsizei, t.depth as glcore::GLsizei );
			let pix_format = if alpha {glcore::GL_RGBA} else {glcore::GL_RGB};
			let data_type = glcore::GL_UNSIGNED_BYTE;
			let li = level as glcore::GLint;
			if t.depth != 0u	{
				glcore::glTexImage3D( *t.target, li, int_format, wi, hi, di,
					0, pix_format, data_type, ptr::null() );
			}else if t.height != 0u	{
				glcore::glTexImage2D( *t.target, li, int_format, wi, hi,
					0, pix_format, data_type, ptr::null() );
			}else	{
				glcore::glTexImage1D( *t.target, li, int_format, wi,
					0, pix_format, data_type, ptr::null() );
			}
			level += 1u;
		}
		t.levels.total = level;
		glcore::glGetError();	//debug
	}

	fn init_depth( &mut self, t : @Texture, stencil : bool )	{
		self.bind( t );
		assert!( t.samples == 0u && t.levels.total == 0u );
		let (wi,hi,di) = ( t.width as glcore::GLsizei, t.height	as glcore::GLsizei, t.depth as glcore::GLsizei );
		let (ifm,pfm)	= if stencil { (glcore::GL_DEPTH24_STENCIL8, glcore::GL_DEPTH_STENCIL) }
			else { (glcore::GL_DEPTH_COMPONENT16, glcore::GL_DEPTH_COMPONENT) };
		let data_type = glcore::GL_UNSIGNED_BYTE;
		if t.depth != 0u	{
			glcore::glTexImage3D( *t.target, 0, ifm as glcore::GLint, wi, hi, di,
				0, pfm, data_type, ptr::null() );
		}else	{
			glcore::glTexImage2D( *t.target, 0, ifm as glcore::GLint, wi, hi,
				0, pfm, data_type, ptr::null() );
		}
		t.levels.total = 1u;
		glcore::glGetError();	//debug
	}

	#[cfg(multisample)]
	fn init_multi( &mut self, t : @Texture, int_format : glcore::GLint, fixed_loc : bool )	{
		self.bind( t );
		assert!( t.samples != 0u && t.levels.total == 0u );
		let (wi,hi,di,si) = (
			t.width as glcore::GLsizei, t.height	as glcore::GLsizei,
			t.depth as glcore::GLsizei, t.samples	as glcore::GLsizei );
		let fixed = fixed_loc as glcore::GLboolean;
		if t.depth != 0u	{
			glcore::glTexImage3DMultisample( *t.target, si, int_format, wi, hi, di,	fixed );
		}else {
			glcore::glTexImage2DMultisample( *t.target, si, int_format, wi, hi,		fixed );
		}
		t.levels.total = 1u;
		glcore::glGetError();	//debug
	}

	fn load_2D<T>( &mut self, t : @Texture, level : uint, int_format : glcore::GLint,
			pix_format : glcore::GLenum, pix_type : glcore::GLenum, data : &const ~[T])	{
		self.bind( t );
		glcore::glPixelStorei( glcore::GL_UNPACK_ALIGNMENT, 1 as glcore::GLint );
		assert!( t.width>0 && t.height>0 && t.samples==0u );
		assert!( t.levels.total >= level );
		if t.levels.total==level	{ t.levels.total += 1; }
		let (w,h) = t.get_level_size( level );
		let raw = unsafe{vec::raw::to_ptr(*data)} as *glcore::GLvoid;
		glcore::glTexImage2D( *t.target, level as glcore::GLint, int_format,
			w as glcore::GLint, h as glcore::GLint, 0 as glcore::GLint,
			pix_format, pix_type, raw );
		glcore::glGetError();	//debug
	}

	fn load_sub_2D<T>( &mut self, t : @Texture, level : uint, r : &frame::Rect,
			pix_format : glcore::GLenum, pix_type : glcore::GLenum, data : &[T])	{
		self.bind( t );
		assert!( t.width>0 && t.height>0 && t.samples==0u && t.levels.total>level );
		assert!( r.w*r.h == data.len() );
		let (w,h) = t.get_level_size( level );
		assert!( frame::Rect::new(w,h).contains_rect( r ) );
		let raw = unsafe{vec::raw::to_ptr(data)} as *glcore::GLvoid;
		glcore::glTexSubImage2D( *t.target, level as glcore::GLint,
			r.x as glcore::GLint, r.y as glcore::GLint,
			r.w as glcore::GLsizei, r.h as glcore::GLsizei,
			pix_format, pix_type, raw );
		glcore::glGetError();	//debug
	}

	fn generate_levels( &self, t : @Texture )	{
		assert!( self.is_bound( t ) );
		assert!( t.samples == 0u && t.levels.total > 0u );
		glcore::glGenerateMipmap( *t.target );
		t.levels.total = t.count_levels();
	}

	fn limit_levels( &self, t : @Texture, base : uint, max : uint )	{
		assert!( self.is_bound( t ) );
		assert!( base <= max );
		if t.levels.min != base	{
			t.levels.min = base;
			glcore::glTexParameteri( *t.target, glcore::GL_TEXTURE_BASE_LEVEL, base as glcore::GLint );
		}
		if t.levels.max != max	{
			t.levels.max = max;
			glcore::glTexParameteri( *t.target, glcore::GL_TEXTURE_MAX_LEVEL, max as glcore::GLint );
		}
	}
}


pub fn map_target( s : ~str )-> Target	{
	Target(match s	{
		~"1D"		=> glcore::GL_TEXTURE_1D,
		~"Rect"		=> glcore::GL_TEXTURE_RECTANGLE,
		~"2D"		=> glcore::GL_TEXTURE_2D,
		~"2DArray"	=> glcore::GL_TEXTURE_2D_ARRAY,
		//~"2DMS"		=> glcore::GL_TEXTURE_2D_MULTISAMPLE,	//TEMP!
		~"3D"		=> glcore::GL_TEXTURE_3D,
		_	=> fail!(fmt!( "Unable to map texture target %s", s ))
	})
}


pub impl context::Context	{
	fn create_texture( &self, st:~str, w:uint, h:uint, d:uint, s:uint )-> @Texture	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenTextures( 1, ptr::addr_of(&hid) );
		@Texture{ handle:Handle(hid), target:map_target(st),
			width:w, height:h, depth:d, samples:s,
			levels:@mut LevelInfo{total:0u,min:0u,max:1000u},
			sampler:@mut Sampler::new(3u,1) }
	}
}