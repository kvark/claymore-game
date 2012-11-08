extern mod glcore;

type Handle = glcore::GLuint;
const null:Handle = 0;

struct Buffer	{
	handle : Handle
}
struct BufferBinding	{
	target	: glcore::GLenum,
	active	: Handle
}
struct FramebufferBinding	{
	target	: glcore::GLenum,
	active	: Handle
}

struct Context	{
	buffer_array		: BufferBinding,
	buffer_element		: BufferBinding,
	framebuffer_draw	: FramebufferBinding,
	framebuffer_read	: FramebufferBinding,
}

pub fn create_context()->~Context	{
	~Context{
		buffer_array		: BufferBinding{		target:glcore::GL_ARRAY_BUFFER,			active:null },
		buffer_element		: BufferBinding{		target:glcore::GL_ELEMENT_ARRAY_BUFFER,	active:null },
		framebuffer_draw	: FramebufferBinding{	target:glcore::GL_DRAW_FRAMEBUFFER,		active:null },
		framebuffer_read	: FramebufferBinding{	target:glcore::GL_READ_FRAMEBUFFER,		active:null },
	}
}