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
	pure fn get_point( anchor : Anchor, margin : &(int,int,int) )-> Point	{
		let (bx,by) = self.base;
		let (sx,sy) = self.size;
		let &(mx,mt,mb) = margin;
		match anchor	{
			ATopLeft	=> (bx+mx,by+mt),
			ATopRight	=> (bx+sx-mx,by+mt),
			ABotLeft	=> (bx+mx,by+sy-mb),
			ACenter		=> (bx+(mx+sx-mx)/2,by+(mt+sy-mb)/2),
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
	fn call( prog : @engine::shade::Program, data : engine::shade::DataMap )-> engine::call::Call	{
		engine::call::CallDraw(
			self.fbo, copy self.pmap,
			self.vao, self.quad, self.quad.get_range(),
			prog, data, copy self.rast
			)
	}
}

trait Element	{
	pure fn get_size()-> Point;
	fn draw( &Context, &Point )-> engine::call::Call;
}

impl () : Element	{
	pure fn get_size()-> Point	{(0,0)}
	fn draw( _hc : &Context, _base : &Point )-> engine::call::Call	{
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
	margin		: (int,int,int),	// x, top, bottom
	children	: ~[Frame],
}

impl Frame	{
	fn adjust( r:&Rect )	{
		let (rbx,rby) = r.base;
		let (rsx,rsy) = r.size;
		let (bx,by) = self.area.base;
		let (sx,sy) = self.area.size;
		let (mx,mt,mb) = self.margin;
		self.area.base = (
			int::min( bx, rbx-mx ),
			int::min( by, rby-mt )
		);
		self.area.size = (
			int::max( sx, mx+mx+rsx ),
			int::max( sy, mt+mb+rsy )
		);
	}

	fn update()-> Point	{
		self.area.size = {
			let (mx,mt,mb) = self.margin;
			let (sx,sy) = self.min_size;
			let (ex,ey) = self.element.get_size();
			( int::max(sx,mx+ex+mx), int::max(sy,mt+ey+mb) )
		};
		for uint::range(0,self.children.len()) |i|	{
			let child = &self.children[i];
			let size = child.update();
			let (destination,relation,source) = child.alignment;
			let (src_x,src_y) = Rect{base:(0,0),size:size}.get_point( source, &(0,0,0) );
			let fr = match relation	{
				RelParent	=> &self,
				RelHead		=> &self.children[0],
				RelTail		=> &self.children[i-1u],
			};
			let (dst_x,dst_y) = fr.area.get_point( destination, &fr.margin );
			child.area.base = ( dst_x-src_x, dst_y-src_y );
			self.adjust( &copy child.area );
		}
		io::println(fmt!( "Frame '%s' updated to: %s", self.name, self.area.to_string() ));
		self.area.size
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
		let (bx,by) = self.area.base;
		let (mx,mt,_) = self.margin;
		let c0 = self.element.draw( hc, &(bx+mx,by+mt) );
		let mut queue = ~[c0];
		for self.children.each() |child|	{
			queue.push_all_move( child.draw_all(hc) );
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
		Frame{
			name		: copy fi.name,
			min_size	: fi.size,
			area		: Rect{base:(0,0),size:fi.size},
			alignment	: parse_alignment( fi.align ),
			element		: @() as @Element,
			margin		: fi.margin,
			children	: convert_frames( &fi.children ),
		}
	}
}



#[auto_deserialize]
struct ImageInfo	{
	frame	: ~str,
	path	: ~str,
}

pub struct Image	{
	texture	: @engine::texture::Texture,
	sampler	: engine::texture::Sampler,
	shader	: @engine::shade::Program,
}
impl Image : Element	{
	pure fn get_size()-> Point	{(0,0)}
	fn draw( _hc : &Context, _base : &Point )-> engine::call::Call	{
		engine::call::CallEmpty
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
	fn draw( hc : &Context, base : &Point )-> engine::call::Call	{
		// fill shader data
		let mut data = engine::shade::make_data();
		let sm = engine::texture::make_sampler(1u,0);
		data.insert( ~"t_Text",	engine::shade::UniTexture(0,self.texture,Some(sm)) );
		let vc = lmath::vector::Vec4::new( self.color.r, self.color.g, self.color.b, self.color.a );
		data.insert( ~"u_Color",	engine::shade::UniFloatVec(vc) );
		let &(bx,by) = base, (tx,ty) = hc.size;
		let vt = lmath::vector::Vec4::new(
			2f32 * (self.texture.width	as f32) / (tx as f32),
			2f32 * (self.texture.height	as f32) / (ty as f32),
			2f32 * (bx as f32) / (tx as f32) - 1f32,
			2f32 * (by as f32) / (ty as f32) - 1f32
			);
		data.insert( ~"u_Transform",engine::shade::UniFloatVec(vt) );
		// return
		hc.call( self.program, data )
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
		margin		: (0,0,0),
		children	: convert_frames( &iscreen.frames ),
	};
	//let mut tex_map	= LinearMap::<~str,@Texture>();
	let mut image_map	= LinearMap::<~str,@Image>();
	let mut label_map	= LinearMap::<~str,@Label>();

	for iscreen.images.each() |_iimage|	{
		/*let image = @Image{ texture:0 };
		image_map.insert( copy iimage.frame, image );
		if !iscreen.root.populate( &iimage.frame, image as @Element )	{
			fail ~"Image frame not found: " + iimage.frame
		}*/
	}
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
		label_map.insert( copy ilabel.frame, label );
		if !root.populate( &ilabel.frame, label as @Element )	{
			fail ~"Text frame not found: " + ilabel.frame
		}
	}
	Screen{
		root	: root,
		images	: image_map,
		labels	: label_map,
	}
}
