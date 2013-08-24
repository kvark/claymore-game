extern mod engine;
extern mod lmath;
extern mod std;

use core::hashmap::linear::LinearMap;
use core::to_str::ToStr;
use std::serialize::Decoder;

use lmath::vec::vec4;
use engine::gr_low;
use engine::gr_mid::{call,font};

use scene = scene::common;


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


fn parse_anchor( s : &str )-> Anchor	{
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

fn parse_relation( s : &str )-> Relation	{
	match s	{
		"parent"	=> RelParent,
		"head"		=> RelHead,
		"tail"		=> RelTail,
		_	=> fail!( ~"Unknown relation: " + s )
	} 
}

fn parse_alignment( expression : &str )-> Alignment	{
	let s = vec::build( |push|	{
		expression.each_split( |c| {c=='=' || c=='.'}, |s| {push(s.to_owned());true} );
	});
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
		fmt!( "[%d:%d)x[%d:%d)", bx, bx+sx, by, by+sy )
	}
}

pub impl Rect	{
	fn get_corner( &self )-> Point	{
		let (bx,by) = self.base;
		let (sx,sy) = self.size;
		(bx+sx,by+sy)
	}
	fn get_point( &self, anchor : Anchor, m : &Margin )-> Point	{
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
	rast	: gr_low::rast::State,
	size	: (uint,uint),
}

pub impl Context	{
	fn call( &self, prog : @gr_low::shade::Program, data : gr_low::shade::DataMap,
		rast_override : Option<&gr_low::rast::State> )-> call::Call	{
		let rast = match rast_override	{
			Some(ro)	=> copy *ro,
			None		=> copy self.rast,
		};
		call::CallDraw( copy self.input, copy self.output, rast, prog, data )
	}

	fn transform( &self, r : &Rect )-> gr_low::shade::Uniform	{
		let (tx,ty) = self.size, (bx,by) = r.base, (sx,sy) = r.size;
		let dx = 2f32 / (tx as f32);
		let dy = 2f32 / (ty as f32);
		let vt = vec4::new(
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
	fn draw( &self, _hc : &Context, _r : &Rect )-> call::Call	{
		call::CallEmpty
	}
}



pub struct Frame	{
	name		: ~str,
	min_size	: Point,
	area		: Rect,		// in absolute coords
	alignment	: Alignment,
	element		: @Element,
	margin		: Margin,
	children	: ~[Frame],
}

pub impl Frame	{
	fn get_size( &self, content : Point )-> Point	{
		let m = &self.margin;
		let (sx,sy) = self.min_size;
		let (ex,ey) = content;
		( int::max(sx,m.side+ex+m.side), int::max(sy,m.bot+ey+m.top) )
	}

	fn get_draw_rect( &self )-> Rect	{
		/*let m = &self.margin;
		let (bx,by) = self.area.base;
		let (sx,sy) = self.area.size;
		Rect{
			base:(bx+m.side,by+m.bot),
			size:(sx-m.side-m.side,sy-m.bot-m.top),
		}*/
		self.area
	}

	priv fn update_size( &mut self, lg : &engine::journal::Log )-> Point	{
		let (ex,ey) = self.element.get_size();
		let (cx,cy) = self.get_size((ex,ey));
		if self.children.is_empty()	{
			self.area.size = (cx,cy);
			return (cx,cy);
		}
		let no_margin = Margin{side:0,bot:0,top:0};
		let BIG : int = 10000;
		let mut x_min=BIG, y_min=BIG, x_max=-BIG, y_max=-BIG;
		for uint::range(0,self.children.len()) |i|	{
			let child = &mut self.children[i];
			let size = child.update_size(lg);
			let Alignment(destination,relation,source) = child.alignment;
			let (src_x,src_y) = Rect{base:(0,0),size:size}.
				get_point( destination, &no_margin );
			let (dst_x,dst_y) = match relation	{
				RelParent	=> (0,0),
				RelHead		=> { assert!( i>0u );
					(copy self.children[0].area).
						get_point( source, &no_margin )
				},
				RelTail		=> { assert!( i>0u );
					(copy self.children[i-1u].area).
						get_point( source, &no_margin )
				}
			};
			lg.add(fmt!( "\tFrame1 '%s' rel (%d,%d) := (%d,%d)", child.name,
				src_x,src_y, dst_x,dst_y ));
			child.area.base = ( dst_x-src_x, dst_y-src_y );
			let (x1,y1) = child.area.get_point( ALeftBot, &no_margin );
			let (x2,y2) = child.area.get_point( ARightTop,&no_margin );
			assert!( x1<=x2 && y1<=y2 );
			x_min = int::min(x_min,x1); y_min = int::min(y_min,y1);
			x_max = int::max(x_max,x2); y_max = int::max(y_max,y2);
			lg.add(fmt!( "\tUpdated1 '%s' to: %s, (%d,%d),(%d,%d)",
				child.name, child.area.to_str(), x1,y1, x2,y2 ));
		}
		let content = ( int::max(ex,x_max-x_min), int::max(ey,y_max-y_min) ); 
		lg.add(fmt!( "\tFrame3 '%s' bounding box is [%d-%d]x[%d-%d]", self.name, x_min, x_max, y_min, y_max ));
		self.area.size = self.get_size(content);
		self.area.size
	}

	priv fn update_base( &mut self, lg : &engine::journal::Log )	{
		let no_margin = Margin{side:0,bot:0,top:0};
		for uint::range(0,self.children.len()) |i|	{
			let child = &mut self.children[i];
			let Alignment(destination,relation,source) = child.alignment;
			let (src_x,src_y) = Rect{base:(0,0),size:child.area.size}.
				get_point( destination, &Margin{side:0,bot:0,top:0} );
			let (dst_x,dst_y) = match relation	{
				RelParent	=> self.area.get_point( source, &self.margin ),
				RelHead		=> (copy self.children[0+0u].area)
					.get_point( source, &no_margin ),
				RelTail		=> (copy self.children[i-1u].area)
					.get_point( source, &no_margin ),
			};
			lg.add(fmt!( "\tFrame2 '%s' rel (%d,%d) := (%d,%d)", child.name,
				src_x,src_y, dst_x,dst_y ));
			child.area.base = ( dst_x-src_x, dst_y-src_y );
			child.update_base(lg);
			lg.add(fmt!( "\tUpdated2 '%s' to: %s", child.name, child.area.to_str() ));
		}
	}

	fn update( &mut self, lg : &engine::journal::Log )	{
		lg.add( ~"Updating HUD: " + self.name );
		self.update_size( lg );
		assert!( self.area.size == self.min_size );
		self.update_base( lg );
	}

	fn trace( &self, x : int, y : int, fun : &fn(&Frame,uint) )-> uint	{
		for self.children.each() |child|	{
			let (bx,by) = child.area.base;
			let (sx,sy) = child.area.size;
			if bx<=x && bx+sx>x && by<=y && by+sy>y	{
				let d = child.trace( x, y, fun ) + 1u;
				fun(self,d);
				return d
			}
		}
		fun(self,0);
		0u
	}

	fn with_frame_mut<T>( &mut self, name :&~str, fun : &fn(&mut Frame)->T )-> Option<T>	{
		if self.name == *name	{
			return Some( fun(self) )
		}
		for uint::range(0,self.children.len())	|i|	{
			//TODO: remove unsafe on Rust-0.6
			let res = unsafe{ self.children[i].with_frame_mut(name,fun) };
			if res.is_some()	{
				return res
			}
		}
		None
	}

	fn populate( &mut self, name : &~str, elem : @Element )-> bool	{
		let res = do self.with_frame_mut(name) |fr|	{fr.element=elem;};
		res.is_some()
	}

	fn draw_all( &self, hc : &Context )-> ~[call::Call]	{
		let c0 = self.element.draw( hc, &self.get_draw_rect() );
		let mut queue = ~[c0];
		for self.children.each() |child|	{
			queue.push_all_move( child.draw_all(hc) );
		}
		queue
	}

	fn draw_debug( &self, hc : &Context, prog : @gr_low::shade::Program,
		data : &mut gr_low::shade::DataMap, rast : &gr_low::rast::State )-> call::Call	{
		data.insert( ~"u_Transform", hc.transform(&self.area) );
		hc.call( prog, copy *data, Some(rast) )
	}

	fn draw_debug_all( &self, hc : &Context, prog : @gr_low::shade::Program,
		data : &mut gr_low::shade::DataMap, rast : &gr_low::rast::State )-> ~[call::Call]	{
		let c0 = self.draw_debug(hc,prog,data,rast);
		let mut queue = ~[c0];
		for self.children.each() |child|	{
			queue.push_all_move( child.draw_debug_all(hc,prog,data,rast) );
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
	texture	: @gr_low::texture::Texture,
	sampler	: gr_low::texture::Sampler,
	program	: @gr_low::shade::Program,
	center	: (f32,f32),
}

impl Element for Image	{
	fn get_size( &self )-> Point	{
		(self.texture.width as int, self.texture.height as int)
	}
	fn draw( &self, hc : &Context, rect : &Rect )-> call::Call	{
		// fill shader data
		let mut data = gr_low::shade::make_data();
		data.insert( ~"t_Image",	gr_low::shade::UniTexture(
			0, self.texture, Some(self.sampler) ));
		let (cx,cy) = self.center, (sx,sy) = rect.size;
		let vc = vec4::new( cx, cy,
			(sx as f32)/(self.texture.width as f32),
			(sy as f32)/(self.texture.height as f32)
			);
		data.insert( ~"u_Center",	gr_low::shade::UniFloatVec(vc) );
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
	texture	: @gr_low::texture::Texture,
	content	: ~str,
	program	: @gr_low::shade::Program,
	color	: gr_low::rast::Color,
	font	: @font::Font,
}

impl Element for Label	{
	fn get_size( &self )-> Point	{
		(self.texture.width as int, self.texture.height as int)
	}
	fn draw( &self, hc : &Context, rect : &Rect )-> call::Call	{
		// fill shader data
		let mut data = gr_low::shade::make_data();
		let sm = gr_low::texture::Sampler::new(1u,0);
		data.insert( ~"t_Text",	gr_low::shade::UniTexture(0,self.texture,Some(sm)) );
		let vc = vec4::new( self.color.r, self.color.g, self.color.b, self.color.a );
		data.insert( ~"u_Color",	gr_low::shade::UniFloatVec(vc) );
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
	labels	: LinearMap<~str,@mut Label>,
	textures: LinearMap<~str,@gr_low::texture::Texture>,
	fonts	: LinearMap<FontInfo,@font::Font>,
}


pub fn load_screen( path : ~str, ct : &mut gr_low::context::Context,
		ft : @font::Context, lg : &engine::journal::Log )-> Screen	{
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
	let mut map_texture	: LinearMap<~str,@gr_low::texture::Texture> = LinearMap::new();
	lg.add(fmt!( "\tParsing %u images", iscreen.images.len() ));
	let mut map_image : LinearMap<~str,@Image> = LinearMap::new();
	let prog_image = engine::load::load_program( ct, ~"data/code/hud/image", lg );
	for iscreen.images.each() |iimage|	{
		let path = ~"data/texture/hud/" + iimage.path;
		let (texture,new) = match map_texture.find(&path)	{
			Some(t)	=> (*t,false),
			None	=> (engine::load::load_texture_2D( ct, &path, false ), true),
		};
		if new	{
			map_texture.insert(path,texture);
		}
		let image = @Image	{
			texture	: texture,
			sampler	: gr_low::texture::Sampler::new(1u,0),
			program	: prog_image,
			center	: iimage.center,
		};
		map_image.insert( copy iimage.frame, image );
		if !root.populate( &iimage.frame, image as @Element )	{
			fail!( ~"\tImage frame not found: " + iimage.frame )
		}
	}
	lg.add(fmt!( "\tParsing %u labels", iscreen.labels.len() ));
	let mut map_font	: LinearMap<FontInfo,@font::Font>	= LinearMap::new();
	let mut map_label	: LinearMap<~str,@mut Label>				= LinearMap::new();
	let prog_label = engine::load::load_program( ct, ~"data/code/hud/text", lg );
	for iscreen.labels.each() |ilabel|	{
		let (font,new) = match map_font.find(&ilabel.font)	{
			Some(f)	=> (*f,false),
			None	=>	{
				let &(fname,fsx,fsy) = &ilabel.font;
				let (kern_x,kern_y) = ilabel.kern;
				let f = ft.load_font( ~"data/font/"+fname, 0u, fsx, fsy, kern_x, kern_y );
				(f,true)
			}
		};
		if new	{
			map_font.insert( copy ilabel.font, font );
		}
		let label = @mut Label{
			texture	: font.bake( ct, ilabel.text, ilabel.bound, lg ),
			content	: copy ilabel.text,
			program	: prog_label,
			color	: gr_low::rast::Color::new( ilabel.color ),
			font	: font,
		};
		map_label.insert( copy ilabel.frame, label );
		if !root.populate( &ilabel.frame, label as @Element )	{
			fail!( ~"\tText frame not found: " + ilabel.frame )
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


pub struct Blink<T>	{
	element	: @T,
	visible	: bool,
}

impl<T:Element> Element for Blink<T>	{
	fn get_size( &self )-> Point	{
		self.element.get_size()
	}
	fn draw( &self, ct : &Context, r : &Rect )-> call::Call	{
		if self.visible	{
			self.element.draw(ct,r)
		}else	{
			call::CallEmpty
		}
	}
}


pub struct EditLabel	{
	text	: @mut Label,
	size	: (uint,uint),
	cursor	: @mut Blink<Image>,
	active	: bool,
}

pub type KeyInput = ();

pub impl EditLabel	{
	fn obtain( screen : &mut Screen, base_name : ~str )-> EditLabel	{
		let cursor_name = base_name + ~".cursor";
		let blink = @mut Blink	{
			element	: *screen.images.get(&cursor_name),
			visible	: false,
		};
		let (sx,sy) = do screen.root.with_frame_mut( &cursor_name ) |fr|	{
			fr.element = blink as @Element;
			fr.area.size
		}.expect( ~"Frame not found: " + base_name );
		EditLabel{
			text	: *screen.labels.get(&base_name),
			size	: (sx as uint, sy as uint),
			cursor	: blink,
			active	: false,
		}
	}

	fn change( &self, input : &str, ct : &mut gr_low::context::Context, lg : &engine::journal::Log )	{
		let mut text = copy self.text.content;
		str::each_char(input, |key|	{
			if key == (259 as char)	{
				if !text.is_empty()	{
					str::pop_char( &mut text );
				}
			}else	{
				str::push_char( &mut text, key );
			};
			true
		});
		self.text.texture = self.text.font.bake( ct, text, (1000,100), lg );	//self.size
		self.text.content = text;
	}
}

impl engine::anim::Act for EditLabel	{
	fn update( &mut self )-> bool	{
		let time_ms = (engine::anim::get_time() * 1000f) as uint;
		self.cursor.visible = self.active && (time_ms % 1000u < 500u);
		true
	}
}
