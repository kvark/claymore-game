extern mod glcore;

type Handle = glcore::GLuint;
const null:Handle = 0;


struct Buffer	{
	handle : Handle
}
struct BufferBinding	{
	target		: glcore::GLenum,
	mut active	: Handle
}
struct RenderbufferBinding	{
	target		: glcore::GLenum,
	mut active	: Handle	
}
struct FramebufferBinding	{
	target		: glcore::GLenum,
	mut active	: Handle
}


pub struct Context	{
	mut program			: Handle,
	buffer_array		: BufferBinding,
	buffer_element		: BufferBinding,
	renderbuffer		: RenderbufferBinding,
	framebuffer_draw	: FramebufferBinding,
	framebuffer_read	: FramebufferBinding,
	mut active_tex_unit	: uint,
}


pub fn create()->Context	{
	Context{
		program				: null,
		buffer_array		: BufferBinding{		target:glcore::GL_ARRAY_BUFFER,			active:null },
		buffer_element		: BufferBinding{		target:glcore::GL_ELEMENT_ARRAY_BUFFER,	active:null },
		renderbuffer		: RenderbufferBinding{	target:glcore::GL_RENDERBUFFER,			active:null },
		framebuffer_draw	: FramebufferBinding{	target:glcore::GL_DRAW_FRAMEBUFFER,		active:null },
		framebuffer_read	: FramebufferBinding{	target:glcore::GL_READ_FRAMEBUFFER,		active:null },
		active_tex_unit		: 0u,
	}
}

impl Context	{
}