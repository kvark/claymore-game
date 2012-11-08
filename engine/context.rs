extern mod glcore;
extern mod std;

type Target = glcore::GLenum;
type Handle = glcore::GLuint;
const NULL	:Handle	= 0;
const MAX_VERTEX_ATTRIBS	:uint	= 8;

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

type Location	= glcore::GLuint;
struct VertexData	{
	enabled	: bool,
}


pub struct Context	{
	mut program			: shade::Handle,
	buffer_array		: buf::Binding,
	buffer_element		: buf::Binding,
	renderbuffer		: RenderbufferBinding,
	framebuffer_draw	: FramebufferBinding,
	framebuffer_read	: FramebufferBinding,
	vertex_data			: ~[VertexData],
	texture				: texture::Binding,
}


pub fn create()->Context	{
	// default VAO
	let mut vao_handle = 0 as glcore::GLuint;
	unsafe	{
		glcore::glGenVertexArrays( 1, ptr::addr_of(&vao_handle) );
	}
	glcore::glBindVertexArray( vao_handle );
	// fill up the context
	let vdata	= VertexData{ enabled:false };
	Context{
		program				: shade::Handle(0),
		buffer_array		: buf::Binding{		target:buf::Target(glcore::GL_ARRAY_BUFFER),			active:buf::Handle(0) },
		buffer_element		: buf::Binding{		target:buf::Target(glcore::GL_ELEMENT_ARRAY_BUFFER),	active:buf::Handle(0) },
		renderbuffer		: RenderbufferBinding{	target:glcore::GL_RENDERBUFFER,			active:NULL },
		framebuffer_draw	: FramebufferBinding{	target:glcore::GL_DRAW_FRAMEBUFFER,		active:NULL },
		framebuffer_read	: FramebufferBinding{	target:glcore::GL_READ_FRAMEBUFFER,		active:NULL },
		vertex_data			: vec::from_elem( MAX_VERTEX_ATTRIBS, vdata ),
		texture				: texture::Binding{ active_unit:0u,
			active:send_map::linear::LinearMap::<texture::Slot,texture::Handle>() },
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
		self.check(~"sync_back");
		was_ok
	}
}