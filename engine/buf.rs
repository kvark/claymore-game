extern mod glcore;

const MAX_VERTEX_ATTRIBS	:uint	= 8;
pub enum Handle	= glcore::GLuint;
pub enum Target = glcore::GLenum;


pub struct Object	{
	handle		: Handle,
	priv pool	: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
	}
}


pub struct Binding	{
	target			: Target,
	priv mut active	: Handle,
	priv pool		: @mut ~[Handle],
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


struct VertexData	{
	mut enabled	: bool,
	mut attrib	: mesh::Attribute,
}


pub struct VertexArray	{
	handle			: Handle,
	data			: ~[VertexData],
	element			: buf::Binding,
	priv pool		: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
	}
}

impl VertexArray : context::State	{
	fn sync_back()->bool	{
		//FIXME
		true
	}
}

pub struct VaBinding	{
	priv mut active	: Handle,
	priv pool		: @mut ~[Handle],
}

impl VaBinding	{
	pure fn is_active( va : &VertexArray )-> bool	{
		*self.active == *va.handle
	}
}


pub fn create_binding( value : glcore::GLenum )-> Binding	{
	Binding{
		target : Target(value), active : Handle(0), pool : @mut~[]
	}
}

pub fn create_va_binding()-> VaBinding	{
	VaBinding{
		active : Handle(0), pool : @mut~[]
	}
}


priv fn _create_zero_data()-> ~[VertexData]	{
	let default = @Object{ handle:Handle(0), pool:@mut ~[] };
	do vec::from_fn(MAX_VERTEX_ATTRIBS) |_i|	{
		VertexData{ enabled: false, attrib: mesh::Attribute{
				kind: glcore::GL_NONE, count: 0u,
				normalized: true, interpolated: true,
				buffer: default, stride: 0u, offset: 0u,
		}}
	}
}

pub fn default_vertex_array()-> VertexArray	{
	VertexArray{ handle : Handle(0), data : _create_zero_data(),
		element	: create_binding( glcore::GL_ELEMENT_ARRAY_BUFFER ),
		pool : @mut ~[],
	}	
}



impl context::Context	{
	fn create_vertex_array()-> VertexArray	{
		let mut hid = 0 as glcore::GLuint;
		unsafe	{
			glcore::glGenVertexArrays( 1, ptr::addr_of(&hid) );
		}
		VertexArray{ handle : Handle(hid), data : _create_zero_data(),
			element	: create_binding( glcore::GL_ELEMENT_ARRAY_BUFFER ),
			pool : self.vertex_array.pool,
		}
	}

	priv fn _bind_vertex_array( h : Handle )	{
		if *self.vertex_array.active != *h	{
			self.vertex_array.active = h;
			glcore::glBindVertexArray( *h );
		}
	}
	fn bind_vertex_array( va : &VertexArray )	{
		self._bind_vertex_array( va.handle );
	}
	fn unbind_vertex_array()	{
		self._bind_vertex_array( Handle(0) );
	}

	fn create_buffer()-> Object	{
		let mut hid = 0 as glcore::GLuint;
		unsafe	{
			glcore::glGenBuffers( 1, ptr::addr_of(&hid) );
		}
		Object{ handle:Handle(hid), pool:self.array_buffer.pool }
	}

	priv fn _bind_buffer( binding : &Binding, h : Handle )	{
		if *binding.active != *h	{
			binding.active = h;
			glcore::glBindBuffer( *binding.target, *h );
		}
	}
	fn bind_element_buffer( va : &VertexArray, obj : &Object  )	{
		assert *self.vertex_array.active == *va.handle;
		self._bind_buffer( &va.element, obj.handle );
	}
	fn bind_buffer( obj : &Object )	{
		self._bind_buffer( &self.array_buffer, obj.handle );
	}
	fn unbind_buffer()	{
		self._bind_buffer( &self.array_buffer,	Handle(0) );
	}

	fn allocate_buffer( obj : &Object, size : uint, dynamic : bool )	{
		self.bind_buffer( obj );
		let usage = if dynamic {glcore::GL_STATIC_DRAW} else {glcore::GL_DYNAMIC_DRAW};
		glcore::glBufferData( *self.array_buffer.target, size as glcore::GLsizeiptr, ptr::null(), usage );
	}

	fn load_buffer<T>( obj : &Object, data : &[T], dynamic : bool )	{
		self.bind_buffer( obj );
		let usage = if dynamic {glcore::GL_STATIC_DRAW} else {glcore::GL_DYNAMIC_DRAW};
		let size = data.len() * sys::size_of::<T>() as glcore::GLsizeiptr;
		unsafe	{
			let raw = vec::raw::to_ptr(data) as *glcore::GLvoid;
			glcore::glBufferData( *self.array_buffer.target, size, raw, usage );
		}
	}

	fn create_buffer_sized( size : uint )-> Object	{
		let obj = self.create_buffer();
		self.allocate_buffer( &obj, size, true );
		obj
	}

	fn create_buffer_loaded<T>( data : ~[T] )-> Object	{
		let obj = self.create_buffer();
		self.load_buffer( &obj, data, false );
		obj
	}

	fn cleanup_buffers()	{
		while self.vertex_array.pool.len()!=0	{
			let h = self.vertex_array.pool.pop();
			assert *h != 0;
			if *h == *self.vertex_array.active	{
				self.unbind_vertex_array();
			}
			unsafe	{
				glcore::glDeleteVertexArrays( 1, ptr::addr_of(&*h) );
			}
		}
		while self.array_buffer.pool.len()!=0	{
			let h = self.array_buffer.pool.pop();
			assert *h != 0;
			//ISSUE: active index buffers are not checked
			if *h == *self.array_buffer.active	{
				self.unbind_buffer();
			}
			glcore::glDeleteBuffers( 1, ptr::addr_of(&*h) );
		}
	}
}
