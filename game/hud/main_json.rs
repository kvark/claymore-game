extern mod engine;

use std;
use std::cell::RefCell;
use std::hashmap::HashMap;
use std::to_str::ToStr;
use extra::serialize::Decoder;

use cgmath::vector::Vec4;
use engine::gr_low;
use engine::gr_mid::{call,font};
use scene;


pub enum Anchor	{
	ALeftTop,
	AMidTop,
	ARightTop,
	ALeftMid,
	ACenter,
	ARightMid,
	ALeftBot,
	AMidBot,
	ARightBot,
}

pub enum Relation	{
	RelParent,
	RelHead,
	RelTail,
}

pub struct Alignment(Anchor,Relation,Anchor);
pub type Point = (int,int);


fn parse_anchor( s: &str )-> Anchor	{
	match s	{
		"left-top"	=> ALeftTop,
		"mid-top"	=> AMidTop,
		"right-top"	=> ARightTop,
		"left-mid"	=> ALeftMid,
		"center"	=> ACenter,
		"right-mid"	=> ARightMid,
		"left-bot"	=> ALeftBot,
		"mid-bot"	=> AMidBot,
		"right-bot"	=> ARightBot,
		_	=> fail!( ~"Unknown anchor: " + s )
	}
}

fn parse_relation( s: &str )-> Relation	{
	match s	{
		"parent"	=> RelParent,
		"head"		=> RelHead,
		"tail"		=> RelTail,
		_	=> fail!( ~"Unknown relation: " + s )
	} 
}

fn parse_alignment( expression: &str )-> Alignment	{
	let s = expression.split( |c:char| {c=='=' || c=='.'} ).map( |w| w.to_owned() ).to_owned_vec();
	assert!( s.len() == 3u );
	Alignment(parse_anchor(s[0]), parse_relation(s[1]), parse_anchor(s[2]))
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

impl ToStr for Rect	{
	fn to_str( &self )-> ~str	{
		let (bx,by) = self.base;
		let (sx,sy) = self.size;
		format!( "[{:i}:{:i})x[{:i}:{:i})", bx, bx+sx, by, by+sy )
	}
}

impl Rect	{
	pub fn get_corner( &self )-> Point	{
		let (bx,by) = self.base;
		let (sx,sy) = self.size;
		(bx+sx,by+sy)
	}
	pub fn get_point( &self, anchor: Anchor, m: &Margin )-> Point	{
		let (bx,by) = self.base;
		let (sx,sy) = self.size;
		match anchor	{
			ALeftTop	=> (bx+m.side,					by+sy-m.top),
			AMidTop		=> (bx+(m.side+sx-m.side)/2,	by+sy-m.top),
			ARightTop	=> (bx+sx-m.side,				by+sy-m.top),
			ALeftMid	=> (bx+m.side,					by+(m.bot+sy-m.top)/2),
			ACenter		=> (bx+(m.side+sx-m.side)/2,	by+(m.bot+sy-m.top)/2),
			ARightMid	=> (bx+sx-m.side,				by+(m.bot+sy-m.top)/2),
			ALeftBot	=> (bx+m.side,					by+m.bot),
			AMidBot		=> (bx+(m.side+sx-m.side)/2,	by+m.bot),
			ARightBot	=> (bx+sx-m.side,				by+m.bot),
		}
	}
}


pub struct Context	{
	input	: call::Input,
	output	: call::Output,
	rast	: gr_low::rast::Rast,
	size	: (uint,uint),
}

impl Context	{
	pub fn call( &self, prog: &gr_low::shade::ProgramPtr, data: gr_low::shade::DataMap,
		rast_override: Option<&gr_low::rast::Rast> )-> call::Call	{
		let rast = match rast_override	{
			Some(ro)	=> *ro,
			None		=> self.rast,
		};
		call::CallDraw( self.input.clone(), self.output.clone(), rast, prog.clone(), data )
	}

	pub fn transform( &self, r: &Rect )-> gr_low::shade::Uniform	{
		let (tx,ty) = self.size;
		let (bx,by) = r.base;
		let (sx,sy) = r.size;
		let dx = 2f32 / (tx as f32);
		let dy = 2f32 / (ty as f32);
		let vt = Vec4::new(
			dx * (sx as f32),
			dy * (sy as f32),
			dx * (bx as f32) - 1f32,
			dy * (by as f32) - 1f32
			);
		gr_low::shade::UniFloatVec(vt)
	}
}

pub trait Element	{
	fn get_size( &self )-> Point;
	fn draw( &self, &Context, &Rect )-> call::Call;
}

impl Element for ()	{
	fn get_size( &self )-> Point	{(0,0)}
	fn draw( &self, _hc: &Context, _r: &Rect )-> call::Call	{
		call::CallEmpty
	}
}



pub struct Frame	{
	name		: ~str,
	min_size	: Point,
	area		: Rect,		// in absolute coords
	alignment	: Alignment,
	element		: ~Element,
	margin		: Margin,
	children	: ~[Frame],
}

impl Frame	{
	pub fn get_size( &self, content: Point )-> Point	{
		let m = &self.margin;
		let (sx,sy) = self.min_size;
		let (ex,ey) = content;
		( std::num::max(sx,m.side+ex+m.side), std::num::max(sy,m.bot+ey+m.top) )
	}

	pub fn get_draw_rect( &self )-> Rect	{
		/*let m = &self.margin;
		let (bx,by) = self.area.base;
		let (sx,sy) = self.area.size;
		Rect{
			base:(bx+m.side,by+m.bot),
			size:(sx-m.side-m.side,sy-m.bot-m.top),
		}*/
		self.area
	}

	fn update_size( &mut self, lg: &engine::journal::Log )-> Point	{
		let (ex,ey) = self.element.get_size();
		let (cx,cy) = self.get_size((ex,ey));
		if self.children.is_empty()	{
			self.area.size = (cx,cy);
			return (cx,cy);
		}
		let no_margin = Margin{side:0,bot:0,top:0};
		let BIG : int = 10000;
		let mut x_min=BIG;
		let mut y_min=BIG;
		let mut x_max=-BIG;
		let mut y_max=-BIG;
		for i in range(0,self.children.len())	{
			let size = self.children[i].update_size(lg);
			let Alignment(destination,relation,source) = self.children[i].alignment;
			let (src_x,src_y) = Rect{base:(0,0),size:size}.
				get_point( destination, &no_margin );
			let (dst_x,dst_y) = match relation	{
				RelParent	=> (0,0),
				RelHead		=> { assert!( i>0u );
					(self.children[0].area).
						get_point( source, &no_margin )
				},
				RelTail		=> { assert!( i>0u );
					(self.children[i-1u].area).
						get_point( source, &no_margin )
				}
			};
			{
				let child = &mut self.children[i];
				lg.add(format!( "\tFrame1 '{:s}' rel ({:i},{:i}) := ({:i},{:i})", child.name,
					src_x,src_y, dst_x,dst_y ));
				child.area.base = ( dst_x-src_x, dst_y-src_y );
				let (x1,y1) = child.area.get_point( ALeftBot, &no_margin );
				let (x2,y2) = child.area.get_point( ARightTop,&no_margin );
				assert!( x1<=x2 && y1<=y2 );
				x_min = std::num::min(x_min,x1); y_min = std::num::min(y_min,y1);
				x_max = std::num::max(x_max,x2); y_max = std::num::max(y_max,y2);
				lg.add(format!( "\tUpdated1 '{:s}' to: {:s}, ({:i},{:i}),({:i},{:i})",
					child.name, child.area.to_str(), x1,y1, x2,y2 ));
			}
		}
		let content = ( std::num::max(ex,x_max-x_min), std::num::max(ey,y_max-y_min) ); 
		lg.add(format!( "\tFrame3 '{:s}' bounding box is [{:i}-{:i}]x[{:i}-{:i}]", self.name, x_min, x_max, y_min, y_max ));
		self.area.size = self.get_size(content);
		self.area.size
	}

	fn update_base( &mut self, lg: &engine::journal::Log )	{
		let no_margin = Margin{side:0,bot:0,top:0};
		for i in range(0,self.children.len())	{
			let Alignment(destination,relation,source) = self.children[i].alignment;
			let (src_x,src_y) = Rect{base:(0,0),size:self.children[i].area.size}.
				get_point( destination, &Margin{side:0,bot:0,top:0} );
			let (dst_x,dst_y) = match relation	{
				RelParent	=> self.area.get_point( source, &self.margin ),
				RelHead		=> (self.children[0+0u].area)
					.get_point( source, &no_margin ),
				RelTail		=> (self.children[i-1u].area)
					.get_point( source, &no_margin ),
			};
			{
				let child = &mut self.children[i];
				lg.add(format!( "\tFrame2 '{:s}' rel ({:i},{:i}) := ({:i},{:i})", child.name,
					src_x,src_y, dst_x,dst_y ));
				child.area.base = ( dst_x-src_x, dst_y-src_y );
				child.update_base(lg);
				lg.add(format!( "\tUpdated2 '{:s}' to: {:s}", child.name, child.area.to_str() ));
			}
		}
	}

	pub fn update( &mut self, lg: &engine::journal::Log )	{
		lg.add( ~"Updating HUD: " + self.name );
		self.update_size( lg );
		assert!( self.area.size == self.min_size );
		self.update_base( lg );
	}

	pub fn trace( &self, x: int, y: int, fun: |&Frame,uint| )-> uint	{
		for child in self.children.iter()	{
			let (bx,by) = child.area.base;
			let (sx,sy) = child.area.size;
			if bx<=x && bx+sx>x && by<=y && by+sy>y	{
				let d = child.trace( x, y, |x,i| fun(x,i) ) + 1u;
				fun(self,d);
				return d
			}
		}
		fun(self,0);
		0u
	}

	pub fn with_frame_mut<T>( &mut self, name: &~str, fun: |&mut Frame|->T )-> Option<T>	{
		if self.name == *name	{
			return Some( fun(self) )
		}
		for i in range(0,self.children.len())	{
			let res = self.children[i].with_frame_mut( name, |f| fun(f) );
			if res.is_some()	{
				return res
			}
		}
		None
	}

	pub fn populate( &mut self, _name: &~str, _elem: ~Element )-> bool	{
		//let res = self.with_frame_mut( name,	|fr| {fr.element=elem} );
		//res.is_some()	//FIXME
		false
	}

	pub fn draw_all( &self, hc: &Context )-> ~[call::Call]	{
		let c0 = self.element.draw( hc, &self.get_draw_rect() );
		let mut queue = ~[c0];
		for child in self.children.iter()	{
			queue.push_all_move( child.draw_all(hc) );
		}
		queue
	}

	pub fn draw_debug( &self, hc: &Context, prog: &gr_low::shade::ProgramPtr,
			data: &mut gr_low::shade::DataMap, rast: &gr_low::rast::Rast )-> call::Call	{
		data.set( ~"u_Transform", hc.transform(&self.area) );
		hc.call( prog, data.clone(), Some(rast) )
	}

	pub fn draw_debug_all( &self, hc: &Context, prog: &gr_low::shade::ProgramPtr,
			data: &mut gr_low::shade::DataMap, rast: &gr_low::rast::Rast )-> ~[call::Call]	{
		let c0 = self.draw_debug(hc,prog,data,rast);
		let mut queue = ~[c0];
		for child in self.children.iter()	{
			queue.push_all_move( child.draw_debug_all(hc,prog,data,rast) );
		}
		queue
	}
}


#[deriving(Decodable)]
pub struct FrameInfo	{
	name	: ~str,
	size	: Point,
	align	: ~str,
	margin	: (int,int,int),
	children: ~[FrameInfo],
}

pub fn convert_frames( fi_array: &[FrameInfo] )-> ~[Frame]	{
	fi_array.iter().map( |fi|	{
		let (mx,mb,mt) = fi.margin;
		Frame{
			name		: fi.name.clone(),
			min_size	: fi.size,
			area		: Rect{base:(0,0),size:fi.size},
			alignment	: parse_alignment( fi.align ),
			element		: ~() as ~Element,
			margin		: Margin{side:mx,bot:mb,top:mt},
			children	: convert_frames( fi.children ),
		}
	}).collect()
}



#[deriving(Decodable)]
struct ImageInfo	{
	frame	: ~str,
	path	: ~str,
	center	: (f32,f32),
}

pub struct Image	{
	texture	: RefCell<gr_low::texture::TexturePtr>,
	sampler	: gr_low::texture::Sampler,
	program	: gr_low::shade::ProgramPtr,
	center	: (f32,f32),
}

pub type ImagePtr = ~Image;

impl Element for Image	{
	fn get_size( &self )-> Point	{
		self.texture.with(|t| (t.borrow().width as int, t.borrow().height as int))
	}
	fn draw( &self, hc: &Context, rect: &Rect )-> call::Call	{
		// fill shader data
		let mut data = gr_low::shade::DataMap::new();
		data.set( ~"t_Image",	gr_low::shade::UniTexture(
			0, self.texture.get(), Some(self.sampler) ));
		let (cx,cy) = self.center;
		let (sx,sy) = rect.size;
		let vc = self.texture.with(|t| Vec4::new( cx, cy,
			(sx as f32)/(t.borrow().width as f32),
			(sy as f32)/(t.borrow().height as f32)
			));
		data.set( ~"u_Center",		gr_low::shade::UniFloatVec(vc) );
		data.set( ~"u_Transform",	hc.transform(rect) );
		// return
		hc.call( &self.program, data, None )
	}
}

pub type FontInfo = (~str,uint,uint);

#[deriving(Decodable)]
struct LabelInfo	{
	frame		: ~str,
	text		: ~str,
	font		: FontInfo,
	kern		: (int,int),
	color		: uint,
	bound		: (uint,uint),
}

pub struct Label	{
	texture	: RefCell<gr_low::texture::TexturePtr>,
	content	: RefCell<~str>,
	program	: gr_low::shade::ProgramPtr,
	color	: gr_low::rast::Color,
	font	: font::FontPtr,
}

pub type LabelPtr = ~Label;

impl Element for Label	{
	fn get_size( &self )-> Point	{
		self.texture.with(|t| (t.borrow().width as int, t.borrow().height as int))
	}
	fn draw( &self, hc: &Context, rect: &Rect )-> call::Call	{
		// fill shader data
		let mut data = gr_low::shade::DataMap::new();
		let sm = gr_low::texture::Sampler::new(1u,0);
		data.set( ~"t_Text",	gr_low::shade::UniTexture(0,self.texture.get(),Some(sm)) );
		let vc = Vec4::new( self.color.r, self.color.g, self.color.b, self.color.a );
		data.set( ~"u_Color",	gr_low::shade::UniFloatVec(vc) );
		let dr = Rect{ base:rect.base, size:self.get_size() };
		data.set( ~"u_Transform", hc.transform(&dr) );
		// return
		hc.call( &self.program, data, None )
	}
}


#[deriving(Decodable)]
struct ScreenInfo	{
	frames	: ~[FrameInfo],
	images	: ~[ImageInfo],
	labels	: ~[LabelInfo],
}

pub struct Screen    {
	root	: Frame,
	images	: HashMap<~str,ImagePtr>,
	labels	: HashMap<~str,LabelPtr>,
	textures: HashMap<~str,gr_low::texture::TexturePtr>,
	fonts	: HashMap<FontInfo,font::FontPtr>,
}


pub fn load_screen( path: &str, ct: &mut gr_low::context::Context,
		ft: &font::Context, lg: &engine::journal::Log )-> Screen	{
	lg.add( ~"Loading HUD screen: " + path );
	let iscreen = scene::load_json::load_config::<ScreenInfo>( path );
	let (wid,het) = ct.get_screen_size();
	let size = (wid as int,het as int);
	let root = Frame{
		name		: ~"root",
		min_size	: size,
		area		: Rect{ base:(0,0), size:size },
		alignment	: Alignment(ACenter,RelParent,ACenter),
		element		: ~() as ~Element,
		margin		: Margin{side:0,bot:0,top:0},
		children	: convert_frames( iscreen.frames ),
	};
	let mut map_texture	: HashMap<~str,gr_low::texture::TexturePtr> = HashMap::new();
	lg.add(format!( "\tParsing {:u} images", iscreen.images.len() ));
	let map_image : HashMap<~str,ImagePtr> = HashMap::new();
	let prog_image = engine::load::load_program( ct, "data/code/hud/image", lg );
	for iimage in iscreen.images.iter()	{
		let path = ~"data/texture/hud/" + iimage.path;
		let (texture,new) = match map_texture.find(&path)	{
			Some(t)	=> (t.clone(),false),
			None	=> (engine::load::load_texture_2D( ct, path, false ), true),
		};
		if new	{
			map_texture.insert(path, texture.clone());
		}
		let _image = ~Image	{
			texture	: RefCell::new(texture),
			sampler	: gr_low::texture::Sampler::new(1u,0),
			program	: prog_image.clone(),
			center	: iimage.center,
		};
		//map_image.insert( iimage.frame.clone(), image.clone() );	//FIXME
		//if !root.populate( &iimage.frame, image as ~Element )	{	//FIXME
			fail!( ~"\tImage frame not found: " + iimage.frame )
		//}
	}
	lg.add(format!( "\tParsing {:u} labels", iscreen.labels.len() ));
	let mut map_font	: HashMap<FontInfo,font::FontPtr>	= HashMap::new();
	let map_label	: HashMap<~str,LabelPtr>			= HashMap::new();
	let prog_label = engine::load::load_program( ct, "data/code/hud/text", lg );
	for ilabel in iscreen.labels.iter()	{
		let (font,new) = match map_font.find(&ilabel.font)	{
			Some(f)	=> (f.clone(),false),
			None	=>	{
				let (fname,fsx,fsy) = ilabel.font.clone();
				let (kern_x,kern_y) = ilabel.kern;
				let f = ft.load( ~"data/font/"+fname, 0u, [fsx,fsy], [kern_x,kern_y], lg );
				(f.to_ptr(),true)
			}
		};
		if new	{
			map_font.insert( ilabel.font.clone(), font.clone() );
		}
		let texture = font.borrow().bake( ct, ilabel.text, ilabel.bound, lg );
		let _label = ~Label{
			texture	: RefCell::new( texture ),
			content	: RefCell::new( ilabel.text.clone() ),
			program	: prog_label.clone(),
			color	: gr_low::rast::Color::new( ilabel.color ),
			font	: font,
		};
		//map_label.insert( ilabel.frame.clone(), label.clone() );	//FIXME
		//if !root.populate( &ilabel.frame, label as ~Element )	{	//FIXME
			fail!( ~"\tText frame not found: " + ilabel.frame )
		//}
	}
	lg.add("\tDone");
	Screen{
		root	: root,
		images	: map_image,
		labels	: map_label,
		textures: map_texture,
		fonts	: map_font,
	}
}


pub struct Blink<T>	{
	element	: ~T,
	visible	: RefCell<bool>,
}

pub type BlinkPtr<T> = ~Blink<T>;

impl<T:Element> Element for Blink<T>	{
	fn get_size( &self )-> Point	{
		self.element.get_size()
	}
	fn draw( &self, ct: &Context, r: &Rect )-> call::Call	{
		if self.visible.get()	{
			self.element.draw(ct,r)
		}else	{
			call::CallEmpty
		}
	}
}


pub struct EditLabel	{
	text	: LabelPtr,
	size	: (uint,uint),
	cursor	: BlinkPtr<Image>,
	active	: bool,
}

pub type EditLabelPtr = std::rc::Rc<RefCell<EditLabel>>;
pub type KeyInput = ();

impl EditLabel	{
	pub fn obtain( screen: &mut Screen, base_name: ~str )-> EditLabelPtr	{
		let cursor_name = base_name + ".cursor";
		let blink = ~Blink	{
			element	: screen.images.pop( &cursor_name ).unwrap(),	//FIXME
			visible	: RefCell::new( false ),
		};
		let (sx,sy) = screen.root.with_frame_mut( &cursor_name, |fr| {
			//fr.element = blink as ~Element;	//FIXME
			fr.area.size
		}).expect( ~"Frame not found: " + base_name );
		std::rc::Rc::new(RefCell::new( EditLabel{
			text	: screen.labels.pop(&base_name).unwrap(),	//FIXME
			size	: (sx as uint, sy as uint),
			cursor	: blink,
			active	: false,
		}))
	}

	pub fn change( &self, input: &str, ct: &mut gr_low::context::Context, lg: &engine::journal::Log )	{
		let st = &self.text;
		let mut text = st.content.get();
		for key in input.chars()	{
			if key == 'b'{//(259 as char)	{//FIXME
				if !text.is_empty()	{
					text.pop_char();
				}
			}else	{
				text.push_char(key);
			}
		}
		let newtex = st.font.borrow().bake( ct, text, (1000,100), lg );
		st.texture.set( newtex );	//self.size
		st.content.set( text );
	}
}

impl engine::anim::Act for EditLabel	{
	fn update( &mut self, time: engine::anim::float )-> bool	{
		let time_ms = (time * 1000.0) as uint;
		self.cursor.visible.set( self.active && (time_ms % 1000u < 500u) );
		true
	}
}
