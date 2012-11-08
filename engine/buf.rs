extern mod glcore;

pub enum Handle	= glcore::GLuint;
pub enum Target = glcore::GLenum;


struct Object	{
	handle : Handle,

	drop	{
		unsafe	{
			// assert: not bound
			glcore::glDeleteBuffers( 1, ptr::addr_of(&*self.handle) );
		}
	}
}


pub struct Binding	{
	target		: Target,
	mut active	: Handle,
}

impl Binding : context::State	{
	fn sync_back()->bool	{
		let query =
			if *self.target == glcore::GL_ARRAY_BUFFER	{
				glcore::GL_ARRAY_BUFFER_BINDING
			}else
			if *self.target == glcore::GL_ELEMENT_ARRAY_BUFFER	{
				glcore::GL_ELEMENT_ARRAY_BUFFER_BINDING
			}else	{
				fail( fmt!("Unknown binding to query: %d",*self.target as int) );
			};
		let hid = 0 as glcore::GLint;
		unsafe	{
			glcore::glGetIntegerv( query, ptr::addr_of(&hid) );
		}
		let h = Handle( hid as glcore::GLuint );
		if *self.active != *h	{
			self.active = h;
			false
		}else	{ true }
	}
}


impl Binding	{
	priv fn _bind( h : Handle )	{
		if *self.active != *h	{
			self.active = h;
			glcore::glBindBuffer( *self.target, *h );
		}
	}
	fn bind( object : &Object )	{ self._bind(object.handle) }
	fn unbind()	{ self._bind(Handle(0)) }
	
	fn allocate( size : uint, dynamic : bool )	{
		let usage = if dynamic {glcore::GL_STATIC_DRAW} else {glcore::GL_DYNAMIC_DRAW};
		glcore::glBufferData( *self.target, size as glcore::GLsizeiptr, ptr::null(), usage );
	}

	fn load<T>( data : &[T], dynamic : bool )	{
		let usage = if dynamic {glcore::GL_STATIC_DRAW} else {glcore::GL_DYNAMIC_DRAW};
		let size = data.len() * sys::size_of::<T>() as glcore::GLsizeiptr;
		unsafe	{
			let raw = vec::raw::to_ptr(data) as *glcore::GLvoid;
			glcore::glBufferData( *self.target, size, raw, usage );
		}
	}
}


impl context::Context	{
	fn create_buffer()->Object	{
		let mut hid = 0 as glcore::GLuint;
		unsafe	{
			glcore::glGenBuffers( 1, ptr::addr_of(&hid) );
		}
		Object{ handle:Handle(hid) }
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