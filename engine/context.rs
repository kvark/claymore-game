extern mod glcore;

pub trait State	{
	// query all
	fn sync_back()->bool;
}

pub struct Capabilities	{
	max_color_attachments : uint,
}

priv fn read_cap( what : glcore::GLenum )-> uint	{
	let mut value = 0 as glcore::GLint;
	unsafe	{
		glcore::glGetIntegerv( what, ptr::addr_of(&value) );
	}
	value as uint
}


pub trait GLType	{
	pure fn to_gl_type()-> glcore::GLenum;
}
impl i8 : GLType	{
	pure fn to_gl_type()-> glcore::GLenum	{glcore::GL_BYTE}
}
impl u8 : GLType	{
	pure fn to_gl_type()-> glcore::GLenum	{glcore::GL_UNSIGNED_BYTE}
}
impl i16 : GLType	{
	pure fn to_gl_type()-> glcore::GLenum	{glcore::GL_SHORT}
}
impl u16 : GLType	{
	pure fn to_gl_type()-> glcore::GLenum	{glcore::GL_UNSIGNED_SHORT}
}
impl i32 : GLType	{
	pure fn to_gl_type()-> glcore::GLenum	{glcore::GL_INT}
}
impl u32 : GLType	{
	pure fn to_gl_type()-> glcore::GLenum	{glcore::GL_UNSIGNED_INT}
}
impl f32 : GLType	{
	pure fn to_gl_type()-> glcore::GLenum	{glcore::GL_FLOAT}
}


pub struct ClearData	{
	mut color	: rast::Color,
	mut depth	: float,
	mut stencil	: uint,
}


pub struct Context	{
	caps				: Capabilities,
	mut rast			: rast::State,
	priv clear_data		: ClearData,
	// bindings
	shader				: shade::Binding,
	vertex_array		: buf::VaBinding,
	array_buffer		: buf::Binding,
	render_buffer		: frame::RenBinding,
	frame_buffer_draw	: frame::Binding,
	frame_buffer_read	: frame::Binding,
	texture				: texture::Binding,
	// defaults
	default_vertex_array: @buf::VertexArray,
	default_frame_buffer: @frame::Buffer,
}


pub fn create( wid : uint, het : uint )-> Context	{
	// read caps
	let caps = Capabilities{
		max_color_attachments : read_cap( glcore::GL_MAX_COLOR_ATTACHMENTS ),
	};
	let color = rast::Color{r:0f32,g:0f32,b:0f32,a:0f32};
	// fill up the context
	Context{
		caps				: caps,
		rast				: rast::create_rast(wid,het),
		clear_data			: ClearData{ color:color, depth:1f, stencil:0u },
		shader				: shade::create_binding(),
		vertex_array		: buf::create_va_binding(),
		array_buffer		: buf::create_binding( glcore::GL_ARRAY_BUFFER ),
		render_buffer		: frame::create_ren_binding(),
		frame_buffer_draw	: frame::create_binding( glcore::GL_DRAW_FRAMEBUFFER ),
		frame_buffer_read	: frame::create_binding( glcore::GL_READ_FRAMEBUFFER ),
		texture				: texture::create_binding(),
		default_vertex_array: @buf::default_vertex_array(),
		default_frame_buffer: @frame::default_frame_buffer(wid,het),
	}
}


impl ClearData : State	{
	fn sync_back()-> bool	{
		let mut color = vec::from_elem( 4, 0 as glcore::GLfloat );
		let mut depth = 0 as glcore::GLdouble;
		let mut stencil = 0 as glcore::GLint;
		unsafe	{
			glcore::glGetFloatv(	glcore::GL_COLOR_CLEAR_VALUE,	vec::raw::to_ptr(color)	);
			glcore::glGetDoublev(	glcore::GL_DEPTH_CLEAR_VALUE,	ptr::addr_of(&depth)	);
			glcore::glGetIntegerv(	glcore::GL_STENCIL_CLEAR_VALUE,	ptr::addr_of(&stencil)	);
		}
		self.color.r==color[0] as f32 &&
		self.color.g==color[1] as f32 &&
		self.color.b==color[2] as f32 &&
		self.color.a==color[3] as f32 &&
		self.depth==depth as float &&
		self.stencil==stencil as uint
	}
}


impl Context	{
	fn check( where : ~str )	{
		let code = glcore::glGetError();
		if code != 0	{
			let decode =
				if code	== glcore::GL_INVALID_ENUM		{~"(enum)"}		else
				if code	== glcore::GL_INVALID_VALUE		{~"(value)"}	else
				if code	== glcore::GL_INVALID_OPERATION	{~"(operation)"}else
				if code	== glcore::GL_OUT_OF_MEMORY		{~"(memory)"}	else
				{~"(unknown)"};
			fail( fmt!("%s: GL Error: %d %s",where,code as int,decode) );
		}
	}
	fn cleanup()	{
		self.cleanup_shaders();
		self.cleanup_buffers();
		self.cleanup_frames();
		self.cleanup_textures();
	}
	fn set_clear_color( c : &rast::Color )	{
		if self.clear_data.color != *c	{
			self.clear_data.color = *c;
			glcore::glClearColor(
				c.r as glcore::GLfloat, c.g as glcore::GLfloat,
				c.b as glcore::GLfloat, c.a as glcore::GLfloat );
		}
	}
	fn set_clear_depth( d : float )	{
		if self.clear_data.depth != d	{
			self.clear_data.depth = d;
			glcore::glClearDepth( d as glcore::GLdouble );
		}
	}
	fn set_clear_stencil( s : uint )	{
		if self.clear_data.stencil != s	{
			self.clear_data.stencil = s;
			glcore::glClearStencil( s as glcore::GLint );
		}
	}
}


impl Context : State	{
	fn sync_back()->bool	{
		let mut was_ok = true;
		self.rast.verify();
		was_ok &= self.clear_data.sync_back();
		was_ok &= self.shader.sync_back();
		was_ok &= self.array_buffer.sync_back();
		was_ok &= self.render_buffer.sync_back();
		was_ok &= self.frame_buffer_draw.sync_back();
		was_ok &= self.frame_buffer_read.sync_back();
		was_ok &= self.texture.sync_back();
		self.check(~"sync_back");
		was_ok
	}
}