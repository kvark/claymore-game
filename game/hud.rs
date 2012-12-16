extern mod engine;
extern mod lmath;
extern mod std;

use send_map::linear::LinearMap;
use std::json;
use std::serialization::{deserialize,Deserializer,Deserializable};


pub enum Anchor	{
	ATopLeft,
	ATopRight,
	ABotLeft,
	ACenter,
}

pub enum Relation	{
	RelParent,
	RelHead,
	RelTail,
}

pub type Alignment = (Anchor,Relation,Anchor);
pub type Point = (int,int);


pure fn parse_anchor( s : &str )-> Anchor	{
	if s == "top-left"	{ATopLeft}	else
	if s == "top-right"	{ATopRight}	else
	if s == "bot-left"	{ABotLeft}	else
	if s == "center"	{ACenter}	else
	{ fail ~"Unknown anchor: " + s }
}

pure fn parse_relation( s : &str )-> Relation	{
	if s == ~"parent"	{RelParent}	else
	if s == ~"head"		{RelHead}	else
	if s == ~"tail"		{RelTail}	else
	{ fail ~"Unknown relation: " + s }
}

pure fn parse_alignment( expression : &str )-> Alignment	{
	let s = do str::split(expression) |c|	{c=='=' || c=='.'};
	assert s.len() == 3u;
	(parse_anchor(s[0]), parse_relation(s[1]), parse_anchor(s[2]))
}


struct Margin	{
	side	: int,
	bot		: int,
	top		: int,
}

pub struct Rect    {
	base	: Point,
	size	: Point,
}

impl Rect : engine::space::Pretty	{
	pure fn to_string()-> ~str	{
		let (bx,by) = self.base;
		let (sx,sy) = self.size;
		fmt!( "[%d-%d)x[%d-%d)", bx, bx+sx, by, by+sy )
	}
}

impl Rect	{
	pure fn get_corner()-> Point	{
		let (bx,by) = self.base;
		let (sx,sy) = self.size;
		(bx+sx,by+sy)
	}
	pure fn get_point( anchor : Anchor, m : &Margin )-> Point	{
		let (bx,by) = self.base;
		let (sx,sy) = self.size;
		match anchor	{
			ATopLeft	=> (bx+m.side,by+sy-m.top),
			ATopRight	=> (bx+sx-m.side,by+sy-m.top),
			ABotLeft	=> (bx+m.side,by+m.bot),
			ACenter		=> (bx+(m.side+sx-m.side)/2,by+(m.bot+sy-m.top)/2),
		}
	}
}


pub struct Context	{
	vao		: @engine::buf::VertexArray,
	quad	: @engine::mesh::Mesh,
	fbo		: @engine::frame::Buffer,
	pmap	: engine::call::PlaneMap,
	rast	: engine::rast::State,
	size	: (uint,uint),
}

impl Context	{
	fn call( prog : @engine::shade::Program, data : engine::shade::DataMap,
		rast_override : Option<&engine::rast::State> )-> engine::call::Call	{
		let r = copy match rast_override	{
			Some(ro)	=> *ro,
			None		=> self.rast,
		};
		engine::call::CallDraw(
			self.fbo, copy self.pmap,
			self.vao, self.quad, self.quad.get_range(),
			prog, data, r)
	}
	pure fn transform( r : &Rect )-> engine::shade::Uniform	{
		let (tx,ty) = self.size, (bx,by) = r.base, (sx,sy) = r.size;
		let dx = 2f32 / (tx as f32);
		let dy = 2f32 / (ty as f32);
		let vt = lmath::vector::Vec4::new(
			dx * (sx as f32),
			dy * (sy as f32),
			dx * (bx as f32) - 1f32,
			dy * (by as f32) - 1f32
			);
		engine::shade::UniFloatVec(vt)
	}
}

trait Element	{
	pure fn get_size()-> Point;
	fn draw( &Context, &Rect )-> engine::call::Call;
}

impl () : Element	{
	pure fn get_size()-> Point	{(0,0)}
	fn draw( _hc : &Context, _r : &Rect )-> engine::call::Call	{
		engine::call::CallEmpty
	}
}

impl @Element : Deserializable {
	static fn deserialize<D:Deserializer>( &self, d : &D )-> @Element {
		@deserialize::<(),D>(d) as @Element
	}
}


pub struct Frame	{
	name		: ~str,
	min_size	: Point,
	mut area	: Rect,				// in absolute coords
	alignment	: Alignment,
	element		: @Element,
	margin		: Margin,
	children	: ~[Frame],
}

impl Frame	{
	pure fn get_size()-> Point	{
		let m = &self.margin;
		let (sx,sy) = self.min_size;
		let (ex,ey) = self.element.get_size();
		( int::max(sx,m.side+ex+m.side), int::max(sy,m.bot+ey+m.top) )
	}

	pure fn get_draw_rect()-> Rect	{
		let m = &self.margin;
		let (bx,by) = self.area.base;
		let (sx,sy) = self.area.size;
		Rect{
			base:(bx+m.side,by+m.bot),
			size:(sx-m.side-m.side,sy-m.bot-m.top),
		}
	}

	fn adjust( r:&Rect )	{
		let (rbx,rby) = r.base;
		let (rsx,rsy) = r.size;
		let (bx,by) = self.area.base;
		let (sx,sy) = self.area.size;
		let m = &self.margin;
		self.area.base = (
			int::min( bx, rbx-m.side ),
			int::min( by, rby-m.bot )
		);
		self.area.size = (
			int::max( sx, m.side+rsx+m.side ),
			int::max( sy, m.bot+rsy+m.top )
		);
	}

	fn update()	{
		for uint::range(0,self.children.len()) |i|	{
			let child = &self.children[i];
			let size = child.get_size();
			let (destination,relation,source) = child.alignment;
			let (src_x,src_y) = Rect{base:(0,0),size:size}.
				get_point( source, &Margin{side:0,bot:0,top:0} );
			assert match relation	{
				RelParent	=> true,
				_			=> i!=0u,
			};
			let fr = match relation	{
				RelParent	=> &self,
				RelHead		=> &self.children[0],
				RelTail		=> &self.children[i-1u],
			};
			let (dst_x,dst_y) = fr.area.get_point( destination, &fr.margin );
			io::println(fmt!( "\tFrame '%s' rel (%d,%d) := '%s' (%d,%d)", child.name,
				src_x,src_y, fr.name, dst_x,dst_y ));
			child.area.base = ( dst_x-src_x, dst_y-src_y );
			child.area.size = size;
			child.update();
			io::println(fmt!( "\tUpdated '%s' to: %s", child.name, child.area.to_string() ));
			self.adjust( &copy child.area );
		}
	}

	fn populate( &mut self, name : &~str, elem : @Element )-> bool	{
		if self.name == *name	{
			self.element = elem;
			return true
		}
		for uint::range(0,self.children.len())	|i|	{
			//TODO: remove unsafe on Rust-0.5
			if unsafe{self.children[i].populate(name,elem)}	{
				return true
			}
		}
		false
	}

	fn draw_all( hc : &Context )-> ~[engine::call::Call]	{
		let c0 = self.element.draw( hc, &self.get_draw_rect() );
		let mut queue = ~[c0];
		for self.children.each() |child|	{
			queue.push_all_move( child.draw_all(hc) );
		}
		queue
	}

	fn draw_debug( hc : &Context, prog : @engine::shade::Program,
		data : &mut engine::shade::DataMap, rast : &engine::rast::State )-> ~[engine::call::Call]	{
		data.insert( ~"u_Transform", hc.transform(&self.area) );
		let c0 = hc.call( prog, copy *data, Some(rast) );
		let mut queue = ~[c0];
		for self.children.each() |child|	{
			queue.push_all_move( child.draw_debug(hc,prog,data,rast) );
		}
		queue
	}
}


#[auto_deserialize]
pub struct FrameInfo	{
	name	: ~str,
	size	: Point,
	align	: ~str,
	margin	: (int,int,int),
	children: ~[FrameInfo],
}

pub fn convert_frames( fi_array : &~[FrameInfo] )-> ~[Frame]	{
	do vec::map(*fi_array) |fi|	{
		let (mx,mb,mt) = fi.margin;
		Frame{
			name		: copy fi.name,
			min_size	: fi.size,
			area		: Rect{base:(0,0),size:fi.size},
			alignment	: parse_alignment( fi.align ),
			element		: @() as @Element,
			margin		: Margin{side:mx,bot:mb,top:mt},
			children	: convert_frames( &fi.children ),
		}
	}
}



#[auto_deserialize]
struct ImageInfo	{
	frame	: ~str,
	path	: ~str,
	center	: (f32,f32),
}

pub struct Image	{
	texture	: @engine::texture::Texture,
	sampler	: engine::texture::Sampler,
	program	: @engine::shade::Program,
	center	: (f32,f32),
}
impl Image : Element	{
	pure fn get_size()-> Point	{
		(self.texture.width as int, self.texture.height as int)
	}
	fn draw( hc : &Context, rect : &Rect )-> engine::call::Call	{
		// fill shader data
		let mut data = engine::shade::make_data();
		data.insert( ~"t_Image",	engine::shade::UniTexture(
			0, self.texture, Some(self.sampler) ));
		let (cx,cy) = self.center, (sx,sy) = rect.size;
		let vc = lmath::vector::Vec4::new( cx, cy,
			(sx as f32)/(self.texture.width as f32),
			(sy as f32)/(self.texture.height as f32)
			);
		data.insert( ~"u_Center",	engine::shade::UniFloatVec(vc) );
		data.insert( ~"u_Transform", hc.transform(rect) );
		// return
		hc.call( self.program, data, None )
	}
}


#[auto_deserialize]
struct LabelInfo	{
	frame		: ~str,
	content		: ~str,
	font		: (~str,uint,uint),
	color		: uint,
	bound		: (uint,uint),
}

pub struct Label	{
	texture	: @engine::texture::Texture,
	content	: ~str,
	program	: @engine::shade::Program,
	color	: engine::rast::Color,
}
impl Label : Element	{
	pure fn get_size()-> Point	{
		(self.texture.width as int, self.texture.height as int)
	}
	fn draw( hc : &Context, rect : &Rect )-> engine::call::Call	{
		// fill shader data
		let mut data = engine::shade::make_data();
		let sm = engine::texture::make_sampler(1u,0);
		data.insert( ~"t_Text",	engine::shade::UniTexture(0,self.texture,Some(sm)) );
		let vc = lmath::vector::Vec4::new( self.color.r, self.color.g, self.color.b, self.color.a );
		data.insert( ~"u_Color",	engine::shade::UniFloatVec(vc) );
		let dr = Rect{ base:rect.base, size:self.get_size() };
		data.insert( ~"u_Transform", hc.transform(&dr) );
		// return
		hc.call( self.program, data, None )
	}
}


#[auto_deserialize]
struct ScreenInfo	{
	frames	: ~[FrameInfo],
	images	: ~[ImageInfo],
	labels	: ~[LabelInfo],
}

pub struct Screen    {
	root	: Frame,
	images	: LinearMap<~str,@Image>,
	labels	: LinearMap<~str,@Label>,
}


pub fn load_screen(path : ~str, ct : &engine::context::Context,
		ft : @engine::font::Context )-> Screen	{
	let iscreen = scene::load_config::<ScreenInfo>(path);
	let (wid,het) = ct.screen_size;
	let size = (wid as int,het as int);
	let mut root = Frame{
		name		: ~"root",
		min_size	: size,
		area		: Rect{ base:(0,0), size:size },
		alignment	: (ACenter,RelParent,ACenter),
		element		: @() as @Element,
		margin		: Margin{side:0,bot:0,top:0},
		children	: convert_frames( &iscreen.frames ),
	};
	//let mut tex_map	= LinearMap::<~str,@Texture>();
	let mut map_image = LinearMap::<~str,@Image>();
	let prog_image = @engine::load::load_program( ct, ~"data/code/hud/image" );
	for iscreen.images.each() |iimage|	{
		let path = ~"data/texture/" + iimage.path;
		let texture = @engine::load::load_texture_2D( ct, &path, false );
		let image = @Image	{
			texture	: texture,
			sampler	: engine::texture::make_sampler(1u,0),
			program	: prog_image,
			center	: iimage.center,
		};
		map_image.insert( copy iimage.frame, image );
		if !root.populate( &iimage.frame, image as @Element )	{
			fail ~"Image frame not found: " + iimage.frame
		}
	}
	let mut map_label = LinearMap::<~str,@Label>();
	let prog_label = @engine::load::load_program( ct, ~"data/code/hud/text" );
	for iscreen.labels.each() |ilabel|	{
		let &(fname,fsx,fsy) = &ilabel.font;
		let font = ft.load_font( ~"data/font/"+fname, 0u, fsx, fsy, 0f, 0f );
		let label = @Label{
			texture	: @font.bake( ct, ilabel.content, ilabel.bound ),
			content	: ilabel.content,
			program	: prog_label,
			color	: engine::rast::make_color(ilabel.color),
		};
		map_label.insert( copy ilabel.frame, label );
		if !root.populate( &ilabel.frame, label as @Element )	{
			fail ~"Text frame not found: " + ilabel.frame
		}
	}
	Screen{
		root	: root,
		images	: map_image,
		labels	: map_label,
	}
}
