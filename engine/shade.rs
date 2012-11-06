extern mod std;
extern mod glcore;
extern mod lmath;


pub type Location = glcore::GLint;

pub trait Uniform	{
	fn load( loc : Location );
}

struct Parameter	{
	loc		: Location,
	storage	: glcore::GLenum,
	value	: @Uniform	//FIXME (wait for Rust)
}

pub type DataMap = std::map::HashMap<~str,@Uniform>;



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


pub struct Shader	{
	handle	: glcore::GLuint,
	target	: glcore::GLenum,
	alive	: bool,
	info	: ~str,

	drop	{
		glcore::glDeleteShader( self.handle );
	}
}

pub fn createShader( target : glcore::GLenum, code : &str )-> Shader	{
	let h = glcore::glCreateShader( target );
	let mut length = code.len() as i32;
	do str::as_c_str(code) |text|	{
		unsafe {
			glcore::glShaderSource(	h, 1i32, ptr::addr_of(&text), ptr::addr_of(&length) );
		}
	}
	glcore::glCompileShader( h );
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
	if (!ok)	{
		io::println( fmt!("Shader: %s",message) );	//TEMP
	}
	return Shader{ handle:h, target:target, alive:ok, info:message };
}
