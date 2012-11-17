extern mod glcore;
extern mod lmath;
use send_map::linear::LinearMap;

pub enum Handle		= glcore::GLuint;
pub enum Location	= glcore::GLint;

impl Handle : cmp::Eq	{
	pure fn eq( h : &Handle )-> bool	{ self == *h }
	pure fn ne( h : &Handle )-> bool	{ !self.eq(h) }
}

pub struct Binding	{
	priv mut active_program	: Handle,
	priv pool_objects		: @mut ~[Handle],
	priv pool_programs		: @mut ~[Handle],
}

impl Binding : context::State	{
	fn sync_back()-> bool	{
		let mut hid = 0 as glcore::GLint;
		unsafe	{
			glcore::glGetIntegerv( glcore::GL_CURRENT_PROGRAM, ptr::addr_of(&hid) );
		}
		let program = Handle( hid as glcore::GLuint );
		if *self.active_program != *program	{
			self.active_program = program;
			false
		}else	{true}
	}
}

pub pure fn create_binding()-> Binding	{
	Binding{ active_program:Handle(0), pool_objects:@mut ~[], pool_programs:@mut ~[] }
}


enum Uniform	{
	Unitialized,
	UniFloat(float),
	UniInt(int),
	UniFloatVec(lmath::vector::vec4),
	UniIntVec(lmath::vector::ivec4),
	UniQuat(lmath::quaternion::quat4),
	UniMatrix(bool,lmath::matrix::mat4),
	UniTexture(uint,@texture::Texture),
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
			(&UniMatrix(b1,m1),&UniMatrix(b2,m2))	=> b1==b2 && m1==m2,
			(&UniTexture(u1,_),&UniTexture(u2,_))	=> u1==u2,
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
	priv unsafe fn read( h : Handle )-> bool	{
		let t = self.storage;
		let loc = *self.loc;
		assert loc >= 0;
		if t == glcore::GL_FLOAT	{
			let mut v = 0f32;
			glcore::glGetUniformfv( *h, loc, ptr::addr_of(&v) );
			self.value = UniFloat(v as float);
		}else
		if t == glcore::GL_INT	{
			let mut v = 0i32;
			glcore::glGetUniformiv( *h, loc, ptr::addr_of(&v) );
			self.value = UniInt(v as int);
		}else
		if t == glcore::GL_FLOAT_VEC4	{
			let mut v = lmath::vector::Vec4::zero::<f32>();
			glcore::glGetUniformfv( *h, loc, v.to_ptr() );
			self.value = UniFloatVec(v);
		}else
		if t == glcore::GL_INT_VEC4	{
			let mut v = lmath::vector::Vec4::zero::<i32>();
			glcore::glGetUniformiv( *h, loc, v.to_ptr() );
			self.value = UniIntVec(v);
		}else
		if t == glcore::GL_FLOAT_MAT4	{
			let mut v = lmath::matrix::Mat4::zero::<f32>();
			glcore::glGetUniformfv( *h, loc, ptr::addr_of(&v.x.x) );
			self.value = UniMatrix(false,v);	
		}else	{return false;}
		true
	}

	priv fn write()	{
		let loc = *self.loc;
		match copy self.value	{
			Unitialized		=> fail(fmt!( "Uninitalized parameter at location %d", loc as int )),
			UniFloat(v)		=> glcore::glUniform1f( loc, v as glcore::GLfloat ),
			UniInt(v)		=> glcore::glUniform1i( loc, v as glcore::GLint ),
			UniFloatVec(v)	=> glcore::glUniform4fv( loc, 1, v.to_ptr() ),
			UniIntVec(v)	=> glcore::glUniform4iv( loc, 1, v.to_ptr() ),
			UniQuat(v)		=> glcore::glUniform4fv( loc, 1, v.to_ptr() ),
			UniMatrix(b,v)	=> glcore::glUniformMatrix4fv( loc, 1, b as glcore::GLboolean, ptr::addr_of(&v.x.x) ),
			UniTexture(u,_)	=> glcore::glUniform1i( loc, u as glcore::GLint ),
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
	priv pool	: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
	}
}

pub struct Program	{
	handle	: Handle,
	alive	: bool,
	info	: ~str,
	attribs	: AttriMap,
	params	: ParaMap,
	priv mut outputs	: ~[~str],
	priv pool			: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
	}
}

impl Program	{
	fn read_parameter( name : ~str )-> Uniform	{
		match self.params.find_ref(&name)	{
			Some(ref par) =>	{
				par.read( self.handle );
				par.value
			},
			None => {Unitialized}
		}
	}
	fn find_output( name : &~str )-> uint	{
		match self.outputs.position_elem(name)	{
			Some(p)	=> p,
			None	=>	{
				/*let mut p = -1 as glcore::GLint;
				do str::as_c_str(*name) |text|	{
					unsafe {
						//FIXME:doesn't work!
						glcore::glGetFragDataLocation( *self.handle, text );
					}
				}
				assert p >= 0;
				let pu = p as uint;*/
				let pu = 0u;
				if self.outputs.len() <= pu	{
					do vec::grow_fn( &mut self.outputs, pu+1u-self.outputs.len()) |_i| {~""};
				}
				self.outputs[pu] = copy *name;
				pu
			}
		}
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
		let loc = glcore::glGetAttribLocation( *h, raw_bytes );
		rez.insert( name, Attribute{ loc:loc as uint, storage:storage, size:size } );
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
		let loc = glcore::glGetUniformLocation( *h, raw_bytes );
		io::println(fmt!( "Discovered param '%s' at location %d", name, loc as int ));
		let p = Parameter{ loc:Location(loc), storage:storage, size:size, value:Unitialized };
		//p.read( h );	// no need to read them here
		rez.insert( name, p );
	}
	rez
}


pure fn check_sampler( target : glcore::GLenum, storage : glcore::GLenum )	{
	if target == glcore::GL_TEXTURE_1D	{
		assert [glcore::GL_SAMPLER_1D].contains( &storage );
	}else
	if target == glcore::GL_TEXTURE_2D	{
		assert [glcore::GL_SAMPLER_2D].contains( &storage );
	}else
	if target == glcore::GL_TEXTURE_3D	{
		assert [glcore::GL_SAMPLER_3D].contains( &storage );
	}else	{
		fail(fmt!( "Unknown texture target: %x", target as uint ));
	}
}

pure fn map_shader_type( t : char )-> glcore::GLenum	{
	if t=='v'	{glcore::GL_VERTEX_SHADER}		else
	if t=='f'	{glcore::GL_FRAGMENT_SHADER}	else
	if t=='g'	{glcore::GL_GEOMETRY_SHADER}	else
	{fail(fmt!( "Unknown shader type: %c", t ))}
}


impl context::Context	{
	pub fn create_shader( t : char, code : &str )-> Object	{
		let target = map_shader_type(t);
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
		Object{ handle:h, target:target, alive:ok, info:message, pool:self.shader.pool_objects }
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
			params	:query_parameters(h),
			outputs :~[], pool:self.shader.pool_programs }
	}

	priv fn _bind_program( h : Handle )	{
		if *self.shader.active_program != *h	{
			self.shader.active_program = h;
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
					let mut val = *value;
					match *value	{
						UniTexture(_,t)	=>	{
							check_sampler( *t.target, par.storage );
							self.texture.bind_to( tex_unit, t );
							val = UniTexture( tex_unit, t );
							tex_unit += 1;
						},
						_	=> {}
					}
					if par.value != val	{
						par.value = val;
						par.write();
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

	fn cleanup_shaders()	{
		while self.shader.pool_objects.len()!=0	{
			let h = self.shader.pool_objects.pop();
			glcore::glDeleteShader( *h );
		}
		while self.shader.pool_programs.len()!=0	{
			let h = self.shader.pool_programs.pop();
			assert *h != 0;
			if h == self.shader.active_program	{
				self.unbind_program();
			}
			glcore::glDeleteProgram( *h );
		}
	}
}


pub fn create_data()-> DataMap	{
	LinearMap::<~str,Uniform>()
}
