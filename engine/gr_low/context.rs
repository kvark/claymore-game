extern mod glcore;

use core::io::WriterUtil;

use buf;
use frame;
use rast;
use rast::Stage;
use shade;
use texture;


pub trait ProxyState	{
	// query all
	fn sync_back( &mut self ) -> bool;
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
	fn to_gl_type( &self )-> glcore::GLenum;
}
impl GLType for i8	{
	fn to_gl_type( &self )-> glcore::GLenum	{glcore::GL_BYTE}
}
impl GLType for u8	{
	fn to_gl_type( &self )-> glcore::GLenum	{glcore::GL_UNSIGNED_BYTE}
}
impl GLType for i16	{
	fn to_gl_type( &self )-> glcore::GLenum	{glcore::GL_SHORT}
}
impl GLType for u16	{
	fn to_gl_type( &self )-> glcore::GLenum	{glcore::GL_UNSIGNED_SHORT}
}
impl GLType for i32	{
	fn to_gl_type( &self )-> glcore::GLenum	{glcore::GL_INT}
}
impl GLType for u32	{
	fn to_gl_type( &self )-> glcore::GLenum	{glcore::GL_UNSIGNED_INT}
}
impl GLType for f32	{
	fn to_gl_type( &self )-> glcore::GLenum	{glcore::GL_FLOAT}
}


pub struct ClearData	{
	color	: rast::Color,
	depth	: float,
	stencil	: uint,
}

impl ProxyState for ClearData	{
	fn sync_back( &mut self )-> bool	{
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


pub struct Context	{
	caps				: Capabilities,
	rast				: rast::State,
	priv clear_data		: ClearData,
	// bindings
	shader				: shade::Binding,
	vertex_array		: buf::VaBinding,
	array_buffer		: buf::Binding,
	element_buffer		: buf::Binding,
	render_buffer		: frame::RenBinding,
	frame_buffer_draw	: frame::Binding,
	frame_buffer_read	: frame::Binding,
	texture				: texture::Binding,
	// defaults
	screen_size			: (uint,uint),
	default_rast		: rast::State,
	default_frame_buffer: @mut frame::Buffer,
}


pub fn create( wid : uint, het : uint )-> Context	{
	// read caps
	let caps	= Capabilities{
		max_color_attachments : read_cap( glcore::GL_MAX_COLOR_ATTACHMENTS ),
	};
	let rast	= rast::make_default( wid, het );
	let color	= rast::Color{r:0f32,g:0f32,b:0f32,a:0f32};
	let def_fb	= frame::Buffer::new_default();
	// fill up the context
	Context{
		caps				: caps,
		rast				: copy rast,
		clear_data			: ClearData{ color:color, depth:1f, stencil:0u },
		shader				: shade::Binding::new(),
		vertex_array		: buf::VaBinding::new(),
		array_buffer		: buf::Binding::new( glcore::GL_ARRAY_BUFFER ),
		element_buffer		: buf::Binding::new( glcore::GL_ELEMENT_ARRAY_BUFFER ),
		render_buffer		: frame::RenBinding::new(),
		frame_buffer_draw	: frame::Binding::new( glcore::GL_DRAW_FRAMEBUFFER, def_fb ),
		frame_buffer_read	: frame::Binding::new( glcore::GL_READ_FRAMEBUFFER, def_fb ),
		texture				: texture::Binding::new(),
		screen_size			: (wid,het),
		default_rast		: rast,
		default_frame_buffer: def_fb,
	}
}

impl ProxyState for Context	{
	fn sync_back( &mut self )->bool	{
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

pub impl Context	{
	fn check( &self, where : &str )	{
		let code = glcore::glGetError();
		if code == 0	{return}
		let message = match code	{
			glcore::GL_INVALID_ENUM			=> ~"enum",
			glcore::GL_INVALID_VALUE		=> ~"value",
			glcore::GL_INVALID_OPERATION	=> ~"operation",
			glcore::GL_OUT_OF_MEMORY		=> ~"memory",
			glcore::GL_INVALID_FRAMEBUFFER_OPERATION	=> ~"framebuffer",
			_	=> ~"unknown"
		};
		fail!(fmt!( "%s: GL error 0x%x (%s)", where, code as uint, message ))
	}
	fn set_clear_color( &mut self, c : &rast::Color )	{
		if self.clear_data.color != *c	{
			self.clear_data.color = *c;
			glcore::glClearColor(
				c.r as glcore::GLfloat, c.g as glcore::GLfloat,
				c.b as glcore::GLfloat, c.a as glcore::GLfloat );
		}
	}
	fn set_clear_depth( &mut self, d : float )	{
		if self.clear_data.depth != d	{
			self.clear_data.depth = d;
			glcore::glClearDepth( d as glcore::GLdouble );
		}
	}
	fn set_clear_stencil( &mut self, s : uint )	{
		if self.clear_data.stencil != s	{
			self.clear_data.stencil = s;
			glcore::glClearStencil( s as glcore::GLint );
		}
	}
}


pub struct Log	{
	depth		: uint,
	priv wr		: @io::Writer,
}

pub impl Log	{
	fn create( path : ~str, depth : uint )->Log	{
		match io::file_writer( &path::Path(path), &[io::Create,io::Truncate] )	{
			Ok(wr)	=> Log{ depth:depth, wr:wr },
			Err(e)	=> fail!( e.to_str() ),
		}
	}
	fn add( &self, message : ~str )	{
		let d = str::find(message,char::is_alphanumeric).expect(~"Bad log record");
		if d<self.depth	{
			self.wr.write_line(message)
		}
	}
}
