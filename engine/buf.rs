extern mod glcore;

type Handle	= glcore::GLuint;


struct Object	{
	handle : Handle,

	drop	{
		unsafe	{
			// assert: not bound
			glcore::glDeleteBuffers( 1, ptr::addr_of(&self.handle) );
		}
	}
}


pub struct Binding	{
	target		: glcore::GLenum,
	mut active	: Handle,
}

impl Binding	{
	priv fn _bind( h : Handle )	{
		if self.active != h	{
			self.active = h;
			glcore::glBindBuffer( self.target, h );
		}
	}
	fn bind( object : &Object )	{ self._bind(object.handle) }
	fn unbind()	{ self._bind(0 as Handle) }
	
	fn allocate( size : uint, dynamic : bool )	{
		let usage = if dynamic {glcore::GL_STATIC_DRAW} else {glcore::GL_DYNAMIC_DRAW};
		glcore::glBufferData( self.target, size as glcore::GLsizeiptr, ptr::null(), usage );
	}

	fn load<T>( data : ~[T], dynamic : bool )	{
		let usage = if dynamic {glcore::GL_STATIC_DRAW} else {glcore::GL_DYNAMIC_DRAW};
		let size = data.len() * sys::size_of::<T>() as glcore::GLsizeiptr;
		unsafe	{
			let raw = vec::raw::to_ptr(data) as *libc::c_void;
			glcore::glBufferData( self.target, size, raw, usage );
		}
	}
}


impl context::Context	{
	fn create_buffer()->Object	{
		let mut h = 0 as Handle;
		unsafe	{
			glcore::glGenBuffers( 1, ptr::addr_of(&h) );
		}
		Object{ handle:h }
	}

	fn create_buffer_sized( size : uint )->Object	{
		let obj = self.create_buffer();
		let slot = &self.buffer_array;
		slot.bind( &obj );
		slot.allocate( size, true );
		slot.unbind();
		obj
	}

	fn create_buffer_loaded<T>( data : ~[T] )->Object	{
		let obj = self.create_buffer();
		let slot = &self.buffer_array;
		slot.bind( &obj );
		slot.load( data, false );
		slot.unbind();
		obj
	}
}