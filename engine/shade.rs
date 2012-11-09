extern mod glcore;
extern mod lmath;
use send_map::linear::LinearMap;

pub enum Handle		= glcore::GLuint;
pub enum Location	= glcore::GLint;

impl Handle : cmp::Eq	{
	pure fn eq( h : &Handle )-> bool	{ self == *h }
	pure fn ne( h : &Handle )-> bool	{ !self.eq(h) }
}


enum Uniform	{
	Unitialized,
	UniFloat(float),
	UniInt(int),
	UniFloatVec(lmath::vector::vec4),
	UniIntVec(lmath::vector::ivec4),
	UniQuat(lmath::quaternion::quat4),
	UniTex2D(uint,@texture::Texture),
}

impl Uniform : cmp::Eq	{
	pure fn eq( v : &Uniform )-> bool	{
		match (&self,v)	{
			(&UniFloat(f1),&UniFloat(f2))			=> f1==f2,
			(&UniInt(i1),&UniInt(i2))				=> i1==i2,
			(&UniFloatVec(fv1),&UniFloatVec(fv2))	=> fv1==fv2,
			//FIXME: waiting for lmath to cover that
			//(&UniIntVec(fi1),&UniIntVec(fi2))		=> fi1==fi2,
			(&UniQuat(q1),&UniQuat(q2))				=> q1==q2,
			(_,_)									=> false
		}
	}
	pure fn ne( v : &Uniform )-> bool	{ !self.eq(v) }
}


struct Parameter	{
	loc		: Location,
	storage	: glcore::GLenum,
	size	: glcore::GLint,
	mut value	: Uniform
}

pub struct Attribute	{
	loc		: uint,
	storage	: glcore::GLenum,
	size	: glcore::GLint
}

impl Parameter	{
	priv fn read( h : Handle )-> bool	{
		let t = self.storage;
		let loc = *self.loc;
		assert loc >= 0;
		if t == glcore::GL_FLOAT	{
			unsafe	{
				let mut v = 0f32;
				glcore::glGetUniformfv( *h, loc, ptr::addr_of(&v) );
				self.value = UniFloat(v as float);
			}
		}else
		if t == glcore::GL_INT	{
			unsafe	{
				let mut v = 0i32;
				glcore::glGetUniformiv( *h, loc, ptr::addr_of(&v) );
				self.value = UniInt(v as int);
			}
		}else
		if t == glcore::GL_FLOAT_VEC4	{
			unsafe	{
				let mut v = lmath::vector::Vec4::new(0f32,0f32,0f32,0f32);
				glcore::glGetUniformfv( *h, loc, v.to_ptr() );
				self.value = UniFloatVec(v);
			}
		}else
		if t == glcore::GL_INT_VEC4	{
			unsafe	{
				let mut v = lmath::vector::Vec4::new(0i32,0i32,0i32,0i32);
				glcore::glGetUniformiv( *h, loc, v.to_ptr() );
				self.value = UniIntVec(v);
			}
		}else	{return false;}
		true
	}

	priv fn write()	{
		let loc = *self.loc;
		match copy self.value	{
			Unitialized		=> fail(fmt!( "Uninitalized parameter at location %d", loc as int )),
			UniFloat(v)		=> glcore::glUniform1f( loc, v as glcore::GLfloat ),
			UniInt(v)		=> glcore::glUniform1i( loc, v as glcore::GLint ),
			UniFloatVec(v)	=> glcore::glUniform4fv( loc, 4, v.to_ptr() ),
			UniIntVec(v)	=> glcore::glUniform4iv( loc, 4, v.to_ptr() ),
			_				=> fail(fmt!( "Unknown parameter at location %d", loc as int )),
		}
	}
}


pub type AttriMap	= LinearMap<~str,Attribute>;
pub type ParaMap	= LinearMap<~str,Parameter>;
pub type DataMap	= LinearMap<~str,Uniform>;

struct Object	{
	handle	: Handle,
	target	: glcore::GLenum,
	alive	: bool,
	info	: ~str,

	drop	{
		glcore::glDeleteShader( *self.handle );
	}
}

pub struct Program	{
	handle	: Handle,
	alive	: bool,
	info	: ~str,
	attribs	: AttriMap,
	params	: ParaMap,

	drop	{
		// assert: not current
		glcore::glDeleteProgram( *self.handle );
	}
}


priv fn query_attributes( h : Handle )-> AttriMap	{
	//assert glcore::glGetError() == 0;
	let mut num		= 0 as glcore::GLint;
	let mut max_len	= 0 as glcore::GLint;
	let mut info_bytes	: ~[libc::c_char];
	let mut raw_bytes	: *libc::c_char;
	unsafe	{
		glcore::glGetProgramiv( *h, glcore::GL_ACTIVE_ATTRIBUTES, ptr::addr_of(&num) );
		glcore::glGetProgramiv( *h, glcore::GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, ptr::addr_of(&max_len) );
		info_bytes = vec::from_elem( max_len as uint, 0 as libc::c_char );
		raw_bytes = vec::raw::to_ptr(info_bytes);
	}
	let mut rez		= send_map::linear::linear_map_with_capacity::<~str,Attribute>( num as uint );
	while num>(0 as glcore::GLint)	{
		num -= 1;
		let mut length	= 0 as glcore::GLint;
		let mut size	= 0 as glcore::GLint;
		let mut storage	= 0 as glcore::GLenum;
		let mut name 	: ~str;
		unsafe	{
			glcore::glGetActiveAttrib( *h, num as glcore::GLuint, max_len,
				ptr::addr_of(&length), ptr::addr_of(&size),
				ptr::addr_of(&storage), raw_bytes );
			name = str::raw::from_c_str_len( raw_bytes, length as uint );
		}
		info_bytes[length] = 0;
		let location = glcore::glGetAttribLocation( *h, raw_bytes );
		rez.insert( name, Attribute{ loc:location as uint, storage:storage, size:size } );
	}
	rez
}


priv fn query_parameters( h : Handle )-> ParaMap	{
	//assert glcore::glGetError() == 0;
	let mut num		= 0 as glcore::GLint;
	let mut max_len	= 0 as glcore::GLint;
	let mut info_bytes	: ~[libc::c_char];
	let mut raw_bytes	: *libc::c_char;
	unsafe	{
		glcore::glGetProgramiv( *h, glcore::GL_ACTIVE_UNIFORMS, ptr::addr_of(&num) );
		glcore::glGetProgramiv( *h, glcore::GL_ACTIVE_UNIFORM_MAX_LENGTH, ptr::addr_of(&max_len) );
		info_bytes	= vec::from_elem( max_len as uint, 0 as libc::c_char );
		raw_bytes	= vec::raw::to_ptr(info_bytes);
	}
	let mut rez		= send_map::linear::linear_map_with_capacity::<~str,Parameter>( num as uint );
	while num>(0 as glcore::GLint)	{
		num -= 1;
		let mut length	= 0 as glcore::GLint;
		let mut size	= 0 as glcore::GLint;
		let mut storage	= 0 as glcore::GLenum;
		let mut name 	: ~str;
		unsafe	{
			glcore::glGetActiveUniform( *h, num as glcore::GLuint, max_len,
				ptr::addr_of(&length), ptr::addr_of(&size),
				ptr::addr_of(&storage), raw_bytes );
			name = str::raw::from_c_str_len( raw_bytes, length as uint );
		}
		info_bytes[length] = 0;
		let location = glcore::glGetUniformLocation( *h, raw_bytes );
		let p = Parameter{ loc:Location(location), storage:storage, size:size, value:Unitialized };
		p.read( h );
		rez.insert( name, p );
	}
	rez
}


impl context::Context	{
	pub fn create_shader( target : glcore::GLenum, code : &str )-> Object	{
		let h = Handle( glcore::glCreateShader(target) );
		let mut length = code.len() as glcore::GLint;
		do str::as_c_str(code) |text|	{
			unsafe {
				glcore::glShaderSource(	*h, 1i32, ptr::addr_of(&text), ptr::addr_of(&length) );
			}
		}
		glcore::glCompileShader( *h );
		// get info message
		let mut message:~str;
		let mut status = 0 as glcore::GLint;
		length = 0;
		unsafe	{
			glcore::glGetShaderiv( *h, glcore::GL_COMPILE_STATUS, ptr::addr_of(&status) );
			glcore::glGetShaderiv( *h, glcore::GL_INFO_LOG_LENGTH, ptr::addr_of(&length) );
			let info_bytes = vec::from_elem( length as uint, 0 as libc::c_char );
			let raw_bytes = vec::raw::to_ptr(info_bytes);
			glcore::glGetShaderInfoLog( *h, length, ptr::addr_of(&length), raw_bytes );
			message = str::raw::from_c_str( raw_bytes );
		}
		let ok = (status != (0 as glcore::GLint));
		if !ok	{
			io::println( fmt!("Shader: %s",message) );	//TEMP
		}
		Object{ handle:h, target:target, alive:ok, info:message }
	}
	
	pub fn create_program( shaders : ~[Object] )-> Program	{
		let h = Handle( glcore::glCreateProgram() );
		for shaders.each |s| {
			glcore::glAttachShader( *h, *s.handle );
		}
		glcore::glLinkProgram( *h );
		// get info message
		let mut message:~str;
		let mut status = 0 as glcore::GLint;
		let mut length = 0 as glcore::GLint;
		unsafe	{
			glcore::glGetProgramiv( *h, glcore::GL_LINK_STATUS, ptr::addr_of(&status) );
			glcore::glGetProgramiv( *h, glcore::GL_INFO_LOG_LENGTH, ptr::addr_of(&length) );
			let info_bytes = vec::from_elem( length as uint, 0 as libc::c_char );
			let raw_bytes = vec::raw::to_ptr(info_bytes);
			glcore::glGetProgramInfoLog( *h, length, ptr::addr_of(&length), raw_bytes );
			message = str::raw::from_c_str( raw_bytes );
		}
		let ok = (status != (0 as glcore::GLint));
		if !ok	{
			io::println( fmt!("Program: %s",message) );	//TEMP
		}
		// done
		Program{ handle:h, alive:ok, info:message,
			attribs	:query_attributes(h),
			params	:query_parameters(h) }
	}

	priv fn _bind_program( h : Handle )	{
		if *self.program != *h	{
			self.program = h;
			glcore::glUseProgram( *h );
		}
	}

	//FIXME: accept Map trait once HashMap<~str> are supported
	fn bind_program( p : &Program, data : &DataMap )->bool	{
		self._bind_program( p.handle );
		let mut tex_unit = 0;
		for data.each |name,value|	{
			match p.params.find_ref(name)	{
				Some(ref par) =>	{
					match *value	{
						UniTex2D(_,t)	=>	{
							assert [glcore::GL_TEXTURE_2D].contains( &*t.target );
							self.texture.bind_to( tex_unit, t );
							match par.value	{
								UniTex2D(unit,_) if unit==tex_unit	=> {}
								_	=> { glcore::glUniform1i( *par.loc, tex_unit as glcore::GLint ); }
							}
							par.value = UniTex2D( tex_unit, t );
							tex_unit += 1;
						},
						_	=> {
							if par.value != *value	{
								par.value = *value;
								par.write();
							}
						}
					}
				},
				None => {}
			}
		}
		true
	}

	fn unbind_program()	{
		self._bind_program( Handle(0) );
	}

	fn get_active_program()->Handle	{
		let mut hid = 0 as glcore::GLint;
		unsafe	{
			glcore::glGetIntegerv( glcore::GL_CURRENT_PROGRAM, ptr::addr_of(&hid) );
		}
		Handle(hid as glcore::GLuint)
	}
}


pub fn create_data()-> DataMap	{
	LinearMap::<~str,Uniform>()
}
