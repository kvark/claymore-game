extern mod glcore;

use core::managed;

use context;
use mesh;


static MAX_VERTEX_ATTRIBS	:uint	= 8;

#[deriving(Eq)]
pub struct ObjectHandle( glcore::GLuint );
#[deriving(Eq)]
pub struct ArrayHandle( glcore::GLuint );
pub struct Target( glcore::GLenum );


#[deriving(Eq)]
pub struct Object	{
	handle		: ObjectHandle,
}

impl Drop for ObjectHandle	{
	fn finalize( &self )	{
		if **self != 0	{
			glcore::glDeleteBuffers( 1, ptr::addr_of(&**self) );
		}
	}
}


pub struct Binding	{
	target		: Target,
	default		: @Object,
	priv active	: @Object,
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
		let h = ObjectHandle( hid as glcore::GLuint );
		if self.active.handle != h	{
			self.active = self.default;
			false
		}else	{ true }
	}
}

pub impl Binding	{
	fn new( value : glcore::GLenum, default : @Object )-> Binding	{
		Binding{
			target : Target(value), default : default, active : default
		}
	}
	priv fn bind( &mut self, ob : @Object )	{
		if !managed::ptr_eq(self.active,ob)	{
			self.active = ob;
			glcore::glBindBuffer( *self.target, *ob.handle );
		}
	}
}


struct VertexData	{
	enabled	: bool,
	attrib	: mesh::Attribute,
}

pub struct VertexArray	{
	handle			: ArrayHandle,
	data			: ~[VertexData],
	element			: @Object,
}

impl Drop for ArrayHandle	{
	fn finalize( &self )	{
		if **self != 0	{
			glcore::glDeleteVertexArrays( 1, ptr::addr_of(&**self) );
		}
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
}


pub struct VaBinding	{
	priv active	: @mut VertexArray,
	default		: @mut VertexArray,
	default_object	: @Object,
}

impl VaBinding	{
	priv fn create_data( buf : @Object )-> ~[VertexData]	{
		do vec::from_fn(MAX_VERTEX_ATTRIBS) |_i|	{
			VertexData{ enabled: false, attrib: mesh::Attribute{
					kind: glcore::GL_NONE, count: 0u,
					normalized: true, interpolated: true,
					buffer: buf, stride: 0u, offset: 0u,
			}}
		}
	}

	pub fn create_zero_data( &self )-> ~[VertexData]	{
		VaBinding::create_data( self.default_object )
	}

	pub fn is_active( &self, va : @mut VertexArray )-> bool	{
		managed::mut_ptr_eq(self.active, va)
	}

	pub fn new()-> VaBinding	{
		let def_object = @Object{ handle : ObjectHandle(0) };
		let def = @mut VertexArray{ handle : ArrayHandle(0),
			data : VaBinding::create_data(def_object),
			element	: def_object,
		};
		VaBinding{
			active	: def,
			default	: def,
			default_object: def_object,
		}
	}
}


pub impl context::Context	{
	fn create_vertex_array( &self )-> @mut VertexArray	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenVertexArrays( 1, ptr::addr_of(&hid) );
		@mut VertexArray{ handle : ArrayHandle(hid), data : self.vertex_array.create_zero_data(),
			element	: self.vertex_array.default_object }
	}

	fn bind_vertex_array( &mut self, va : @mut VertexArray )	{
		if !self.vertex_array.is_active( va )	{
			self.vertex_array.active = va;
			self.element_buffer.active = va.element;
			glcore::glBindVertexArray( *va.handle );
		}
	}
	fn unbind_vertex_array( &mut self )	{
		self.bind_vertex_array( self.vertex_array.default );
	}

	fn create_buffer( &self )-> @Object	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenBuffers( 1, ptr::addr_of(&hid) );
		@Object{ handle:ObjectHandle(hid) }
	}

	fn bind_element_buffer( &mut self, va : @mut VertexArray, obj : @Object  )	{
		assert!( self.vertex_array.is_active(va) );
		va.element = obj;
		self.element_buffer.bind( obj );
	}
	fn bind_buffer( &mut self, obj : @Object )	{
		self.array_buffer.bind( obj );
	}
	fn unbind_buffer( &mut self )	{
		self.array_buffer.bind( self.array_buffer.default );
	}

	fn allocate_buffer( &mut self, obj : @Object, size : uint, dynamic : bool )	{
		self.bind_buffer( obj );
		let usage = if dynamic {glcore::GL_DYNAMIC_DRAW} else {glcore::GL_STATIC_DRAW};
		glcore::glBufferData( *self.array_buffer.target, size as glcore::GLsizeiptr, ptr::null(), usage );
	}

	fn load_buffer<T>( &mut self, obj : @Object, data : &[T], dynamic : bool )	{
		self.bind_buffer( obj );
		let usage = if dynamic {glcore::GL_DYNAMIC_DRAW} else {glcore::GL_STATIC_DRAW};
		let size = data.len() * sys::size_of::<T>() as glcore::GLsizeiptr;
		let raw = unsafe{vec::raw::to_ptr(data)} as *glcore::GLvoid;
		glcore::glBufferData( *self.array_buffer.target, size, raw, usage );
	}

	fn create_buffer_sized( &mut self, size : uint )-> @Object	{
		let obj = self.create_buffer();
		self.allocate_buffer( obj, size, true );
		obj
	}

	fn create_buffer_loaded<T>( &mut self, data : &[T] )-> @Object	{
		let obj = self.create_buffer();
		self.load_buffer( obj, data, false );
		obj
	}
}
