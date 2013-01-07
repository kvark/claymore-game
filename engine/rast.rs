extern mod glcore;


#[deriving_eq]
pub struct Color	{
	r:f32, g:f32, b:f32, a:f32,
}

pub pure fn make_color( hex : uint )-> Color	{
	let k = 1f32/255f32;
	Color{
		r : (((hex>>24)&0xFF) as f32) * k,
		g : (((hex>>16)&0xFF) as f32) * k,
		b : (((hex>> 8)&0xFF) as f32) * k,
		a : (((hex>> 0)&0xFF) as f32) * k,
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


pub enum Viewport = frame::Rect;

impl Viewport : Stage	{
	fn activate( &mut self, new : &Viewport, _poly : uint )	{
		if **self == **new 	{return}
		**self = **new;
		glcore::glViewport( new.x as glcore::GLint, new.y as glcore::GLint,
			new.w as glcore::GLsizei, new.h as glcore::GLsizei );
	}
	fn verify( &mut self )	{
		let v = vec::from_elem( 4, 0 as glcore::GLint );
		unsafe	{
			glcore::glGetIntegerv( glcore::GL_VIEWPORT, vec::raw::to_ptr(v) );
		}
		assert
			self.x == v[0] as uint &&
			self.y == v[1] as uint &&
			self.w == v[2] as uint &&
			self.h == v[3] as uint;
	}
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


#[deriving_eq]
pub struct StencilSide	{
	function		: glcore::GLenum,
	ref_value		: int,
	read_mask		: int,
	op_fail			: glcore::GLenum,
	op_depth_fail	: glcore::GLenum,
	op_pass			: glcore::GLenum,
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

priv pure fn create_stencil()-> StencilSide	{
	StencilSide{
		function:glcore::GL_ALWAYS, ref_value:0, read_mask:-1,
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
			self.front.ref_value	== vals[1] as int && 
			self.front.read_mask	== vals[2] as int && 
			self.front.op_fail		== vals[3] as glcore::GLenum &&
			self.front.op_depth_fail== vals[4] as glcore::GLenum &&
			self.front.op_pass		== vals[5] as glcore::GLenum &&
			self.back.function		== vals[6] as glcore::GLenum && 
			self.back.ref_value		== vals[7] as int && 
			self.back.read_mask		== vals[8] as int && 
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

pub struct BlendChannel	{
	equation	: glcore::GLenum,
	source		: glcore::GLenum,
	destination	: glcore::GLenum,
}

impl BlendChannel	{
	fn verify( &mut self, we : glcore::GLenum, ws : glcore::GLenum, wd : glcore::GLenum )	{
		let mut v = vec::from_elem( 3, 0 as glcore::GLint );
		unsafe	{
			glcore::glGetIntegerv( we, ptr::addr_of(&v[0]) );
			glcore::glGetIntegerv( ws, ptr::addr_of(&v[1]) );
			glcore::glGetIntegerv( wd, ptr::addr_of(&v[2]) );
		}
		assert self.equation	== v[0] as glcore::GLenum &&
			self.source			== v[1] as glcore::GLenum &&
			self.destination	== v[2] as glcore::GLenum;
	}
}

priv pure fn create_blend()-> BlendChannel	{
	BlendChannel{ equation:glcore::GL_FUNC_ADD, source:glcore::GL_ONE, destination:glcore::GL_ZERO }
}

pub struct Blend	{
	on		: bool,
	color	: BlendChannel,
	alpha	: BlendChannel,
	value	: Color,
}

impl Blend : Stage	{
	fn activate( &mut self, new : &Blend, _poly : uint )	{
		if self.on != new.on	{
			self.on = new.on;
			set_state( glcore::GL_BLEND, new.on );
		}
		if !new.on	{return;}
		if self.color.equation!=new.color.equation || self.alpha.equation!=new.alpha.equation	{
			self.color.equation = new.color.equation;
			self.alpha.equation = new.alpha.equation;
			if new.color.equation == new.alpha.equation	{
				glcore::glBlendEquation( new.color.equation );
			}else	{
				glcore::glBlendEquationSeparate( new.color.equation, new.alpha.equation );
			}
		}
		if    self.color.source!=new.color.source || self.color.destination!=new.color.destination ||
			  self.alpha.source!=new.alpha.source || self.alpha.destination!=new.alpha.destination	{
			self.color = new.color;
			self.alpha = new.alpha;
			if new.color.source==new.alpha.source && new.color.destination==new.alpha.destination	{
				glcore::glBlendFunc( new.color.source, new.color.destination );
			}else	{
				glcore::glBlendFuncSeparate(
					new.color.source, new.color.destination,
					new.alpha.source, new.alpha.destination );
			}
		}
		if self.value != new.value	{
			self.value = new.value;
			glcore::glBlendColor(
				new.value.r as glcore::GLfloat, new.value.g as glcore::GLfloat,
				new.value.b as glcore::GLfloat, new.value.a as glcore::GLfloat );
		}

	}
	fn verify( &mut self )	{
		assert self.on == ask_state( glcore::GL_BLEND );
		self.color.verify( glcore::GL_BLEND_EQUATION_RGB,	glcore::GL_BLEND_SRC_RGB,	glcore::GL_BLEND_DST_RGB	);
		self.alpha.verify( glcore::GL_BLEND_EQUATION_ALPHA,	glcore::GL_BLEND_SRC_ALPHA,	glcore::GL_BLEND_DST_ALPHA	);
		let mut cv = vec::from_elem( 4, 0 as glcore::GLfloat );
		unsafe	{
			glcore::glGetFloatv( glcore::GL_BLEND_COLOR, vec::raw::to_ptr(cv) );
		}
		assert
			cv[0] == self.value.r as f32 &&
			cv[1] == self.value.g as f32 &&
			cv[2] == self.value.b as f32 &&
			cv[3] == self.value.a as f32;
	}
}


pub struct Mask	{
	//TODO: different draw buffers
	stencil_front	: int,
	stencil_back	: int,
	depth	: bool,
	red		: bool,
	green	: bool,
	blue	: bool,
	alpha	: bool,
}

impl Mask : Stage	{
	fn activate( &mut self, new : &Mask, _poly : uint )	{
		if self.stencil_front != new.stencil_front || self.stencil_back != new.stencil_back	{
			self.stencil_front = new.stencil_front;
			self.stencil_back = new.stencil_back;
			glcore::glStencilMaskSeparate(
				new.stencil_front as glcore::GLuint,
				new.stencil_back as glcore::GLuint );
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
		let bools	= vec::from_elem( 5, false as glcore::GLboolean );
		let sf		= 0 as glcore::GLint;
		let sb		= 0 as glcore::GLint;
		unsafe	{
			glcore::glGetBooleanv( glcore::GL_COLOR_WRITEMASK,	vec::raw::to_ptr(bools) );
			glcore::glGetBooleanv( glcore::GL_DEPTH_WRITEMASK,	ptr::addr_of(&bools[4]) );
			glcore::glGetIntegerv( glcore::GL_STENCIL_WRITEMASK,		ptr::addr_of(&sf) );
			glcore::glGetIntegerv( glcore::GL_STENCIL_BACK_WRITEMASK,	ptr::addr_of(&sb) );
		}
		assert
			self.red	== (bools[0]==glcore::GL_TRUE) &&
			self.green	== (bools[1]==glcore::GL_TRUE) &&
			self.blue	== (bools[2]==glcore::GL_TRUE) &&
			self.alpha	== (bools[3]==glcore::GL_TRUE) &&
			self.depth	== (bools[4]==glcore::GL_TRUE) &&
			self.stencil_front	== sf as int	&&
			self.stencil_back	== sb as int;
	}
}


pub struct State	{
	view	: Viewport,
	prime	: Primitive,
	offset	: Offset,
	scissor	: Scissor,
	multi	: Multisample,
	stencil	: Stencil,
	depth	: Depth,
	blend	: Blend,
	mask	: Mask,
}


impl State : Stage	{
	//FIXME
	fn activate( &mut self, new : &State, poly : uint )	{
		self.view	.activate( &new.view,		poly );
		self.prime	.activate( &new.prime,		poly );
		self.offset	.activate( &new.offset,		poly );
		self.scissor.activate( &new.scissor,	poly );
		self.multi	.activate( &new.multi, 		poly );
		self.stencil.activate( &new.stencil,	poly );
		self.depth	.activate( &new.depth,		poly );
		self.blend	.activate( &new.blend,		poly );
		self.mask	.activate( &new.mask,		poly );
	}
	fn verify( &mut self )	{
		self.view	.verify();
		self.prime	.verify();
		self.offset	.verify();
		self.scissor.verify();
		self.multi	.verify();
		self.stencil.verify();
		self.depth	.verify();
		self.blend	.verify();
		self.mask	.verify();
	}
}


pub pure fn map_polygon_fill( dim : int )-> glcore::GLenum	{
	[glcore::GL_POINT,glcore::GL_LINE,glcore::GL_FILL][dim-1]
}

pub pure fn map_comparison( s : ~str )-> glcore::GLenum	{
	match s	{
		~"!"	=> glcore::GL_NEVER,
		~"*"	=> glcore::GL_ALWAYS,
		~"=="	=> glcore::GL_EQUAL,
		~"!="	=> glcore::GL_NOTEQUAL,
		~"<"	=> glcore::GL_LESS,
		~"<="	=> glcore::GL_LEQUAL,
		~">"	=> glcore::GL_GREATER,
		~">="	=> glcore::GL_GEQUAL,
		_		=> fail(fmt!( "Can not recognize comparison %s", s ))
	}
}

pub pure fn map_operation( c : char )-> glcore::GLenum	{
	match c	{
		'.'	=> glcore::GL_KEEP,
		'0'	=> glcore::GL_ZERO,
		'='	=> glcore::GL_REPLACE,
		'!' => glcore::GL_INVERT,
		'+'	=> glcore::GL_INCR,
		'-' => glcore::GL_DECR,
		'^' => glcore::GL_INCR_WRAP,
		'v' => glcore::GL_DECR_WRAP,
		_	=> fail(fmt!( "Can not recognize stencil operation '%c'", c ))
	}
}

pub pure fn map_equation( s : ~str )-> glcore::GLenum	{
	match s	{
		~"s+d"	=> glcore::GL_FUNC_ADD,
		~"s-d"	=> glcore::GL_FUNC_SUBTRACT,
		~"d-s"	=> glcore::GL_FUNC_REVERSE_SUBTRACT,
		~"max"	=> glcore::GL_MAX,
		~"min"	=> glcore::GL_MIN,
		_		=> fail(fmt!( "Can not recognize blend equation %s", s ))
	}
}

pub pure fn map_factor( s : ~str )-> glcore::GLenum	{
	match s	{
		~"0"	=> glcore::GL_ZERO,
		~"1"	=> glcore::GL_ONE,
		~"Sc"	=> glcore::GL_SRC_COLOR,
		~"1-Sc"	=> glcore::GL_ONE_MINUS_SRC_COLOR,
		~"Dc"	=> glcore::GL_DST_COLOR,
		~"1-Dc"	=> glcore::GL_ONE_MINUS_DST_COLOR,
		~"Cc"	=> glcore::GL_CONSTANT_COLOR,
		~"1-Cc"	=> glcore::GL_ONE_MINUS_CONSTANT_COLOR,
		~"Sa"	=> glcore::GL_SRC_ALPHA,
		~"1-Sa"	=> glcore::GL_ONE_MINUS_SRC_ALPHA,
		~"Da"	=> glcore::GL_DST_ALPHA,
		~"1-Da"	=> glcore::GL_ONE_MINUS_DST_ALPHA,
		~"Ca"	=> glcore::GL_CONSTANT_ALPHA,
		~"1-Ca"	=> glcore::GL_ONE_MINUS_CONSTANT_ALPHA,
		~"Sa^"	=> glcore::GL_SRC_ALPHA_SATURATE,
		_		=> fail(fmt!( "Can not recognize blend factor %s", s ))
	}
}


impl State	{
	pub fn set_stencil( &mut self, fun : ~str, cf : char, cdf : char, cp : char, mask : int )	{
		self.stencil.test = true;
		self.stencil.front.function			= map_comparison(fun);
		self.stencil.front.op_fail			= map_operation(cf);
		self.stencil.front.op_depth_fail	= map_operation(cdf);
		self.stencil.front.op_pass			= map_operation(cp);
		self.stencil.back = self.stencil.front;
		self.mask.stencil_front = mask;
		self.mask.stencil_back = mask;
	}
	pub fn set_depth( &mut self, fun : ~str, mask : bool )	{
		self.depth.test = true;
		self.depth.fun = map_comparison(fun);
		self.mask.depth = mask;
	}
	pub fn set_blend( &mut self, eq : ~str, src : ~str, dst : ~str )	{
		self.blend.on = true;
		self.blend.color.equation	= map_equation(eq);
		self.blend.color.source		= map_factor(src);
		self.blend.color.destination= map_factor(dst);
		self.blend.alpha = self.blend.color;
	}
}


// Creates a default GL context rasterizer state
// make sure to verify that it matches GL specification
pub pure fn make_default( wid : uint, het : uint )-> State	{
	State{
		view : Viewport( frame::make_rect(wid,het) ),
		prime : Primitive{
			poly_mode:glcore::GL_FILL, front_cw:false, cull:false,
			cull_mode:glcore::GL_BACK, line_width:1f32
		},
		offset : Offset{
			on_fill:false, on_line:false, on_point:false, factor:0f32, units:0f32
		},
		scissor : Scissor{
			test:false, area:frame::make_rect(wid,het)
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
		blend : Blend{
			on:false, color:create_blend(), alpha:create_blend(),
			value:Color{r:0f32,g:0f32,b:0f32,a:0f32}
		},
		mask : Mask{
			stencil_front:-1, stencil_back:-1, depth:true,
			red:true, green:true, blue:true, alpha:true
		}
	}
}
