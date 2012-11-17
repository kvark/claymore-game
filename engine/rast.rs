extern mod glcore;


pub struct Color	{
	r:f32, g:f32, b:f32, a:f32,
}

impl Color : cmp::Eq	{
	pure fn eq( other : &Color )-> bool	{
		self.r==other.r && self.g==other.g && self.b==other.b && self.a==other.a
	}
	pure fn ne( other : &Color )-> bool	{
		!self.eq( other )
	}
}


priv trait Stage	{
	fn activate( &mut self, new : &self, poly : uint );
	fn verify( &mut self );	//FIXME:waiting for the Copy derivation
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
	fn activate( &mut self, new : &Primitive, poly : uint )	{
		if poly == 3u	{
			if self.poly_mode != new.poly_mode	{
				self.poly_mode = new.poly_mode;
				glcore::glPolygonMode( glcore::GL_FRONT_AND_BACK, new.poly_mode );
			}
			if self.front_cw != new.front_cw	{
				self.front_cw = new.front_cw;
				let mode = if new.front_cw {glcore::GL_CW} else {glcore::GL_CCW};
				glcore::glFrontFace( mode );
			}
			if self.cull != new.cull	{
				self.cull = new.cull;
				set_state( glcore::GL_CULL_FACE, new.cull );
			}
			if new.cull && self.cull_mode != new.cull_mode	{
				self.cull_mode = new.cull_mode;
				glcore::glCullFace( new.cull_mode );
			}
		}else
		if poly == 2u	{
			if self.line_width != new.line_width	{
				self.line_width = new.line_width;
				glcore::glLineWidth( new.line_width );
			}
		}
	}
	fn verify( &mut self )	{
		//let mut mode	= 0 as glcore::GLint;
		let mut front	= 0 as glcore::GLint;
		let mut cmode	= 0 as glcore::GLint;
		let mut lw		= 0 as glcore::GLfloat;
		unsafe	{	//crashes on that... (OSX 10.8)
			//glcore::glGetIntegerv(	glcore::GL_POLYGON_MODE,	ptr::addr_of(&mode) );
			glcore::glGetIntegerv(	glcore::GL_FRONT_FACE,		ptr::addr_of(&front) );
			glcore::glGetIntegerv(	glcore::GL_CULL_FACE_MODE,	ptr::addr_of(&cmode) );
			glcore::glGetFloatv(	glcore::GL_LINE_WIDTH,		ptr::addr_of(&lw) );
		}
		assert //self.poly_mode == mode as glcore::GLenum &&
			self.front_cw == (front==glcore::GL_CW as glcore::GLint) &&
			self.cull == ask_state( glcore::GL_CULL_FACE ) &&
			self.cull_mode == cmode as glcore::GLenum && 
			self.line_width == lw as f32;
	}
}


pub struct Offset	{
	on_fill	: bool,
	on_line : bool,
	on_point: bool,
	factor	: f32,
	units	: f32,
}

impl Offset : Stage	{
	fn activate( &mut self, new : &Offset, _poly : uint )	{
		if self.on_fill != new.on_fill 	{
			self.on_fill = new.on_fill;
			set_state( glcore::GL_POLYGON_OFFSET_FILL, new.on_fill )
		}
		if self.on_line != new.on_line	{
			self.on_line = new.on_line;
			set_state( glcore::GL_POLYGON_OFFSET_LINE, new.on_line )
		}
		if self.on_point != new.on_point	{
			self.on_point = new.on_point;
			set_state( glcore::GL_POLYGON_OFFSET_POINT, new.on_point )
		}
		let on = new.on_fill || new.on_line || new.on_point;
		if on && (self.factor!=new.factor || self.units!=new.units)	{
			self.factor = new.factor;
			self.units = new.units;
			glcore::glPolygonOffset( new.factor, new.units );
		}
	}
	fn verify( &mut self )	{
		let mut f = 0 as glcore::GLfloat;
		let mut u = 0 as glcore::GLfloat;
		unsafe	{
			glcore::glGetFloatv( glcore::GL_POLYGON_OFFSET_FACTOR,	ptr::addr_of(&f) );
			glcore::glGetFloatv( glcore::GL_POLYGON_OFFSET_UNITS,	ptr::addr_of(&u) );
		}
		assert
			self.on_fill	== ask_state( glcore::GL_POLYGON_OFFSET_FILL ) &&
			self.on_line	== ask_state( glcore::GL_POLYGON_OFFSET_LINE ) &&
			self.on_point	== ask_state( glcore::GL_POLYGON_OFFSET_POINT ) &&
			self.factor	== f as f32 && self.units == u as f32;
	}
}


pub struct Scissor	{
	test	: bool,
	area	: frame::Rect,
}

impl Scissor : Stage	{
	fn activate( &mut self, new : &Scissor, _poly : uint )	{
		if self.test != new.test	{
			self.test = new.test;
			set_state( glcore::GL_SCISSOR_TEST, new.test);
		}
		if new.test && self.area != new.area	{
			self.area = new.area;
			glcore::glScissor( new.area.x as glcore::GLint, new.area.y as glcore::GLint,
				new.area.w as glcore::GLsizei, new.area.h as glcore::GLsizei );
		}
	}
	fn verify( &mut self )	{
		let mut v = vec::from_elem( 4, 0 as glcore::GLint );
		unsafe	{
			glcore::glGetIntegerv( glcore::GL_SCISSOR_BOX, vec::raw::to_ptr(v) );
		}
		let mut r = frame::Rect{ x:v[0] as uint, y:v[1] as uint, w:v[2] as uint, h:v[3] as uint };
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
	fn activate( &mut self, new : &Multisample, _poly : uint )	{
		if self.on != new.on	{
			self.on = new.on;
			set_state( glcore::GL_MULTISAMPLE, new.on );
		}
		if !new.on	{return;}
		if self.alpha != new.alpha	{
			self.alpha = new.alpha;
			set_state( glcore::GL_SAMPLE_ALPHA_TO_COVERAGE, new.alpha );
		}
		if self.cover != new.cover	{
			self.cover = new.cover;
			set_state( glcore::GL_SAMPLE_COVERAGE, new.cover );
		}
		if new.cover && (self.invert!=new.invert || self.value!=new.value)	{
			self.invert = new.invert;
			self.value = new.value;
			glcore::glSampleCoverage( new.value, new.invert as glcore::GLboolean );
		}
	}
	fn verify( &mut self )	{
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
	fn activate( &mut self, new : &StencilSide, side : glcore::GLenum )	{
		if self.function!=new.function || self.ref_value!=new.ref_value || self.read_mask!=new.read_mask	{
			self.function = new.function;
			self.ref_value = new.ref_value;
			self.read_mask = new.read_mask;
			glcore::glStencilFuncSeparate( side, new.function, new.ref_value as glcore::GLint, new.read_mask as glcore::GLuint );
		}
		if self.op_fail!=new.op_fail || self.op_depth_fail!=new.op_depth_fail || self.op_pass!=new.op_pass	{
			self.op_fail = new.op_fail;
			self.op_depth_fail = new.op_depth_fail;
			self.op_pass = new.op_pass;
			glcore::glStencilOpSeparate( side, new.op_fail, new.op_depth_fail, new.op_pass );
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
	fn activate( &mut self, new : &Stencil, _poly : uint )	{
		if self.test != new.test	{
			self.test = new.test;
			set_state( glcore::GL_STENCIL_TEST, new.test );
		}
		if !new.test	{return;}
		if new.front == new.back	{
			if self.front != new.front || self.back != new.back	{
				self.front = new.front;
				self.back.activate( &new.back, glcore::GL_FRONT_AND_BACK );
			}
		}else	{
			self.front	.activate( &new.front,	glcore::GL_FRONT );
			self.back	.activate( &new.back,	glcore::GL_BACK );
		}
	}
	fn verify( &mut self )	{
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
	fun		: glcore::GLenum,
	r0		: f32,
	r1		: f32,
}

impl Depth : Stage	{
	fn activate( &mut self, new : &Depth, _poly : uint )	{
		if self.test != new.test	{
			self.test = new.test;
			set_state( glcore::GL_DEPTH_TEST, new.test );
		}
		if !new.test	{return;}
		if self.fun != new.fun	{
			self.fun = new.fun;
			glcore::glDepthFunc( new.fun );
		}
		if self.r0 != new.r0 || self.r1 != new.r1	{
			self.r0 = new.r0;
			self.r1 = new.r1;
			glcore::glDepthRange( new.r0 as glcore::GLdouble, new.r1 as glcore::GLdouble )
		}
	}
	fn verify( &mut self )	{
		let mut val = 0 as glcore::GLint;
		let mut r = vec::from_elem( 2, 0 as glcore::GLfloat );
		unsafe	{
			glcore::glGetIntegerv(	glcore::GL_DEPTH_FUNC, ptr::addr_of(&val) );
			glcore::glGetFloatv(	glcore::GL_DEPTH_RANGE, vec::raw::to_ptr(r) );
		}
		assert self.test == ask_state( glcore::GL_DEPTH_TEST ) &&
			self.fun == val as glcore::GLenum &&
			self.r0 == r[0] as f32 && self.r1 == r[1] as f32;
	}
}


pub struct Blend	{
	on		: bool,
}

pub struct Mask	{
	//TODO: different draw buffers
	stencil	: bool,
	depth	: bool,
	red		: bool,
	green	: bool,
	blue	: bool,
	alpha	: bool,
}

impl Mask : Stage	{
	fn activate( &mut self, new : &Mask, _poly : uint )	{
		if self.stencil != new.stencil	{
			self.stencil = new.stencil;
			glcore::glStencilMask( new.stencil as glcore::GLuint );
		}
		if self.depth != new.depth	{
			self.depth = new.depth;
			glcore::glDepthMask( new.depth as glcore::GLboolean );
		}
		if self.red!=new.red || self.green!=new.green || self.blue!=new.blue || self.alpha!=new.alpha	{
			self.red = new.red;
			self.green = new.green;
			self.blue = new.blue;
			self.alpha = new.alpha;
			glcore::glColorMask( new.red as glcore::GLboolean, new.green as glcore::GLboolean,
				new.blue as glcore::GLboolean, new.alpha as glcore::GLboolean );
		}
	}
	fn verify( &mut self )	{
		let bools = vec::from_elem( 6, false as glcore::GLboolean );
		unsafe	{
			glcore::glGetBooleanv( glcore::GL_COLOR_WRITEMASK,	vec::raw::to_ptr(bools) );
			glcore::glGetBooleanv( glcore::GL_DEPTH_WRITEMASK,	ptr::addr_of(&bools[4]) );
			glcore::glGetBooleanv( glcore::GL_STENCIL_WRITEMASK,ptr::addr_of(&bools[5]) );
		}
		assert
			self.red	== (bools[0]==glcore::GL_TRUE) &&
			self.green	== (bools[1]==glcore::GL_TRUE) &&
			self.blue	== (bools[2]==glcore::GL_TRUE) &&
			self.alpha	== (bools[3]==glcore::GL_TRUE) &&
			self.depth	== (bools[4]==glcore::GL_TRUE) &&
			self.stencil== (bools[5]==glcore::GL_TRUE);
	}
}


pub struct State	{
	prime	: Primitive,
	offset	: Offset,
	scissor	: Scissor,
	multi	: Multisample,
	stencil	: Stencil,
	depth	: Depth,
	//blend	: Blend,
	mask	: Mask,
}


impl State : Stage	{
	//FIXME
	fn activate( &mut self, new : &State, poly : uint )	{
		self.prime	.activate( &new.prime,		poly );
		self.offset	.activate( &new.offset,		poly );
		self.scissor.activate( &new.scissor,	poly );
		self.multi	.activate( &new.multi, 		poly );
		self.stencil.activate( &new.stencil,	poly );
		self.depth	.activate( &new.depth,		poly );
	}
	fn verify( &mut self )	{
		self.prime	.verify();
		self.offset	.verify();
		self.scissor.verify();
		self.multi	.verify();
		self.stencil.verify();
		self.depth	.verify();
	}
}

// Creates a default GL context rasterizer state
// make sure to verify that it matches GL specification
pub fn create_rast( wid : uint, het : uint )-> State	{
	State{
		prime : Primitive{
			poly_mode:glcore::GL_FILL, front_cw:false, cull:false,
			cull_mode:glcore::GL_BACK, line_width:1f32
		},
		offset : Offset{
			on_fill:false, on_line:false, on_point:false, factor:0f32, units:0f32
		},
		scissor : Scissor{
			test:false, area:frame::Rect{x:0u,y:0u,w:wid,h:het}
		},
		multi : Multisample{
			on:true, alpha:false, cover:false, value:1f32, invert:false
		},
		stencil : Stencil{
			test:false, front:create_stencil(), back:create_stencil()
		},
		depth : Depth{
			test:false, fun:glcore::GL_LESS, r0:0f32, r1:1f32
		},
		mask : Mask{
			stencil:true, depth:true, red:true, green:true, blue:true, alpha:true
		}
	}
}
