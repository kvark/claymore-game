extern mod gl;

use std;
use std::{managed,ptr};

use gr_low;


static MAX_VERTEX_ATTRIBS	:uint	= 8;

#[deriving(Eq)]
pub struct ObjectHandle( gl::types::GLuint );
#[deriving(Eq)]
pub struct ArrayHandle( gl::types::GLuint );
pub struct Target( gl::types::GLenum );


#[deriving(Eq)]
pub struct Object	{
	handle		: ObjectHandle,
}

impl Drop for ObjectHandle	{
	fn drop( &mut self )	{
		unsafe{ gl::DeleteBuffers( 1, ptr::to_unsafe_ptr(&**self) ); }
	}
}


pub struct Binding	{
	target		: Target,
	priv active	: Option<@Object>,
}

impl gr_low::context::ProxyState for Binding	{
	fn sync_back( &mut self )-> bool	{
		let query =
			if *self.target == gl::ARRAY_BUFFER	{
				gl::ARRAY_BUFFER_BINDING
			}else
			if *self.target == gl::ELEMENT_ARRAY_BUFFER	{
				gl::ELEMENT_ARRAY_BUFFER_BINDING
			}else	{
				fail!( format!("Unknown binding to query: {:i}",*self.target as int) );
			};
		let mut hid = 0 as gl::types::GLint;
		unsafe{ gl::GetIntegerv( query, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		hid == match self.active	{
			Some(o)	=> *o.handle as gl::types::GLint,
			None	=> 0
		}
	}
}

impl Binding	{
	pub fn new( value : gl::types::GLenum )-> Binding	{
		Binding{
			target : Target(value), active : None
		}
	}

	fn bind( &mut self, ob : @Object )	{
		let need_bind = match self.active	{
			Some(o)	=> !managed::ptr_eq(o,ob),
			None	=> true
		};
		if need_bind	{
			self.active = Some(ob);
			gl::BindBuffer( *self.target, *ob.handle );
		}
	}

	fn unbind( &mut self )	{
		if self.active.is_some()	{
			self.active = None;
			gl::BindBuffer( *self.target, 0 );
		}
	}
}


#[deriving(Eq)]
pub struct Attribute	{
	// semantics
	kind			: gl::types::GLenum,
	count			: uint,
	normalized		: bool,
	interpolated	: bool,
	// location
	buffer			: @Object,
	stride			: uint,
	offset			: uint,
}

impl Attribute	{
	pub fn new( format : &str, buffer : @Object, stride : uint, offset : uint )-> (Attribute,uint)	{
		assert!( (format.len()==3u && ['.','!'].contains(&format.char_at(2))) ||
			format.len()==2u || (format.len()==4u && format.slice(2,4)==".!") );
		let count = (format[0] - "0"[0]) as uint;
		let is_fixed_point	= format.len()>2u	&& format.char_at(2)=='.';
		let can_interpolate	= format.len()<=2u	|| format.char_at(format.len()-1u)!='!';
		let (el_size,el_type) = match format.char_at(1)	{
			'b'	=> (1u,gl::BYTE),
			'B'	=> (1u,gl::UNSIGNED_BYTE),
			'h'	=> (2u,gl::SHORT),
			'H'	=> (2u,gl::UNSIGNED_SHORT),
			'i'	=> (4u,gl::INT),
			'I'	=> (4u,gl::UNSIGNED_INT),
			'f'	=> (4u,gl::FLOAT),
			_	=> fail!("Unknown attribute format: {:s}", format)
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

	pub fn new_index( format : &str, buffer : @Object )-> (Attribute,uint)	{
		Attribute::new( format, buffer, 0u, 0u )
	}

	pub fn compatible( &self, at : &gr_low::shade::Attribute )-> bool	{
		//io::println(format!( "Checking compatibility: kind=0x%x, count={:u}, storage=0x%x",
		//	self.kind as uint, self.count, at.storage as uint ));
		let (count,unit) = at.decompose();
		count == self.count && if at.is_integer()	{
			if unit == gl::INT	{
				[gl::BYTE,gl::SHORT,gl::INT]		.contains( &self.kind ) ||
				[gl::UNSIGNED_BYTE,gl::UNSIGNED_SHORT]	.contains( &self.kind )
			}else
			if unit == gl::UNSIGNED_INT	{
				[gl::UNSIGNED_BYTE,gl::UNSIGNED_SHORT,gl::UNSIGNED_INT].contains( &self.kind )
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
	fn drop( &mut self )	{
		if **self != 0	{
			unsafe{ gl::DeleteVertexArrays( 1, ptr::to_unsafe_ptr(&**self) ); }
		}
	}
}

impl gr_low::context::ProxyState for VertexArray	{
	fn sync_back( &mut self )->bool	{
		//FIXME
		true
	}
}

impl VertexArray	{
	pub fn get_mask( &self )-> uint	{
		let mut m = 0u;
		for (i,vd) in self.data.iter().enumerate()	{
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
	fn make_data()-> ~[VertexData]	{
		std::vec::from_fn(MAX_VERTEX_ATTRIBS, |_i|	{
			VertexData{ enabled: false, attrib: None }
		})

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


impl gr_low::context::Context	{
	pub fn create_vertex_array( &self )-> @mut VertexArray	{
		let mut hid = 0 as gl::types::GLuint;
		unsafe{ gl::GenVertexArrays( 1, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		@mut VertexArray{
			handle	: ArrayHandle(hid),
			data	: VaBinding::make_data(),
			element	: None
		}
	}

	pub fn bind_vertex_array( &mut self, va : @mut VertexArray )	{
		if !self.vertex_array.is_active( va )	{
			self.vertex_array.active = va;
			self.element_buffer.active = va.element;
			gl::BindVertexArray( *va.handle );
		}
	}
	pub fn unbind_vertex_array( &mut self )	{
		self.bind_vertex_array( self.vertex_array.default );
	}

	pub fn create_buffer( &self )-> @Object	{
		let mut hid = 0 as gl::types::GLuint;
		unsafe{ gl::GenBuffers( 1, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		@Object{ handle:ObjectHandle(hid) }
	}

	pub fn bind_element_buffer( &mut self, va : @mut VertexArray, obj : @Object  )	{
		assert!( self.vertex_array.is_active(va) );
		va.element = Some(obj);
		self.element_buffer.bind( obj );
	}
	pub fn bind_buffer( &mut self, obj : @Object )	{
		self.array_buffer.bind( obj );
	}
	pub fn unbind_buffer( &mut self )	{
		self.array_buffer.unbind();
	}

	pub fn allocate_buffer( &mut self, obj : @Object, size : uint, dynamic : bool )	{
		self.bind_buffer( obj );
		let usage = if dynamic {gl::DYNAMIC_DRAW} else {gl::STATIC_DRAW};
		unsafe{ gl::BufferData( *self.array_buffer.target, size as gl::types::GLsizeiptr, ptr::null(), usage ); }
	}

	pub fn load_buffer<T>( &mut self, obj : @Object, data : &[T], dynamic : bool )	{
		self.bind_buffer( obj );
		let usage = if dynamic {gl::DYNAMIC_DRAW} else {gl::STATIC_DRAW};
		let size = (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr;
		unsafe{
			let raw = data.as_ptr() as *gl::types::GLvoid;
			gl::BufferData( *self.array_buffer.target, size, raw, usage );
		}
	}

	pub fn create_buffer_sized( &mut self, size : uint )-> @Object	{
		let obj = self.create_buffer();
		self.allocate_buffer( obj, size, true );
		obj
	}

	pub fn create_buffer_loaded<T>( &mut self, data : &[T] )-> @Object	{
		let obj = self.create_buffer();
		self.load_buffer( obj, data, false );
		obj
	}

	pub fn create_attribute<T:gr_low::context::GLType>( &mut self, vdata : &[T], count : uint, norm : bool )-> Attribute	{
		Attribute{
			kind: vdata[0].to_gl_type(),
			count: count,
			normalized: norm,
			interpolated: true,
			buffer: self.create_buffer_loaded( vdata ),
			stride: count * std::mem::size_of::<T>(),
			offset: 0u
		}
	}
}
