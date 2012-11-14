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
	mut program			: shade::Handle,
	mut vertex_array	: buf::Handle,
	array_buffer		: buf::Binding,
	renderbuffer		: frame::RenBinding,
	framebuffer_draw	: frame::Binding,
	framebuffer_read	: frame::Binding,
	texture				: texture::Binding,
}


pub fn create()-> Context	{
	// read caps
	let caps = Capabilities{
		max_color_attachments : read_cap( glcore::GL_MAX_COLOR_ATTACHMENTS ),
	};
	// fill up the context
	let slots	= send_map::linear::LinearMap::<texture::Slot,texture::Handle>();
	Context{
		caps				: caps,
		rast				: rast::create_rast(),
		program				: shade::Handle(0),
		vertex_array		: buf::Handle(0),
		array_buffer		: buf::Binding{	target:buf::Target(glcore::GL_ARRAY_BUFFER),active:buf::Handle(0) },
		renderbuffer		: frame::RenBinding{	target:glcore::GL_RENDERBUFFER,			active:frame::Handle(0) },
		framebuffer_draw	: frame::Binding{	target:glcore::GL_DRAW_FRAMEBUFFER,		active:frame::Handle(0) },
		framebuffer_read	: frame::Binding{	target:glcore::GL_READ_FRAMEBUFFER,		active:frame::Handle(0) },
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
		let _program = self.get_active_program();
		if *_program != *self.program	{
			self.program = _program;
			was_ok = false;
		}
		was_ok &= self.array_buffer.sync_back();
		was_ok &= self.renderbuffer.sync_back();
		was_ok &= self.framebuffer_draw.sync_back();
		was_ok &= self.framebuffer_read.sync_back();
		was_ok &= self.texture.sync_back();
		self.check(~"sync_back");
		was_ok
	}
}