extern mod glcore;

const MAX_VERTEX_ATTRIBS	:uint	= 8;
pub enum Handle	= glcore::GLuint;
pub enum Target = glcore::GLenum;


pub struct Object	{
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
	
	
}


struct VertexData	{
	mut enabled	: bool,
	mut attrib	: mesh::Attribute,
}


pub struct VertexArray	{
	handle		: Handle,
	data		: ~[VertexData],
	array		: buf::Binding,
	element		: buf::Binding,

	drop	{
		unsafe	{
			//FIXME: check current
			glcore::glDeleteVertexArrays( 1, ptr::addr_of(&*self.handle) );
		}
	}
}


impl context::Context	{
	fn create_vertex_array()-> VertexArray	{
		let mut hid = 0 as glcore::GLuint;
		unsafe	{
			glcore::glGenVertexArrays( 1, ptr::addr_of(&hid) );
		}
		let default = @Object{ handle:Handle(0) };
		let data = do vec::from_fn(MAX_VERTEX_ATTRIBS) |_i|	{
			VertexData{ enabled: false, attrib: mesh::Attribute{
					kind: glcore::GL_NONE, count: 0u,
					normalized: true, interpolated: true,
					buffer: default, stride: 0u, offset: 0u,
			}}
		};
		VertexArray{ handle:Handle(hid), data:data,
			array	:Binding{	target:Target(glcore::GL_ARRAY_BUFFER),			active:Handle(0) },
			element	:Binding{	target:Target(glcore::GL_ELEMENT_ARRAY_BUFFER),	active:Handle(0) }
		}
	}

	priv fn _bind_vertex_array( h : Handle )	{
		if *self.vertex_array != *h	{
			self.vertex_array = h;
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
		Object{ handle:Handle(hid) }
	}

	priv fn _bind_buffer( binding : &Binding, h : Handle )	{
		if *binding.active != *h	{
			binding.active = h;
			glcore::glBindBuffer( *binding.target, *h );
		}
	}
	fn bind_buffer( va : &VertexArray, obj : &Object, attrib : bool  )	{
		assert *self.vertex_array == *va.handle;
		let binding = if attrib {&va.array} else {&va.element};
		self._bind_buffer( binding, obj.handle );
	}
	fn unbind_buffers( va : &VertexArray )	{
		assert *self.vertex_array == *va.handle;
		self._bind_buffer( &va.array,	Handle(0) );
		self._bind_buffer( &va.element,	Handle(0) );
	}

	fn allocate_buffer( va : &VertexArray, obj : &Object, size : uint, dynamic : bool )	{
		assert *self.vertex_array == *va.handle;
		self.bind_buffer( va, obj, true );
		let usage = if dynamic {glcore::GL_STATIC_DRAW} else {glcore::GL_DYNAMIC_DRAW};
		glcore::glBufferData( *va.array.target, size as glcore::GLsizeiptr, ptr::null(), usage );
	}

	fn load_buffer<T>( va : &VertexArray, obj : &Object, data : &[T], dynamic : bool )	{
		assert *self.vertex_array == *va.handle;
		self.bind_buffer( va, obj, true );
		let usage = if dynamic {glcore::GL_STATIC_DRAW} else {glcore::GL_DYNAMIC_DRAW};
		let size = data.len() * sys::size_of::<T>() as glcore::GLsizeiptr;
		unsafe	{
			let raw = vec::raw::to_ptr(data) as *glcore::GLvoid;
			glcore::glBufferData( *va.array.target, size, raw, usage );
		}
	}

	fn create_buffer_sized( va : &VertexArray, size : uint )-> Object	{
		assert *self.vertex_array == *va.handle;
		let obj = self.create_buffer();
		self.allocate_buffer( va, &obj, size, true );
		obj
	}

	fn create_buffer_loaded<T>( va : &VertexArray, data : ~[T] )-> Object	{
		assert *self.vertex_array == *va.handle;
		let obj = self.create_buffer();
		self.load_buffer( va, &obj, data, false );
		obj
	}
}
