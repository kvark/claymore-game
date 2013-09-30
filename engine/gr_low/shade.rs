extern mod gl;
extern mod cgmath;

use std;
use std::hashmap::HashMap;
use std::{managed,ptr,vec};

use cgmath::array::Array;
use cgmath::vector::*;
use cgmath::matrix::*;

use gr_low::{context,texture};
use journal;


pub struct Location( gl::types::GLint );
#[deriving(Eq)]
pub struct ObjectHandle( gl::types::GLuint );
#[deriving(Eq)]
pub struct ProgramHandle( gl::types::GLuint );


pub struct Binding	{
	priv active		: Option<@Program>,
}

impl context::ProxyState for Binding	{
	fn sync_back( &mut self )-> bool	{
		let mut hid = 0 as gl::types::GLint;
		unsafe{ gl::GetIntegerv( gl::CURRENT_PROGRAM, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		hid == match self.active	{
			Some(p)	=> *p.handle as gl::types::GLint,
			None	=> 0
		}
	}
}

impl Binding	{
	pub fn new()-> Binding	{
		Binding{ active:None }
	}
}


#[deriving(Clone)]
pub enum Uniform	{
	Uninitialized,
	UniFloat(float),
	UniInt(int),
	UniFloatVec(Vec4<f32>),
	UniIntVec(Vec4<i32>),
	UniFloatVecArray(~[Vec4<f32>]),
	UniMatrix(bool,Mat4<f32>),
	UniTexture(uint,@texture::Texture,Option<texture::Sampler>),
}

impl std::cmp::Eq for Uniform	{
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
}


pub struct Attribute	{
	loc		: uint,
	storage	: gl::types::GLenum,
	size	: uint,
}

impl Attribute	{
	pub fn is_integer( &self )-> bool	{
		![gl::FLOAT,gl::FLOAT_VEC2,gl::FLOAT_VEC3,
			gl::FLOAT_VEC4].contains( &self.storage )
	}
	pub fn decompose( &self )-> (uint,gl::types::GLenum)	{
		match self.storage	{
			gl::FLOAT_VEC2			=> (2,gl::FLOAT),
			gl::FLOAT_VEC3			=> (3,gl::FLOAT),
			gl::FLOAT_VEC4			=> (4,gl::FLOAT),
			gl::INT_VEC2				=> (2,gl::INT),
			gl::INT_VEC3				=> (3,gl::INT),
			gl::INT_VEC4				=> (4,gl::INT),
			gl::UNSIGNED_INT_VEC2	=> (2,gl::UNSIGNED_INT),
			gl::UNSIGNED_INT_VEC3	=> (3,gl::UNSIGNED_INT),
			gl::UNSIGNED_INT_VEC4	=> (4,gl::UNSIGNED_INT),
			_	=> (1,self.storage)
		}
	}
}


struct Parameter	{
	loc		: Location,
	storage	: gl::types::GLenum,
	size	: uint,
	value	: @mut Uniform,
}

impl Parameter	{
	fn read( &self, h : &ProgramHandle )-> bool	{
		let loc = *self.loc;
		assert!( loc>=0 && self.size==1u );
		*self.value = match self.storage	{
			gl::FLOAT	=> {
				let mut v = 0f32;
				unsafe{ gl::GetUniformfv( **h, loc, ptr::to_mut_unsafe_ptr(&mut v) ); }
				UniFloat(v as float)
			},
			gl::INT	=>	{
				let mut v = 0i32;
				unsafe{ gl::GetUniformiv( **h, loc, ptr::to_mut_unsafe_ptr(&mut v) ); }
				UniInt(v as int)
			},
			gl::FLOAT_VEC4	=>	{
				let mut v = Vec4::zero();
				unsafe{ gl::GetUniformfv( **h, loc, ptr::to_mut_unsafe_ptr(&mut v.x) ); }
				UniFloatVec(v)
			},
			gl::INT_VEC4	=>	{
				let mut v = Vec4::zero();
				unsafe{ gl::GetUniformiv( **h, loc, ptr::to_mut_unsafe_ptr(&mut v.x) ); }
				UniIntVec(v)
			},
			gl::FLOAT_MAT4	=>	{
				let mut v = Mat4::zero();
				unsafe{ gl::GetUniformfv( **h, loc, ptr::to_mut_unsafe_ptr(&mut v.x.x) ); }
				UniMatrix(false,v)
			},
			_	=>	{return false}
		};
		true
	}

	fn write( &self )	{
		let loc = *self.loc;
		match &*self.value	{
			&Uninitialized	=> fail!("Uninitialized parameter at location %d", loc as int),
			&UniFloat(v)	=> gl::Uniform1f( loc, v as gl::types::GLfloat ),
			&UniInt(v)		=> gl::Uniform1i( loc, v as gl::types::GLint ),
			&UniFloatVec(ref v)		=> unsafe{ gl::Uniform4fv( loc, 1, ptr::to_unsafe_ptr(&v.x) )},
			&UniIntVec(ref v)		=> unsafe{ gl::Uniform4iv( loc, 1, ptr::to_unsafe_ptr(&v.x) )},
			&UniFloatVecArray(ref v)		=> unsafe{
				gl::Uniform4fv( loc, self.size as gl::types::GLint,
					vec::raw::to_ptr(*v) as *gl::types::GLfloat )},
			&UniMatrix(b, ref v)			=> unsafe{
				gl::UniformMatrix4fv( loc, 1, b as gl::types::GLboolean, ptr::to_unsafe_ptr(&v.x.x) )},
			&UniTexture(u,_,_)		=> gl::Uniform1i( loc, u as gl::types::GLint ),
		}
	}
}


pub type AttriMap	= HashMap<~str,Attribute>;
pub type ParaMap	= HashMap<~str,Parameter>;
#[deriving(Clone)]
pub struct DataMap(	HashMap<~str,Uniform> );

impl DataMap	{
	pub fn new()-> DataMap	{
		DataMap( HashMap::new() )
	}
	pub fn log( &self, lg : &journal::Log )	{
		for (name,val) in self.iter()	{
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
				&UniTexture(u, ref t, ref os)	=>	{
					let smp = match os	{
						&Some(ref s) => ~"\n\t\t\t" + s.to_str(),
						&None => ~""
					};
					fmt!("slot[%u]: %s%s", u, t.to_str(), smp)
				},
			};
			lg.add(fmt!( "\t\t%s\t= %s", *name, sv ));
		}
	}
}


struct Object	{
	handle	: ObjectHandle,
	target	: gl::types::GLenum,
	alive	: bool,
	info	: ~str,
}


impl Drop for ObjectHandle	{
	fn drop( &mut self )	{
		gl::DeleteShader( **self );
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
	fn drop( &mut self )	{
		gl::DeleteProgram( **self );
	}
}

impl Program	{
	pub fn read_parameter( &self, name : ~str )-> Uniform	{
		match self.params.find(&name)	{
			Some(par) =>	{
				par.read( &self.handle );
				(*par.value).clone()	//FIXME
			},
			None => Uninitialized
		}
	}
	
	pub fn find_output( &self, name : &~str )-> uint	{
		let outs : &mut ~[~str] = self.outputs;
		match outs.position_elem(name)	{
			Some(p)	=> p,
			None	=>	{
				/*let mut p = -1 as gl::types::GLint;
				do std::str::raw::as_c_str(*name) |text|	{
					//FIXME:doesn't work!
					gl::GetFragDataLocation( *self.handle, text );
				}
				assert p >= 0;
				let pu = p as uint;*/
				/*let pu = 0u;
				if self.outputs.len() <= pu	{
					do vec::grow_fn( &mut self.outputs, pu+1u-self.outputs.len()) |_i| {~""};
				}
				self.outputs[pu] = *name;
				pu*/
				outs.push( (*name).clone() );
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


fn query_attributes( h : &ProgramHandle, lg : &journal::Log )-> AttriMap	{
	//assert gl::GetError() == 0;
	let mut num		= 0 as gl::types::GLint;
	let mut max_len	= 0 as gl::types::GLint;
	unsafe{
		gl::GetProgramiv( **h, gl::ACTIVE_ATTRIBUTES,			ptr::to_mut_unsafe_ptr(&mut num) );
		gl::GetProgramiv( **h, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,	ptr::to_mut_unsafe_ptr(&mut max_len) );
	}
	let mut info_bytes	= vec::from_elem( max_len as uint, 0 as gl::types::GLchar );
	let raw_bytes		= vec::raw::to_mut_ptr(info_bytes);
	lg.add(fmt!( "\tQuerying %d attributes:", num as int ));
	let mut rez	: HashMap<~str,Attribute>	= HashMap::with_capacity::( num as uint );
	for i in range(0u,num as uint)	{
		let mut length	= 0 as gl::types::GLint;
		let mut size	= 0 as gl::types::GLint;
		let mut storage	= 0 as gl::types::GLenum;
		let (name,loc) = unsafe{
			gl::GetActiveAttrib( **h, i as gl::types::GLuint, max_len,
				ptr::to_mut_unsafe_ptr(&mut length), ptr::to_mut_unsafe_ptr(&mut size),
				ptr::to_mut_unsafe_ptr(&mut storage), raw_bytes );
			info_bytes[length] = 0;
			let raw_str = raw_bytes as *gl::types::GLchar;
			let name = std::str::raw::from_c_str( raw_str );
			let loc = gl::GetAttribLocation( **h, raw_str );
			(name,loc)
		};
		lg.add(fmt!( "\t\t[%d] = '%s',\tformat %d", loc as int, name, storage as int ));
		rez.insert( name, Attribute{ loc:loc as uint, storage:storage, size:size as uint } );
	}
	rez
}


fn query_parameters( h : &ProgramHandle, lg : &journal::Log )-> ParaMap	{
	//assert gl::GetError() == 0;
	let mut num		= 0 as gl::types::GLint;
	let mut max_len	= 0 as gl::types::GLint;
	unsafe{
		gl::GetProgramiv( **h, gl::ACTIVE_UNIFORMS,				ptr::to_mut_unsafe_ptr(&mut num) );
		gl::GetProgramiv( **h, gl::ACTIVE_UNIFORM_MAX_LENGTH,	ptr::to_mut_unsafe_ptr(&mut max_len) );
	}
	let mut info_bytes	= vec::from_elem( max_len as uint, 0 as gl::types::GLchar );
	let raw_bytes		= vec::raw::to_mut_ptr(info_bytes);
	lg.add(fmt!( "\tQuerying %d parameters:", num as int ));
	let mut rez	: HashMap<~str,Parameter>	= HashMap::with_capacity( num as uint );
	for i in range(0u,num as uint)	{
		let mut length	= 0 as gl::types::GLint;
		let mut size	= 0 as gl::types::GLint;
		let mut storage	= 0 as gl::types::GLenum;
		let (name,loc) = unsafe{
			gl::GetActiveUniform( **h, i as gl::types::GLuint, max_len,
				ptr::to_mut_unsafe_ptr(&mut length), ptr::to_mut_unsafe_ptr(&mut size),
				ptr::to_mut_unsafe_ptr(&mut storage), raw_bytes );
			info_bytes[length] = 0;
			let raw_str = raw_bytes as *gl::types::GLchar;
			let name = std::str::raw::from_c_str( raw_str );
			let loc = gl::GetUniformLocation( **h, raw_str );
			(name,loc)
		};
		lg.add(fmt!( "\t\t[%d-%d]\t= '%s',\tformat %d", loc as int, ((loc + size) as int) -1, name, storage as int ));
		let p = Parameter{ loc:Location(loc), storage:storage, size:size as uint, value:@mut Uninitialized };
		//p.read( h );	// no need to read them here
		rez.insert( name, p );
	}
	rez
}


pub fn check_sampler( target : gl::types::GLenum, storage : gl::types::GLenum )	{
	let expected_target = match storage	{
		gl::SAMPLER_1D			=> gl::TEXTURE_1D,
		gl::SAMPLER_2D			|
		gl::SAMPLER_2D_SHADOW	=> gl::TEXTURE_2D,
		gl::SAMPLER_2D_RECT		=> gl::TEXTURE_RECTANGLE,
		gl::SAMPLER_2D_ARRAY		=> gl::TEXTURE_2D_ARRAY,
		gl::SAMPLER_3D			=> gl::TEXTURE_3D,
		_	=> fail!("Unknown sampler: %x", storage as uint)
	};
	assert!( target == expected_target );
}

pub fn map_shader_type( t : char )-> gl::types::GLenum	{
	match t	{
		'v'	=> gl::VERTEX_SHADER,
		'g' => gl::GEOMETRY_SHADER,
		'f'	=> gl::FRAGMENT_SHADER,
		_	=> fail!("Unknown shader type: %c", t)
	}
}


impl context::Context	{
	pub fn create_shader( &self, t : char, code : &str )-> @Object	{
		assert_eq!( std::sys::size_of::<gl::types::GLchar>(), 1 );
		let target = map_shader_type(t);
		let h = ObjectHandle( gl::CreateShader(target) );
		let mut length = code.len() as gl::types::GLint;
		// temporary fix for Linux Radeon HD4000
		code.replace("150 core","140").with_c_str( |text|	{
			unsafe{
				gl::ShaderSource(	*h, 1i32, ptr::to_unsafe_ptr(&text), ptr::to_unsafe_ptr(&length) );
			}
		});
		gl::CompileShader( *h );
		// get info message
		let mut status = 0 as gl::types::GLint;
		length = 0;
		let message = unsafe	{
			gl::GetShaderiv( *h, gl::COMPILE_STATUS,	ptr::to_mut_unsafe_ptr(&mut status) );
			gl::GetShaderiv( *h, gl::INFO_LOG_LENGTH,	ptr::to_mut_unsafe_ptr(&mut length) );
			let mut info_bytes	= vec::from_elem( length as uint, 0 as gl::types::GLchar );
			let raw_bytes		= vec::raw::to_mut_ptr( info_bytes );
			gl::GetShaderInfoLog( *h, length, ptr::to_mut_unsafe_ptr(&mut length), raw_bytes );
			std::str::raw::from_buf_len( raw_bytes as *u8, length as uint )
		};
		let ok = (status != (0 as gl::types::GLint));
		if !ok	{
			print!( "Failed shader code:\n{}\n", code );
			fail!( ~"\tGLSL " + message )
		}
		@Object{ handle:h, target:target,
			alive:ok, info:message }
	}
	
	pub fn create_program( &self, shaders : &[@Object], lg : &journal::Log )-> @Program	{
		let h = ProgramHandle( gl::CreateProgram() );
		for s in shaders.iter() {
			gl::AttachShader( *h, *s.handle );
		}
		gl::LinkProgram( *h );
		lg.add(fmt!( "Linked program %d", *h as int ));
		// get info message
		let mut status = 0 as gl::types::GLint;
		let mut length = 0 as gl::types::GLint;
		let message = unsafe	{
			gl::GetProgramiv( *h, gl::LINK_STATUS,		ptr::to_mut_unsafe_ptr(&mut status) );
			gl::GetProgramiv( *h, gl::INFO_LOG_LENGTH,	ptr::to_mut_unsafe_ptr(&mut length) );
			let mut info_bytes	= vec::from_elem( length as uint, 0 as gl::types::GLchar );
			let raw_bytes		= vec::raw::to_mut_ptr( info_bytes );
			gl::GetProgramInfoLog( *h, length, ptr::to_mut_unsafe_ptr(&mut length), raw_bytes );
			std::str::raw::from_buf_len( raw_bytes as *u8, length as uint )
		};
		let ok = (status != (0 as gl::types::GLint));
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
	pub fn bind_program( &mut self, p : @Program, data : &DataMap )->bool	{
		let need_bind = match self.shader.active	{
			Some(prog)	=> !managed::ptr_eq(p,prog),
			None		=> true
		};
		if need_bind	{
			self.shader.active = Some(p);
			gl::UseProgram( *p.handle );
		}
		let mut tex_unit = 0;
		for (name,par) in p.params.iter()	{
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
						*par.value = (*value).clone();
						par.write();
					}
				},
				None	=>	{
					let msg = match par.value	{
						@Uninitialized	=> ~"not inialized",
						_				=> ~"missing",
					};
					fail!("Program %d parameter is %s: name=%s, loc=%d",
						*p.handle as int, msg, *name, *par.loc as int)
				}
			}
		}
		true
	}

	pub fn unbind_program( &mut self )	{
		if self.shader.active.is_some()	{
			self.shader.active = None;
			gl::UseProgram( 0 );
		}
	}
}
