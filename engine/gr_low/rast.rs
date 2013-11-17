extern mod gl;

use std::{ptr,vec};

use gr_low::frame;


#[deriving(Clone,Eq)]
pub struct Color	{
	r:f32, g:f32, b:f32, a:f32,
}

impl Color	{
	pub fn new( hex : uint )-> Color	{
		let k = 1f32/255f32;
		Color{
			r	: (((hex>>24)&0xFF) as f32) * k,
			g	: (((hex>>16)&0xFF) as f32) * k,
			b	: (((hex>> 8)&0xFF) as f32) * k,
			a	: (((hex>> 0)&0xFF) as f32) * k,
		}
	}
	
	pub fn from_array3( a : &[f32] )-> Color	{
		Color{
			r	: a[0],
			g	: a[1],
			b	: a[2],
			a	: 1f32,
		}
	}
}


pub trait Stage	{
	fn activate( &mut self, &Self, uint );
	fn verify( &self );
}


fn set_state( state : gl::types::GLenum, on : bool )	{
	if on	{
		gl::Enable( state );
	}else	{
		gl::Disable( state );
	}
}

fn ask_state( state : gl::types::GLenum )-> bool	{
	gl::IsEnabled( state ) == gl::TRUE
}


#[deriving(Clone)]
pub struct Viewport( frame::Rect );

impl Stage for Viewport	{
	fn activate( &mut self, new : &Viewport, _poly : uint )	{
		if **self == **new 	{return}
		**self = **new;
		gl::Viewport( new.x as gl::types::GLint, new.y as gl::types::GLint,
			new.w as gl::types::GLsizei, new.h as gl::types::GLsizei );
	}
	fn verify( &self )	{
		let mut v = vec::from_elem( 4, 0 as gl::types::GLint );
		unsafe	{
			gl::GetIntegerv( gl::VIEWPORT, vec::raw::to_mut_ptr(v) );
		}
		assert!(
			self.x == v[0] as uint &&
			self.y == v[1] as uint &&
			self.w == v[2] as uint &&
			self.h == v[3] as uint );
	}
}


#[deriving(Clone)]
pub struct Primitive	{
	poly_mode	: gl::types::GLenum,
	front_cw	: bool,
	cull		: bool,
	cull_mode	: gl::types::GLenum,
	line_width	: f32,
}

impl Primitive	{
	pub fn get_poly_size( &self )-> uint	{
		match self.poly_mode	{
			gl::FILL	=> 3u,
			gl::LINE	=> 2u,
			gl::POINT	=> 1u,
			_	=> fail!( "Unknown poly mode: {:i}", self.poly_mode as int )
		}
	}
}

pub fn map_poly_mode( dim : uint )-> gl::types::GLenum	{
	match dim	{
		1	=> gl::POINT,
		2	=> gl::LINE,
		3	=> gl::FILL,
		_	=> fail!( "Unknown poly dim: {:u}", dim )
	}
}

impl Stage for Primitive	{
	fn activate( &mut self, new : &Primitive, poly : uint )	{
		if poly == 3u	{
			if self.poly_mode != new.poly_mode	{
				self.poly_mode = new.poly_mode;
				gl::PolygonMode( gl::FRONT_AND_BACK, new.poly_mode );
			}
			if self.front_cw != new.front_cw	{
				self.front_cw = new.front_cw;
				let mode = if new.front_cw {gl::CW} else {gl::CCW};
				gl::FrontFace( mode );
			}
			if self.cull != new.cull	{
				self.cull = new.cull;
				set_state( gl::CULL_FACE, new.cull );
			}
			if new.cull && self.cull_mode != new.cull_mode	{
				self.cull_mode = new.cull_mode;
				gl::CullFace( new.cull_mode );
			}
		}else
		if self.line_width != new.line_width	{
			self.line_width = new.line_width;
			gl::LineWidth( new.line_width );
		}
	}
	
	fn verify( &self )	{
		//let mut mode	= 0 as gl::types::GLint;
		let mut front	= 0 as gl::types::GLint;
		let mut cmode	= 0 as gl::types::GLint;
		let mut lw		= 0 as gl::types::GLfloat;
		unsafe	{	//FIXME: crashes on that... (OSX 10.8)
			//gl::GetIntegerv(	gl::POLYGON_MODE,	ptr::to_unsafe_ptr(&mode) );
			gl::GetIntegerv(gl::FRONT_FACE,		ptr::to_mut_unsafe_ptr(&mut front) );
			gl::GetIntegerv(gl::CULL_FACE_MODE,	ptr::to_mut_unsafe_ptr(&mut cmode) );
			gl::GetFloatv(	gl::LINE_WIDTH,		ptr::to_mut_unsafe_ptr(&mut lw) );
		}
		assert!( //self.poly_mode == mode as gl::types::GLenum &&
			self.front_cw == (front==gl::CW as gl::types::GLint) &&
			self.cull == ask_state( gl::CULL_FACE ) &&
			self.cull_mode == cmode as gl::types::GLenum && 
			self.line_width == lw as f32 );
	}
}


#[deriving(Clone)]
pub struct Offset	{
	on_fill	: bool,
	on_line : bool,
	on_point: bool,
	factor	: f32,
	units	: f32,
}

impl Stage for Offset	{
	fn activate( &mut self, new : &Offset, poly : uint )	{
		if poly == 3u && self.on_fill != new.on_fill 	{
			self.on_fill = new.on_fill;
			set_state( gl::POLYGON_OFFSET_FILL, new.on_fill )
		}
		if poly == 2u && self.on_line != new.on_line	{
			self.on_line = new.on_line;
			set_state( gl::POLYGON_OFFSET_LINE, new.on_line )
		}
		if poly == 1u && self.on_point != new.on_point	{
			self.on_point = new.on_point;
			set_state( gl::POLYGON_OFFSET_POINT, new.on_point )
		}
		let on = [new.on_point,new.on_line,new.on_fill][ poly-1u ];
		if on && (self.factor!=new.factor || self.units!=new.units)	{
			self.factor = new.factor;
			self.units = new.units;
			gl::PolygonOffset(
				new.factor	as gl::types::GLfloat,
				new.units	as gl::types::GLfloat );
		}
	}

	fn verify( &self )	{
		let mut f = 0 as gl::types::GLfloat;
		let mut u = 0 as gl::types::GLfloat;
		unsafe	{
			gl::GetFloatv( gl::POLYGON_OFFSET_FACTOR,	ptr::to_mut_unsafe_ptr(&mut f) );
			gl::GetFloatv( gl::POLYGON_OFFSET_UNITS,	ptr::to_mut_unsafe_ptr(&mut u) );
		}
		assert!(
			self.on_fill	== ask_state( gl::POLYGON_OFFSET_FILL ) &&
			self.on_line	== ask_state( gl::POLYGON_OFFSET_LINE ) &&
			self.on_point	== ask_state( gl::POLYGON_OFFSET_POINT ) &&
			self.factor	== f as f32 && self.units == u as f32 );
	}
}


#[deriving(Clone)]
pub struct Scissor	{
	test	: bool,
	area	: frame::Rect,
}

impl Stage for Scissor	{
	fn activate( &mut self, new : &Scissor, _poly : uint )	{
		if self.test != new.test	{
			self.test = new.test;
			set_state( gl::SCISSOR_TEST, new.test );
		}
		if new.test && self.area != new.area	{
			self.area = new.area;
			gl::Scissor( new.area.x as gl::types::GLint, new.area.y as gl::types::GLint,
				new.area.w as gl::types::GLsizei, new.area.h as gl::types::GLsizei );
		}
	}

	fn verify( &self )	{
		let mut v = vec::from_elem( 4, 0 as gl::types::GLint );
		unsafe	{
			gl::GetIntegerv( gl::SCISSOR_BOX, vec::raw::to_mut_ptr(v) );
		}
		let r = frame::Rect{ x:v[0] as uint, y:v[1] as uint, w:v[2] as uint, h:v[3] as uint };
		assert!( self.test == ask_state( gl::SCISSOR_TEST ) &&
			self.area == r );
	}
}


#[deriving(Clone)]
pub struct Multisample	{
	on		: bool,
	alpha	: bool,
	cover	: bool,
	invert	: bool,
	value	: f32,
}

impl Stage for Multisample	{
	fn activate( &mut self, new : &Multisample, _poly : uint )	{
		if self.on != new.on	{
			self.on = new.on;
			set_state( gl::MULTISAMPLE, new.on );
		}
		if !new.on	{return;}
		if self.alpha != new.alpha	{
			self.alpha = new.alpha;
			set_state( gl::SAMPLE_ALPHA_TO_COVERAGE, new.alpha );
		}
		if self.cover != new.cover	{
			self.cover = new.cover;
			set_state( gl::SAMPLE_COVERAGE, new.cover );
		}
		if new.cover && (self.invert!=new.invert || self.value!=new.value)	{
			self.invert = new.invert;
			self.value = new.value;
			gl::SampleCoverage( new.value, new.invert as gl::types::GLboolean );
		}
	}

	fn verify( &self )	{
		let mut value	= 0 as gl::types::GLfloat;
		let mut invert	= 0 as gl::types::GLboolean;
		unsafe	{
			gl::GetFloatv(	gl::SAMPLE_COVERAGE_VALUE,	ptr::to_mut_unsafe_ptr(&mut value) );
			gl::GetBooleanv(gl::SAMPLE_COVERAGE_INVERT,	ptr::to_mut_unsafe_ptr(&mut invert) );
		}
		assert!( self.on == ask_state( gl::MULTISAMPLE ) &&
			self.alpha == ask_state( gl::SAMPLE_ALPHA_TO_COVERAGE ) &&
			self.cover == ask_state( gl::SAMPLE_COVERAGE ) &&
			self.value == value as f32 && self.invert == (invert==gl::TRUE) );
	}
}


#[deriving(Clone,Eq)]
pub struct StencilSide	{
	function		: gl::types::GLenum,
	ref_value		: int,
	read_mask		: int,
	op_fail			: gl::types::GLenum,
	op_depth_fail	: gl::types::GLenum,
	op_pass			: gl::types::GLenum,
}


impl StencilSide	{
	fn new()-> StencilSide	{
		StencilSide{
			function:gl::ALWAYS, ref_value:0, read_mask:-1,
			op_fail:gl::KEEP, op_depth_fail:gl::KEEP, op_pass:gl::KEEP
		}
	}

	pub fn activate( &mut self, new : &StencilSide, side : gl::types::GLenum )	{
		if self.function!=new.function || self.ref_value!=new.ref_value || self.read_mask!=new.read_mask	{
			self.function = new.function;
			self.ref_value = new.ref_value;
			self.read_mask = new.read_mask;
			gl::StencilFuncSeparate( side, new.function, new.ref_value as gl::types::GLint, new.read_mask as gl::types::GLuint );
		}
		if self.op_fail!=new.op_fail || self.op_depth_fail!=new.op_depth_fail || self.op_pass!=new.op_pass	{
			self.op_fail = new.op_fail;
			self.op_depth_fail = new.op_depth_fail;
			self.op_pass = new.op_pass;
			gl::StencilOpSeparate( side, new.op_fail, new.op_depth_fail, new.op_pass );
		}
	}
}


#[deriving(Clone)]
pub struct Stencil	{
	test	: bool,
	front	: StencilSide,
	back	: StencilSide,
}

impl Stage for Stencil	{
	fn activate( &mut self, new : &Stencil, _poly : uint )	{
		if self.test != new.test	{
			self.test = new.test;
			set_state( gl::STENCIL_TEST, new.test );
		}
		if !new.test	{return;}
		if new.front == new.back	{
			if self.front != new.front || self.back != new.back	{
				self.front = new.front;
				self.back.activate( &new.back, gl::FRONT_AND_BACK );
			}
		}else	{
			self.front	.activate( &new.front,	gl::FRONT );
			self.back	.activate( &new.back,	gl::BACK );
		}
	}

	fn verify( &self )	{
		let mut vals = vec::from_elem( 12, 0 as gl::types::GLint );
		let queries = [gl::STENCIL_FUNC,gl::STENCIL_REF,gl::STENCIL_VALUE_MASK,
			gl::STENCIL_FAIL,gl::STENCIL_PASS_DEPTH_FAIL,gl::STENCIL_PASS_DEPTH_PASS,
			gl::STENCIL_BACK_FUNC,gl::STENCIL_BACK_REF,gl::STENCIL_BACK_VALUE_MASK,
			gl::STENCIL_BACK_FAIL,gl::STENCIL_BACK_PASS_DEPTH_FAIL,gl::STENCIL_BACK_PASS_DEPTH_PASS
			];
		
		for (i,q) in queries.iter().enumerate()	{
			unsafe	{
				gl::GetIntegerv( *q, ptr::to_mut_unsafe_ptr(&mut vals[i]) );
			}
		}
		assert!( self.test == ask_state( gl::STENCIL_TEST ) &&
			self.front.function		== vals[0] as gl::types::GLenum && 
			self.front.ref_value	== vals[1] as int && 
			self.front.read_mask	== vals[2] as int && 
			self.front.op_fail		== vals[3] as gl::types::GLenum &&
			self.front.op_depth_fail== vals[4] as gl::types::GLenum &&
			self.front.op_pass		== vals[5] as gl::types::GLenum &&
			self.back.function		== vals[6] as gl::types::GLenum && 
			self.back.ref_value		== vals[7] as int && 
			self.back.read_mask		== vals[8] as int && 
			self.back.op_fail		== vals[9] as gl::types::GLenum &&
			self.back.op_depth_fail	== vals[10] as gl::types::GLenum &&
			self.back.op_pass		== vals[11] as gl::types::GLenum );
	}
}


#[deriving(Clone)]
pub struct Depth	{
	test	: bool,
	fun		: gl::types::GLenum,
	r0		: f32,
	r1		: f32,
}

impl Stage for Depth	{
	fn activate( &mut self, new : &Depth, _poly : uint )	{
		if self.test != new.test	{
			self.test = new.test;
			set_state( gl::DEPTH_TEST, new.test );
		}
		if !new.test	{return;}
		if self.fun != new.fun	{
			self.fun = new.fun;
			gl::DepthFunc( new.fun );
		}
		if self.r0 != new.r0 || self.r1 != new.r1	{
			self.r0 = new.r0;
			self.r1 = new.r1;
			gl::DepthRange( new.r0 as gl::types::GLdouble, new.r1 as gl::types::GLdouble )
		}
	}

	fn verify( &self )	{
		let mut val = 0 as gl::types::GLint;
		let mut r = vec::from_elem( 2, 0 as gl::types::GLfloat );
		unsafe	{
			gl::GetIntegerv(	gl::DEPTH_FUNC, ptr::to_mut_unsafe_ptr(&mut val) );
			gl::GetFloatv(	gl::DEPTH_RANGE, vec::raw::to_mut_ptr(r) );
		}
		assert!( self.test == ask_state( gl::DEPTH_TEST ) &&
			self.fun == val as gl::types::GLenum &&
			self.r0 == r[0] as f32 && self.r1 == r[1] as f32 );
	}
}


#[deriving(Clone)]
pub struct BlendChannel	{
	equation	: gl::types::GLenum,
	source		: gl::types::GLenum,
	destination	: gl::types::GLenum,
}

impl BlendChannel	{
	fn new()-> BlendChannel	{
		BlendChannel{ equation:gl::FUNC_ADD, source:gl::ONE, destination:gl::ZERO }
	}

	pub fn verify( &self, we : gl::types::GLenum, ws : gl::types::GLenum, wd : gl::types::GLenum )	{
		let mut v = vec::from_elem( 3, 0 as gl::types::GLint );
		unsafe	{
			gl::GetIntegerv( we, ptr::to_mut_unsafe_ptr(&mut v[0]) );
			gl::GetIntegerv( ws, ptr::to_mut_unsafe_ptr(&mut v[1]) );
			gl::GetIntegerv( wd, ptr::to_mut_unsafe_ptr(&mut v[2]) );
		}
		assert!( self.equation	== v[0] as gl::types::GLenum &&
			self.source			== v[1] as gl::types::GLenum &&
			self.destination	== v[2] as gl::types::GLenum );
	}
}


#[deriving(Clone)]
pub struct Blend	{
	on		: bool,
	color	: BlendChannel,
	alpha	: BlendChannel,
	value	: Color,
}

impl Stage for Blend	{
	fn activate( &mut self, new : &Blend, _poly : uint )	{
		if self.on != new.on	{
			self.on = new.on;
			set_state( gl::BLEND, new.on );
		}
		if !new.on	{return;}
		if self.color.equation!=new.color.equation || self.alpha.equation!=new.alpha.equation	{
			self.color.equation = new.color.equation;
			self.alpha.equation = new.alpha.equation;
			if new.color.equation == new.alpha.equation	{
				gl::BlendEquation( new.color.equation );
			}else	{
				gl::BlendEquationSeparate( new.color.equation, new.alpha.equation );
			}
		}
		if	self.color.source!=new.color.source || self.color.destination!=new.color.destination ||
			self.alpha.source!=new.alpha.source || self.alpha.destination!=new.alpha.destination	{
			self.color = new.color;
			self.alpha = new.alpha;
			if new.color.source==new.alpha.source && new.color.destination==new.alpha.destination	{
				gl::BlendFunc( new.color.source, new.color.destination );
			}else	{
				gl::BlendFuncSeparate(
					new.color.source, new.color.destination,
					new.alpha.source, new.alpha.destination );
			}
		}
		if self.value != new.value	{
			self.value = new.value;
			gl::BlendColor(
				new.value.r as gl::types::GLfloat, new.value.g as gl::types::GLfloat,
				new.value.b as gl::types::GLfloat, new.value.a as gl::types::GLfloat );
		}
	}

	fn verify( &self )	{
		assert!( self.on == ask_state( gl::BLEND ));
		self.color.verify( gl::BLEND_EQUATION_RGB,	gl::BLEND_SRC_RGB,	gl::BLEND_DST_RGB	);
		self.alpha.verify( gl::BLEND_EQUATION_ALPHA,	gl::BLEND_SRC_ALPHA,	gl::BLEND_DST_ALPHA	);
		let mut cv = vec::from_elem( 4, 0 as gl::types::GLfloat );
		unsafe	{
			gl::GetFloatv( gl::BLEND_COLOR, vec::raw::to_mut_ptr(cv) );
		}
		assert!(
			cv[0] == self.value.r as f32 &&
			cv[1] == self.value.g as f32 &&
			cv[2] == self.value.b as f32 &&
			cv[3] == self.value.a as f32 );
	}
}


#[deriving(Clone)]
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

impl Stage for Mask	{
	fn activate( &mut self, new : &Mask, _poly : uint )	{
		if self.stencil_front != new.stencil_front || self.stencil_back != new.stencil_back	{
			self.stencil_front = new.stencil_front;
			self.stencil_back = new.stencil_back;
			gl::StencilMaskSeparate(
				new.stencil_front as gl::types::GLuint,
				new.stencil_back as gl::types::GLuint );
		}
		if self.depth != new.depth	{
			self.depth = new.depth;
			gl::DepthMask( new.depth as gl::types::GLboolean );
		}
		if self.red!=new.red || self.green!=new.green || self.blue!=new.blue || self.alpha!=new.alpha	{
			self.red = new.red;
			self.green = new.green;
			self.blue = new.blue;
			self.alpha = new.alpha;
			gl::ColorMask( new.red as gl::types::GLboolean, new.green as gl::types::GLboolean,
				new.blue as gl::types::GLboolean, new.alpha as gl::types::GLboolean );
		}
	}

	fn verify( &self )	{
		let mut bools	= vec::from_elem( 5, false as gl::types::GLboolean );
		let mut sf		= 0 as gl::types::GLint;
		let mut sb		= 0 as gl::types::GLint;
		unsafe	{
			gl::GetBooleanv( gl::COLOR_WRITEMASK,		vec::raw::to_mut_ptr(bools) );
			gl::GetBooleanv( gl::DEPTH_WRITEMASK,		ptr::to_mut_unsafe_ptr(&mut bools[4]) );
			gl::GetIntegerv( gl::STENCIL_WRITEMASK,		ptr::to_mut_unsafe_ptr(&mut sf) );
			gl::GetIntegerv( gl::STENCIL_BACK_WRITEMASK,ptr::to_mut_unsafe_ptr(&mut sb) );
		}
		assert!(
			self.red	== (bools[0]==gl::TRUE) &&
			self.green	== (bools[1]==gl::TRUE) &&
			self.blue	== (bools[2]==gl::TRUE) &&
			self.alpha	== (bools[3]==gl::TRUE) &&
			self.depth	== (bools[4]==gl::TRUE) &&
			self.stencil_front	== sf as int &&
			self.stencil_back	== sb as int );
	}
}


#[deriving(Clone)]
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


impl Stage for State	{
	//FIXME
	fn activate( &mut self, new : &State, p0 : uint )	{
		self.view	.activate( &new.view,		p0 );
		self.prime	.activate( &new.prime,		p0 );
		let p1 = if p0==3u	{
			new.prime.get_poly_size()
		}else	{p0};
		self.offset	.activate( &new.offset,		p1 );
		self.scissor.activate( &new.scissor,	p1 );
		self.multi	.activate( &new.multi, 		p1 );
		self.stencil.activate( &new.stencil,	p1 );
		self.depth	.activate( &new.depth,		p1 );
		self.blend	.activate( &new.blend,		p1 );
		self.mask	.activate( &new.mask,		p1 );
	}

	fn verify( &self )	{
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


pub fn map_polygon_fill( dim : int )-> gl::types::GLenum	{
	[gl::POINT,gl::LINE,gl::FILL][dim-1]
}

pub fn map_comparison( s : &str )-> gl::types::GLenum	{
	match s	{
		"!"		=> gl::NEVER,
		"*"		=> gl::ALWAYS,
		"=="	=> gl::EQUAL,
		"!="	=> gl::NOTEQUAL,
		"<"		=> gl::LESS,
		"<="	=> gl::LEQUAL,
		">"		=> gl::GREATER,
		">="	=> gl::GEQUAL,
		_		=> fail!("Unknown comparison {:s}", s)
	}
}

pub fn map_operation( c : char )-> gl::types::GLenum	{
	match c	{
		'.'	=> gl::KEEP,
		'0'	=> gl::ZERO,
		'='	=> gl::REPLACE,
		'!' => gl::INVERT,
		'+'	=> gl::INCR,
		'-' => gl::DECR,
		'^' => gl::INCR_WRAP,
		'v' => gl::DECR_WRAP,
		_	=> fail!("Unknown stencil operation '{:c}'", c)
	}
}

pub fn map_equation( s : &str )-> gl::types::GLenum	{
	match s	{
		"s+d"	=> gl::FUNC_ADD,
		"s-d"	=> gl::FUNC_SUBTRACT,
		"d-s"	=> gl::FUNC_REVERSE_SUBTRACT,
		"max"	=> gl::MAX,
		"min"	=> gl::MIN,
		_		=> fail!("Unknown blend equation {:s}", s)
	}
}

pub fn map_factor( s : &str )-> gl::types::GLenum	{
	match s	{
		"0"		=> gl::ZERO,
		"1"		=> gl::ONE,
		"Sc"	=> gl::SRC_COLOR,
		"1-Sc"	=> gl::ONE_MINUS_SRC_COLOR,
		"Dc"	=> gl::DST_COLOR,
		"1-Dc"	=> gl::ONE_MINUS_DST_COLOR,
		"Cc"	=> gl::CONSTANT_COLOR,
		"1-Cc"	=> gl::ONE_MINUS_CONSTANT_COLOR,
		"Sa"	=> gl::SRC_ALPHA,
		"1-Sa"	=> gl::ONE_MINUS_SRC_ALPHA,
		"Da"	=> gl::DST_ALPHA,
		"1-Da"	=> gl::ONE_MINUS_DST_ALPHA,
		"Ca"	=> gl::CONSTANT_ALPHA,
		"1-Ca"	=> gl::ONE_MINUS_CONSTANT_ALPHA,
		"Sa^"	=> gl::SRC_ALPHA_SATURATE,
		_		=> fail!("Unknown blend factor {:s}", s)
	}
}


impl State	{
	pub fn set_offset( &mut self, value : f32 )	{
		self.offset.on_fill	= true;
		self.offset.on_line	= true;
		self.offset.on_point= true;
		self.offset.factor	= value;
		self.offset.units	= value;
	}
	pub fn set_stencil( &mut self, fun : &str, cf : char, cdf : char, cp : char, mask : int )	{
		self.stencil.test = true;
		self.stencil.front.function			= map_comparison(fun);
		self.stencil.front.op_fail			= map_operation(cf);
		self.stencil.front.op_depth_fail	= map_operation(cdf);
		self.stencil.front.op_pass			= map_operation(cp);
		self.stencil.back = self.stencil.front;
		self.mask.stencil_front = mask;
		self.mask.stencil_back = mask;
	}
	pub fn set_depth( &mut self, fun : &str, mask : bool )	{
		self.depth.test = true;
		self.depth.fun = map_comparison(fun);
		self.mask.depth = mask;
	}
	pub fn set_blend( &mut self, eq : &str, src : &str, dst : &str )	{
		self.blend.on = true;
		self.blend.color.equation	= map_equation(eq);
		self.blend.color.source		= map_factor(src);
		self.blend.color.destination= map_factor(dst);
		self.blend.alpha = self.blend.color;
	}
}


// Creates a default GL context rasterizer state
// make sure to verify that it matches GL specification
pub fn make_default( wid : uint, het : uint )-> State	{
	State{
		view : Viewport( frame::Rect::new(wid,het) ),
		prime : Primitive{
			poly_mode:gl::FILL, front_cw:false, cull:false,
			cull_mode:gl::BACK, line_width:1f32
		},
		offset : Offset{
			on_fill:false, on_line:false, on_point:false, factor:0.0, units:0.0
		},
		scissor : Scissor{
			test:false, area:frame::Rect::new(wid,het)
		},
		multi : Multisample{
			on:true, alpha:false, cover:false, value:1f32, invert:false
		},
		stencil : Stencil{
			test:false, front:StencilSide::new(), back:StencilSide::new()
		},
		depth : Depth{
			test:false, fun:gl::LESS, r0:0f32, r1:1f32
		},
		blend : Blend{
			on:false, color:BlendChannel::new(), alpha:BlendChannel::new(),
			value:Color::new(0)
		},
		mask : Mask{
			stencil_front:-1, stencil_back:-1, depth:true,
			red:true, green:true, blue:true, alpha:true
		}
	}
}
