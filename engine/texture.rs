extern mod glcore;

pub enum Handle	= glcore::GLuint;
pub enum Target	= glcore::GLenum;


pub struct Texture	{
	handle	: Handle,
	target	: Target,
	width		: uint,
	height		: uint,
	depth		: uint,
	mut levels	: uint,
	samples		: uint,
	priv mut level_base	: uint,
	priv mut level_max	: uint,
	priv pool	: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
	}
}

impl Texture	{
	pure fn get_level_size( lev : uint )-> (uint,uint)	{
		assert self.width>0u && self.height>0u && lev<self.levels;
		(((self.width-1u)>>lev)+1u,((self.height-1u)>>lev)+1u)
	}
	pure fn get_level_limits()-> (uint,uint)	{
		(self.level_base, self.level_max)
	}
}

impl Texture : context::State	{
	fn sync_back()->bool	{
		//FIXME
		true
	}
}


pub struct Slot	{
	unit	: uint,
	target	: Target
}

//FIXME: waiting for Rust to do that automatically
impl Slot : to_bytes::IterBytes	{
	pure fn iter_bytes( lsb0 : bool, f : to_bytes::Cb)	{
		self.unit.iter_bytes( lsb0, f );
		(*self.target).iter_bytes( lsb0, f );
	}
}
impl Slot : cmp::Eq	{
	pure fn eq( v : &Slot )->bool	{
		self.unit == v.unit && *self.target == *v.target
	}
	pure fn ne( v : &Slot )->bool	{ !self.eq(v) }
}


pub struct Binding	{
	mut active_unit	: uint,
	mut active		: send_map::linear::LinearMap<Slot,Handle>,
	priv pool		: @mut ~[Handle],
}


impl Binding	{
	priv fn _bind( target : Target, h : Handle )	{
		let slot = Slot{ unit:self.active_unit, target:target };
		if self.active.contains_key(&slot) && *self.active.get(&slot) == *h	{
			return;
		}
		self.active.insert( slot, h );
		glcore::glBindTexture( *target, *h );
	}
	
	fn switch( unit : uint )	{
		if self.active_unit != unit	{
			self.active_unit = unit;
			glcore::glActiveTexture( glcore::GL_TEXTURE0 + (unit as glcore::GLenum) );
		}
	}

	fn bind( t : &Texture )	{
		self._bind( t.target, t.handle );
	}
	fn bind_to( unit: uint, t : &Texture )	{
		self.switch( unit );
		self.bind( t );
	}
	fn unbind( target : Target )	{
		self._bind( target, Handle(0) );
	}

	pure fn get_bound( target : Target )->Handle	{
		let slot = Slot{ unit:self.active_unit, target:target };
		match self.active.find( &slot )	{
			Some(s)	=> s,
			None	=> Handle(0)
		}
	}

	pure fn is_bound( t : &Texture )-> bool	{
		*self.get_bound( t.target ) == *t.handle
	}

	pure fn find( t : &Texture )-> int	{
		for self.active.each |slot,handle|	{
			if *(*handle) == *t.handle	{
				assert *slot.target == *t.target;
				return slot.unit as int;
			}
		}
		return -1;
	}

	fn init_2D( t : &Texture, num_levels : uint, int_format : glcore::GLint, alpha_or_fixed_loc : bool )	{
		self.bind( t );
		assert t.samples == 0u || num_levels == 1u;
		t.levels = 0;
		while t.levels<num_levels	{
			t.levels += 1;
			let (w,h) = t.get_level_size( num_levels-1u );
			if t.samples != 0u	{
				glcore::glTexImage2DMultisample( *t.target, t.samples as glcore::GLsizei, int_format,
					w as glcore::GLsizei, h as glcore::GLsizei, alpha_or_fixed_loc as glcore::GLboolean );
			}else	{
				let pix_format = if alpha_or_fixed_loc {glcore::GL_RGBA} else {glcore::GL_RGB};
				glcore::glTexImage2D( *t.target, t.levels as glcore::GLint, int_format,
					w as glcore::GLsizei, h as glcore::GLsizei, 0 as glcore::GLint,
					pix_format, glcore::GL_UNSIGNED_BYTE, ptr::null() );
			}
		}
	}

	fn load_2D<T>(	t : &Texture, level : uint, int_format : glcore::GLint,
					pix_format : glcore::GLenum, pix_type : glcore::GLenum, data : &[T])	{
		self.bind( t );
		assert t.width>0 && t.height>0 && t.samples==0u;
		let w = (((t.width-1)>>level)+1)	as glcore::GLsizei;
		let h = (((t.height-1)>>level)+1)	as glcore::GLsizei;
		unsafe	{
			let raw = vec::raw::to_ptr(data) as *glcore::GLvoid;
			glcore::glTexImage2D( *t.target, level as glcore::GLint, int_format,
				w, h, 0 as glcore::GLint, pix_format, pix_type, raw );
		}
		if t.levels==0	{ t.levels=1; }
	}

	fn wrap( t : &Texture, method : int )	{
		assert self.is_bound( t );
		assert t.samples == 0u;
		let wr =	if method>0	{glcore::GL_REPEAT}
			else	if method<0 {glcore::GL_MIRRORED_REPEAT}
			else				{glcore::GL_CLAMP_TO_EDGE}
			as glcore::GLint;
		glcore::glTexParameteri( *t.target, glcore::GL_TEXTURE_WRAP_S, wr );
		glcore::glTexParameteri( *t.target, glcore::GL_TEXTURE_WRAP_T, wr );
		glcore::glTexParameteri( *t.target, glcore::GL_TEXTURE_WRAP_R, wr );
	}

	fn filter( t : &Texture, dim : uint )	{
		assert self.is_bound( t );
		assert t.samples == 0u;
		let min_filter =	if dim==3u	{glcore::GL_LINEAR_MIPMAP_LINEAR}
			else			if dim==2u	{glcore::GL_LINEAR}
			else						{glcore::GL_POINT}
			as glcore::GLint;
		let mag_filter =	if dim>1u	{glcore::GL_LINEAR}
			else						{glcore::GL_POINT}
			as glcore::GLint;
		glcore::glTexParameteri( *t.target, glcore::GL_TEXTURE_MIN_FILTER, min_filter );
		glcore::glTexParameteri( *t.target, glcore::GL_TEXTURE_MAG_FILTER, mag_filter );
	}

	fn limit_levels( t : &Texture, base : uint, max : uint )	{
		assert self.is_bound( t );
		assert base <= max;
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


impl Binding : context::State	{
	fn sync_back()->bool	{
		let mut was_ok = true;
		let mut id = 0 as glcore::GLint;
		unsafe	{
			glcore::glGetIntegerv( glcore::GL_ACTIVE_TEXTURE, ptr::addr_of(&id) );
		}
		let cur_unit = id as uint - (glcore::GL_TEXTURE0 as uint);
		if self.active_unit != cur_unit	{
			was_ok = false;
			self.active_unit = cur_unit;
		}
		// Rust wouldn't allow us to mutate while scanning
		for (copy self.active).each |slot,handle|	{
			let t = *slot.target;
			let query =	if t == glcore::GL_TEXTURE_2D	{glcore::GL_TEXTURE_BINDING_2D}
			else	{
				fail(fmt!( "Unkown binding %d", *slot.target as int ));
			};
			self.switch( slot.unit );
			unsafe	{
				glcore::glGetIntegerv( query, ptr::addr_of(&id) );		
			}
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


pub fn create_binding()-> Binding	{
	let slots	= send_map::linear::LinearMap::<texture::Slot,texture::Handle>();
	Binding{ active_unit:0u, active:slots, pool:@mut ~[] }
}


impl context::Context	{
	fn create_texture( t:glcore::GLenum, w:uint, h:uint, d:uint, s:uint )->Texture	{
		let mut hid = 0 as glcore::GLuint;
		unsafe	{
			glcore::glGenTextures( 1, ptr::addr_of(&hid) );
		}
		Texture{ handle:Handle(hid), target:Target(t),
			width:w, height:h, depth:d, levels:0, samples:s,
			level_base:0u, level_max:1000u,
			pool:self.texture.pool }
	}
	fn cleanup_textures()	{
		while self.texture.pool.len()!=0	{
			let han = self.texture.pool.pop();
			assert *han != 0;
			for (copy self.texture.active).each() |s,h|	{
				if *han == **h	{
					self.texture.switch( s.unit );
					self.texture.unbind( s.target );
				}
			}
			unsafe	{
				glcore::glDeleteTextures( 1, ptr::addr_of(&*han) );
			}
		}
	}
}