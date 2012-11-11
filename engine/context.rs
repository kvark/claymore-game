extern mod glcore;

pub trait State	{
	// query all
	fn sync_back()->bool;
}


pub struct Context	{
	mut program			: shade::Handle,
	mut vertex_array	: buf::Handle,
	renderbuffer		: frame::Binding,
	framebuffer_draw	: frame::Binding,
	framebuffer_read	: frame::Binding,
	texture				: texture::Binding,
}


pub fn create()-> Context	{
	// fill up the context
	let slots	= send_map::linear::LinearMap::<texture::Slot,texture::Handle>();
	Context{
		program				: shade::Handle(0),
		vertex_array		: buf::Handle(0),
		renderbuffer		: frame::Binding{	target:glcore::GL_RENDERBUFFER,			active:frame::Handle(0) },
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
		was_ok &= self.texture.sync_back();
		self.check(~"sync_back");
		was_ok
	}
}