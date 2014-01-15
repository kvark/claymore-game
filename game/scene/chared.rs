extern mod cgmath;
extern mod engine;
extern mod gen_scene;

use std;
use glfw;
use cgmath::angle;
use cgmath::angle::ToRad;
use cgmath::quaternion::{Quat};
use cgmath::transform::Transform;
use cgmath::vector::{EuclideanVector,Vector,Vec3,Vec4};
use engine::anim;
use engine::anim::{Act,Player};
use engine::{gr_low,gr_mid};

use hud = hud::main_json;
use input;
use main;
use render;
use scene;


struct Envir	{
	input	: gr_mid::call::Input,
	prog	: gr_low::shade::ProgramPtr,
	data	: gr_low::shade::DataMap,
	rast	: gr_low::rast::State,
}

struct CamControl	{
	node		: engine::space::NodePtr,
	origin		: Vec3<f32>,
	speed_rot	: f32,
	speed_zoom	: f32,
	last_scroll	: Option<f32>,
	in_rotation	: bool,
}

impl CamControl	{
	pub fn update( &mut self, state : &input::State )	{
		// calculate rotation
		if self.in_rotation	{
			let dt = 1f32/30f32;	//FIXME
			let axis = Vec3::new( 0f32, 0f32, if state.mouse[0]>0.5 {1f32} else {-1f32} );
			let angle = angle::deg( dt as f32 * self.speed_rot );
			let qr = Quat::from_axis_angle( &axis, angle.to_rad() );
			let sq = engine::space::make( 1.0, qr, Vec3::zero() );
			let mut sn = self.node.borrow().borrow_mut();
			sn.get().space = sq.concat( &sn.get().space );
		}
	}

	pub fn on_scroll( &mut self, scroll : f32 )	{
		let mut sn = self.node.borrow().borrow_mut();
		let v_origin = self.origin.sub_v( &sn.get().space.disp );
		let dist_min = 20f32;
		let dist_max = 200f32;
		let dist = v_origin.length();
		let dist_raw : f32 = dist - scroll * self.speed_zoom;
		let dist_diff = std::num::clamp( dist_raw, dist_min, dist_max ) - dist;
		let p = (sn.get().space.disp).sub_v( &v_origin.mul_s(dist_diff/dist) );
		sn.get().space.disp = p;
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
	skel	: engine::space::ArmaturePtr,
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
	start	: anim::float,
	hud_screen	: hud::Screen,
	hud_context	: hud::Context,
	hud_debug	: gr_low::shade::ProgramPtr,
	edit_label	: hud::EditLabelPtr,
	mouse_point	: (int,int),
	input_queue	: ~str,
	hud_active	: ActiveHud,
}


impl Scene	{
	pub fn reset( &mut self, time : anim::float )	{
		self.start = time;
	}

	pub fn loose_focus( &mut self )	{
		self.edit_label.borrow().borrow_mut().get().active = false;
		self.hud_active = AhInactive;
	}

	pub fn on_input( &mut self, event : &input::Event, state : &input::State )	{
		match event	{
			&input::EvCharacter(c)	=> self.input_queue.push_char(c),
			&input::EvKeyboard(key,press)	=> {
				match (key,press)	{
					(glfw::KeyEscape,true)		=> self.loose_focus(),
					(glfw::KeyBackspace,true)	=> self.input_queue.push_char( 'c' ),	//FIXME
					_	=> ()
				}
			},
			&input::EvMouseClick(key,press) if key==0	=> {
				if press	{
					self.loose_focus();
					let (x,y) = self.mouse_point;
					let mut found_name = false;
					let mut found_any = false;
					self.hud_screen.root.trace(x,y,	|frame,depth|	{
						if depth>0u	{
							found_any = true;
						}
						if depth==0u && frame.name == ~"id.name.text"	{
							found_name = true;
						}
					});
					self.control.in_rotation = !found_any;
					if found_name	{
						self.edit_label.borrow().borrow_mut().get().active = true;
						self.hud_active = AhEditName;
					}
				}else	{
					self.control.in_rotation = false;
				}
			},
			&input::EvScroll(_,scroll)	=> self.control.on_scroll(scroll),
			&input::EvRender(_)	=>	{
				let tv = state.time_view;
				if true	{
					let (mx,my) = self.hud_screen.root.min_size;
					let x = ((0.0+state.mouse[0]) * (mx as f32)) as int;
					let y = ((1.0-state.mouse[1]) * (my as f32)) as int;
					self.mouse_point = (x,y);
					//let name = root.trace( x, y, lg );
					//io::println( ~"Click: " + name );
				}
				if true	{	// update animation
					let t = tv - self.start;
					let mut skel = self.skel.borrow().borrow_mut();
					let r = skel.get().actions[0];
					let nloops = (t / r.duration) as uint;
					let t2 = t - r.duration * (nloops as anim::float);
					skel.get().set_record( r, t2 );
					//skel.get().fill_data( self.girl.mut_data() );
				}
				self.edit_label.borrow().with_mut( |el| el.update(state.time_view) );
				self.control.update( state );
			},
			_	=> ()
		}
	}

	pub fn render( &mut self, el : &main::Elements, output : &gr_mid::call::Output,
			gc : &mut gr_low::context::Context, lg : &engine::journal::Log  )	{
		// clear screen
		let cdata = gr_mid::call::ClearData{
			color	:Some( gr_low::rast::Color::new(0x8080FFFF) ),
			depth	:Some( 1.0 ),
			stencil	:Some( 0u32 ),
		};
		let c0 = gr_mid::call::CallClear( cdata, output.clone(), self.rast_solid.mask );
		let aspect = output.area.aspect();
		if el.environment	{
			let vpi = self.cam.get_inverse_matrix( aspect );
			//self.cam.fill_data( &mut self.envir.data );
			self.envir.data.set( ~"u_ViewProjInverse",
				gr_low::shade::UniMatrix(false,vpi) );
		}
		let c1 = if el.environment	{
			let e = &self.envir;
			gr_mid::call::CallDraw(
				e.input.clone(), output.clone(), e.rast, e.prog.clone(), e.data.clone() )
		}else	{
			gr_mid::call::CallEmpty
		};
		let mut queue = ~[c0,c1];
		if el.character	{
			let area = &output.area;
			let target_size = Vec4::new( area.w as f32, area.h as f32,
  				1f32/(area.w as f32), 1f32/(area.h as f32) );
			let par_ts = gr_low::shade::UniFloatVec( target_size );
			let char_groups = ~[&mut self.gr_main, &mut self.gr_cape, &mut self.gr_hair, &mut self.gr_other];
			for group in char_groups.move_iter()	{
				for ent in group.get_mut().mut_iter()	{
					ent.update_world();
					{
						let gd = &mut ent.data;
						self.shadow.light.fill_data( gd, 1f32, 200f32 );
						gd.set( ~"t_Shadow", self.shadow.par_shadow.clone() );
						gd.set( ~"u_TargetSize",	par_ts.clone() );
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
					for ent in group.get().iter()	{
						queue.push( self.shadow.tech_solid.process( ent,
							self.shadow.output.clone(), self.shadow.rast,
							&mut self.cache, gc, lg ));
					}
				}
				/*for [&self.gr_hair].each() |group|	{
					for group.each() |ent|	{
						queue.push( self.shadow.tech_alpha.process( ent, gc, lg ));
					}
				}*/
			}
		}
		let tech = match self.lbuf {
			Some(ref lbuf)	=>	{
				queue.push( self.depth.call_clear.clone() );
				let char_groups = ~[&mut self.gr_main,&mut self.gr_cape,&mut self.gr_hair];
				for group in char_groups.move_iter()	{			
					for ent in group.get_mut().mut_iter()	{
						queue.push( self.depth.tech_solid.process( ent,
							self.depth.output.clone(), self.depth.rast,
							&mut self.cache, gc, lg ));
						lbuf.fill_data( &mut ent.data );
					}
				}
				for ent in self.gr_other.get_mut().mut_iter()	{
					lbuf.fill_data( &mut ent.data );
				}
				queue.push( lbuf.update_depth( &self.depth.texture ));
				queue.push_all_move( lbuf.bake_layer(
					0u, self.lights, &self.lvolume, self.cam, gc, lg
					));
				lbuf.tech_apply
			},
			None	=> self.technique,
		};
		if el.character	{
			for ent in self.gr_main.get().iter()	{
				queue.push( tech.process( ent, output.clone(), self.rast_solid, &mut self.cache, gc, lg ) );
			}
			for ent in self.gr_cape.get().iter()	{
				queue.push( tech.process( ent, output.clone(), self.rast_cloak, &mut self.cache, gc, lg ) );	
			}
			for ent in self.gr_hair.get().iter()	{
				queue.push( tech.process( ent, output.clone(), self.rast_alpha, &mut self.cache, gc, lg ) );
			}
		}
		if el.shadow	{
			for ent in self.gr_other.get().iter()	{
				queue.push( tech.process( ent, output.clone(), self.rast_solid, &mut self.cache, gc, lg ) );
			}
		}
		if el.hud	{
			if !self.input_queue.is_empty()	{
				match self.hud_active	{
					AhEditName	=> {
						self.edit_label.borrow().with_mut( |el| el.change(self.input_queue, gc, lg) );
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
			let mut rast  = gc.default_rast;
			rast.prime.poly_mode = gr_low::rast::map_polygon_fill(2);
			let mut data = gr_low::shade::DataMap::new();
			let vc = Vec4::new(1f32,0f32,0f32,1f32);
			data.set( ~"u_Color", gr_low::shade::UniFloatVec(vc) );
			self.hud_screen.root.trace(x,y,	|frame,depth| {
				if depth==0u && frame.element.get_size()!=(0,0)	{
					let call = frame.draw_debug( &self.hud_context,
						&self.hud_debug, &mut data, &rast );
					queue.push(call);
				}
			});
		}
		if el.hud_debug	{
			queue.push_all_move({
				let mut rast  = gc.default_rast;
				rast.prime.poly_mode = gr_low::rast::map_polygon_fill(2);
				let mut data = gr_low::shade::DataMap::new();
				let vc = Vec4::new(1f32,0f32,0f32,1f32);
				data.set( ~"u_Color", gr_low::shade::UniFloatVec(vc) );
				self.hud_screen.root.draw_debug_all( &self.hud_context,
					&self.hud_debug, &mut data, &rast )
			});
		}
		gc.flush( queue, lg );
	}
}


pub fn create( el : &main::Elements, gc : &mut gr_low::context::Context, fcon : &gr_mid::font::Context,
		lg : &engine::journal::Log )-> Scene	{
	let vao = gc.create_vertex_array();
	let mut scene = if true	{ //new method
		let iscene = gen_scene::chared::main::load();
		let icustom = gen_scene::chared::custom::load();
		scene::load::parse( "data/scene/claymore-2a", &iscene, icustom, gc, Some(vao.clone()), lg )
	}else	{
		scene::load_json::load_scene( "data/scene/claymore-2a", gc, Some(vao.clone()), lg )
	};
	let detail_info = scene::load_json::load_config::<~[scene::load_json::EntityInfo]>( "data/details.json" );
	let mut details = scene::load_json::parse_group( &mut scene.context, detail_info, gc, Some(vao.clone()), lg );
	// techniques & rast states
	let tech = @gr_mid::draw::load_technique( "data/code/tech/forward/spot-shadow" );
	let mut rast = gc.default_rast;
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
	let mut group = scene.entities.divide( &"noTrasnform" );
	group.swap_entity( &"boots", &mut details );
	let cape = group.divide( &"polySurface172" );
	let hair = group.divide( &"Hair_Geo2" );
	lg.add(format!( "Group size: {:u}", group.get().len() ));
	let envir = {
		let mesh = @gr_mid::mesh::create_quad( gc );
		let mut data = gr_low::shade::DataMap::new();
		let samp = gr_low::texture::Sampler::new(3u,1);
		let use_spherical = false;
		let prog = if use_spherical	{
			let tex = scene.context.textures.get( &~"data/texture/Topanga_Forest_B_3k.hdr" );
			data.set( ~"t_Environment",	gr_low::shade::UniTexture(0, tex.clone(), Some(samp)) );
			engine::load::load_program( gc, "data/code-game/envir", lg )
		}else	{
			let tex = engine::load::load_texture_2D( gc, "data/texture/bg2.jpg", true );
			data.set( ~"t_Image",		gr_low::shade::UniTexture(0, tex, Some(samp)) );
			engine::load::load_program( gc, "data/code-game/copy", lg )
		};
		let rast = gc.default_rast;
		//rast.set_depth( ~"<=", false );
		Envir{
			input	: gr_mid::call::Input::new( &vao, mesh ),
			prog	: prog,
			data	: data,
			rast	: rast,
		}		
	};
	// load char HUD
	let mut hud_screen = hud::load_screen( "data/hud/char.json", gc, fcon, lg );
	hud_screen.root.update( lg );
	let hc = {
		let mut hud_rast = gc.default_rast;
		hud_rast.set_blend( "s+d", "Sa", "1-Sa" );
		let quad = @gr_mid::mesh::create_quad(gc);
		let pmap = gr_mid::call::PlaneMap::new_main( gc, ~"o_Color" );
		let out = gr_mid::call::Output::new( &gc.default_frame_buffer, pmap );
		hud::Context{
			input	: gr_mid::call::Input::new( &vao, quad ),
			output	: out,
			rast	: hud_rast,
			size	: gc.get_screen_size(),
		}
	};
	let edit_label = hud::EditLabel::obtain( &mut hud_screen, ~"id.name.text" );
	let hdebug = engine::load::load_program( gc, "data/code/hud/debug", lg );
	//arm.set_record( arm.actions[0], 0f );
	let depth = render::depth::Data::create( gc );
	let lbuf = if el.lbuffer!=0	{
		Some( render::lbuf::Context::create( gc, 2u, el.lbuffer ))
	}else	{None};
	let lvolume = render::lbuf::LightVolume::create( gc, lg );
	let shadow = render::shadow::create_data( gc, *scene.lights.get(&~"Lamp"), 0x200u );
	// load camera
	let cam = *scene.cameras.get(&~"Camera");
	//cam.proj = shadow.light.proj;
	//cam.node = shadow.light.node;
	lg.add(format!( "Camera fov:{:s}, range:{:f}-{:f}",
		cam.proj.fovy.to_str(),
		cam.proj.near,
		cam.proj.far ));
	lg.add( ~"\tWorld :" + cam.node.borrow().with(|c| c.world_space().to_str()) );
	let control = CamControl{
		node	: cam.node,
		origin	: Vec3::new(0f32,0f32,75f32),
		speed_rot	: 1.5f32,
		speed_zoom	: 15f32,
		last_scroll	: None,
		in_rotation	: false,
	};
	let lights = scene.lights.values().map(|&l| l).to_owned_vec();
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
		start	: 0.0,
		hud_screen	: hud_screen,
		hud_context : hc,
		hud_debug	: hdebug,
		edit_label	: edit_label,
		mouse_point	: (0,0),
		input_queue	: ~"",
		hud_active	: AhInactive,
	}
}
