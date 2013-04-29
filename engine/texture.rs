extern mod glcore;

use core::cmp::Eq;
use core::hashmap::linear::LinearMap;
use core::to_bytes;
use core::to_str::ToStr;

use context;
use frame;


pub type Mode	= glcore::GLenum;
#[deriving(Eq)]
pub struct Handle( glcore::GLuint );
#[deriving(Eq)]
pub struct Target( glcore::GLenum );


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


pub struct Texture	{
	handle	: Handle,
	target	: Target,
	width		: uint,
	height		: uint,
	depth		: uint,
	samples		: uint,
	priv levels		: uint,
	priv sampler	: @mut Sampler,
	priv level_base	: uint,
	priv level_max	: uint,
	priv pool	: @mut ~[Handle],
}

#[unsafe_destructor]
impl Drop for Texture	{
	fn finalize( &self )	{
		self.pool.push( self.handle );
	}
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
	fn get_levels( &self )-> uint	{
		self.levels
	}
	fn get_level_size( &self, lev : uint )-> (uint,uint)	{
		assert!( self.width>0u && self.height>0u );
		(uint::max(1u,self.width>>lev),uint::max(1u,self.height>>lev))
	}
	fn get_level_limits( &self )-> (uint,uint)	{
		(self.level_base, self.level_max)
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
		self.samples==0u && (!self.is_filtering_mapmap() || self.levels==1u)
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
	active_unit	: uint,
	active		: LinearMap<Slot,Handle>,
	priv pool	: @mut ~[Handle],
}

impl context::ProxyState for Binding	{
	fn sync_back( &mut self )->bool	{
		let mut was_ok = true;
		let mut id = 0 as glcore::GLint;
		glcore::glGetIntegerv( glcore::GL_ACTIVE_TEXTURE, ptr::addr_of(&id) );
		let cur_unit = id as uint - (glcore::GL_TEXTURE0 as uint);
		if self.active_unit != cur_unit	{
			was_ok = false;
			self.active_unit = cur_unit;
		}
		// Rust wouldn't allow us to mutate while scanning
		for (copy self.active).each |&(slot,handle)|	{
			let t = *slot.target;
			let query =	if t == glcore::GL_TEXTURE_2D	{glcore::GL_TEXTURE_BINDING_2D}
			else	{
				fail!(fmt!( "Unkown binding %d", *slot.target as int ));
			};
			self.switch( slot.unit );
			glcore::glGetIntegerv( query, ptr::addr_of(&id) );
			if *(*handle) != id as glcore::GLuint	{
				io::println("bad2");
				was_ok = false;
				self.active.swap( *slot, Handle(id as glcore::GLuint) );
			}
		}
		self.switch( cur_unit );
		was_ok
	}
}

pub impl Binding	{
	fn new()-> Binding	{
		let slots	= LinearMap::new();
		Binding{ active_unit:0u, active:slots, pool:@mut ~[] }
	}

	priv fn _bind( &mut self, target : Target, h : Handle )	{
		let slot = Slot{ unit:self.active_unit, target:target };
		if self.active.contains_key(&slot) && *self.active.get(&slot) == h	{
			return;
		}
		self.active.insert( slot, h );
		glcore::glBindTexture( *target, *h );
	}
	
	fn switch( &mut self, unit : uint )	{
		if self.active_unit != unit	{
			self.active_unit = unit;
			glcore::glActiveTexture( glcore::GL_TEXTURE0 + (unit as glcore::GLenum) );
		}
	}

	fn bind( &mut self, t : &Texture )	{
		self._bind( t.target, t.handle );
	}

	fn bind_to( &mut self, unit: uint, t : &Texture )	{
		self.switch( unit );
		self.bind( t );
	}

	fn bind_sampler( &mut self, t : &Texture, s : &Sampler )	{
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
		self._bind( target, Handle(0) );
	}

	fn get_bound( &self, target : Target )->Handle	{
		let slot = Slot{ unit:self.active_unit, target:target };
		match self.active.find( &slot )	{
			Some(s)	=> *s,
			None	=> Handle(0)
		}
	}

	fn is_bound( &self, t : &Texture )-> bool	{
		self.get_bound( t.target ) == t.handle
	}

	fn find( &self, t : &Texture )-> int	{
		for self.active.each |&(slot,handle)|	{
			if *handle == t.handle	{
				assert!( slot.target == t.target );
				return slot.unit as int;
			}
		}
		-1
	}

	fn init( &mut self, t : &mut Texture, num_levels : uint, int_format : glcore::GLint, alpha : bool )	{
		self.bind( t );
		assert!( t.samples == 0u && (t.depth == 0u || num_levels == 1u) );
		t.levels = 0u;
		while t.levels<num_levels	{
			let (w,h) = t.get_level_size( t.levels );
			let (wi,hi,di) = ( w as glcore::GLsizei, h as glcore::GLsizei, t.depth as glcore::GLsizei );
			let pix_format = if alpha {glcore::GL_RGBA} else {glcore::GL_RGB};
			let data_type = glcore::GL_UNSIGNED_BYTE;
			let li = t.levels as glcore::GLint;
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
			t.levels += 1u;
		}
		glcore::glGetError();	//debug
	}

	fn init_depth( &mut self, t : &mut Texture, stencil : bool )	{
		self.bind( t );
		assert!( t.samples == 0u && t.levels == 0u );
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
		t.levels = 1u;
		glcore::glGetError();	//debug
	}

	#[cfg(multisample)]
	fn init_multi( &mut self, t : &mut Texture, int_format : glcore::GLint, fixed_loc : bool )	{
		self.bind( t );
		assert!( t.samples != 0u && t.levels == 0u );
		let (wi,hi,di,si) = (
			t.width as glcore::GLsizei, t.height	as glcore::GLsizei,
			t.depth as glcore::GLsizei, t.samples	as glcore::GLsizei );
		let fixed = fixed_loc as glcore::GLboolean;
		if t.depth != 0u	{
			glcore::glTexImage3DMultisample( *t.target, si, int_format, wi, hi, di,	fixed );
		}else {
			glcore::glTexImage2DMultisample( *t.target, si, int_format, wi, hi,		fixed );
		}
		t.levels = 1u;
		glcore::glGetError();	//debug
	}

	fn load_2D<T>( &mut self, t : &mut Texture, level : uint, int_format : glcore::GLint,
			pix_format : glcore::GLenum, pix_type : glcore::GLenum, data : &const ~[T])	{
		self.bind( t );
		glcore::glPixelStorei( glcore::GL_UNPACK_ALIGNMENT, 1 as glcore::GLint );
		assert!( t.width>0 && t.height>0 && t.samples==0u );
		assert!( t.levels >= level );
		if t.levels==level	{ t.levels+=1; }
		let (w,h) = t.get_level_size( level );
		let raw = unsafe{vec::raw::to_ptr(*data)} as *glcore::GLvoid;
		glcore::glTexImage2D( *t.target, level as glcore::GLint, int_format,
			w as glcore::GLint, h as glcore::GLint, 0 as glcore::GLint,
			pix_format, pix_type, raw );
		glcore::glGetError();	//debug
	}

	fn load_sub_2D<T>( &mut self, t : &mut Texture, level : uint, r : &frame::Rect,
			pix_format : glcore::GLenum, pix_type : glcore::GLenum, data : &~[T])	{
		self.bind( t );
		assert!( t.width>0 && t.height>0 && t.samples==0u && t.levels>level );
		assert!( r.w*r.h == data.len() );
		let (w,h) = t.get_level_size( level );
		assert!( frame::Rect::new(w,h).contains_rect( r ) );
		let raw = unsafe{vec::raw::to_ptr(*data)} as *glcore::GLvoid;
		glcore::glTexSubImage2D( *t.target, level as glcore::GLint,
			r.x as glcore::GLint, r.y as glcore::GLint,
			r.w as glcore::GLsizei, r.h as glcore::GLsizei,
			pix_format, pix_type, raw );
		glcore::glGetError();	//debug
	}

	fn generate_levels( &self, t : &mut Texture )-> uint	{
		assert!( self.is_bound( t ) );
		assert!( t.samples == 0u && t.levels > 0u );
		glcore::glGenerateMipmap( *t.target );
		t.levels = t.count_levels();
		t.levels
	}

	fn limit_levels( &self, t : &mut Texture, base : uint, max : uint )	{
		assert!( self.is_bound( t ) );
		assert!( base <= max );
		if t.level_base != base	{
			t.level_base = base;
			glcore::glTexParameteri( *t.target, glcore::GL_TEXTURE_BASE_LEVEL, base as glcore::GLint );
		}
		if t.level_max != max	{
			t.level_max = max;
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
	fn create_texture( &self, st:~str, w:uint, h:uint, d:uint, s:uint )->Texture	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenTextures( 1, ptr::addr_of(&hid) );
		Texture{ handle:Handle(hid), target:map_target(st),
			width:w, height:h, depth:d, samples:s,
			levels:0, sampler:@mut Sampler::new(3u,1),
			level_base:0u, level_max:1000u,
			pool:self.texture.pool }
	}
	fn cleanup_textures( &mut self, lg : &context::Log )	{
		let pool : &mut ~[Handle] = self.texture.pool;
		while !pool.is_empty()	{
			let han = pool.pop();
			assert!( *han != 0 );
			for (copy self.texture.active).each() |&(s,h)|	{
				if han == *h	{
					self.texture.switch( s.unit );
					self.texture.unbind( s.target );
				}
			}
			lg.add(fmt!( "Deleting texture id %d", *han as int ));
			glcore::glDeleteTextures( 1, ptr::addr_of(&*han) );
		}
	}
}