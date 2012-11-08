extern mod glcore;
extern mod std;

type Target = glcore::GLenum;
type Handle = glcore::GLuint;
const NULL	:Handle	= 0;
const MAX_VERTEX_ATTRIBS	:uint	= 8;
const MAX_TEX_UNITS			:uint	= 16;

trait State	{
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


struct TexUnit	{
	active	: std::map::HashMap<Target,Handle>
}
struct TexBinding	{
	target	: Target
}
struct Texture	{
	mut active_unit	: uint,
	units			: ~[TexUnit],
	bindings		: std::map::HashMap<Target,TexBinding>
}


pub struct Context	{
	mut program			: Handle,
	buffer_array		: buf::Binding,
	buffer_element		: buf::Binding,
	renderbuffer		: RenderbufferBinding,
	framebuffer_draw	: FramebufferBinding,
	framebuffer_read	: FramebufferBinding,
	vertex_data			: ~[VertexData],
	texture				: Texture,
}


pub fn create()->Context	{
	let unit	= TexUnit{ active : std::map::HashMap::<Target,Handle>() };
	let vdata	= VertexData{ enabled:false };
	Context{
		program				: NULL,
		buffer_array		: buf::Binding{		target:glcore::GL_ARRAY_BUFFER,			active:NULL },
		buffer_element		: buf::Binding{		target:glcore::GL_ELEMENT_ARRAY_BUFFER,	active:NULL },
		renderbuffer		: RenderbufferBinding{	target:glcore::GL_RENDERBUFFER,			active:NULL },
		framebuffer_draw	: FramebufferBinding{	target:glcore::GL_DRAW_FRAMEBUFFER,		active:NULL },
		framebuffer_read	: FramebufferBinding{	target:glcore::GL_READ_FRAMEBUFFER,		active:NULL },
		vertex_data			: vec::from_elem( MAX_VERTEX_ATTRIBS, vdata ),
		texture	: Texture{
			active_unit		: 0u,
			units			: vec::from_elem( MAX_TEX_UNITS, unit ),
			bindings		: std::map::HashMap::<Target,TexBinding>(),
		},
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

impl Context:State	{
	fn sync_back()->bool	{
		let mut was_ok = true;
		let _program = self.get_active_program();
		if _program != self.program	{
			self.program = _program;
			was_ok = false;
		}
		self.check(~"sync_back");
		was_ok
	}
}