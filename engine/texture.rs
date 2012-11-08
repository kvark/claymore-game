extern mod glcore;

pub enum Handle	= glcore::GLuint;
pub enum Target	= glcore::GLenum;


struct Texture	{
	handle	: Handle,
	target	: Target,
	width		: uint,
	height		: uint,
	depth		: uint,
	mut levels	: uint,

	drop	{
		unsafe	{
			// assert: not bound
			glcore::glDeleteTextures( 1, ptr::addr_of(&*self.handle) );
		}
	}
}


pub struct Slot	{
	unit	: uint,
	target	: Target
}

//FIXME: remove when Rust derives that automatically
impl Handle : cmp::Eq	{
	pure fn eq( v : &Handle )->bool	{ self == *v }
	pure fn ne( v : &Handle )->bool	{ !self.eq(v) }
}
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
	mut active		: send_map::linear::LinearMap<Slot,Handle>
}


impl Binding	{
	priv fn _bind( target : Target, h : Handle )	{
		let slot = Slot{ unit:self.active_unit, target:target };
		if self.active.swap( slot, h ) != Some(h)	{
			glcore::glBindTexture( *target, *h );
		}
	}
	priv fn _switch( unit : uint )	{
		if self.active_unit != unit	{
			self.active_unit = unit;
			glcore::glActiveTexture( glcore::GL_TEXTURE0 + (unit as glcore::GLenum) );
		}
	}

	fn bind( unit: uint, t : &Texture )	{
		self._switch( unit );
		self._bind( t.target, t.handle );
	}
	fn unbind( target : Target )	{
		self._bind( target, Handle(0) );
	}

	fn find( t : &Texture )-> int	{
		for self.active.each |slot,handle|	{
			if *handle == t.handle	{
				assert *slot.target == *t.target;
				return slot.unit as int;
			}
		}
		return -1;
	}

	fn init_2D( t : &Texture, int_format : glcore::GLint, alpha : bool, num_levels : uint )	{
		assert num_levels == 1u;
		let mut w = t.width, h = t.height;
		let pix_format = if alpha {glcore::GL_RGBA} else {glcore::GL_RGB};
		t.levels = 0;
		while num_levels>0	{
			glcore::glTexImage2D( *t.target, t.levels as glcore::GLint, int_format,
				t.width as glcore::GLsizei, t.height as glcore::GLsizei, 0 as glcore::GLint,
				pix_format, glcore::GL_UNSIGNED_BYTE, ptr::null() );
			t.levels += 1;
			w = (w+1)>>1;
			h = (h+1)>>1;
		}
	}
}


impl context::Context	{
	fn create_texture( t:Target, w:uint, h:uint, d:uint )->Texture	{
		let mut hid = 0 as glcore::GLuint;
		unsafe	{
			glcore::glGenTextures( 1, ptr::addr_of(&hid) );
		}
		Texture{ handle:Handle(hid), target:t, width:w, height:h, depth:d, levels:0 }
	}
}