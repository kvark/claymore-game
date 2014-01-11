extern mod cgmath;
extern mod engine;
extern mod gen_hud;

use std;
use std::hashmap::HashMap;
use cgmath::vector::Vec4;
use engine::gr_low::{rast,shade};
use engine::gr_low::frame::Rect;
use engine::gr_low::texture;
use engine::gr_mid::{call,font};
use gen = gen_hud::common;



pub fn get<T>( children : &[gen::Child], path : &str, fun : |&str,&gen::Element|->T )-> T	{
	let slash = path.find('/');
	let name = match slash	{
		Some(p)	=> path.slice_to(p),
		None	=> path,
	};
	for &gen::Child(ref cname, ref elem) in children.iter()	{
		if std::str::eq_slice( *cname, name )	{
			return match slash	{
				Some(p)	=>	{
					let rest = path.slice_from( p+1 );
					match elem	{
						&gen::ElBox(_, _, ref bx)	=> get( bx.children, rest, fun ),
						_	=> fail!("Hud child is not a frame: {:s}", name)
					}
				},
				None	=> fun(name,elem),
			}
		}
	}
	fail!("Hud child not found: {:s}", name)
}

pub fn modify( children : &mut ~[gen::Child], path : &str, fun : |&str,&mut gen::Element| )	{
	let slash = path.find('/');
	let name = match slash	{
		Some(p)	=> path.slice_to(p),
		None	=> path,
	};
	for &gen::Child(ref cname, ref mut elem) in children.mut_iter()	{
		if std::str::eq_slice( *cname, name )	{
			return match slash	{
				Some(p)	=>	{
					let rest = path.slice_from( p+1 );
					match elem	{
						&gen::ElBox(_, _, ref mut bx)	=>	{
							modify( &mut bx.children, rest, fun );
						},
						_	=> fail!("Hud child is not a frame: {:s}", name)
					}
				},
				None	=> fun(name,elem),
			}
		}
	}
	fail!("Hud child not found: {:s}", name)
}


struct FontCache	{
	font	: @font::Font,
	cache	: HashMap<~str,@texture::Texture>,
}


pub struct Context	{
	input	: call::Input,
	rast	: rast::State,
	program_solid	: @shade::Program,
	program_image	: @shade::Program,
	program_text	: @shade::Program,
	sampler_image	: texture::Sampler,
	sampler_text	: texture::Sampler,
	cache_images	: HashMap<gen::Path,@texture::Texture>,
	cache_fonts		: HashMap<gen::Font,FontCache>,
}

impl Context	{
	pub fn create( gc : &mut engine::gr_low::context::Context, lg : &engine::journal::Log )-> Context	{
		let mut hud_rast = gc.default_rast;
		hud_rast.set_blend( "s+d", "Sa", "1-Sa" );
		let vao = gc.create_vertex_array();
		let quad = @engine::gr_mid::mesh::create_quad( gc );
		Context{
			input	: call::Input::new( vao, quad ),
			rast	: hud_rast,
			program_solid	: engine::load::load_program( gc, "data/code/hud/solid",	lg ),
			program_image	: engine::load::load_program( gc, "data/code/hud/image",	lg ),
			program_text	: engine::load::load_program( gc, "data/code/hud/text",		lg ),
			sampler_image	: texture::Sampler::new(1u,0),
			sampler_text	: texture::Sampler::new(1u,0),
			cache_images	: HashMap::new(),
			cache_fonts		: HashMap::new(),
		}
	}

	pub fn preload_font<'a>( &'a mut self, font : &gen::Font, fcon : &font::Context, lg : &engine::journal::Log )-> &'a mut FontCache	{
		self.cache_fonts.find_or_insert_with( font.clone(), |f|	{
			let path = ~"data/font/" + f.path;
			FontCache	{
				font	: @fcon.load( path, 0u, f.size, f.kern, lg ),
				cache	: HashMap::new(),
			}
		})
	}

	pub fn preload( &mut self, children : &[gen::Child], gcon : &mut engine::gr_low::context::Context,
			fcon : &font::Context, lg : &engine::journal::Log )	{
		for &gen::Child(_,ref elem) in children.iter()	{
			match elem	{
				&gen::ElBox(_, _, ref bx)	=> self.preload( bx.children, gcon, fcon, lg ),
				&gen::ElImage(ref name)	=>	{
					self.cache_images.find_or_insert_with( name.clone(), |s|	{
						let path = ~"data/texture/hud/" + *s;
						engine::load::load_texture_2D( gcon, path, false )
					});
				},
				&gen::ElText(ref text)	=>	{
					let fc = self.preload_font( &text.font, fcon, lg );
					fc.cache.find_or_insert_with( text.value.clone(), |s|	{
						let bound = ( text.bound[0], text.bound[1] );
						fc.font.bake( gcon, *s, bound, lg )
					});
				},
				&gen::ElSpace(_)	=> (),
			}
		}
	}

	fn make_call( &self, prog : @shade::Program, data : shade::DataMap,
			output : &call::Output, rast_override : Option<rast::State> )-> call::Call	{
		let rast = match rast_override	{
			Some(r)	=> r,
			None	=> self.rast,
		};
		call::CallDraw( self.input.clone(), output.clone(), rast, prog, data )
	}

	fn transform( &self, r : &Rect, screen_size : &gen::Vector )-> shade::Uniform	{
		let dx = 2f32 / (screen_size[0] as f32);
		let dy = 2f32 / (screen_size[1] as f32);
		let vt = Vec4::new(
			dx * (r.w as f32),
			dy * (r.h as f32),
			dx * (r.x as f32) - 1f32,
			//dy * (r.y as f32) - 1f32
			1f32 - dy * ((r.y+r.h) as f32)
			);
		shade::UniFloatVec(vt)
	}

	pub fn get_size( &self, elem : &gen::Element )-> gen::Vector	{
		match elem	{
			&gen::ElImage(ref path)	=>	{
				let t = self.cache_images.get( path );
				[t.width,t.height]
			},
			&gen::ElText(ref text)	=>	{
				let fc = self.cache_fonts.get( &text.font );
				let t = fc.cache.get( &text.value );
				[t.width,t.height]
			},
			&gen::ElSpace(space)	=> space,
			&gen::ElBox(_,_,_)		=> [0,0],
		}
	}

	pub fn draw_all( &self, screen : &gen::Screen, out : &call::Output )-> ~[call::Call]	{
		let size = [out.area.w, out.area.h];
		self.draw( &screen.root, out.area, out, &size )
	}

	pub fn get_color_param( color : uint )-> shade::Uniform	{
		shade::UniFloatVec(Vec4::new(
			((color>>24)&0xFF) as f32 / 255f32,
			((color>>16)&0xFF) as f32 / 255f32,
			((color>>8)	&0xFF) as f32 / 255f32,
			((color>>0)	&0xFF) as f32 / 255f32
			))
	}

	pub fn draw( &self, bx : &gen::Box, area : Rect, out : &call::Output,
			screen_size : &gen::Vector )-> ~[call::Call]	{
		let mut off : gen::Vector = [area.x,area.y];
		let mut calls : ~[call::Call] = ~[];
		for &gen::Child(_,ref element) in bx.children.iter()	{
			let size = match element	{
				&gen::ElImage(ref path)	=>	{
					let t = *self.cache_images.find( path ).
						expect(format!( "Image '{:s}' is not loaded", *path ));
					let mut data = shade::DataMap::new();
					data.set( ~"t_Image",		shade::UniTexture(
						0, t, Some(self.sampler_image) ));
					let rect = Rect{ x:off[0], y:off[1], w:t.width, h:t.height };
					let vc = Vec4::new( 0f32, 0f32, 1f32, 1f32 );
					data.set( ~"u_Center",		shade::UniFloatVec(vc) );
					data.set( ~"u_Transform",	self.transform(&rect,screen_size) );
					calls.push( self.make_call( self.program_image, data, out, None ));
					[t.width,t.height]
				},
				&gen::ElText(ref text)	=>	{
					let fc = self.cache_fonts.find( &text.font ).
						expect(format!( "Font '{:s}' is not loaded", text.font.path ));
					let t = *fc.cache.find( &text.value ).
						expect(format!( "Text '{:s}' is not loaded", text.value ));
					let mut data = shade::DataMap::new();
					data.set( ~"t_Text",	shade::UniTexture(
						0, t, Some(self.sampler_text) ));
					data.set( ~"u_Color",	Context::get_color_param(text.color) );
					let dr = Rect{ x:off[0], y:off[1], w:t.width, h:t.height };
					data.set( ~"u_Transform", self.transform(&dr,screen_size) );
					// return
					calls.push( self.make_call( self.program_text, data, out, None ));
					[t.width,t.height]
				},
				&gen::ElSpace(space)	=> space,
				&gen::ElBox(ref sx, ref sy, ref subox)	=>	{
					let size = [ sx.apply(area.w), sy.apply(area.h) ];
					let a2 = Rect{ x:off[0], y:off[1], w:off[0]+size[0], h:off[1]+size[1] };
					let sub = self.draw( subox, a2, out, screen_size );
					calls.push_all_move( sub );
					size
				},
			};
			match bx.align	{
				gen::AlignHor	=> {off[0] += size[0]},
				gen::AlignVer	=> {off[1] += size[1]},
			}
			assert!( off[0] <= area.w && off[1] <= area.h );
		}
		// draw box
		let mut abox = area.clone();
		match bx.align	{
			gen::AlignHor	=> {abox.w = off[0] - area.x},
			gen::AlignVer	=> {abox.h = off[1] - area.y},
		}
		let mut data = shade::DataMap::new();
		data.set( ~"u_Transform",self.transform(&abox,screen_size) );
		let c0 = match bx.ground	{
			gen::GroundNone	=> call::CallEmpty,
			gen::GroundSolid( color )	=> {
				data.set( ~"u_Color", 	Context::get_color_param(color) );
				self.make_call( self.program_solid, data, out, None )
			},
			gen::GroundFrame( color, size )	=> {
				data.set( ~"u_Color", 	Context::get_color_param(color) );
				let mut rast = self.rast.clone();
				rast.prime.poly_mode = rast::map_poly_mode(2);
				rast.prime.line_width = size as f32;
				self.make_call( self.program_solid, data, out, Some(rast) )
			},
			gen::GroundImage( ref _path, ref _center )	=> {
				call::CallEmpty	//TODO
			},
		};
		calls.insert(0,c0);
		calls
	}
}
