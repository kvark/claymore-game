extern mod lmath;
extern mod engine;
extern mod gen_hud;
extern mod gen_scene;

use lmath::quat::*;
use lmath::vec::*;
use cgmath::projection;
use engine::anim::Player;
use engine::{gr_low,gr_mid};
use engine::gr_mid::draw::Mod;
use engine::space::{Interpolate,Space};

use input;
use hud_new;
use scene;


pub struct Character	{
	entity		: engine::object::Entity,
	skeleton	: @mut engine::space::Armature,
	record		: @engine::space::ArmatureRecord,
	priv start_time	: float,
}

pub impl Character	{
	fn update( &mut self )-> bool	{
		let time = engine::anim::get_time();
		let mut moment  = time - self.start_time;
		if moment>self.record.duration	{
			self.record = self.skeleton.find_record(~"ArmatureAction").expect(~"character Idle not found");
			self.start_time = time;
			moment = 0f;
		}
		self.skeleton.set_record( self.record, moment );
		self.skeleton.fill_data( &mut self.entity.data );
		true
	}
}


pub struct View	{
	cam				: scene::common::Camera,
	trans_duration	: float,
	points			: ~[engine::space::QuatSpace],
	source			: Option<engine::space::QuatSpace>,
	destination		: uint,
	start_time		: float,
}

pub impl View	{
	fn update( &mut self, dir : int )-> bool	{
		let time = engine::anim::get_time();
		if dir != 0 && time > self.start_time + 0.5f	{
			let l = self.points.len() as int;
			self.destination = ((self.destination as int) + dir + l) % l as uint;
			self.source = Some( self.cam.node.space );
			self.start_time = time;
		}
		match copy self.source	{
			Some(source)	=>	{
				let moment = (time - self.start_time) / self.trans_duration;
				let dst = self.points[ self.destination ];
				self.cam.node.space = if moment >= 1f	{
						self.source = None;
						dst
					}else	{
						source.interpolate( &dst, moment )
					};
			},
			None	=> ()
		}
		true
	}
}


pub struct Scene	{
	view	: View,
	land	: engine::object::Entity,
	grid	: scene::grid::Grid,
	hero	: Character,
	cache	: gr_mid::draw::Cache,
	hud		: gen_hud::common::Screen,
}

pub impl Scene	{
	fn update( &mut self, input : &input::State, tb : &mut gr_low::texture::Binding, aspect : f32 )-> bool	{
		/*let (_,scroll_y) = window.get_scroll_offset(); //FIXME
		let scroll_y = 0;
		let shift = window.get_key(glfw::KEY_LEFT_SHIFT)!=0;
		// debug keys
		if window.get_key(glfw::KEY_LEFT)!=0	{
			game.debug_move(shift,-1,0);
		}
		if window.get_key(glfw::KEY_RIGHT)!=0	{
			game.debug_move(shift,1,0);
		}
		if window.get_key(glfw::KEY_DOWN)!=0	{
			game.debug_move(shift,0,-1);
		}
		if window.get_key(glfw::KEY_UP)!=0	{
			game.debug_move(shift,0,1);
		}
		// camera rotation
		let _cam_dir = (window.get_key(glfw::KEY_E) - window.get_key(glfw::KEY_Q)) as int;
		*/
		let cam_dir = 0;
		let (i,j,ok) = self.grid.update( tb, &self.view.cam, aspect, input.mouse.x, input.mouse.y );
		let mouse_hit = (input.mouse.buttons & 1) != 0;
		if mouse_hit && self.grid.get_rectangle().contains(i,j)	{
			let sp = &mut self.hero.entity.node.space;
			sp.position = self.grid.get_cell_center(i,j);
			sp.position.z = 1.3f32;
			sp.orientation = quat::new( 0.707f32, 0f32, 0f32, 0.707f32 );
		}
		self.hero.update() && self.view.update( cam_dir ) && ok
	}
	fn render( &mut self, gc : &mut gr_low::context::Context, hc : &hud_new::Context,
			tech : &gr_mid::draw::Technique, output : gr_mid::call::Output, lg : &engine::journal::Log )	{
		let aspect = output.area.aspect();
		{// update matrices
			let light_pos	= vec4::new( 4f32, 1f32, 6f32, 1f32 );
			for [ &mut self.land, &mut self.hero.entity ].each |ent|	{
				let d = &mut ent.data;
				self.view.cam.fill_data( d, aspect );
				d.insert( ~"u_LightPos",	gr_low::shade::UniFloatVec(light_pos) );
				let world = ent.node.world_space().to_matrix();
				d.insert( ~"u_World",		gr_low::shade::UniMatrix(false,world) );
			}
		}
		// clear screen
		let cd = gr_mid::call::ClearData{
			color	:Some( gr_low::rast::Color::new(0x8080FFFF) ),
			depth	:Some( 1f ),
			stencil	:Some( 0u ),
		};
		let c0 = gr_mid::call::CallClear( copy output, cd, copy gc.default_rast.mask );
		// draw battle
		let mut rast = gc.default_rast;
		rast.set_depth( ~"<=", true );
		rast.prime.cull = true;
		let c_land = tech.process( &self.land, copy output, copy rast, &mut self.cache, gc, lg );
		let c_hero = tech.process( &self.hero.entity, copy output, copy rast, &mut self.cache, gc, lg );
		let c_grid = self.grid.call( output.fb, copy output.pmap, self.land.input.va );
		gc.flush( [c0,c_land,c_hero,c_grid] );
		// draw hud
		gc.flush( hc.draw_all( &self.hud, &output ) );
	}
	 fn debug_move( &self, _rot : bool, _x : int, _y : int )	{
		//empty
	}
}


pub fn make_scene( gc : &mut gr_low::context::Context, hc : &mut hud_new::Context, fcon : &gr_mid::font::Context, lg : &engine::journal::Log )-> Scene	{
	// create view
	let view = 	{
		// create camera
		let cam =	{
			let cam_space = engine::space::QuatSpace{
				position 	: vec3::new( 10f32, -10f32, 5f32 ),
				orientation	: quat::new( 0.802f32, 0.447f32, 0.198f32, 0.343f32 ),
				scale		: 1f32
			};
			let cam_node = @mut engine::space::Node{
				name	: ~"cam",
				space	: cam_space,
				parent	: None,
				actions	: ~[],
			};
			scene::common::Camera{
				node	: cam_node,
				proj	: projection::PerspectiveSym{
					vfov	: 45f32,
					aspect	: 1f32,
					near	: 1f32,
					far		: 25f32,
				},
				ear		: engine::audio::Listener{ volume:0f },
			}
		};
		let points = do vec::from_fn(4) |i|	{
			//FIXME: use new quat constructors
			let angle = (i as f32) * 0.25f32 * f32::consts::pi;
			let q = Quat::new( f32::cos(angle), 0f32, 0f32, f32::sin(angle) );
			let cs = cam.node.space;
			engine::space::QuatSpace{
				position	: q.mul_v( &cs.position ),
				orientation	: q.mul_q( &cs.orientation ),
				scale		: cs.scale,
			}
		};
		View{
			cam	: cam,
			trans_duration	: 2f,
			points			: points,
			source			: None,
			destination		: 0,
			start_time		: 0f,
		}
	};
	// load basic material & vao
	let mat = @gr_mid::draw::load_material(~"data/code/mat/phong");
	let vao = gc.create_vertex_array();
	// load battle landscape
	let iscene = gen_scene::battle::main::load();
	let scene = scene::load::parse( ~"data/scene/battle-test", &iscene, ~[], gc, Some(vao), lg );
	let mut battle_land = {
		let mesh = *scene.context.meshes.get( &~"Plane@all" );
		let node = @mut engine::space::Node{
			name	: ~"landscape",
			space	: engine::space::QuatSpace::identity(),
			parent	: None,
			actions	: ~[],
		};
		engine::object::Entity{
			node	: node,
			input	: gr_mid::call::Input::new( vao, mesh ),
			data	: gr_low::shade::make_data(),
			modifier: @() as @gr_mid::draw::Mod,
			material: mat,
		}
	};
	// load protagonist
	let hero =	{
		let mesh = *scene.context.meshes.get( &~"Cube@all" );
		let arm_node = @mut engine::space::Node{
			name	: ~"armature",
			space	: engine::space::QuatSpace::identity(),
			parent	: None,
			actions	: ~[],
		};
		let skel = *scene.context.armatures.get( &~"Armature" );
		skel.root = arm_node;
		let node = @mut engine::space::Node{
			name	: ~"hero",
			space	: engine::space::QuatSpace::identity(),
			parent	: Some(arm_node),
			actions	: ~[],
		};
		let mut ent = engine::object::Entity{
			node	: node,
			input	: gr_mid::call::Input::new( vao, mesh ),
			data	: gr_low::shade::make_data(),
			modifier: skel as @gr_mid::draw::Mod,
			material: mat,
		};
		// load char texture
		let tex = engine::load::load_texture_2D( gc, &~"data/texture/diffuse.jpg", true );
		let s_opt = Some( gr_low::texture::Sampler::new(3u,1) );
		ent.data.insert( ~"t_Main", gr_low::shade::UniTexture(0u,tex,s_opt) );
		let utc = vec4::new(1f32,1f32,0f32,0f32);
		ent.data.insert( ~"u_Tex0Transform", gr_low::shade::UniFloatVec(utc) );
		// done
		Character{
			entity		: ent,
			skeleton	: skel,
			record		: skel.find_record(~"ArmatureAction").expect(~"Hero has to have Idle"),
			start_time	: engine::anim::get_time(),
		}
	};
	// load land texture
	let tex = engine::load::load_texture_2D( gc, &~"data/texture/SoilCracked0103_2_S.jpg", true );
	let s_opt = Some( gr_low::texture::Sampler::new(3u,1) );
	battle_land.data.insert( ~"t_Main", gr_low::shade::UniTexture(0u,tex,s_opt) );
	let utc = vec4::new(10f32,10f32,0f32,0f32);
	battle_land.data.insert( ~"u_Tex0Transform", gr_low::shade::UniFloatVec(utc) );
	// create grid
	let grid = scene::grid::Grid::create( gc, 10u, lg );
	grid.init( &mut gc.texture );
	{	// move hero
		let mut sp = hero.entity.node.space;
		sp.position = grid.get_cell_center(7u,2u);
		sp.position.z = 1.3f32;
		sp.orientation = quat::new( 0.707f32, 0f32, 0f32, 0.707f32 );
	}
	let hud = gen_hud::battle::load();
	hc.preload( hud.children, gc, fcon, lg );
	// done
	Scene{
		view	: view,
		land	: battle_land,
		grid	: grid,
		hero	: hero,
		cache	: gr_mid::draw::make_cache(),
		hud		: hud,
	}
}
