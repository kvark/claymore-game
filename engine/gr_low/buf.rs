extern mod glcore;

use core::managed;

use gr_low;


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

impl gr_low::context::ProxyState for Binding	{
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


#[deriving(Eq)]
pub struct Attribute	{
	// semantics
	kind			: glcore::GLenum,
	count			: uint,
	normalized		: bool,
	interpolated	: bool,
	// location
	buffer			: @Object,
	stride			: uint,
	offset			: uint,
}

pub impl Attribute	{
	fn new( format : &str, buffer : @Object, stride : uint, offset : uint )-> (Attribute,uint)	{
		assert!( (format.len()==3u && ['.','!'].contains(&format.char_at(2))) ||
			format.len()==2u || (format.len()==4u && str::substr(format,2,2)==~".!") );
		let count = (format[0] - "0"[0]) as uint;
		let is_fixed_point	= format.len()>2u	&& format.char_at(2)=='.';
		let can_interpolate	= format.len()<=2u	|| format.char_at(format.len()-1u)!='!';
		let (el_size,el_type) = match format.char_at(1)	{
			'b'	=> (1u,glcore::GL_BYTE),
			'B'	=> (1u,glcore::GL_UNSIGNED_BYTE),
			'h'	=> (2u,glcore::GL_SHORT),
			'H'	=> (2u,glcore::GL_UNSIGNED_SHORT),
			'i'	=> (4u,glcore::GL_INT),
			'I'	=> (4u,glcore::GL_UNSIGNED_INT),
			'f'	=> (4u,glcore::GL_FLOAT),
			_	=> fail!(fmt!( "Unknown attribute format: %s", format ))
		};
		(Attribute{
			kind			: el_type,
			count			: count,
			normalized		: is_fixed_point,
			interpolated	: can_interpolate,
			buffer			: buffer,
			stride			: stride,
			offset			: offset,
		}, count * el_size)
	}

	fn new_index( format : &str, buffer : @Object )-> (Attribute,uint)	{
		Attribute::new( format, buffer, 0u, 0u )
	}

	fn compatible( &self, at : &gr_low::shade::Attribute )-> bool	{
		//io::println(fmt!( "Checking compatibility: kind=0x%x, count=%u, storage=0x%x",
		//	self.kind as uint, self.count, at.storage as uint ));
		let (count,unit) = at.decompose();
		count == self.count && if at.is_integer()	{
			if unit == glcore::GL_INT	{
				[glcore::GL_BYTE,glcore::GL_SHORT,glcore::GL_INT]		.contains( &self.kind ) ||
				[glcore::GL_UNSIGNED_BYTE,glcore::GL_UNSIGNED_SHORT]	.contains( &self.kind )
			}else
			if unit == glcore::GL_UNSIGNED_INT	{
				[glcore::GL_UNSIGNED_BYTE,glcore::GL_UNSIGNED_SHORT,glcore::GL_UNSIGNED_INT].contains( &self.kind )
			}else	{false}
		}else {true}
	}
}


struct VertexData	{
	enabled	: bool,
	attrib	: Option<Attribute>,
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

impl gr_low::context::ProxyState for VertexArray	{
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


pub impl gr_low::context::Context	{
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

	fn create_attribute<T:gr_low::context::GLType>( &mut self, vdata : &[T], count : uint, norm : bool )-> Attribute	{
		Attribute{
			kind: vdata[0].to_gl_type(),
			count: count,
			normalized: norm,
			interpolated: true,
			buffer: self.create_buffer_loaded( vdata ),
			stride: count * sys::size_of::<T>(),
			offset: 0u
		}
	}
}
