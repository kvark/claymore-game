extern mod lmath;
extern mod engine;
extern mod gen_hud;

use core::hashmap::linear::LinearMap;
use lmath::vec::vec4;
use engine::gr_low::{rast,shade};
use engine::gr_low::frame::Rect;
use engine::gr_low::texture;
use engine::gr_mid::{call,font};
use gen = gen_hud::common;



priv fn get<T>( children : &[gen::Child], path : &str, fun : &fn(&gen::Child)->T )-> T	{
	let slash = str::find_char(path,'/');
	let name = match slash	{
		Some(p)	=> path.substr(0,p),
		None	=> path,
	};
	for children.each() |child|	{
		if str::eq_slice( child.name, name )	{
			return match slash	{
				Some(p)	=>	{
					let rest = path.substr( p+1, path.len()-p-1 );
					match &child.element	{
						&gen::ElFrame(ref fr)	=> get( fr.children, rest, fun ),
						_	=> fail!(fmt!("Hud child is not a frame: %s", name))
					}
				},
				None	=> fun(child),
			}
		}
	}
	fail!(fmt!("Hud child not found: %s",name))
}

priv fn modify( children : &mut ~[gen::Child], path : &str, fun : &fn(&mut gen::Child) )	{
	let slash = str::find_char(path,'/');
	let name = match slash	{
		Some(p)	=> path.substr(0,p),
		None	=> path,
	};
	for children.each_mut() |child|	{
		if str::eq_slice( child.name, name )	{
			return match slash	{
				Some(p)	=>	{
					let rest = path.substr( p+1, path.len()-p-1 );
					child.element = match &child.element	{
						&gen::ElFrame(ref fr)	=>	{
							//TODO: mod in place
							let mut f2 = copy *fr;
							modify( &mut f2.children, rest, fun );
							gen::ElFrame(f2)
						},
						_	=> fail!(fmt!("Hud child is not a frame: %s", name))
					}
				},
				None	=> fun(child),
			}
		}
	}
	fail!(fmt!("Hud child not found: %s",name))
}


struct FontCache	{
	font	: @font::Font,
	cache	: LinearMap<~str,@texture::Texture>,
}


pub struct Context	{
	input	: call::Input,
	rast	: rast::State,
	program_image	: @shade::Program,
	program_text	: @shade::Program,
	sampler_image	: texture::Sampler,
	sampler_text	: texture::Sampler,
	cache_images	: LinearMap<gen::Path,@texture::Texture>,
	cache_fonts		: LinearMap<gen::Font,FontCache>,
}

impl Context	{
	pub fn create( gc : &mut engine::gr_low::context::Context, lg : &engine::journal::Log )-> Context	{
		let mut hud_rast = copy gc.default_rast;
		hud_rast.set_blend( ~"s+d", ~"Sa", ~"1-Sa" );
		let vao = gc.create_vertex_array();
		let quad = @engine::gr_mid::mesh::create_quad( gc );
		Context{
			input	: call::Input::new( vao, quad ),
			rast	: hud_rast,
			program_image	: engine::load::load_program( gc, ~"data/code/hud/image", lg ),
			program_text	: engine::load::load_program( gc, ~"data/code/hud/text", lg ),
			sampler_image	: texture::Sampler::new(1u,0),
			sampler_text	: texture::Sampler::new(1u,0),
			cache_images	: LinearMap::new(),
			cache_fonts		: LinearMap::new(),
		}
	}

	pub fn preload( &mut self, children : &[gen::Child], gcon : &mut engine::gr_low::context::Context,
			fcon : &font::Context, lg : &engine::journal::Log )	{
		for children.each() |child|	{
			match &child.element	{
				&gen::ElFrame(ref fr)	=> self.preload( fr.children, gcon, fcon, lg ),
				&gen::ElImage(ref name)	=>	{
					if !self.cache_images.contains_key( name )	{
						let path = ~"data/texture/hud/" + *name;
						let t = engine::load::load_texture_2D( gcon, &path, false );
						self.cache_images.insert( copy *name, t );
					}
				},
				&gen::ElText(ref text)	=>	{
					let f = &text.font;
					{//FIXME
						if !self.cache_fonts.contains_key(f)	{
							let path = ~"data/font/" + f.path;
							let fc = FontCache	{
								font	: @fcon.load( path, 0u, f.size, f.kern, lg ),
								cache	: LinearMap::new(),
							};
							self.cache_fonts.insert( copy text.font, copy fc );
						}
					}
					{
						let fc = self.cache_fonts.find_mut(f).expect("Fonts contents are bad");
						if !fc.cache.contains_key( &text.value )	{
							let bound = ( text.bound[0], text.bound[1] );
							let t = fc.font.bake( gcon, text.value, bound, lg );
							fc.cache.insert( copy text.value, t );
						}
					}
				},
				&gen::ElSpace(_)	=> (),
			}
		}
	}

	priv fn make_call( &self, prog : @shade::Program, data : shade::DataMap,
		output : &call::Output, rast_override : Option<&rast::State> )-> call::Call	{
		let rast = match rast_override	{
			Some(ro)	=> copy *ro,
			None		=> copy self.rast,
		};
		call::CallDraw( copy self.input, copy *output, rast, prog, data )
	}

	priv fn transform( &self, r : &Rect, screen_size : &gen::Vector )-> shade::Uniform	{
		let dx = 2f32 / (screen_size[0] as f32);
		let dy = 2f32 / (screen_size[1] as f32);
		let vt = vec4::new(
			dx * (r.w as f32),
			dy * (r.h as f32),
			dx * (r.x as f32) - 1f32,
			dy * (r.y as f32) - 1f32
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
			&gen::ElFrame(_)		=> [0,0],
		}
	}

	pub fn get_rect_corner( r : &Rect )-> gen::Vector	{
		[r.x+r.w, r.y+r.h]
	}

	pub fn get_rect_point( r : &Rect, a : &gen::Anchor )-> gen::Vector	{
		let b = [(a[0]+1) as uint, (a[1]+1) as uint];
		[r.x + ((r.w*b[0])>>1), r.y + ((r.h*b[1])>>1)]
	}

	pub fn draw_all( &self, screen : &gen::Screen, out : &call::Output )-> ~[call::Call]	{
		let size = [out.area.w, out.area.h];
		let (_,calls) = self.draw( screen.children, out.area, out, &size );
		calls
	}

	pub fn get_color_param( color : uint )-> shade::Uniform	{
		shade::UniFloatVec(vec4::new(
			((color>>24)&0xFF) as f32 / 255f32,
			((color>>16)&0xFF) as f32 / 255f32,
			((color>>8)	&0xFF) as f32 / 255f32,
			((color>>0)	&0xFF) as f32 / 255f32
			))
	}

	pub fn draw( &self, children : &[gen::Child], area : Rect, out : &call::Output,
			screen_size : &gen::Vector )-> (gen::Vector,~[call::Call])	{
		let mut off : gen::Vector = [area.x,area.y];
		let mut calls : ~[call::Call] = ~[];
		for children.each() |&child|	{
			let size = match &child.element	{
				&gen::ElImage(ref path)	=>	{
					let t = *self.cache_images.find( path ).
						expect(fmt!( "Image '%s' is not loaded", *path ));
					let mut data = shade::DataMap::new();
					data.insert( ~"t_Image",		shade::UniTexture(
						0, t, Some(self.sampler_image) ));
					let rect = Rect{ x:off[0], y:off[1], w:t.width, h:t.height };
					let vc = vec4::new( 0f32, 0f32, 1f32, 1f32 );
					data.insert( ~"u_Center",		shade::UniFloatVec(vc) );
					data.insert( ~"u_Transform",	self.transform(&rect,screen_size) );
					calls.push( self.make_call( self.program_image, data, out, None ));
					[t.width,t.height]
				},
				&gen::ElText(ref text)	=>	{
					let fc = self.cache_fonts.find( &text.font ).
						expect(fmt!( "Font '%s' is not loaded", text.font.path ));
					let t = *fc.cache.find( &text.value ).
						expect(fmt!( "Text '%s' is not loaded", text.value ));
					let mut data = shade::DataMap::new();
					data.insert( ~"t_Text",	shade::UniTexture(
						0, t, Some(self.sampler_text) ));
					data.insert( ~"u_Color", Context::get_color_param(text.color) );
					let dr = Rect{ x:off[0], y:off[0], w:t.width, h:t.height };
					data.insert( ~"u_Transform", self.transform(&dr,screen_size) );
					// return
					calls.push( self.make_call( self.program_text, data, out, None ));
					[t.width,t.height]
				},
				&gen::ElSpace(space)	=> space,
				&gen::ElFrame(ref frame)	=>	{
					let a2 = Rect{ x:off[0],y:off[1], w:area.x+area.w-off[0], h:area.y+area.h-off[1] };
					let (size,sub) = self.draw( frame.children, a2, out, screen_size );
					calls.push_all_move( sub );
					size
				},
			};
			off[0] += size[0];
			off[1] += size[1];
		}
		(off,calls)
	}
}
