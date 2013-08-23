extern mod glcore;

use core::managed;

use gr_low::context;
use gr_mid::mesh;


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
		glcore::glDeleteBuffers( 1, ptr::addr_of(&**self) );
	}
}


pub struct Binding	{
	target		: Target,
	priv active	: Option<@Object>,
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
		hid == match self.active	{
			Some(o)	=> *o.handle as glcore::GLint,
			None		=> 0
		}
	}
}

pub impl Binding	{
	fn new( value : glcore::GLenum )-> Binding	{
		Binding{
			target : Target(value), active : None
		}
	}

	priv fn bind( &mut self, ob : @Object )	{
		let need_bind = match self.active	{
			Some(o)	=> !managed::ptr_eq(o,ob),
			None	=> true
		};
		if need_bind	{
			self.active = Some(ob);
			glcore::glBindBuffer( *self.target, *ob.handle );
		}
	}

	priv fn unbind( &mut self )	{
		if self.active.is_some()	{
			self.active = None;
			glcore::glBindBuffer( *self.target, 0 );
		}
	}
}


struct VertexData	{
	enabled	: bool,
	attrib	: Option<mesh::Attribute>,
}

pub struct VertexArray	{
	handle			: ArrayHandle,
	data			: ~[VertexData],
	element			: Option<@Object>,
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
}

impl VaBinding	{
	priv fn make_data()-> ~[VertexData]	{
		do vec::from_fn(MAX_VERTEX_ATTRIBS) |_i|	{
			VertexData{ enabled: false, attrib: None }
		}

	}

	pub fn is_active( &self, va : @mut VertexArray )-> bool	{
		managed::mut_ptr_eq(self.active, va)
	}

	pub fn new()-> VaBinding	{
		let def = @mut VertexArray{
			handle	: ArrayHandle(0),
			data	: VaBinding::make_data(),
			element	: None,
		};
		VaBinding{
			active	: def,
			default	: def,
		}
	}
}


pub impl context::Context	{
	fn create_vertex_array( &self )-> @mut VertexArray	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenVertexArrays( 1, ptr::addr_of(&hid) );
		@mut VertexArray{
			handle	: ArrayHandle(hid),
			data	: VaBinding::make_data(),
			element	: None
		}
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
		va.element = Some(obj);
		self.element_buffer.bind( obj );
	}
	fn bind_buffer( &mut self, obj : @Object )	{
		self.array_buffer.bind( obj );
	}
	fn unbind_buffer( &mut self )	{
		self.array_buffer.unbind();
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
