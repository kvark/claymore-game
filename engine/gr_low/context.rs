extern mod gl;

use std;
use std::ptr;

use gr_low::{buf,frame,rast,shade,texture};
use gr_low::rast::Stage;


pub trait ProxyState	{
	// query all
	fn sync_back( &mut self ) -> bool;
}

pub struct Capabilities	{
	max_color_attachments : uint,
}

fn read_cap( what : gl::types::GLenum )-> uint	{
	let mut value = 0 as gl::types::GLint;
	unsafe	{
		gl::GetIntegerv( what, ptr::to_mut_unsafe_ptr(&mut value) );
	}
	value as uint
}


pub trait GLType	{
	fn to_gl_type( &self )-> gl::types::GLenum;
}
impl GLType for i8	{
	fn to_gl_type( &self )-> gl::types::GLenum	{gl::BYTE}
}
impl GLType for u8	{
	fn to_gl_type( &self )-> gl::types::GLenum	{gl::UNSIGNED_BYTE}
}
impl GLType for i16	{
	fn to_gl_type( &self )-> gl::types::GLenum	{gl::SHORT}
}
impl GLType for u16	{
	fn to_gl_type( &self )-> gl::types::GLenum	{gl::UNSIGNED_SHORT}
}
impl GLType for i32	{
	fn to_gl_type( &self )-> gl::types::GLenum	{gl::INT}
}
impl GLType for u32	{
	fn to_gl_type( &self )-> gl::types::GLenum	{gl::UNSIGNED_INT}
}
impl GLType for f32	{
	fn to_gl_type( &self )-> gl::types::GLenum	{gl::FLOAT}
}


pub struct ClearData	{
	color	: rast::Color,
	depth	: f32,
	stencil	: u32,
}

impl ProxyState for ClearData	{
	fn sync_back( &mut self )-> bool	{
		let mut color = std::vec::from_elem( 4, 0 as gl::types::GLfloat );
		let mut depth = 0 as gl::types::GLdouble;
		let mut stencil = 0 as gl::types::GLint;
		unsafe	{
			gl::GetFloatv(	gl::COLOR_CLEAR_VALUE,	color.as_mut_ptr()	);
			gl::GetDoublev(	gl::DEPTH_CLEAR_VALUE,	ptr::to_mut_unsafe_ptr(&mut depth)		);
			gl::GetIntegerv(gl::STENCIL_CLEAR_VALUE,ptr::to_mut_unsafe_ptr(&mut stencil)	);
		}
		self.color.r==color[0] as f32 &&
		self.color.g==color[1] as f32 &&
		self.color.b==color[2] as f32 &&
		self.color.a==color[3] as f32 &&
		self.depth==depth as f32 &&
		self.stencil==stencil as u32
	}
}


pub struct Context	{
	caps				: Capabilities,
	rast				: rast::State,
	priv clear_data		: ClearData,
	call_count			: uint,
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
	default_rast		: rast::State,
	default_frame_buffer: @mut frame::Buffer,
}


pub fn create( loader: |&str|->Option<extern "system" fn()>,
		wid: uint, het: uint, ns: uint )-> Context	{
	// init GL
	gl::load_with( loader );
	// read caps
	let caps	= Capabilities{
		max_color_attachments : read_cap( gl::MAX_COLOR_ATTACHMENTS ),
	};
	let rast	= rast::make_default( wid, het );
	let color	= rast::Color{ r:0.0, g:0.0, b:0.0, a:0.0 };
	let rbind = frame::RenBinding::new( wid, het ,ns );
	let def_fb	= frame::Buffer::new_default( rbind.default );
	// fill up the context
	Context{
		caps				: caps,
		rast				: rast,
		clear_data			: ClearData{ color:color, depth:1.0, stencil:0 },
		call_count			: 0,
		shader				: shade::Binding::new(),
		vertex_array		: buf::VaBinding::new(),
		array_buffer		: buf::Binding::new( gl::ARRAY_BUFFER ),
		element_buffer		: buf::Binding::new( gl::ELEMENT_ARRAY_BUFFER ),
		render_buffer		: rbind,
		frame_buffer_draw	: frame::Binding::new( gl::DRAW_FRAMEBUFFER, def_fb ),
		frame_buffer_read	: frame::Binding::new( gl::READ_FRAMEBUFFER, def_fb ),
		texture				: texture::Binding::new(),
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
		self.check("sync_back");
		was_ok
	}
}

impl Context	{
	pub fn get_screen_size( &self )-> (uint,uint)	{
		let rb = self.render_buffer.default;
		(rb.width, rb.height)
	}
	pub fn check( &self, where : &str )	{
		let code = gl::GetError();
		if code == 0	{return}
		let message = match code	{
			gl::INVALID_ENUM			=> ~"enum",
			gl::INVALID_VALUE		=> ~"value",
			gl::INVALID_OPERATION	=> ~"operation",
			gl::OUT_OF_MEMORY		=> ~"memory",
			gl::INVALID_FRAMEBUFFER_OPERATION	=> ~"framebuffer",
			_	=> ~"unknown"
		};
		fail!("{:s}: GL error 0x{:x} ({:s})", where, code as uint, message)
	}
	pub fn set_clear_color( &mut self, c : &rast::Color )	{
		if self.clear_data.color != *c	{
			self.clear_data.color = *c;
			gl::ClearColor(
				c.r as gl::types::GLfloat, c.g as gl::types::GLfloat,
				c.b as gl::types::GLfloat, c.a as gl::types::GLfloat );
		}
	}
	pub fn set_clear_depth( &mut self, d : f32 )	{
		if self.clear_data.depth != d	{
			self.clear_data.depth = d;
			gl::ClearDepth( d as gl::types::GLdouble );
		}
	}
	pub fn set_clear_stencil( &mut self, s : u32 )	{
		if self.clear_data.stencil != s	{
			self.clear_data.stencil = s;
			gl::ClearStencil( s as gl::types::GLint );
		}
	}
}
