extern mod engine;
extern mod lmath;
extern mod numeric;

use numeric::Float;
use lmath::quat::*;
use lmath::vec::*;
use engine::anim::Player;
use engine::space::Pretty;

use hud;
use main;
use render;
use scene;


struct Envir	{
	input	: engine::call::DrawInput,
	prog	: @engine::shade::Program,
	data	: engine::shade::DataMap,
	rast	: engine::rast::State,
}

struct CamControl	{
	node		: @engine::space::Node,
	origin		: Vec3<f32>,
	speed_rot	: f32,
	speed_zoom	: f32,
	last_scroll	: Option<float>,
}

pub fn clamp<T:cmp::Ord>( x:T, a:T, b:T )-> T	{
	if x>a {
		if x<b	{x}
		else	{b}
	}else {a}
}

pub impl CamControl	{
	fn update( &mut self, dt : float, nx : float, _ny : float, hit : bool, scroll : float )	{
		// calculate rotation
		if hit	{
			let axis = vec3::new( 0f32, 0f32, if nx>0.5f {1f32} else {-1f32} );
			let angle = dt as f32 * self.speed_rot;
			let qr = Quat::from_angle_axis( angle, &axis );
			let sq = engine::space::QuatSpace{
				position : vec3::new(0f32,0f32,0f32),
				orientation : qr, scale : 1f32 };
			self.node.space = sq * self.node.space;
		}
		// calculate scroll
		if scroll != 0f	{
			let v_origin = self.origin.sub_v( &self.node.space.position );
			let dist_min = 20f32;
			let dist_max = 200f32;
			let dist = v_origin.length();
			let dist_raw = dist - (scroll as f32) * self.speed_zoom;
			let dist_diff = clamp( dist_raw, dist_min, dist_max ) - dist;
			let p = self.node.space.position.sub_v( &v_origin.mul_t(dist_diff/dist) );
			self.node.space.position = p;
		}
	}
}


enum ActiveHud	{
	AhInactive,
	AhEditName,
}

pub struct Scene	{
	gr_main	: scene::EntityGroup,
	gr_cape	: scene::EntityGroup,
	gr_hair	: scene::EntityGroup,
	gr_other: scene::EntityGroup,
	details	: scene::EntityGroup,
	skel	: @engine::space::Armature,
	cam		: @scene::Camera,
	control	: CamControl,
	lights	: ~[@scene::Light],
	envir	: Envir,
	technique	: @engine::draw::Technique,
	rast_solid	: engine::rast::State,
	rast_cloak	: engine::rast::State,
	rast_alpha	: engine::rast::State,
	depth	: render::depth::Data,
	lbuf	: Option<render::lbuf::Context>,
	lvolume	: render::lbuf::LightVolume,
	shadow	: render::shadow::Data,
	start	: float,
	hud_screen	: hud::Screen,
	hud_context	: hud::Context,
	hud_debug	: @engine::shade::Program,
	edit_label	: @hud::EditLabel,
	mouse_point	: (int,int),
	input_queue	: ~str,
	hud_active	: ActiveHud,
}


pub impl Scene	{
	fn loose_focus( &mut self )	{
		self.edit_label.active = false;
		self.hud_active = AhInactive;
	}

	fn update( &mut self, dt : float, nx : float, ny : float, hit : bool, scroll : float, _lg : &engine::context::Log )-> bool	{
		if true	{
			let root = &self.hud_screen.root;
			let (mx,my) = root.min_size;
			let x = ((0f+nx) * (mx as float)) as int;
			let y = ((1f-ny) * (my as float)) as int;
			self.mouse_point = (x,y);
			//let name = root.trace( x, y, lg );
			//io::println( ~"Click: " + name );
			if hit	{
				self.loose_focus();	
				do self.hud_screen.root.trace(x,y)	|frame,depth|	{
					if depth==0u && frame.name == ~"id.name.text"	{
						self.edit_label.active = true;
						self.hud_active = AhEditName;
					}
				};
			}
		}
		(self.edit_label as @engine::anim::Act).update();
		self.control.update( dt, nx, ny, hit, scroll );
		true
	}

	fn on_char( &mut self, key : char )	{
		str::push_char( &mut self.input_queue, key );
	}

	fn on_key_press( &mut self, key : int )	{
		match key	{
			257		=> self.loose_focus(),	//Esc,Enter
			259		=> str::push_char( &mut self.input_queue, key as char ),
			_	=> ()
		}
	}

	fn render( &mut self, el : &main::Elements, ct : &engine::context::Context, lg : &engine::context::Log  )	{
		// clear screen
		let fbo = ct.default_frame_buffer;
		let pmap = engine::call::PlaneMap::new_simple( ~"o_Color", engine::frame::TarEmpty );
		let out_solid = (fbo, copy pmap, self.rast_solid);
		let c0 =
			engine::call::ClearData{
				color	:Some( engine::rast::Color::new(0x8080FFFF) ),
				depth	:Some( 1f ),
				stencil	:Some( 0u ),
			}.gen_call( copy out_solid );
		if el.environment	{
			let vpi = self.cam.get_inverse_matrix();
			//self.cam.fill_data( &mut self.envir.data );
			self.envir.data.insert( ~"u_ViewProjInverse",
				engine::shade::UniMatrix(false,vpi) );
		}
		let c1 = if el.environment	{
			let e = &self.envir;
			engine::call::CallDraw(
				copy e.input, (fbo,copy pmap,copy e.rast),
				e.prog, copy e.data )
		}else	{
			engine::call::CallEmpty
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
			let par_ts = engine::shade::UniFloatVec( target_size );
			for [&self.gr_main, &self.gr_cape, &self.gr_hair, &self.gr_other].each() |group|	{
				for group.each() |ent|	{
					ent.update_world();
					let gd = &mut ent.data;
					self.shadow.light.fill_data( gd, 1f32, 200f32 );
					gd.insert( ~"t_Shadow", copy self.shadow.par_shadow );
					gd.insert( ~"u_TargetSize",	copy par_ts );
					self.cam.fill_data( gd );
					//self.skel.fill_data( gd );
				}	
			}
		}
		if el.shadow	{
			queue.push( copy self.shadow.call_clear );
			if el.character	{
				for [&self.gr_main,&self.gr_cape,&self.gr_hair].each() |group|	{
					for group.each() |ent|	{
						queue.push( self.shadow.tech_solid.process( ent, copy self.shadow.output, ct, lg ));
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
				for [&self.gr_main,&self.gr_cape,&self.gr_hair].each() |group|	{
					for group.each() |ent|	{
						queue.push( self.depth.tech_solid.process( ent, copy self.depth.output, ct, lg ));
						lbuf.fill_data( &mut ent.data );
					}
				}
				for self.gr_other.each() |ent|	{
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
				queue.push( tech.process( ent, copy out_solid, ct, lg ) );
			}
			for self.gr_cape.each() |ent|	{
				let out = (fbo, copy pmap, copy self.rast_cloak );
				queue.push( tech.process( ent, out, ct, lg ) );	
			}
			for self.gr_hair.each() |ent|	{
				let out = (fbo, copy pmap, copy self.rast_alpha );
				queue.push( tech.process( ent, out, ct, lg ) );
			}
		}
		if el.shadow	{
			for self.gr_other.each() |ent|	{
				queue.push( tech.process( ent, copy out_solid, ct, lg ) );
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
			rast.prime.poly_mode = engine::rast::map_polygon_fill(2);
			let mut data = engine::shade::make_data();
			let vc = vec4::new(1f32,0f32,0f32,1f32);
			data.insert( ~"u_Color", engine::shade::UniFloatVec(vc) );
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
				rast.prime.poly_mode = engine::rast::map_polygon_fill(2);
				let mut data = engine::shade::make_data();
				let vc = vec4::new(1f32,0f32,0f32,1f32);
				data.insert( ~"u_Color", engine::shade::UniFloatVec(vc) );
				self.hud_screen.root.draw_debug_all( &self.hud_context,
					self.hud_debug, &mut data, &rast )
			});
		}
		ct.flush(queue);
	}
}


pub fn make_scene( el : &main::Elements, ct : &engine::context::Context, aspect : float, lg : &engine::context::Log )-> Scene	{
	let vao = @mut ct.create_vertex_array();
	let mut scene = scene::load_scene( ~"data/claymore-2a", ct, Some(vao), aspect, lg );
	let detail_info = scene::load_config::<~[scene::EntityInfo]>( ~"data/details.json" );
	let mut details = scene.context.parse_group( detail_info, ct, Some(vao), lg );
	// techniques & rast states
	let tech = @engine::draw::load_technique( ~"data/code/tech/forward/spot-shadow" );
	let pmap = engine::call::PlaneMap::new_simple( ~"o_Color", engine::frame::TarEmpty );
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
	let arm = *scene.context.armatures.get(&~"Armature.002");
	let mut group = scene.entities.divide( &~"noTrasnform" );
	group.swap_entity( &~"boots", &mut details );
	let cape = group.divide( &~"polySurface172" );
	let hair = group.divide( &~"Hair_Geo2" );
	lg.add(fmt!( "Group size: %u", group.len() ));
	let envir = {
		let mesh = @engine::mesh::create_quad( ct );
		let mut data = engine::shade::make_data();
		let samp = engine::texture::Sampler::new(3u,1);
		let use_spherical = false;
		let prog = if use_spherical	{
			let tex = *scene.context.textures.get( &~"data/texture/Topanga_Forest_B_3k.hdr" );
			data.insert( ~"t_Environment",		engine::shade::UniTexture(0,tex,Some(samp)) );
			@engine::load::load_program( ct, ~"data/code-game/envir", lg )
		}else	{
			let tex = @engine::load::load_texture_2D( ct, &~"data/texture/bg2.jpg", true );
			data.insert( ~"t_Image",		engine::shade::UniTexture(0,tex,Some(samp)) );
			@engine::load::load_program( ct, ~"data/code-game/copy", lg )
		};
		let mut rast = copy ct.default_rast;
		//rast.set_depth( ~"<=", false );
		Envir{
			input:(vao,mesh,mesh.get_range()),
			prog:prog,
			data:data,
			rast:rast,
		}		
	};
	// load char HUD
	let fcon = @engine::font::Context::create();
	let mut hud_screen = hud::load_screen( ~"data/hud/char.json", ct, fcon, lg );
	hud_screen.root.update( lg );
	let hc = {
		let mut hud_rast = copy ct.default_rast;
		hud_rast.set_blend( ~"s+d", ~"Sa", ~"1-Sa" );
		let quad = @engine::mesh::create_quad(ct);
		hud::Context{
			input	: (vao,quad,quad.get_range()),
			output	: (ct.default_frame_buffer, copy pmap, hud_rast),
			size	: ct.screen_size,
		}
	};
	let edit_label = @hud::EditLabel::obtain( &mut hud_screen, ~"id.name.text" );
	let hdebug = @engine::load::load_program( ct, ~"data/code/hud/debug", lg );
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
	lg.add(fmt!( "Camera fov:%f, aspect:%f, range:%f-%f",
		cam.proj.vfov.degrees() as float,
		cam.proj.aspect as float,
		cam.proj.near as float,
		cam.proj.far as float ));
	lg.add( ~"\tWorld :" + cam.node.world_space().to_string() );
	let control = CamControl{
		node	: cam.node,
		origin	: vec3::new(0f32,0f32,75f32),
		speed_rot	: 1.5f32,
		speed_zoom	: 15f32,
		last_scroll	: None,
	};
	let mut lights : ~[@scene::Light] = ~[];
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
