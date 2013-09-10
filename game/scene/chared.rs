extern mod numeric;
extern mod lmath;
extern mod engine;
extern mod gen_scene;

use lmath::quat::*;
use lmath::vec::*;
use engine::anim::{Act,Player};
use engine::{gr_low,gr_mid};

use hud;
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

pub fn clamp<T:cmp::Ord>( x:T, a:T, b:T )-> T	{
	if x>a {
		if x<b	{x}
		else	{b}
	}else {a}
}

pub impl CamControl	{
	fn update( &mut self, input : &input::State )	{
		// calculate rotation
		if self.in_rotation	{
			let dt = 1f32/30f32;	//FIXME
			let axis = vec3::new( 0f32, 0f32, if input.mouse.x>0.5f {1f32} else {-1f32} );
			let angle = dt as f32 * self.speed_rot;
			let qr = Quat::from_angle_axis( angle, &axis );
			let sq = engine::space::QuatSpace{
				position : vec3::new(0f32,0f32,0f32),
				orientation : qr, scale : 1f32 };
			self.node.space = sq * self.node.space;
		}
	}

	fn on_scroll( &mut self, scroll : float )	{
		let v_origin = self.origin.sub_v( &copy self.node.space.position );
		let dist_min = 20f32;
		let dist_max = 200f32;
		let dist = v_origin.length();
		let dist_raw = dist - (scroll as f32) * self.speed_zoom;
		let dist_diff = clamp( dist_raw, dist_min, dist_max ) - dist;
		let p = (copy self.node.space.position).sub_v( &v_origin.mul_t(dist_diff/dist) );
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


pub impl Scene	{
	fn loose_focus( &mut self )	{
		self.edit_label.active = false;
		self.hud_active = AhInactive;
	}

	fn update( &mut self, input : &input::State, _lg : &engine::journal::Log )-> bool	{
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

	fn on_input( &mut self, event : &input::Event )	{
		match event	{
			&input::Character(c)	=> str::push_char( &mut self.input_queue, c ),
			&input::Keyboard(key,press)	=> {
				match (key,press)	{
					(257,true)	=> self.loose_focus(),
					(259,true)	=> str::push_char( &mut self.input_queue, key as char ),
					_	=> ()
				}
			},
			&input::MouseClick(key,press) if key==0	=> {
				if press	{
					self.loose_focus();
					let (x,y) = self.mouse_point;
					let mut found_name = false, found_any = false;
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

	fn render( &mut self, el : &main::Elements, ct : &mut gr_low::context::Context, lg : &engine::journal::Log  )	{
		// clear screen
		let pmap = gr_mid::call::PlaneMap::new_simple( ~"o_Color", gr_low::frame::TarEmpty );
		let output = gr_mid::call::Output::new( ct.default_frame_buffer, pmap );
		let cdata = gr_mid::call::ClearData{
			color	:Some( gr_low::rast::Color::new(0x8080FFFF) ),
			depth	:Some( 1f ),
			stencil	:Some( 0u ),
		};
		let c0 = gr_mid::call::CallClear( copy output, cdata, copy self.rast_solid.mask );
		let aspect = ct.get_aspect();
		if el.environment	{
			let vpi = self.cam.get_inverse_matrix( aspect );
			//self.cam.fill_data( &mut self.envir.data );
			self.envir.data.insert( ~"u_ViewProjInverse",
				gr_low::shade::UniMatrix(false,vpi) );
		}
		let c1 = if el.environment	{
			let e = &self.envir;
			gr_mid::call::CallDraw(
				copy e.input, copy output, copy e.rast, e.prog, copy e.data )
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
			let (wid,het) = ct.screen_size;
			let target_size = vec4::new( wid as f32, het as f32,
  				1f32/(wid as f32), 1f32/(het as f32) );
			let par_ts = gr_low::shade::UniFloatVec( target_size );
			for [&mut self.gr_main, &mut self.gr_cape, &mut self.gr_hair, &mut self.gr_other].each() |group|	{
				for group.each_mut() |ent|	{
					ent.update_world();
					{
						let gd = &mut ent.data;
						self.shadow.light.fill_data( gd, 1f32, 200f32 );
						gd.insert( ~"t_Shadow", copy self.shadow.par_shadow );
						gd.insert( ~"u_TargetSize",	copy par_ts );
						self.cam.fill_data( gd, aspect );
						//self.skel.fill_data( gd );
					}
				}	
			}
		}
		if el.shadow	{
			queue.push( copy self.shadow.call_clear );
			if el.character	{
				for [&self.gr_main,&self.gr_cape,&self.gr_hair].each() |group|	{
					for group.each() |ent|	{
						queue.push( self.shadow.tech_solid.process( ent,
							copy self.shadow.output, copy self.shadow.rast,
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
				queue.push( copy self.depth.call_clear );
				for [&mut self.gr_main,&mut self.gr_cape,&mut self.gr_hair].each() |group|	{
					for group.each_mut() |ent|	{
						queue.push( self.depth.tech_solid.process( ent,
							copy self.depth.output, copy self.depth.rast,
							&mut self.cache, ct, lg ));
						lbuf.fill_data( &mut ent.data );
					}
				}
				for self.gr_other.each_mut() |ent|	{
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
			for self.gr_main.each() |ent|	{
				queue.push( tech.process( ent, copy output, copy self.rast_solid, &mut self.cache, ct, lg ) );
			}
			for self.gr_cape.each() |ent|	{
				queue.push( tech.process( ent, copy output, copy self.rast_cloak, &mut self.cache, ct, lg ) );	
			}
			for self.gr_hair.each() |ent|	{
				queue.push( tech.process( ent, copy output, copy self.rast_alpha, &mut self.cache, ct, lg ) );
			}
		}
		if el.shadow	{
			for self.gr_other.each() |ent|	{
				queue.push( tech.process( ent, copy output, copy self.rast_solid, &mut self.cache, ct, lg ) );
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
			let mut rast  = copy ct.default_rast;
			rast.prime.poly_mode = gr_low::rast::map_polygon_fill(2);
			let mut data = gr_low::shade::make_data();
			let vc = vec4::new(1f32,0f32,0f32,1f32);
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
				let mut rast  = copy ct.default_rast;
				rast.prime.poly_mode = gr_low::rast::map_polygon_fill(2);
				let mut data = gr_low::shade::make_data();
				let vc = vec4::new(1f32,0f32,0f32,1f32);
				data.insert( ~"u_Color", gr_low::shade::UniFloatVec(vc) );
				self.hud_screen.root.draw_debug_all( &self.hud_context,
					self.hud_debug, &mut data, &rast )
			});
		}
		ct.flush(queue);
	}
}


pub fn make_scene( el : &main::Elements, ct : &mut gr_low::context::Context, fcon : @gr_mid::font::Context,
		lg : &engine::journal::Log )-> Scene	{
	let vao = ct.create_vertex_array();
	let mut scene = if true	{ //new method
		let iscene = gen_scene::chared::main::load();
		let icustom = gen_scene::chared::custom::load();
		scene::load::parse( ~"data/scene/claymore-2a", &iscene, icustom, ct, Some(vao), lg )
	}else	{
		scene::load_json::load_scene( ~"data/scene/claymore-2a", ct, Some(vao), lg )
	};
	let detail_info = scene::load_json::load_config::<~[scene::load_json::EntityInfo]>( ~"data/details.json" );
	let mut details = scene::load_json::parse_group( &mut scene.context, detail_info, ct, Some(vao), lg );
	// techniques & rast states
	let tech = @gr_mid::draw::load_technique( ~"data/code/tech/forward/spot-shadow" );
	let mut rast = copy ct.default_rast;
	rast.depth.test = true;
	rast.prime.cull = true;
	let r_solid = copy rast;
	rast.prime.cull = false;
	let r_cloak = copy rast;
	rast.prime.cull = true;
	rast.set_blend( ~"s+d", ~"Sa", ~"1-Sa" );
	let r_alpha = copy rast;
	// armature
	let arm = { *scene.context.armatures.get(&~"Armature.002") };
	let mut group = scene.entities.divide( &~"noTrasnform" );
	group.swap_entity( &~"boots", &mut details );
	let cape = group.divide( &~"polySurface172" );
	let hair = group.divide( &~"Hair_Geo2" );
	lg.add(fmt!( "Group size: %u", group.len() ));
	let envir = {
		let mesh = @gr_mid::mesh::create_quad( ct );
		let mut data = gr_low::shade::make_data();
		let samp = gr_low::texture::Sampler::new(3u,1);
		let use_spherical = false;
		let prog = if use_spherical	{
			let tex = *scene.context.textures.get( &~"data/texture/Topanga_Forest_B_3k.hdr" );
			data.insert( ~"t_Environment",	gr_low::shade::UniTexture(0,tex,Some(samp)) );
			engine::load::load_program( ct, ~"data/code-game/envir", lg )
		}else	{
			let tex = engine::load::load_texture_2D( ct, &~"data/texture/bg2.jpg", true );
			data.insert( ~"t_Image",		gr_low::shade::UniTexture(0,tex,Some(samp)) );
			engine::load::load_program( ct, ~"data/code-game/copy", lg )
		};
		let mut rast = copy ct.default_rast;
		//rast.set_depth( ~"<=", false );
		Envir{
			input	: gr_mid::call::Input::new( vao, mesh ),
			prog	: prog,
			data	: data,
			rast	: rast,
		}		
	};
	// load char HUD
	let mut hud_screen = hud::load_screen( ~"data/hud/char.json", ct, fcon, lg );
	hud_screen.root.update( lg );
	let hc = {
		let mut hud_rast = copy ct.default_rast;
		hud_rast.set_blend( ~"s+d", ~"Sa", ~"1-Sa" );
		let quad = @gr_mid::mesh::create_quad(ct);
		let pmap = gr_mid::call::PlaneMap::new_simple( ~"o_Color", gr_low::frame::TarEmpty );
		let out = gr_mid::call::Output::new( ct.default_frame_buffer, pmap );
		hud::Context{
			input	: gr_mid::call::Input::new( vao, quad ),
			output	: out,
			rast	: hud_rast,
			size	: ct.screen_size,
		}
	};
	let edit_label = @mut hud::EditLabel::obtain( &mut hud_screen, ~"id.name.text" );
	let hdebug = engine::load::load_program( ct, ~"data/code/hud/debug", lg );
	//arm.set_record( arm.actions[0], 0f );
	let depth = render::depth::Data::create( ct );
	let lbuf = if el.lbuffer!=0	{
		Some( render::lbuf::Context::create( ct, 2u, el.lbuffer ))
	}else	{None};
	let lvolume = render::lbuf::LightVolume::create( ct, lg );
	let shadow = render::shadow::create_data( ct, *scene.lights.get(&~"Lamp"), 0x200u );
	// load camera
	let cam = *scene.cameras.get(&~"Camera");
	//cam.proj = copy shadow.light.proj;
	//cam.node = shadow.light.node;
	lg.add(fmt!( "Camera fov:%f, range:%f-%f",
		cam.proj.vfov as float,
		cam.proj.near as float,
		cam.proj.far as float ));
	lg.add( ~"\tWorld :" + cam.node.world_space().to_str() );
	let control = CamControl{
		node	: cam.node,
		origin	: vec3::new(0f32,0f32,75f32),
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
		gr_other: copy scene.entities,
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
