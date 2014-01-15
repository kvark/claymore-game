extern mod gl;

use std;
use std::{borrow,cell,ptr,rc};

use gr_low;


static MAX_VERTEX_ATTRIBS	:uint	= 8;

#[deriving(Eq)]
pub struct ObjectHandle( gl::types::GLuint );
#[deriving(Eq)]
pub struct ArrayHandle( gl::types::GLuint );
pub struct Target( gl::types::GLenum );
pub type ObjectPtr = rc::Rc<Object>;

#[deriving(Eq)]
pub struct Object	{
	handle		: ObjectHandle,
}

impl Drop for ObjectHandle	{
	fn drop( &mut self )	{
		let ObjectHandle(ref mut h) = *self;
		unsafe{ gl::DeleteBuffers( 1, ptr::to_unsafe_ptr(h) ); }
	}
}


pub struct Binding	{
	target		: Target,
	priv active	: Option<ObjectPtr>,
}

impl gr_low::context::ProxyState for Binding	{
	fn sync_back( &mut self )-> bool	{
		let Target(t) = self.target;
		let query = match t	{
			gl::ARRAY_BUFFER			=> gl::ARRAY_BUFFER_BINDING,
			gl::ELEMENT_ARRAY_BUFFER	=> gl::ELEMENT_ARRAY_BUFFER_BINDING,
			_	=> fail!("Unknown binding to query: {}", t)
		};
		let mut hid = 0 as gl::types::GLint;
		unsafe{ gl::GetIntegerv( query, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		hid == match self.active	{
			Some(ref o)	=> {
				let ObjectHandle(h) = o.borrow().handle;
				h as gl::types::GLint
			},
			None	=> 0
		}
	}
}

impl Binding	{
	pub fn new( value: gl::types::GLenum )-> Binding	{
		Binding{
			target : Target(value), active : None
		}
	}

	fn bind( &mut self, ob: &ObjectPtr )	{
		let need_bind = match self.active	{
			Some(ref o)	=> !borrow::ref_eq(o.borrow(), ob.borrow()),
			None		=> true
		};
		if need_bind	{
			self.active = Some(ob.clone());
			let Target(t) = self.target;
			let ObjectHandle(h) = ob.borrow().handle;
			gl::BindBuffer( t, h );
		}
	}

	fn unbind( &mut self )	{
		if self.active.is_some()	{
			self.active = None;
			let Target(t) = self.target;
			gl::BindBuffer( t, 0 );
		}
	}
}


#[deriving(Clone,Eq)]
pub struct Attribute	{
	// semantics
	kind			: gl::types::GLenum,
	count			: uint,
	normalized		: bool,
	interpolated	: bool,
	// location
	buffer			: ObjectPtr,
	stride			: uint,
	offset			: uint,
}

impl Attribute	{
	pub fn new( format: &str, buffer: &ObjectPtr, stride: uint, offset: uint )-> (Attribute,uint)	{
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
			buffer			: buffer.clone(),
			stride			: stride,
			offset			: offset,
		}, count * el_size)
	}

	pub fn new_index( format: &str, buffer: &ObjectPtr )-> (Attribute,uint)	{
		Attribute::new( format, buffer, 0u, 0u )
	}

	pub fn compatible( &self, at: &gr_low::shade::Attribute )-> bool	{
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

pub type VertexArrayPtr = rc::Rc<cell::RefCell<VertexArray>>;

pub struct VertexArray	{
	handle			: ArrayHandle,
	data			: ~[VertexData],
	element			: Option<ObjectPtr>,
}

impl Drop for ArrayHandle	{
	fn drop( &mut self )	{
		let ArrayHandle(ref mut h) = *self;
		if *h != 0	{
			unsafe{ gl::DeleteVertexArrays( 1, ptr::to_unsafe_ptr(h) ); }
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
		self.data.iter().enumerate().fold(0u,|m,(i,vd)|	{
			m | if vd.enabled {1<<i} else {0}
		})
	}
}


pub struct VaBinding	{
	priv active	: VertexArrayPtr,
	default		: VertexArrayPtr,
}

impl VaBinding	{
	fn make_data()-> ~[VertexData]	{
		std::vec::from_fn(MAX_VERTEX_ATTRIBS, |_i|	{
			VertexData{ enabled: false, attrib: None }
		})
	}

	pub fn is_active( &self, va: &VertexArrayPtr )-> bool	{
		std::borrow::ref_eq( self.active.borrow(), va.borrow() )	//TODO: ptr_eq
	}

	pub fn new()-> VaBinding	{
		let def = rc::Rc::new( cell::RefCell::new( VertexArray{
			handle	: ArrayHandle(0),
			data	: VaBinding::make_data(),
			element	: None,
		}));
		VaBinding{
			active	: def.clone(),
			default	: def,
		}
	}
}


impl gr_low::context::Context	{
	pub fn create_vertex_array( &self )-> VertexArrayPtr	{
		let mut hid = 0 as gl::types::GLuint;
		unsafe{ gl::GenVertexArrays( 1, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		rc::Rc::new(cell::RefCell::new( VertexArray{
			handle	: ArrayHandle(hid),
			data	: VaBinding::make_data(),
			element	: None
		}))
	}

	pub fn bind_vertex_array( &mut self, vap: &VertexArrayPtr )	{
		if !self.vertex_array.is_active( vap )	{
			vap.borrow().with(|va|	{
				self.element_buffer.active = va.element.clone();
				let ArrayHandle(h) = va.handle;
				gl::BindVertexArray( h );
			});
			self.vertex_array.active = vap.clone();
		}
	}
	pub fn unbind_vertex_array( &mut self )	{
		let va = self.vertex_array.default.clone();
		self.bind_vertex_array( &va );
	}

	pub fn create_buffer( &self )-> ObjectPtr	{
		let mut hid = 0 as gl::types::GLuint;
		unsafe{ gl::GenBuffers( 1, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		rc::Rc::new(Object{ handle:ObjectHandle(hid) })
	}

	pub fn bind_element_buffer( &mut self, vap: &VertexArrayPtr, obj: &ObjectPtr  )	{
		assert!( self.vertex_array.is_active( vap ) );
		vap.borrow().with_mut( |va| va.element = Some(obj.clone()) );
		self.element_buffer.bind( obj );
	}
	pub fn bind_buffer( &mut self, obj: &ObjectPtr )	{
		self.array_buffer.bind( obj );
	}
	pub fn unbind_buffer( &mut self )	{
		self.array_buffer.unbind();
	}

	pub fn allocate_buffer( &mut self, obj: &ObjectPtr, size: uint, dynamic: bool )	{
		self.bind_buffer( obj );
		let usage = if dynamic {gl::DYNAMIC_DRAW} else {gl::STATIC_DRAW};
		let Target(t) = self.array_buffer.target;
		unsafe{ gl::BufferData( t, size as gl::types::GLsizeiptr, ptr::null(), usage ); }
	}

	pub fn load_buffer<T>( &mut self, obj: &ObjectPtr, data: &[T], dynamic: bool )	{
		self.bind_buffer( obj );
		let usage = if dynamic {gl::DYNAMIC_DRAW} else {gl::STATIC_DRAW};
		let size = (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr;
		let Target(t) = self.array_buffer.target;
		unsafe{
			let raw = data.as_ptr() as *gl::types::GLvoid;
			gl::BufferData( t, size, raw, usage );
		}
	}

	pub fn create_buffer_sized( &mut self, size: uint )-> ObjectPtr	{
		let obj = self.create_buffer();
		self.allocate_buffer( &obj, size, true );
		obj
	}

	pub fn create_buffer_loaded<T>( &mut self, data: &[T] )-> ObjectPtr	{
		let obj = self.create_buffer();
		self.load_buffer( &obj, data, false );
		obj
	}

	pub fn create_attribute<T:gr_low::context::GLType>(
			&mut self, vdata: &[T], count: uint, norm: bool )-> Attribute	{
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

