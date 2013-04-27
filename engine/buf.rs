extern mod glcore;

use context;
use mesh;


static MAX_VERTEX_ATTRIBS	:uint	= 8;
pub struct Handle( glcore::GLuint );
pub struct Target( glcore::GLenum );


pub struct Object	{
	handle		: Handle,
	priv pool	: @mut ~[Handle],
}

#[unsafe_destructor]
impl Drop for Object	{
	fn finalize( &self )	{
		self.pool.push( self.handle );
	}
}

impl cmp::Eq for Object	{
	fn eq( &self, other : &Object )-> bool	{
		*self.handle == *other.handle
	}
	fn ne( &self, other : &Object )-> bool	{
		!self.eq( other )
	}
}


pub struct Binding	{
	target		: Target,
	priv active	: Handle,
	priv pool	: @mut ~[Handle],
}

impl context::ProxyState for Binding	{
	fn sync_back( &mut self )-> bool	{
		let query =
			if *self.target == glcore::GL_ARRAY_BUFFER	{
				glcore::GL_ARRAY_BUFFER_BINDING
			}else
			if *self.target == glcore::GL_ELEMENT_ARRAY_BUFFER	{
				glcore::GL_ELEMENT_ARRAY_BUFFER_BINDING
			}else	{
				fail!( fmt!("Unknown binding to query: %d",*self.target as int) );
			};
		let hid = 0 as glcore::GLint;
		glcore::glGetIntegerv( query, ptr::addr_of(&hid) );
		let h = Handle( hid as glcore::GLuint );
		if *self.active != *h	{
			self.active = h;
			false
		}else	{ true }
	}
}

pub impl Binding	{
	fn new( value : glcore::GLenum )-> Binding	{
		Binding{
			target : Target(value), active : Handle(0), pool : @mut~[]
		}
	}
	priv fn _bind( &mut self, h : Handle )	{
		if *self.active != *h	{
			self.active = h;
			glcore::glBindBuffer( *self.target, *h );
		}
	}
}


struct VertexData	{
	enabled	: bool,
	attrib	: mesh::Attribute,
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


pub struct VertexArray	{
	handle			: Handle,
	data			: ~[VertexData],
	element			: Binding,
	priv pool		: @mut ~[Handle],
}

#[unsafe_destructor]
impl Drop for VertexArray	{
	fn finalize( &self )	{
		self.pool.push( self.handle );
	}
}

impl context::ProxyState for VertexArray	{
	fn sync_back( &mut self )->bool	{
		//FIXME
		true
	}
}

pub impl VertexArray	{
	fn get_mask( &self )-> uint	{
		let mut m = 0u;
		for self.data.eachi() |i,vd|	{
			if vd.enabled	{
				m |= 1<<i;
			}
		}
		m
	}
	fn new_default()-> VertexArray	{
		VertexArray{ handle : Handle(0), data : _create_zero_data(),
			element	: Binding::new( glcore::GL_ELEMENT_ARRAY_BUFFER ),
			pool : @mut ~[],
		}	
	}
}


pub struct VaBinding	{
	priv active	: Handle,
	priv pool	: @mut ~[Handle],
}

pub impl VaBinding	{
	fn is_active( &self, va : &VertexArray )-> bool	{
		*self.active == *va.handle
	}

	fn new()-> VaBinding	{
		VaBinding{
			active : Handle(0), pool : @mut~[]
		}
	}
}


pub impl context::Context	{
	fn create_vertex_array( &self )-> VertexArray	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenVertexArrays( 1, ptr::addr_of(&hid) );
		VertexArray{ handle : Handle(hid), data : _create_zero_data(),
			element	: Binding::new( glcore::GL_ELEMENT_ARRAY_BUFFER ),
			pool : self.vertex_array.pool,
		}
	}

	priv fn _bind_vertex_array( &mut self, h : Handle )	{
		if *self.vertex_array.active != *h	{
			self.vertex_array.active = h;
			glcore::glBindVertexArray( *h );
		}
	}
	fn bind_vertex_array( &mut self, va : &VertexArray )	{
		self._bind_vertex_array( va.handle );
	}
	fn unbind_vertex_array( &mut self )	{
		self._bind_vertex_array( Handle(0) );
	}

	fn create_buffer( &self )-> Object	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenBuffers( 1, ptr::addr_of(&hid) );
		Object{ handle:Handle(hid), pool:self.array_buffer.pool }
	}

	fn bind_element_buffer( &self, va : &mut VertexArray, obj : &Object  )	{
		assert!( *self.vertex_array.active == *va.handle );
		va.element._bind( obj.handle );
	}
	fn bind_buffer( &mut self, obj : &Object )	{
		self.array_buffer._bind( obj.handle );
	}
	fn unbind_buffer( &mut self )	{
		self.array_buffer._bind( Handle(0) );
	}

	fn allocate_buffer( &mut self, obj : &Object, size : uint, dynamic : bool )	{
		self.bind_buffer( obj );
		let usage = if dynamic {glcore::GL_DYNAMIC_DRAW} else {glcore::GL_STATIC_DRAW};
		glcore::glBufferData( *self.array_buffer.target, size as glcore::GLsizeiptr, ptr::null(), usage );
	}

	fn load_buffer<T>( &mut self, obj : &Object, data : &[T], dynamic : bool )	{
		self.bind_buffer( obj );
		let usage = if dynamic {glcore::GL_DYNAMIC_DRAW} else {glcore::GL_STATIC_DRAW};
		let size = data.len() * sys::size_of::<T>() as glcore::GLsizeiptr;
		let raw = unsafe{vec::raw::to_ptr(data)} as *glcore::GLvoid;
		glcore::glBufferData( *self.array_buffer.target, size, raw, usage );
	}

	fn create_buffer_sized( &mut self, size : uint )-> Object	{
		let obj = self.create_buffer();
		self.allocate_buffer( &obj, size, true );
		obj
	}

	fn create_buffer_loaded<T>( &mut self, data : &[T] )-> Object	{
		let obj = self.create_buffer();
		self.load_buffer( &obj, data, false );
		obj
	}

	fn cleanup_buffers( &mut self )	{
		let va_pool : &mut ~[Handle] = self.vertex_array.pool;
		while !va_pool.is_empty()	{
			let h = va_pool.pop();
			assert!( *h != 0 );
			if *h == *self.vertex_array.active	{
				self.unbind_vertex_array();
			}
			glcore::glDeleteVertexArrays( 1, ptr::addr_of(&*h) );
		}
		let ab_pool : &mut ~[Handle] = self.array_buffer.pool;
		while !ab_pool.is_empty()	{
			let h = ab_pool.pop();
			assert!( *h != 0 );
			//ISSUE: active index buffers are not checked
			if *h == *self.array_buffer.active	{
				self.unbind_buffer();
			}
			glcore::glDeleteBuffers( 1, ptr::addr_of(&*h) );
		}
	}
}
