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
	// objects
	shader				: shade::Binding,
	mut vertex_array	: buf::Handle,
	array_buffer		: buf::Binding,
	render_buffer		: frame::RenBinding,
	frame_buffer_draw	: frame::Binding,
	frame_buffer_read	: frame::Binding,
	texture				: texture::Binding,
}


pub fn create( wid : uint, het : uint )-> Context	{
	// read caps
	let caps = Capabilities{
		max_color_attachments : read_cap( glcore::GL_MAX_COLOR_ATTACHMENTS ),
	};
	// fill up the context
	let slots	= send_map::linear::LinearMap::<texture::Slot,texture::Handle>();
	Context{
		caps				: caps,
		rast				: rast::create_rast(wid,het),
		shader				: shade::create_binding(),
		vertex_array		: buf::Handle(0),
		array_buffer		: buf::Binding{	target:buf::Target(glcore::GL_ARRAY_BUFFER),active:buf::Handle(0) },
		render_buffer		: frame::RenBinding{	target:glcore::GL_RENDERBUFFER,			active:frame::Handle(0) },
		frame_buffer_draw	: frame::Binding{	target:glcore::GL_DRAW_FRAMEBUFFER,		active:frame::Handle(0) },
		frame_buffer_read	: frame::Binding{	target:glcore::GL_READ_FRAMEBUFFER,		active:frame::Handle(0) },
		texture				: texture::Binding{ active_unit:0u,	active:slots },
	}
}

impl Context	{
	fn check( where : ~str )	{
		let code = glcore::glGetError();
		if code != 0	{
			fail( fmt!("%s: GL Error: %d",where,code as int) );
		}
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