extern mod cgmath;
extern mod engine;
extern mod gen_scene;

use std;
use glfw;
use cgmath::{angle,rotation};
use cgmath::quaternion::*;
use cgmath::vector::*;
use engine::anim::{Act,Player};
use engine::{gr_low,gr_mid};
use engine::space::Space;

use hud = hud::main_json;
use input;
use main;
use render;
use scene;


struct Envir	{
	input	: gr_mid::call::Input,
	prog	: @gr_low::shade::Program,
	data	: gr_low::shade::DataMap,
	rast	: gr_low::rast::State,
}

struct CamControl	{
	node		: @mut engine::space::Node,
	origin		: Vec3<f32>,
	speed_rot	: f32,
	speed_zoom	: f32,
	last_scroll	: Option<float>,
	in_rotation	: bool,
}

impl CamControl	{
	pub fn update( &mut self, input : &input::State )	{
		// calculate rotation
		if self.in_rotation	{
			let dt = 1f32/30f32;	//FIXME
			let axis = Vec3::new( 0f32, 0f32, if input.mouse.x>0.5f {1f32} else {-1f32} );
			let angle = angle::deg( dt as f32 * self.speed_rot );
			let qr = rotation::AxisAngle::new( axis, angle ).to_quat();
			let sq = engine::space::QuatSpace{
				position : Vec3::new(0f32,0f32,0f32),
				orientation : qr, scale : 1f32 };
			self.node.space = sq.concat( &self.node.space );
		}
	}

	pub fn on_scroll( &mut self, scroll : float )	{
		let v_origin = self.origin.sub_v( &self.node.space.position );
		let dist_min = 20f32;
		let dist_max = 200f32;
		let dist = v_origin.length();
		let dist_raw = dist - (scroll as f32) * self.speed_zoom;
		let dist_diff = std::num::clamp( dist_raw, dist_min, dist_max ) - dist;
		let p = (self.node.space.position).sub_v( &v_origin.mul_s(dist_diff/dist) );
		self.node.space.position = p;
	}
}


enum ActiveHud	{
	AhInactive,
	AhEditName,
}

pub struct Scene	{
	gr_main	: scene::common::EntityGroup,
	gr_cape	: scene::common::EntityGroup,
	gr_hair	: scene::common::EntityGroup,
	gr_other: scene::common::EntityGroup,
	details	: scene::common::EntityGroup,
	skel	: @mut engine::space::Armature,
	cam		: @scene::common::Camera,
	control	: CamControl,
	lights	: ~[@scene::common::Light],
	envir	: Envir,
	technique	: @gr_mid::draw::Technique,
	cache		: gr_mid::draw::Cache,
	rast_solid	: gr_low::rast::State,
	rast_cloak	: gr_low::rast::State,
	rast_alpha	: gr_low::rast::State,
	depth	: render::depth::Data,
	lbuf	: Option<render::lbuf::Context>,
	lvolume	: render::lbuf::LightVolume,
	shadow	: render::shadow::Data,
	start	: float,
	hud_screen	: hud::Screen,
	hud_context	: hud::Context,
	hud_debug	: @gr_low::shade::Program,
	edit_label	: @mut hud::EditLabel,
	mouse_point	: (int,int),
	input_queue	: ~str,
	hud_active	: ActiveHud,
}


impl Scene	{
	pub fn loose_focus( &mut self )	{
		self.edit_label.active = false;
		self.hud_active = AhInactive;
	}

	pub fn update( &mut self, input : &input::State, _lg : &engine::journal::Log )-> bool	{
		if true	{
			let (mx,my) = self.hud_screen.root.min_size;
			let x = ((0f+input.mouse.x) * (mx as float)) as int;
			let y = ((1f-input.mouse.y) * (my as float)) as int;
			self.mouse_point = (x,y);
			//let name = root.trace( x, y, lg );
			//io::println( ~"Click: " + name );
		}
		self.edit_label.update();
		self.control.update( input );
		true
	}

	pub fn on_input( &mut self, event : &input::Event )	{
		match event	{
			&input::Character(c)	=> self.input_queue.push_char(c),
			&input::Keyboard(key,press)	=> {
				match (key,press)	{
					(glfw::KeyEscape,true)		=> self.loose_focus(),
					(glfw::KeyBackspace,true)	=> self.input_queue.push_char( 'c' ),	//FIXME
					_	=> ()
				}
			},
			&input::MouseClick(key,press) if key==0	=> {
				if press	{
					self.loose_focus();
					let (x,y) = self.mouse_point;
					let mut found_name = false;
					let mut found_any = false;
					do self.hud_screen.root.trace(x,y)	|frame,depth|	{
						if depth>0u	{
							found_any = true;
						}
						if depth==0u && frame.name == ~"id.name.text"	{
							found_name = true;
						}
					};
					self.control.in_rotation = !found_any;
					if found_name	{
						self.edit_label.active = true;
						self.hud_active = AhEditName;
					}
				}else	{
					self.control.in_rotation = false;
				}
			},
			&input::Scroll(_,scroll)	=> self.control.on_scroll(scroll),
			_	=> ()
		}
	}

	pub fn render( &mut self, el : &main::Elements, ct : &mut gr_low::context::Context,
			output : &gr_mid::call::Output, lg : &engine::journal::Log  )	{
		// clear screen
		let cdata = gr_mid::call::ClearData{
			color	:Some( gr_low::rast::Color::new(0x8080FFFF) ),
			depth	:Some( 1f ),
			stencil	:Some( 0u ),
		};
		let c0 = gr_mid::call::CallClear( cdata, output.clone(), self.rast_solid.mask );
		let aspect = output.area.aspect();
		if el.environment	{
			let vpi = self.cam.get_inverse_matrix( aspect );
			//self.cam.fill_data( &mut self.envir.data );
			self.envir.data.insert( ~"u_ViewProjInverse",
				gr_low::shade::UniMatrix(false,vpi) );
		}
		let c1 = if el.environment	{
			let e = &self.envir;
			gr_mid::call::CallDraw(
				e.input, output.clone(), e.rast, e.prog, e.data.clone() )
		}else	{
			gr_mid::call::CallEmpty
		};
		let mut queue = ~[c0,c1];
		if true	{	// update animation
			let t = engine::anim::get_time() - self.start;
			let r = self.skel.actions[0];
			let nloops = (t / r.duration) as uint;
			let t2 = t - r.duration * (nloops as float);
			self.skel.set_record( r, t2 );
			//self.skel.fill_data( self.girl.mut_data() );
		}
		if el.character	{
			let area = &output.area;
			let target_size = Vec4::new( area.w as f32, area.h as f32,
  				1f32/(area.w as f32), 1f32/(area.h as f32) );
			let par_ts = gr_low::shade::UniFloatVec( target_size );
			let char_groups = ~[&mut self.gr_main, &mut self.gr_cape, &mut self.gr_hair, &mut self.gr_other];
			for group in char_groups.move_iter()	{
				for ent in group.mut_iter()	{
					ent.update_world();
					{
						let gd = &mut ent.data;
						self.shadow.light.fill_data( gd, 1f32, 200f32 );
						gd.insert( ~"t_Shadow", self.shadow.par_shadow.clone() );
						gd.insert( ~"u_TargetSize",	par_ts.clone() );
						self.cam.fill_data( gd, aspect );
						//self.skel.fill_data( gd );
					}
				}	
			}
		}
		if el.shadow	{
			queue.push( self.shadow.call_clear.clone() );
			if el.character	{
				let char_shadow_groups = [&self.gr_main,&self.gr_cape,&self.gr_hair];
				for group in char_shadow_groups.iter()	{
					for ent in group.iter()	{
						queue.push( self.shadow.tech_solid.process( ent,
							self.shadow.output.clone(), self.shadow.rast,
							&mut self.cache, ct, lg ));
					}
				}
				/*for [&self.gr_hair].each() |group|	{
					for group.each() |ent|	{
						queue.push( self.shadow.tech_alpha.process( ent, ct, lg ));
					}
				}*/
			}
		}
		let tech = match self.lbuf {
			Some(ref lbuf)	=>	{
				queue.push( self.depth.call_clear.clone() );
				let char_groups = ~[&mut self.gr_main,&mut self.gr_cape,&mut self.gr_hair];
				for group in char_groups.move_iter()	{
					for ent in group.mut_iter()	{
						queue.push( self.depth.tech_solid.process( ent,
							self.depth.output.clone(), self.depth.rast,
							&mut self.cache, ct, lg ));
						lbuf.fill_data( &mut ent.data );
					}
				}
				for ent in self.gr_other.mut_iter()	{
					lbuf.fill_data( &mut ent.data );
				}
				queue.push( lbuf.update_depth( self.depth.texture ));
				queue.push_all_move( lbuf.bake_layer(
					0u, self.lights, &self.lvolume, self.cam, ct, lg
					));
				lbuf.tech_apply
			},
			None	=> self.technique,
		};
		if el.character	{
			for ent in self.gr_main.iter()	{
				queue.push( tech.process( ent, output.clone(), self.rast_solid, &mut self.cache, ct, lg ) );
			}
			for ent in self.gr_cape.iter()	{
				queue.push( tech.process( ent, output.clone(), self.rast_cloak, &mut self.cache, ct, lg ) );	
			}
			for ent in self.gr_hair.iter()	{
				queue.push( tech.process( ent, output.clone(), self.rast_alpha, &mut self.cache, ct, lg ) );
			}
		}
		if el.shadow	{
			for ent in self.gr_other.iter()	{
				queue.push( tech.process( ent, output.clone(), self.rast_solid, &mut self.cache, ct, lg ) );
			}
		}
		if el.hud	{
			if !self.input_queue.is_empty()	{
				match self.hud_active	{
					AhEditName	=> {
						self.edit_label.change( self.input_queue, ct, lg );
						self.hud_screen.root.update( lg );
					},
					_	=> ()
				}
				self.input_queue = ~"";
			}
			queue.push_all_move(
				self.hud_screen.root.draw_all( &self.hud_context )
				);
			let (x,y) = self.mouse_point;
			let mut rast  = ct.default_rast;
			rast.prime.poly_mode = gr_low::rast::map_polygon_fill(2);
			let mut data = gr_low::shade::DataMap::new();
			let vc = Vec4::new(1f32,0f32,0f32,1f32);
			data.insert( ~"u_Color", gr_low::shade::UniFloatVec(vc) );
			do self.hud_screen.root.trace(x,y)	|frame,depth| {
				if depth==0u && frame.element.get_size()!=(0,0)	{
					let call = frame.draw_debug( &self.hud_context,
							self.hud_debug, &mut data, &rast );
					queue.push(call);
				}
			};
		}
		if el.hud_debug	{
			queue.push_all_move({
				let mut rast  = ct.default_rast;
				rast.prime.poly_mode = gr_low::rast::map_polygon_fill(2);
				let mut data = gr_low::shade::DataMap::new();
				let vc = Vec4::new(1f32,0f32,0f32,1f32);
				data.insert( ~"u_Color", gr_low::shade::UniFloatVec(vc) );
				self.hud_screen.root.draw_debug_all( &self.hud_context,
					self.hud_debug, &mut data, &rast )
			});
		}
		ct.flush( queue, lg );
	}
}


pub fn create( el : &main::Elements, ct : &mut gr_low::context::Context, fcon : &gr_mid::font::Context,
		lg : &engine::journal::Log )-> Scene	{
	let vao = ct.create_vertex_array();
	let mut scene = if true	{ //new method
		let iscene = gen_scene::chared::main::load();
		let icustom = gen_scene::chared::custom::load();
		scene::load::parse( "data/scene/claymore-2a", &iscene, icustom, ct, Some(vao), lg )
	}else	{
		scene::load_json::load_scene( "data/scene/claymore-2a", ct, Some(vao), lg )
	};
	let detail_info = scene::load_json::load_config::<~[scene::load_json::EntityInfo]>( "data/details.json" );
	let mut details = scene::load_json::parse_group( &mut scene.context, detail_info, ct, Some(vao), lg );
	// techniques & rast states
	let tech = @gr_mid::draw::load_technique( "data/code/tech/forward/spot-shadow" );
	let mut rast = ct.default_rast;
	rast.depth.test = true;
	rast.prime.cull = true;
	let r_solid = rast;
	rast.prime.cull = false;
	let r_cloak = rast;
	rast.prime.cull = true;
	rast.set_blend( "s+d", "Sa", "1-Sa" );
	let r_alpha = rast;
	// armature
	let arm = { *scene.context.armatures.get(&~"Armature.002") };
	let mut group = scene.entities.divide( &~"noTrasnform" );
	group.swap_entity( &~"boots", &mut details );
	let cape = group.divide( &~"polySurface172" );
	let hair = group.divide( &~"Hair_Geo2" );
	lg.add(fmt!( "Group size: %u", group.len() ));
	let envir = {
		let mesh = @gr_mid::mesh::create_quad( ct );
		let mut data = gr_low::shade::DataMap::new();
		let samp = gr_low::texture::Sampler::new(3u,1);
		let use_spherical = false;
		let prog = if use_spherical	{
			let tex = *scene.context.textures.get( &~"data/texture/Topanga_Forest_B_3k.hdr" );
			data.insert( ~"t_Environment",	gr_low::shade::UniTexture(0,tex,Some(samp)) );
			engine::load::load_program( ct, "data/code-game/envir", lg )
		}else	{
			let tex = engine::load::load_texture_2D( ct, "data/texture/bg2.jpg", true );
			data.insert( ~"t_Image",		gr_low::shade::UniTexture(0,tex,Some(samp)) );
			engine::load::load_program( ct, "data/code-game/copy", lg )
		};
		let rast = ct.default_rast;
		//rast.set_depth( ~"<=", false );
		Envir{
			input	: gr_mid::call::Input::new( vao, mesh ),
			prog	: prog,
			data	: data,
			rast	: rast,
		}		
	};
	// load char HUD
	let mut hud_screen = hud::load_screen( "data/hud/char.json", ct, fcon, lg );
	hud_screen.root.update( lg );
	let hc = {
		let mut hud_rast = ct.default_rast;
		hud_rast.set_blend( "s+d", "Sa", "1-Sa" );
		let quad = @gr_mid::mesh::create_quad(ct);
		let pmap = gr_mid::call::PlaneMap::new_main( ct, ~"o_Color" );
		let out = gr_mid::call::Output::new( ct.default_frame_buffer, pmap );
		hud::Context{
			input	: gr_mid::call::Input::new( vao, quad ),
			output	: out,
			rast	: hud_rast,
			size	: ct.get_screen_size(),
		}
	};
	let edit_label = @mut hud::EditLabel::obtain( &mut hud_screen, ~"id.name.text" );
	let hdebug = engine::load::load_program( ct, "data/code/hud/debug", lg );
	//arm.set_record( arm.actions[0], 0f );
	let depth = render::depth::Data::create( ct );
	let lbuf = if el.lbuffer!=0	{
		Some( render::lbuf::Context::create( ct, 2u, el.lbuffer ))
	}else	{None};
	let lvolume = render::lbuf::LightVolume::create( ct, lg );
	let shadow = render::shadow::create_data( ct, *scene.lights.get(&~"Lamp"), 0x200u );
	// load camera
	let cam = *scene.cameras.get(&~"Camera");
	//cam.proj = shadow.light.proj;
	//cam.node = shadow.light.node;
	lg.add(fmt!( "Camera fov:%s, range:%f-%f",
		cam.proj.fovy.to_str(),
		cam.proj.near as float,
		cam.proj.far as float ));
	lg.add( ~"\tWorld :" + cam.node.world_space().to_str() );
	let control = CamControl{
		node	: cam.node,
		origin	: Vec3::new(0f32,0f32,75f32),
		speed_rot	: 1.5f32,
		speed_zoom	: 15f32,
		last_scroll	: None,
		in_rotation	: false,
	};
	let mut lights : ~[@scene::common::Light] = ~[];
	do scene.lights.each_value() |&val|	{
		lights.push( val ); true
	};
	Scene	{
		gr_main	: group,
		gr_cape	: cape,
		gr_hair	: hair,
		gr_other: scene.entities,
		details	: details,
		skel	: arm,
		cam		: cam,
		control	: control,
		lights	: lights,
		envir	: envir,
		technique	: tech,
		cache		: gr_mid::draw::make_cache(),
		rast_solid	: r_solid,
		rast_cloak	: r_cloak,
		rast_alpha	: r_alpha,
		depth		: depth,
		lbuf		: lbuf,
		lvolume		: lvolume,
		shadow	: shadow,
		start	: engine::anim::get_time(),
		hud_screen	: hud_screen,
		hud_context : hc,
		hud_debug	: hdebug,
		edit_label	: edit_label,
		mouse_point	: (0,0),
		input_queue	: ~"",
		hud_active	: AhInactive,
	}
}
