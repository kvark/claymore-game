extern mod engine;
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

impl Anchor : Deserializable {
	static fn deserialize<D:Deserializer>( &self, d : &D )-> Anchor {
		let s : ~str = deserialize(d);
		if s == ~"top-left"		{ATopLeft}	else
		if s == ~"top-right"	{ATopRight}	else
		if s == ~"bot-left"		{ABotLeft}	else
		if s == ~"center"		{ACenter}	else
		{ fail ~"Unknown anchor: " + s }
	}
}

impl Relation : Deserializable {
	static fn deserialize<D:Deserializer>( &self, d : &D )-> Relation {
		let s : ~str = deserialize(d);
		if s == ~"parent"	{RelParent}	else
		if s == ~"head"		{RelHead}	else
		if s == ~"tail"		{RelTail}	else
		{ fail ~"Unknown relation: " + s }
	}
}


pub type Alignment = (Anchor,Relation,Anchor);
pub type Point = (int,int);

#[auto_deserialize]
pub struct Rect    {
	base	: Point,
	size	: Point,
}

impl Rect	{
	fn get_corner()-> Point	{
		let (bx,by) = self.base;
		let (sx,sy) = self.size;
		(bx+sx,by+sy)
	}
}


trait Element	{
	pure fn get_size()-> Point;
	fn draw( &Point )-> engine::call::Call;
}

impl () : Element	{
	pure fn get_size()-> Point	{(0,0)}
	fn draw( _base : &Point )-> engine::call::Call	{engine::call::CallEmpty}
}

impl @Element : Deserializable {
	static fn deserialize<D:Deserializer>( &self, d : &D )-> @Element {
		@deserialize::<(),D>(d) as @Element
	}
}


#[auto_deserialize]
pub struct Frame	{
	name		: ~str,
	mut area	: Rect,				// in absolute coords
	alignment	: Alignment,
	element		: @Element,
	margin		: (int,int,int),	// x, top, bottom
	children	: ~[Frame],
}

impl Frame	{
	fn get_point( anchor:Anchor )-> Point	{
		let (bx,by) = self.area.base;
		let (sx,sy) = self.area.size;
		let (mx,mt,mb) = self.margin;
		match anchor	{
			ATopLeft	=> (bx+mx,by+mt),
			ATopRight	=> (bx+sx-mx,by+mt),
			ABotLeft	=> (bx+mx,by+sy-mb),
			ACenter		=> (bx+(mx+sx-mx)/2,by+(mt+sy-mb)/2),
		}
	}

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
		self.area.size = self.element.get_size();
		for uint::range(0,self.children.len()) |i|	{
			let child = &self.children[i];
			let size = child.update();
			let align@(destination,relation,source) = child.alignment;
			let (src_x,src_y) = Frame{
				name		: ~"",
				area		: Rect{base:(0,0),size:size},
				alignment	: align,
				element		: @() as @Element,
				margin		: (0,0,0),
				children	: ~[]
			}.get_point(source);
			let (dst_x,dst_y) = match relation	{
				RelParent	=> &self,
				RelHead		=> &self.children[0],
				RelTail		=> &self.children[i-1u],
			}.get_point(destination);
			child.area.base = ( dst_x-src_x, dst_y-src_y );
			self.adjust( &copy child.area );
		}
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

	fn draw_all()-> ~[engine::call::Call]	{
		let base = self.area.base;
		let c0 = self.element.draw( &base );
		let mut queue = ~[c0];
		for self.children.each() |child|	{
			queue.push_all_move( child.draw_all() );
		}
		queue
	}
}


#[auto_deserialize]
struct ImageInfo	{
	frame	: ~str,
	path	: ~str,
}

pub struct Image	{
	texture	: int
}
impl Image : Element	{
	pure fn get_size()-> Point	{(0,0)}
	fn draw( _base : &Point )-> engine::call::Call	{engine::call::CallEmpty}
}


#[auto_deserialize]
struct LabelInfo	{
	frame		: ~str,
	content		: ~str,
	font		: ~str,
	font_size	: (uint,uint),
	bound		: (uint,uint),
}

pub struct Label	{
	texture	: int
}
impl Label : Element	{
	pure fn get_size()-> Point	{(0,0)}
	fn draw( _base : &Point )-> engine::call::Call	{engine::call::CallEmpty}
}


#[auto_deserialize]
struct ScreenInfo	{
	root	: Frame,
	images	: ~[ImageInfo],
	labels	: ~[LabelInfo],
}

pub struct Screen    {
	root	: Frame,
	images	: LinearMap<~str,@Image>,
	labels	: LinearMap<~str,@Label>,
}


pub fn load_screen(path : ~str)-> Screen	{
	let mut iscreen = scene::load_config::<ScreenInfo>(path);
	//let mut tex_map	= LinearMap::<~str,@Texture>();
	let mut image_map	= LinearMap::<~str,@Image>();
	let mut label_map	= LinearMap::<~str,@Label>();

	for iscreen.images.each() |iimage|	{
		let image = @Image{ texture:0 };
		image_map.insert( copy iimage.frame, image );
		if !iscreen.root.populate( &iimage.frame, image as @Element )	{
			fail ~"Image frame not found: " + iimage.frame
		}
	}
	for iscreen.labels.each() |ilabel|	{
		let label = @Label{ texture:0 };
		label_map.insert( copy ilabel.frame, label );
		if !iscreen.root.populate( &ilabel.frame, label as @Element )	{
			fail ~"Text frame not found: " + ilabel.frame
		}
	}
	Screen{
		root	: iscreen.root,
		images	: image_map,
		labels	: label_map,
	}
}
