extern mod glcore;

type Target = glcore::GLenum;
type Handle = glcore::GLuint;

pub trait State	{
	// query all
	fn sync_back()->bool;
}


struct RenderbufferBinding	{
	target		: Target,
	mut active	: Handle	
}
struct FramebufferBinding	{
	target		: Target,
	mut active	: Handle
}



pub struct Context	{
	mut program			: shade::Handle,
	buffer_array		: buf::Binding,
	buffer_element		: buf::Binding,
	renderbuffer		: RenderbufferBinding,
	framebuffer_draw	: FramebufferBinding,
	framebuffer_read	: FramebufferBinding,
	mut vertex_array	: @buf::VertexArray,
	texture				: texture::Binding,
}


pub fn create()->Context	{
	// fill up the context
	let slots	= send_map::linear::LinearMap::<texture::Slot,texture::Handle>();
	Context{
		program				: shade::Handle(0),
		buffer_array		: buf::Binding{		target:buf::Target(glcore::GL_ARRAY_BUFFER),			active:buf::Handle(0) },
		buffer_element		: buf::Binding{		target:buf::Target(glcore::GL_ELEMENT_ARRAY_BUFFER),	active:buf::Handle(0) },
		renderbuffer		: RenderbufferBinding{	target:glcore::GL_RENDERBUFFER,			active:0 as Handle },
		framebuffer_draw	: FramebufferBinding{	target:glcore::GL_DRAW_FRAMEBUFFER,		active:0 as Handle },
		framebuffer_read	: FramebufferBinding{	target:glcore::GL_READ_FRAMEBUFFER,		active:0 as Handle },
		vertex_array		: @buf::VertexArray{ handle:buf::Handle(0), data:~[] },
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
		was_ok &= self.buffer_array.sync_back();
		was_ok &= self.buffer_element.sync_back();
		was_ok &= self.texture.sync_back();
		self.check(~"sync_back");
		was_ok
	}
}