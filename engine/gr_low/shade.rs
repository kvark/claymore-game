extern mod glcore;
extern mod lmath;

use core::hashmap::linear::LinearMap;
use core::managed;

use lmath::vec::*;
use lmath::mat::*;

use gr_low::{context,texture};
use journal;


pub struct Location( glcore::GLint );
#[deriving(Eq)]
pub struct ObjectHandle( glcore::GLuint );
#[deriving(Eq)]
pub struct ProgramHandle( glcore::GLuint );


pub struct Binding	{
	priv active		: Option<@Program>,
}

impl context::ProxyState for Binding	{
	fn sync_back( &mut self )-> bool	{
		let mut hid = 0 as glcore::GLint;
		glcore::glGetIntegerv( glcore::GL_CURRENT_PROGRAM, ptr::addr_of(&hid) );
		hid == match self.active	{
			Some(p)	=> *p.handle as glcore::GLint,
			None	=> 0
		}
	}
}

pub impl Binding	{
	fn new()-> Binding	{
		Binding{ active:None }
	}
}


pub enum Uniform	{
	Uninitialized,
	UniFloat(float),
	UniInt(int),
	UniFloatVec(vec4),
	UniIntVec(ivec4),
	UniFloatVecArray(~[vec4]),
	UniMatrix(bool,mat4),
	UniTexture(uint,@texture::Texture,Option<texture::Sampler>),
}

impl cmp::Eq for Uniform	{
	fn eq( &self, v : &Uniform )-> bool	{
		match (self,v)	{
			(&Uninitialized,&Uninitialized)						=> true,
			(&UniFloat(f1),&UniFloat(f2))					=> f1==f2,
			(&UniInt(i1),&UniInt(i2))						=> i1==i2,
			(&UniFloatVec(fv1),&UniFloatVec(fv2))			=> fv1==fv2,
			(&UniIntVec(fi1),&UniIntVec(fi2))				=> fi1==fi2,
			(&UniFloatVecArray(ref fa1),&UniFloatVecArray(ref fa2))	=> fa1==fa2,
			(&UniMatrix(b1,m1),&UniMatrix(b2,m2))			=> b1==b2 && m1==m2,
			(&UniTexture(u1,_,_),&UniTexture(u2,_,_))		=> u1==u2,
			(_,_)											=> false
		}
	}
	fn ne( &self, v : &Uniform )-> bool	{ !self.eq(v) }
}


pub struct Attribute	{
	loc		: uint,
	storage	: glcore::GLenum,
	size	: uint,
}

pub impl Attribute	{
	fn is_integer( &self )-> bool	{
		![glcore::GL_FLOAT,glcore::GL_FLOAT_VEC2,glcore::GL_FLOAT_VEC3,
			glcore::GL_FLOAT_VEC4].contains( &self.storage )
	}
	fn decompose( &self )-> (uint,glcore::GLenum)	{
		match self.storage	{
			glcore::GL_FLOAT_VEC2			=> (2,glcore::GL_FLOAT),
			glcore::GL_FLOAT_VEC3			=> (3,glcore::GL_FLOAT),
			glcore::GL_FLOAT_VEC4			=> (4,glcore::GL_FLOAT),
			glcore::GL_INT_VEC2				=> (2,glcore::GL_INT),
			glcore::GL_INT_VEC3				=> (3,glcore::GL_INT),
			glcore::GL_INT_VEC4				=> (4,glcore::GL_INT),
			glcore::GL_UNSIGNED_INT_VEC2	=> (2,glcore::GL_UNSIGNED_INT),
			glcore::GL_UNSIGNED_INT_VEC3	=> (3,glcore::GL_UNSIGNED_INT),
			glcore::GL_UNSIGNED_INT_VEC4	=> (4,glcore::GL_UNSIGNED_INT),
			_	=> (1,self.storage)
		}
	}
}


struct Parameter	{
	loc		: Location,
	storage	: glcore::GLenum,
	size	: uint,
	value	: @mut Uniform,
}

priv impl Parameter	{
	fn read( &self, h : &ProgramHandle )-> bool	{
		let loc = *self.loc;
		assert!( loc>=0 && self.size==1u );
		*self.value = match self.storage	{
			glcore::GL_FLOAT	=> {
				let mut v = 0f32;
				glcore::glGetUniformfv( **h, loc, ptr::addr_of(&v) );
				UniFloat(v as float)
			},
			glcore::GL_INT	=>	{
				let mut v = 0i32;
				glcore::glGetUniformiv( **h, loc, ptr::addr_of(&v) );
				UniInt(v as int)
			},
			glcore::GL_FLOAT_VEC4	=>	{
				let mut v = vec4::zero();
				glcore::glGetUniformfv( **h, loc, v.to_ptr() );
				UniFloatVec(v)
			},
			glcore::GL_INT_VEC4	=>	{
				let mut v = ivec4::zero();
				glcore::glGetUniformiv( **h, loc, v.to_ptr() );
				UniIntVec(v)
			},
			glcore::GL_FLOAT_MAT4	=>	{
				let mut v = mat4::zero();
				glcore::glGetUniformfv( **h, loc, ptr::addr_of(&v.x.x) );
				UniMatrix(false,v)
			},
			_	=>	{return false}
		};
		true
	}

	fn write( &self )	{
		let loc = *self.loc;
		match &*self.value	{
			&Uninitialized	=> fail!(fmt!( "Uninitialized parameter at location %d", loc as int )),
			&UniFloat(v)	=> glcore::glUniform1f( loc, v as glcore::GLfloat ),
			&UniInt(v)		=> glcore::glUniform1i( loc, v as glcore::GLint ),
			&UniFloatVec(ref v)		=> glcore::glUniform4fv( loc, 1, ptr::addr_of(&v.x) ),
			&UniIntVec(ref v)		=> glcore::glUniform4iv( loc, 1, v.to_ptr() ),
			&UniFloatVecArray(ref v)		=> glcore::glUniform4fv( loc, self.size as glcore::GLint,
				unsafe{vec::raw::to_ptr(*v)} as *glcore::GLfloat ),
			&UniMatrix(b, ref v)			=> glcore::glUniformMatrix4fv( loc, 1, b as glcore::GLboolean, ptr::addr_of(&v.x.x) ),
			&UniTexture(u,_,_)		=> glcore::glUniform1i( loc, u as glcore::GLint ),
		}
	}
}


pub type AttriMap	= LinearMap<~str,Attribute>;
pub type ParaMap	= LinearMap<~str,Parameter>;
pub struct DataMap(	LinearMap<~str,Uniform> );

impl DataMap	{
	pub fn new()-> DataMap	{
		DataMap( LinearMap::new() )
	}
	pub fn log( &self, lg : &journal::Log )	{
		for self.each |&(name,val)|	{
			let sv = match val	{
				&Uninitialized		=> ~"uninitialized",
				&UniFloat(v)		=> fmt!("float(%f)",v),
				&UniInt(v)			=> fmt!("int(%i)",v),
				&UniFloatVec(ref v)	=> fmt!("float4(%f,%f,%f,%f)",
					v.x as float, v.y as float, v.z as float, v.w as float),
				&UniIntVec(ref v)	=> fmt!("int4(%i,%i,%i,%i)",
					v.x as int, v.y as int, v.z as int, v.w as int),
				&UniFloatVecArray(ref _v)		=> ~"float4[]",
				&UniMatrix(b, ref _v)			=> fmt!("mat4(), transpose=%b", b),
				&UniTexture(u, ref t, ref os)	=> fmt!("slot[%u]: %s %s",
					u, t.to_str(), match os	{ &Some(ref s) => s.to_str(), &None => ~"" })
			};
			lg.add(fmt!( "\t\t%s = %s", *name, sv ));
		}
	}
}


struct Object	{
	handle	: ObjectHandle,
	target	: glcore::GLenum,
	alive	: bool,
	info	: ~str,
}


impl Drop for ObjectHandle	{
	fn finalize( &self )	{
		glcore::glDeleteShader( **self );
	}
}


pub struct Program	{
	handle	: ProgramHandle,
	alive	: bool,
	info	: ~str,
	attribs	: AttriMap,
	params	: ParaMap,
	priv outputs	: @mut ~[~str],	//FIXME
}


impl Drop for ProgramHandle	{
	fn finalize( &self )	{
		glcore::glDeleteProgram( **self );
	}
}

pub impl Program	{
	fn read_parameter( &self, name : ~str )-> Uniform	{
		match self.params.find(&name)	{
			Some(par) =>	{
				par.read( &self.handle );
				copy *par.value
			},
			None => Uninitialized
		}
	}
	fn find_output( &self, name : &~str )-> uint	{
		let outs : &mut ~[~str] = self.outputs;
		match outs.position_elem(name)	{
			Some(p)	=> p,
			None	=>	{
				/*let mut p = -1 as glcore::GLint;
				do str::as_c_str(*name) |text|	{
					//FIXME:doesn't work!
					glcore::glGetFragDataLocation( *self.handle, text );
				}
				assert p >= 0;
				let pu = p as uint;*/
				/*let pu = 0u;
				if self.outputs.len() <= pu	{
					do vec::grow_fn( &mut self.outputs, pu+1u-self.outputs.len()) |_i| {~""};
				}
				self.outputs[pu] = copy *name;
				pu*/
				outs.push( copy *name );
				outs.len() - 1u
			}
		}
	}
}

impl context::ProxyState for Program	{
	fn sync_back( &mut self )-> bool	{
		true
	}
}


priv fn query_attributes( h : &ProgramHandle, lg : &journal::Log )-> AttriMap	{
	//assert glcore::glGetError() == 0;
	let num		= 0 as glcore::GLint;
	let max_len	= 0 as glcore::GLint;
	glcore::glGetProgramiv( **h, glcore::GL_ACTIVE_ATTRIBUTES, ptr::addr_of(&num) );
	glcore::glGetProgramiv( **h, glcore::GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, ptr::addr_of(&max_len) );
	let mut info_bytes	= vec::from_elem( max_len as uint, 0 as libc::c_char );
	let raw_bytes		= unsafe{ vec::raw::to_ptr(info_bytes) };
	lg.add(fmt!( "\tQuerying %d attributes:", num as int ));
	let mut rez		= LinearMap::with_capacity::<~str,Attribute>( num as uint );
	for uint::range(0u,num as uint) |i|	{
		let mut length	= 0 as glcore::GLint;
		let mut size	= 0 as glcore::GLint;
		let mut storage	= 0 as glcore::GLenum;
		glcore::glGetActiveAttrib( **h, i as glcore::GLuint, max_len,
			ptr::addr_of(&length), ptr::addr_of(&size),
			ptr::addr_of(&storage), raw_bytes );
		let name = unsafe{ str::raw::from_c_str_len( raw_bytes, length as uint ) };
		info_bytes[length] = 0;
		let loc = glcore::glGetAttribLocation( **h, raw_bytes );
		lg.add(fmt!( "\t\t[%d] = '%s',\tformat %d", loc as int, name, storage as int ));
		rez.insert( name, Attribute{ loc:loc as uint, storage:storage, size:size as uint } );
	}
	rez
}


priv fn query_parameters( h : &ProgramHandle, lg : &journal::Log )-> ParaMap	{
	//assert glcore::glGetError() == 0;
	let num		= 0 as glcore::GLint;
	let max_len	= 0 as glcore::GLint;
	glcore::glGetProgramiv( **h, glcore::GL_ACTIVE_UNIFORMS, ptr::addr_of(&num) );
	glcore::glGetProgramiv( **h, glcore::GL_ACTIVE_UNIFORM_MAX_LENGTH, ptr::addr_of(&max_len) );
	let mut info_bytes	= vec::from_elem( max_len as uint, 0 as libc::c_char );
	let raw_bytes		= unsafe{ vec::raw::to_ptr(info_bytes) };
	lg.add(fmt!( "\tQuerying %d parameters:", num as int ));
	let mut rez		= LinearMap::with_capacity::<~str,Parameter>( num as uint );
	for uint::range(0u,num as uint) |i|	{
		let mut length	= 0 as glcore::GLint;
		let mut size	= 0 as glcore::GLint;
		let mut storage	= 0 as glcore::GLenum;
		glcore::glGetActiveUniform( **h, i as glcore::GLuint, max_len,
			ptr::addr_of(&length), ptr::addr_of(&size),
			ptr::addr_of(&storage), raw_bytes );
		let name = unsafe{ str::raw::from_c_str_len( raw_bytes, length as uint ) };
		info_bytes[length] = 0;
		let loc = glcore::glGetUniformLocation( **h, raw_bytes );
		lg.add(fmt!( "\t\t[%d-%d]\t= '%s',\tformat %d", loc as int, ((loc + size) as int) -1, name, storage as int ));
		let p = Parameter{ loc:Location(loc), storage:storage, size:size as uint, value:@mut Uninitialized };
		//p.read( h );	// no need to read them here
		rez.insert( name, p );
	}
	rez
}


pub fn check_sampler( target : glcore::GLenum, storage : glcore::GLenum )	{
	let expected_target = match storage	{
		glcore::GL_SAMPLER_1D			=> glcore::GL_TEXTURE_1D,
		glcore::GL_SAMPLER_2D			|
		glcore::GL_SAMPLER_2D_SHADOW	=> glcore::GL_TEXTURE_2D,
		glcore::GL_SAMPLER_2D_RECT		=> glcore::GL_TEXTURE_RECTANGLE,
		glcore::GL_SAMPLER_2D_ARRAY		=> glcore::GL_TEXTURE_2D_ARRAY,
		glcore::GL_SAMPLER_3D			=> glcore::GL_TEXTURE_3D,
		_	=> fail!(fmt!( "Unknown sampler: %x", storage as uint ))
	};
	assert!( target == expected_target );
}

pub fn map_shader_type( t : char )-> glcore::GLenum	{
	match t	{
		'v'	=> glcore::GL_VERTEX_SHADER,
		'g' => glcore::GL_GEOMETRY_SHADER,
		'f'	=> glcore::GL_FRAGMENT_SHADER,
		_	=> fail!(fmt!( "Unknown shader type: %c", t ))
	}
}


pub impl context::Context	{
	fn create_shader( &self, t : char, code : &str )-> @Object	{
		let target = map_shader_type(t);
		let h = ObjectHandle( glcore::glCreateShader(target) );
		let mut length = code.len() as glcore::GLint;
		// temporary fix for Linux Radeon HD4000
		do str::as_c_str(str::replace(code,"150 core","140")) |text|	{
			glcore::glShaderSource(	*h, 1i32, ptr::addr_of(&text), ptr::addr_of(&length) );
		}
		glcore::glCompileShader( *h );
		// get info message
		let mut status = 0 as glcore::GLint;
		length = 0;
		let message = unsafe	{
			glcore::glGetShaderiv( *h, glcore::GL_COMPILE_STATUS, ptr::addr_of(&status) );
			glcore::glGetShaderiv( *h, glcore::GL_INFO_LOG_LENGTH, ptr::addr_of(&length) );
			let info_bytes	= vec::from_elem( length as uint, 0 as libc::c_char );
			let raw_bytes	= vec::raw::to_ptr(info_bytes);
			glcore::glGetShaderInfoLog( *h, length, ptr::addr_of(&length), raw_bytes );
			str::raw::from_c_str( raw_bytes )
		};
		let ok = (status != (0 as glcore::GLint));
		if !ok	{
			io::println(~"Failed shader code:");
			io::println(code);
			fail!( ~"\tGLSL " + message )
		}
		@Object{ handle:h, target:target,
			alive:ok, info:message }
	}
	
	fn create_program( &self, shaders : ~[@Object], lg : &journal::Log )-> @Program	{
		let h = ProgramHandle( glcore::glCreateProgram() );
		for shaders.each |s| {
			glcore::glAttachShader( *h, *s.handle );
		}
		glcore::glLinkProgram( *h );
		lg.add(fmt!( "Linked program %d", *h as int ));
		// get info message
		let mut status = 0 as glcore::GLint;
		let mut length = 0 as glcore::GLint;
		let message = unsafe	{
			glcore::glGetProgramiv( *h, glcore::GL_LINK_STATUS, ptr::addr_of(&status) );
			glcore::glGetProgramiv( *h, glcore::GL_INFO_LOG_LENGTH, ptr::addr_of(&length) );
			let info_bytes	= vec::from_elem( length as uint, 0 as libc::c_char );
			let raw_bytes	= vec::raw::to_ptr(info_bytes);
			glcore::glGetProgramInfoLog( *h, length, ptr::addr_of(&length), raw_bytes );
			str::raw::from_c_str( raw_bytes )
		};
		let ok = (status != (0 as glcore::GLint));
		if !ok	{
			fail!( ~"\tGLSL program error: " + message )
		}
		// done
		let attribs	= query_attributes( &h, lg );
		let params	= query_parameters( &h, lg );
		@Program{ handle:h,
			alive:ok, info:message,
			attribs	:attribs,
			params	:params,
			outputs :@mut ~[],
		}
	}

	//FIXME: accept Map trait once HashMap<~str> are supported
	fn bind_program( &mut self, p : @Program, data : &DataMap )->bool	{
		let need_bind = match self.shader.active	{
			Some(prog)	=> !managed::ptr_eq(p,prog),
			None		=> true
		};
		if need_bind	{
			self.shader.active = Some(p);
			glcore::glUseProgram( *p.handle );
		}
		let mut tex_unit = 0;
		for p.params.each() |&(name,par)|	{
			match data.find(name)	{
				Some(&UniTexture(_,t,s_opt))	=> {
					check_sampler( *t.target, par.storage );
					self.texture.bind_to( tex_unit, t );
					match s_opt	{
						Some(ref s) => self.texture.bind_sampler( t, s ),
						None	=> ()
					}
					let old_unit = match *par.value	{
						UniTexture(unit,_,_)	=> unit,
						UniInt(val)				=> val as uint,
						_						=> !tex_unit,
					};
					*par.value = UniTexture( tex_unit, t, s_opt );
					if old_unit != tex_unit	{
						par.write();
					}
					tex_unit += 1;
				},
				Some(value)	=>	{
					if *par.value != *value	{
						//io::println(fmt!( "Uploading value '%s'", *name ));
						*par.value = copy *value;
						par.write();
					}
				},
				None	=>	{
					let msg = match &*par.value	{
						&Uninitialized	=> ~"not inialized",
						_				=> ~"missing",
					};
					fail!(fmt!( "Program %d parameter is %s: name=%s, loc=%d",
						*p.handle as int, msg, *name, *par.loc as int ))
				}
			}
		}
		true
	}

	fn unbind_program( &mut self )	{
		if self.shader.active.is_some()	{
			self.shader.active = None;
			glcore::glUseProgram( 0 );
		}
	}
}
