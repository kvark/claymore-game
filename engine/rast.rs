extern mod glcore;

pub struct Color	{
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


pub struct Primitive	{
	poly_mode	: glcore::GLenum,
	front_cw	: bool,
	cull		: bool,
	cull_mode	: glcore::GLenum,
	line_width	: f32,
}

impl Primitive : Stage	{
	fn activate( cache : &mut Primitive, poly : uint )	{
		if poly == 3u	{
			if cache.poly_mode != self.poly_mode	{
				cache.poly_mode = self.poly_mode;
				glcore::glPolygonMode( glcore::GL_FRONT_AND_BACK, self.poly_mode );
			}
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
		let mut mode	= 0 as glcore::GLint;
		let mut front	= 0 as glcore::GLint;
		let mut cmode	= 0 as glcore::GLint;
		let mut lw		= 0 as glcore::GLfloat;
		unsafe	{
			glcore::glGetIntegerv(	glcore::GL_POLYGON_MODE,	ptr::addr_of(&mode) );
			glcore::glGetIntegerv(	glcore::GL_FRONT_FACE,		ptr::addr_of(&front) );
			glcore::glGetIntegerv(	glcore::GL_CULL_FACE_MODE,	ptr::addr_of(&cmode) );
			glcore::glGetFloatv(	glcore::GL_LINE_WIDTH,		ptr::addr_of(&lw) );
		}
		assert self.poly_mode == mode as glcore::GLenum &&
			self.front_cw == (front==glcore::GL_CW as glcore::GLint) &&
			self.cull == ask_state( glcore::GL_CULL_FACE ) &&
			self.cull_mode == cmode as glcore::GLenum && 
			self.line_width == lw as f32;
	}
}


pub struct Offset	{
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


pub struct Scissor	{
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
		if r.w==0u	{
			// When a GL context is first attached to a window,
			// width and height are set to the dimensions of that window.
			// We can't track that ATM, so we skip the check
			r.w = self.area.w;
			r.h = self.area.h;
		}
		assert self.test == ask_state( glcore::GL_SCISSOR_TEST ) &&
			self.area == r;
	}
}


pub struct Multisample	{
	on		: bool,
	alpha	: bool,
	cover	: bool,
	invert	: bool,
	value	: f32,
}

impl Multisample : Stage	{
	fn activate( cache : &mut Multisample, _poly : uint )	{
		if cache.on != self.on	{
			cache.on = self.on;
			set_state( glcore::GL_MULTISAMPLE, self.on );
		}
		if !self.on	{return;}
		if cache.alpha != self.alpha	{
			cache.alpha = self.alpha;
			set_state( glcore::GL_SAMPLE_ALPHA_TO_COVERAGE, self.alpha );
		}
		if cache.cover != self.cover	{
			cache.cover = self.cover;
			set_state( glcore::GL_SAMPLE_COVERAGE, self.cover );
		}
		if self.cover && (cache.invert!=self.invert || cache.value!=self.value)	{
			cache.invert = self.invert;
			cache.value = self.value;
			glcore::glSampleCoverage( self.value, self.invert as glcore::GLboolean );
		}
	}
	fn verify()	{
		let mut value = 0 as glcore::GLfloat;
		let mut invert = glcore::GL_FALSE;
		unsafe	{
			glcore::glGetFloatv( glcore::GL_SAMPLE_COVERAGE_VALUE,	ptr::addr_of(&value) );
			glcore::glGetBooleanv( glcore::GL_SAMPLE_COVERAGE_INVERT,	ptr::addr_of(&invert) );
		}
		assert self.on == ask_state( glcore::GL_MULTISAMPLE ) &&
			self.alpha == ask_state( glcore::GL_SAMPLE_ALPHA_TO_COVERAGE ) &&
			self.cover == ask_state( glcore::GL_SAMPLE_COVERAGE ) &&
			self.value == value as f32 && self.invert == (invert==glcore::GL_TRUE);
	}
}


pub struct StencilSide	{
	function		: glcore::GLenum,
	ref_value		: uint,
	read_mask		: uint,
	op_fail			: glcore::GLenum,
	op_depth_fail	: glcore::GLenum,
	op_pass			: glcore::GLenum,
}

impl StencilSide : cmp::Eq	{
	pure fn eq( other : &StencilSide )-> bool	{
		self.function == other.function &&
		self.ref_value == other.ref_value &&
		self.read_mask == other.read_mask &&
		self.op_fail == other.op_fail &&
		self.op_depth_fail == other.op_depth_fail &&
		self.op_pass == other.op_pass
	}
	pure fn ne( other : &StencilSide )-> bool	{
		!self.eq( other )
	}
}

impl StencilSide	{
	fn activate( cache : &mut StencilSide, side : glcore::GLenum )	{
		if cache.function!=self.function || cache.ref_value!=self.ref_value || cache.read_mask!=self.read_mask	{
			cache.function = self.function;
			cache.ref_value = self.ref_value;
			cache.read_mask = self.read_mask;
			glcore::glStencilFuncSeparate( side, self.function, self.ref_value as glcore::GLint, self.read_mask as glcore::GLuint );
		}
		if cache.op_fail!=self.op_fail || cache.op_depth_fail!=self.op_depth_fail || cache.op_pass!=self.op_pass	{
			cache.op_fail = self.op_fail;
			cache.op_depth_fail = self.op_depth_fail;
			cache.op_pass = self.op_pass;
			glcore::glStencilOpSeparate( side, self.op_fail, self.op_depth_fail, self.op_pass );
		}
	}
}

priv fn create_stencil()-> StencilSide	{
	StencilSide{
		function:glcore::GL_ALWAYS, ref_value:0u, read_mask:!0,
		op_fail:glcore::GL_KEEP, op_depth_fail:glcore::GL_KEEP, op_pass:glcore::GL_KEEP
	}
}


pub struct Stencil	{
	test	: bool,
	front	: StencilSide,
	back	: StencilSide,
}

impl Stencil : Stage	{
	fn activate( cache : &mut Stencil, _poly : uint )	{
		if cache.test != self.test	{
			cache.test = self.test;
			set_state( glcore::GL_STENCIL_TEST, self.test );
		}
		if !self.test	{return;}
		if self.front == self.back	{
			if cache.front != self.front || cache.back != self.back	{
				cache.front = self.front;
				self.front.activate( &mut cache.back, glcore::GL_FRONT_AND_BACK );
			}
		}else	{
			self.front	.activate( &mut cache.front,	glcore::GL_FRONT );
			self.back	.activate( &mut cache.back,		glcore::GL_BACK );
		}
	}
	fn verify()	{
		let mut vals = vec::from_elem( 12, 0 as glcore::GLint );
		let queries = ~[glcore::GL_STENCIL_FUNC,glcore::GL_STENCIL_REF,glcore::GL_STENCIL_VALUE_MASK,
			glcore::GL_STENCIL_FAIL,glcore::GL_STENCIL_PASS_DEPTH_FAIL,glcore::GL_STENCIL_PASS_DEPTH_PASS,
			glcore::GL_STENCIL_BACK_FUNC,glcore::GL_STENCIL_BACK_REF,glcore::GL_STENCIL_BACK_VALUE_MASK,
			glcore::GL_STENCIL_BACK_FAIL,glcore::GL_STENCIL_BACK_PASS_DEPTH_FAIL,glcore::GL_STENCIL_BACK_PASS_DEPTH_PASS];
		
		for queries.eachi() |i,q|	{
			unsafe	{
				glcore::glGetIntegerv( *q, ptr::addr_of(&vals[i]) );
			}
		}
		assert self.test == ask_state( glcore::GL_STENCIL_TEST ) &&
			self.front.function		== vals[0] as glcore::GLenum && 
			self.front.ref_value	== vals[1] as uint && 
			self.front.read_mask	== vals[2] as uint && 
			self.front.op_fail		== vals[3] as glcore::GLenum &&
			self.front.op_depth_fail== vals[4] as glcore::GLenum &&
			self.front.op_pass		== vals[5] as glcore::GLenum &&
			self.back.function		== vals[6] as glcore::GLenum && 
			self.back.ref_value		== vals[7] as uint && 
			self.back.read_mask		== vals[8] as uint && 
			self.back.op_fail		== vals[9] as glcore::GLenum &&
			self.back.op_depth_fail	== vals[10] as glcore::GLenum &&
			self.back.op_pass		== vals[11] as glcore::GLenum;
	}
}


pub struct Depth	{
	test	: bool,
	clear	: Color
}

pub struct Blend	{
	on		: bool,
}

pub struct Mask	{
	stencil	: bool,
	depth	: bool,
	red		: bool,
	green	: bool,
	blue	: bool,
	alpha	: bool
}


pub struct State	{
	prime	: Primitive,
	offset	: Offset,
	scissor	: Scissor,
	multi	: Multisample,
	stencil	: Stencil,
	//depth	: Depth,
	//blend	: Blend,
	//mask	: Mask,
}


impl State	{
	//fn activate_primitive( cur : &mut Primitive, new : &Primitive)
}

pub fn create_rast()-> State	{
	State{
		prime : Primitive{
			poly_mode:glcore::GL_FILL, front_cw:true, cull:false,
			cull_mode:glcore::GL_NONE, line_width:1f32
		},
		offset : Offset{
			on:false, factor:0f32, units:0f32
		},
		scissor : Scissor{
			test:false, area:frame::Rect{x:0u,y:0u,w:0u,h:0u}
		},
		multi : Multisample{
			on:true, alpha:false, cover:false, value:0f32, invert:false
		},
		stencil : Stencil{
			test:true, front:create_stencil(), back:create_stencil()
		}
	}
}

pub fn query_rast()-> State	{
	create_rast()	//FIXME
}
