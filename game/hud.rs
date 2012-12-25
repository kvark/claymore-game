extern mod engine;
extern mod lmath;
extern mod std;

use lmath::vec::vec4::*;
use send_map::linear::LinearMap;
use std::json;
use std::serialize::{Decoder,Decodable};


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


pure fn parse_anchor( s : &~str )-> Anchor	{
	match *s	{
		~"left-top"	=> ALeftTop,
		~"mid-top"	=> AMidTop,
		~"right-top"=> ARightTop,
		~"left-mid"	=> ALeftMid,
		~"center"	=> ACenter,
		~"right-mid"=> ARightMid,
		~"left-bot"	=> ALeftBot,
		~"mid-bot"	=> AMidBot,
		~"right-bot"=> ARightBot,
		_	=> fail ~"Unknown anchor: " + *s
	}
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
	Alignment(parse_anchor(&s[0]), parse_relation(s[1]), parse_anchor(&s[2]))
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
		fmt!( "[%d:%d)x[%d:%d)", bx, bx+sx, by, by+sy )
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
	input	: engine::call::DrawInput,
	output	: engine::call::DrawOutput,
	size	: (uint,uint),
}

impl Context	{
	fn call( prog : @engine::shade::Program, data : engine::shade::DataMap,
		rast_override : Option<&engine::rast::State> )-> engine::call::Call	{
		let &(fbo,pmap,rast_orig) = &self.output;
		let r = copy match rast_override	{
			Some(ro)	=> *ro,
			None		=> rast_orig,
		};
		engine::call::CallDraw( copy self.input, (fbo,pmap,r), prog, data )
	}
	pure fn transform( r : &Rect )-> engine::shade::Uniform	{
		let (tx,ty) = self.size, (bx,by) = r.base, (sx,sy) = r.size;
		let dx = 2f32 / (tx as f32);
		let dy = 2f32 / (ty as f32);
		let vt = Vec4::new(
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
	pure fn get_size( content : Point )-> Point	{
		let m = &self.margin;
		let (sx,sy) = self.min_size;
		let (ex,ey) = content;
		( int::max(sx,m.side+ex+m.side), int::max(sy,m.bot+ey+m.top) )
	}

	pure fn get_draw_rect()-> Rect	{
		/*let m = &self.margin;
		let (bx,by) = self.area.base;
		let (sx,sy) = self.area.size;
		Rect{
			base:(bx+m.side,by+m.bot),
			size:(sx-m.side-m.side,sy-m.bot-m.top),
		}*/
		self.area
	}

	priv fn update_size( lg : &engine::context::Log )-> Point	{
		let (ex,ey) = self.element.get_size();
		let (cx,cy) = self.get_size((ex,ey));
		if self.children.is_empty()	{
			self.area.size = (cx,cy);
			return (cx,cy);
		}
		let no_margin = Margin{side:0,bot:0,top:0};
		const BIG : int = 10000;
		let mut x_min=BIG, y_min=BIG, x_max=-BIG, y_max=-BIG;
		for uint::range(0,self.children.len()) |i|	{
			let child = &self.children[i];
			let size = child.update_size(lg);
			let Alignment(destination,relation,source) = child.alignment;
			let (src_x,src_y) = Rect{base:(0,0),size:size}.
				get_point( destination, &no_margin );
			let (dst_x,dst_y) = match relation	{
				RelParent	=> (0,0),
				RelHead		=> { assert i>0u;
					let fr = &self.children[0];
					fr.area.get_point( source, &no_margin )
				},
				RelTail		=> { assert i>0u;
					let fr = &self.children[i-1u];
					fr.area.get_point( source, &no_margin )
				}
			};
			lg.add(fmt!( "\tFrame1 '%s' rel (%d,%d) := (%d,%d)", child.name,
				src_x,src_y, dst_x,dst_y ));
			child.area.base = ( dst_x-src_x, dst_y-src_y );
			let (x1,y1) = child.area.get_point( ALeftBot, &no_margin );
			let (x2,y2) = child.area.get_point( ARightTop,&no_margin );
			assert x1<=x2 && y1<=y2;
			x_min = int::min(x_min,x1); y_min = int::min(y_min,y1);
			x_max = int::max(x_max,x2); y_max = int::max(y_max,y2);
			lg.add(fmt!( "\tUpdated1 '%s' to: %s, (%d,%d),(%d,%d)",
				child.name, child.area.to_string(), x1,y1, x2,y2 ));
		}
		let content = ( int::max(ex,x_max-x_min), int::max(ey,y_max-y_min) ); 
		lg.add(fmt!( "\tFrame3 '%s' bounding box is [%d-%d]x[%d-%d]", self.name, x_min, x_max, y_min, y_max ));
		self.area.size = self.get_size(content);
		self.area.size
	}

	priv fn update_base( lg : &engine::context::Log )	{
		let no_margin = Margin{side:0,bot:0,top:0};
		for uint::range(0,self.children.len()) |i|	{
			let child = &self.children[i];
			let Alignment(destination,relation,source) = child.alignment;
			let (src_x,src_y) = Rect{base:(0,0),size:child.area.size}.
				get_point( destination, &Margin{side:0,bot:0,top:0} );
			let (dst_x,dst_y) = match relation	{
				RelParent	=> self.area.get_point( source, &self.margin ),
				RelHead		=> self.children[0+0u].area.get_point( source, &no_margin ),
				RelTail		=> self.children[i-1u].area.get_point( source, &no_margin ),
			};
			lg.add(fmt!( "\tFrame2 '%s' rel (%d,%d) := (%d,%d)", child.name,
				src_x,src_y, dst_x,dst_y ));
			child.area.base = ( dst_x-src_x, dst_y-src_y );
			child.update_base(lg);
			lg.add(fmt!( "\tUpdated2 '%s' to: %s", child.name, child.area.to_string() ));
		}
	}

	fn update( lg : &engine::context::Log )	{
		lg.add( ~"Updating HUD: " + self.name );
		self.update_size( lg );
		assert self.area.size == self.min_size;
		self.update_base( lg );
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


#[auto_decode]
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



#[auto_decode]
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
		let vc = Vec4::new( cx, cy,
			(sx as f32)/(self.texture.width as f32),
			(sy as f32)/(self.texture.height as f32)
			);
		data.insert( ~"u_Center",	engine::shade::UniFloatVec(vc) );
		data.insert( ~"u_Transform", hc.transform(rect) );
		// return
		hc.call( self.program, data, None )
	}
}

pub type FontInfo = (~str,uint,uint);

#[auto_decode]
struct LabelInfo	{
	frame		: ~str,
	text		: ~str,
	font		: FontInfo,
	kern		: (float,float),
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
		let vc = Vec4::new( self.color.r, self.color.g, self.color.b, self.color.a );
		data.insert( ~"u_Color",	engine::shade::UniFloatVec(vc) );
		let dr = Rect{ base:rect.base, size:self.get_size() };
		data.insert( ~"u_Transform", hc.transform(&dr) );
		// return
		hc.call( self.program, data, None )
	}
}


#[auto_decode]
struct ScreenInfo	{
	frames	: ~[FrameInfo],
	images	: ~[ImageInfo],
	labels	: ~[LabelInfo],
}

pub struct Screen    {
	root	: Frame,
	images	: LinearMap<~str,@Image>,
	labels	: LinearMap<~str,@Label>,
	textures: LinearMap<~str,@engine::texture::Texture>,
	fonts	: LinearMap<FontInfo,@engine::font::Font>,
}


pub fn load_screen(path : ~str, ct : &engine::context::Context,
		ft : @engine::font::Context, lg : &engine::context::Log )-> Screen	{
	lg.add( ~"Loading HUD screen: " + path );
	let iscreen = scene::load_config::<ScreenInfo>( path );
	let (wid,het) = ct.screen_size;
	let size = (wid as int,het as int);
	let mut root = Frame{
		name		: ~"root",
		min_size	: size,
		area		: Rect{ base:(0,0), size:size },
		alignment	: Alignment(ACenter,RelParent,ACenter),
		element		: @() as @Element,
		margin		: Margin{side:0,bot:0,top:0},
		children	: convert_frames( &iscreen.frames ),
	};
	let mut map_texture	= LinearMap::<~str,@engine::texture::Texture>();
	lg.add(fmt!( "\tParsing %u images", iscreen.images.len() ));
	let mut map_image = LinearMap::<~str,@Image>();
	let prog_image = @engine::load::load_program( ct, ~"data/code/hud/image", lg );
	for iscreen.images.each() |iimage|	{
		let path = ~"data/texture/hud/" + iimage.path;
		let texture = match map_texture.find(&path)	{
			Some(t)	=> t,
			None	=>	{
				let t = @engine::load::load_texture_2D( ct, &path, false );
				map_texture.insert(path,t);
				t
			}
		};
		let image = @Image	{
			texture	: texture,
			sampler	: engine::texture::make_sampler(1u,0),
			program	: prog_image,
			center	: iimage.center,
		};
		map_image.insert( copy iimage.frame, image );
		if !root.populate( &iimage.frame, image as @Element )	{
			fail ~"\tImage frame not found: " + iimage.frame
		}
	}
	lg.add(fmt!( "\tParsing %u labels", iscreen.labels.len() ));
	let mut map_font	= LinearMap::<FontInfo,@engine::font::Font>();
	let mut map_label	= LinearMap::<~str,@Label>();
	let prog_label = @engine::load::load_program( ct, ~"data/code/hud/text", lg );
	for iscreen.labels.each() |ilabel|	{
		let font = match map_font.find(&ilabel.font)	{
			Some(f)	=> f,
			None	=>	{
				let &(fname,fsx,fsy) = &ilabel.font;
				let (kern_x,kern_y) = ilabel.kern;
				let f = @ft.load_font( ~"data/font/"+fname, 0u, fsx, fsy, kern_x, kern_y );
				map_font.insert( copy ilabel.font, f );
				f
			}
		};
		let label = @Label{
			texture	: @font.bake( ct, ilabel.text, ilabel.bound, lg ),
			content	: copy ilabel.text,
			program	: prog_label,
			color	: engine::rast::make_color(ilabel.color),
		};
		map_label.insert( copy ilabel.frame, label );
		if !root.populate( &ilabel.frame, label as @Element )	{
			fail ~"\tText frame not found: " + ilabel.frame
		}
	}
	lg.add(~"\tDone");
	Screen{
		root	: root,
		images	: map_image,
		labels	: map_label,
		textures: map_texture,
		fonts	: map_font,
	}
}
