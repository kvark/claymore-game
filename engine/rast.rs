extern mod glcore;

struct Color	{
	r:f32, g:f32, b:f32, a:f32,
}

trait Stage	{
	fn activate( cache : &mut self, poly : uint );
	fn verify();
}


priv fn set_state( state : glcore::GLenum, on : bool )	{
	if on	{
		glcore::glEnable( state );
	}else	{
		glcore::glDisable( state );
	}
}

priv fn ask_state( state : glcore::GLenum )-> bool	{
	glcore::glIsEnabled( state ) == glcore::GL_TRUE
}


struct Primitive	{
	front_cw	: bool,
	cull		: bool,
	cull_mode	: glcore::GLenum,
	line_width	: f32,
}

impl Primitive : Stage	{
	fn activate( cache : &mut Primitive, poly : uint )	{
		if poly == 3u	{
			if cache.front_cw != self.front_cw	{
				cache.front_cw = self.front_cw;
				let mode = if self.front_cw {glcore::GL_CW} else {glcore::GL_CCW};
				glcore::glFrontFace( mode );
			}
			if cache.cull != self.cull	{
				cache.cull = self.cull;
				set_state( glcore::GL_CULL_FACE, self.cull );
			}
			if self.cull && cache.cull_mode != self.cull_mode	{
				cache.cull_mode = self.cull_mode;
				glcore::glCullFace( self.cull_mode );
			}
		}else
		if poly == 2u	{
			if cache.line_width != self.line_width	{
				cache.line_width = self.line_width;
				glcore::glLineWidth( self.line_width );
			}
		}
	}
	fn verify()	{
		let mut front	= 0 as glcore::GLint;
		let mut cmode	= 0 as glcore::GLint;
		let mut lw		= 0 as glcore::GLfloat;
		unsafe	{
			glcore::glGetIntegerv(	glcore::GL_FRONT_FACE,		ptr::addr_of(&front) );
			glcore::glGetIntegerv(	glcore::GL_CULL_FACE_MODE,	ptr::addr_of(&cmode) );
			glcore::glGetFloatv(	glcore::GL_LINE_WIDTH,		ptr::addr_of(&lw) );
		}
		assert self.front_cw == (front==glcore::GL_CW as glcore::GLint) &&
			self.cull == ask_state( glcore::GL_CULL_FACE ) &&
			self.cull_mode == cmode as glcore::GLenum && 
			self.line_width == lw as f32;
	}
}


struct Offset	{
	on		: bool,
	factor	: f32,
	units	: f32,
}

impl Offset : Stage	{
	fn activate( cache : &mut Offset, _poly : uint )	{
		if cache.on != self.on 	{
			cache.on = self.on;
			set_state( glcore::GL_POLYGON_OFFSET_FILL, self.on )
		}
		if self.on && (cache.factor!=self.factor || cache.units!=self.units)	{
			cache.factor = self.factor;
			cache.units = self.units;
			glcore::glPolygonOffset( self.factor, self.units );
		}
	}
	fn verify()	{
		let mut f = 0 as glcore::GLfloat;
		let mut u = 0 as glcore::GLfloat;
		assert self.on == ask_state( glcore::GL_POLYGON_OFFSET_FILL ) &&
			self.factor == f as f32 && self.units == u as f32;
	}
}


struct Scissor	{
	test	: bool,
	area	: frame::Rect,
}

impl Scissor : Stage	{
	fn activate( cache : &mut Scissor, _poly : uint )	{
		if cache.test != self.test	{
			cache.test = self.test;
			set_state( glcore::GL_SCISSOR_TEST, self.test);
		}
		if self.test && cache.area != self.area	{
			cache.area = self.area;
			glcore::glScissor( self.area.x as glcore::GLint, self.area.y as glcore::GLint,
				self.area.w as glcore::GLsizei, self.area.h as glcore::GLsizei );
		}
	}
	fn verify()	{
		let mut r = frame::Rect{x:0u,y:0u,w:0u,h:0u};
		unsafe	{
			glcore::glGetIntegerv( glcore::GL_SCISSOR_BOX, ptr::addr_of(&r.x) as *glcore::GLint );
		}
		assert self.test == ask_state( glcore::GL_SCISSOR_TEST ) &&
			self.area == r;
	}
}


struct Multisample	{
	on		: bool,
	alpha	: bool,
	cover	: bool,
	invert	: bool,
	value	: uint,
}


struct Stencil	{
	test	: bool,
	clear	: uint,
	ref_val	: uint
}

struct Depth	{
	test	: bool,
	clear	: Color
}

struct Blend	{
	on		: bool,
}

struct Mask	{
	stencil	: bool,
	depth	: bool,
	red		: bool,
	green	: bool,
	blue	: bool,
	alpha	: bool
}


pub struct Rast	{
	prime	: Primitive,
	//offset	: Offset,
	//scissor	: Scissor,
	//multi	: Multisample,
	//stencil	: Stencil,
	//depth	: Depth,
	//blend	: Blend,
	//mask	: Mask,
}


impl Rast	{
	//fn activate_primitive( cur : &mut Primitive, new : &Primitive)
}

pub fn create_rast()-> Rast	{
	Rast{
		prime : Primitive{
			front_cw:true, cull:false, cull_mode:glcore::GL_NONE, line_width:1f32
		}
	}
}

pub fn query_rast()-> Rast	{
	create_rast()	//FIXME
}
