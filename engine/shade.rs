extern mod std;
extern mod glcore;
extern mod lmath;


pub type Location = glcore::GLint;

pub trait Uniform	{
	fn load( loc : Location );
}

pub impl float : Uniform	{
	fn load( loc : Location )	{
		glcore::glUniform1f( loc, self as glcore::GLfloat );
	}
}

pub impl int : Uniform	{
	fn load( loc : Location )	{
		glcore::glUniform1i( loc, self as glcore::GLint );
	}
}


pub impl lmath::vector::vec4 : Uniform	{
	fn load( loc : Location )	{
		glcore::glUniform4fv( loc, 4, self.to_ptr() );
	}
}

pub impl lmath::vector::ivec4 : Uniform	{
	fn load( loc : Location )	{
		glcore::glUniform4iv( loc, 4, self.to_ptr() );
	}
}

pub impl lmath::quaternion::quat4 : Uniform	{
	fn load( loc : Location )	{
		glcore::glUniform4fv( loc, 4, self.to_ptr() );
	}
}


struct Parameter	{
	loc		: Location,
	storage	: glcore::GLenum,
	size	: glcore::GLint,
	value	: @Uniform	//FIXME (wait for Rust)
}

struct Attribute	{
	loc		: Location,
	storage	: glcore::GLenum,
	size	: glcore::GLint
}

impl @Attribute : Copy	{
}

pub type AttriMap	= std::map::HashMap<~str,@Attribute>;
pub type ParaMap	= std::map::HashMap<~str,@Parameter>;
pub type DataMap	= std::map::HashMap<~str,@Uniform>;

pub struct Shader	{
	handle	: glcore::GLuint,
	target	: glcore::GLenum,
	alive	: bool,
	info	: ~str,

	drop	{
		glcore::glDeleteShader( self.handle );
	}
}

pub struct Program	{
	handle	: glcore::GLuint,
	alive	: bool,
	info	: ~str,
	attribs	: AttriMap,
	params	: ParaMap,
}


pub fn createShader( target : glcore::GLenum, code : &str )-> Shader	{
	let h = glcore::glCreateShader( target );
	let mut length = code.len() as glcore::GLint;
	do str::as_c_str(code) |text|	{
		unsafe {
			glcore::glShaderSource(	h, 1i32, ptr::addr_of(&text), ptr::addr_of(&length) );
		}
	}
	glcore::glCompileShader( h );
	// get info message
	let mut message:~str;
	let mut status = 0 as glcore::GLint;
	length = 0;
	unsafe	{
		glcore::glGetShaderiv( h, glcore::GL_COMPILE_STATUS, ptr::addr_of(&status) );
		glcore::glGetShaderiv( h, glcore::GL_INFO_LOG_LENGTH, ptr::addr_of(&length) );
		let info_bytes = vec::from_elem( length as uint, 0 as libc::c_char );
		let raw_bytes = vec::raw::to_ptr(info_bytes);
		glcore::glGetShaderInfoLog( h, length, ptr::addr_of(&length), raw_bytes );
		message = str::raw::from_c_str( raw_bytes );
	}
	let ok = (status != (0 as glcore::GLint));
	if !ok	{
		io::println( fmt!("Shader: %s",message) );	//TEMP
	}
	Shader{ handle:h, target:target, alive:ok, info:message }
}


fn queryAttributes( h: glcore::GLuint )-> AttriMap	{
	let mut rez = std::map::HashMap::<~str,@Attribute>();
	let mut num		= 0 as glcore::GLint;
	let mut max_len	= 0 as glcore::GLint;
	let mut raw_bytes	: *libc::c_char;
	unsafe	{
		glcore::glGetProgramiv( h, glcore::GL_ACTIVE_ATTRIBUTES, ptr::addr_of(&num) );
		glcore::glGetProgramiv( h, glcore::GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, ptr::addr_of(&max_len) );
		let info_bytes = vec::from_elem( max_len as uint, 0 as libc::c_char );
		raw_bytes = vec::raw::to_ptr(info_bytes);
	}
	while num>(0 as glcore::GLint)	{
		num -= 1;
		let mut length	= 0 as glcore::GLint;
		let mut size	= 0 as glcore::GLint;
		let mut storage	= 0 as glcore::GLenum;
		let mut name : ~str;
		unsafe	{
			glcore::glGetActiveAttrib( h, num as glcore::GLuint, max_len,
				ptr::addr_of(&length), ptr::addr_of(&size),
				ptr::addr_of(&storage), raw_bytes );
			name = str::raw::from_c_str( raw_bytes );
		}
		let location = glcore::glGetAttribLocation( h, raw_bytes );
		rez.insert( name, @Attribute{ loc:location, storage:storage, size:size } );
	}
	rez
}


fn queryParameters( h : glcore::GLuint )-> ParaMap	{
	let mut rez = std::map::HashMap::<~str,@Parameter>();
	let mut num		= 0 as glcore::GLint;
	let mut max_len	= 0 as glcore::GLint;
	let mut raw_bytes	: *libc::c_char;
	unsafe	{
		glcore::glGetProgramiv( h, glcore::GL_ACTIVE_UNIFORMS, ptr::addr_of(&num) );
		glcore::glGetProgramiv( h, glcore::GL_ACTIVE_UNIFORM_MAX_LENGTH, ptr::addr_of(&max_len) );
		let info_bytes = vec::from_elem( max_len as uint, 0 as libc::c_char );
		raw_bytes = vec::raw::to_ptr(info_bytes);
	}
	while num>(0 as glcore::GLint)	{
		num -= 1;
		let mut length	= 0 as glcore::GLint;
		let mut size	= 0 as glcore::GLint;
		let mut storage	= 0 as glcore::GLenum;
		let mut name : ~str;
		unsafe	{
			glcore::glGetActiveUniform( h, num as glcore::GLuint, max_len,
				ptr::addr_of(&length), ptr::addr_of(&size),
				ptr::addr_of(&storage), raw_bytes );
			name = str::raw::from_c_str( raw_bytes );
		}
		let location = glcore::glGetUniformLocation( h, raw_bytes );
		let value = @1 as Uniform;
		rez.insert( name, @Parameter{ loc:location, storage:storage, size:size, value:value } );
	}
	rez
}


pub fn createProgram( shaders : ~[Shader] )-> Program	{
	let h = glcore::glCreateProgram();
	for shaders.each |s| {
		glcore::glAttachShader( h, s.handle );
	}
	glcore::glLinkProgram( h );
	// get info message
	let mut message:~str;
	let mut status = 0 as glcore::GLint;
	let mut length = 0 as glcore::GLint;
	unsafe	{
		glcore::glGetProgramiv( h, glcore::GL_LINK_STATUS, ptr::addr_of(&status) );
		glcore::glGetProgramiv( h, glcore::GL_INFO_LOG_LENGTH, ptr::addr_of(&length) );
		let info_bytes = vec::from_elem( length as uint, 0 as libc::c_char );
		let raw_bytes = vec::raw::to_ptr(info_bytes);
		glcore::glGetProgramInfoLog( h, length, ptr::addr_of(&length), raw_bytes );
		message = str::raw::from_c_str( raw_bytes );
	}
	let ok = (status != (0 as glcore::GLint));
	if (!ok)	{
		io::println( fmt!("Program: %s",message) );	//TEMP
	}
	// done
	Program{ handle:h, alive:ok, info:message,
		attribs	:queryAttributes(h),
		params	:queryParameters(h) }
}

impl Program	{
	fn bind()	{
		glcore::glUseProgram( self.handle );
	}
}
