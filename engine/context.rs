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


pub struct Context	{
	caps				: Capabilities,
	mut rast			: rast::State,
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
	// fill up the context
	Context{
		caps				: caps,
		rast				: rast::create_rast(wid,het),
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

impl Context	{
	fn check( where : ~str )	{
		let code = glcore::glGetError();
		if code != 0	{
			fail( fmt!("%s: GL Error: %d",where,code as int) );
		}
	}
	fn cleanup()	{
		self.cleanup_shaders();
		self.cleanup_buffers();
		self.cleanup_frames();
		self.cleanup_textures();
	}
}

impl Context : State	{
	fn sync_back()->bool	{
		let mut was_ok = true;
		self.rast.verify();
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