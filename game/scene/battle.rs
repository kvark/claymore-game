extern mod lmath;
extern mod engine;

use lmath::quat::*;
use lmath::vec::*;
use cgmath::projection;
use engine::anim::Player;
use engine::draw::Mod;
use engine::space::{Interpolate,Space};

use input;
use scene::grid;
use scene = scene::common;


pub struct Character	{
	entity		: engine::draw::Entity,
	skeleton	: @mut engine::space::Armature,
	record		: @engine::space::ArmatureRecord,
	priv start_time	: float,
}

pub impl Character	{
	fn update( &mut self )-> bool	{
		let time = engine::anim::get_time();
		let mut moment  = time - self.start_time;
		if moment>self.record.duration	{
			self.record = self.skeleton.find_record(~"Idle").expect(~"!");
			self.start_time = time;
			moment = 0f;
		}
		self.skeleton.set_record( self.record, moment );
		self.skeleton.fill_data( &mut self.entity.data );
		true
	}
}


pub struct View	{
	cam				: scene::Camera,
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
	land	: engine::draw::Entity,
	grid	: grid::Grid,
	hero	: Character,
}

pub impl Scene	{
	fn update( &mut self, input : &input::State, tb : &mut engine::texture::Binding )-> bool	{
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
		let (i,j,ok) = self.grid.update( tb, &self.view.cam, input.mouse.x, input.mouse.y );
		let mouse_hit = (input.mouse.buttons & 1) != 0;
		if mouse_hit && self.grid.get_rectangle().contains(i,j)	{
			let sp = &mut self.hero.entity.node.space;
			sp.position = self.grid.get_cell_center(i,j);
			sp.position.z = 1.3f32;
			sp.orientation = quat::new( 0.707f32, 0f32, 0f32, 0.707f32 );
		}
		self.hero.update() && self.view.update( cam_dir ) && ok
	}
	fn render( &mut self, ct : &mut engine::context::Context, tech : &engine::draw::Technique, output : engine::call::DrawOutput, lg : &engine::context::Log )	{
		{// update matrices
			let light_pos	= vec4::new( 4f32, 1f32, 6f32, 1f32 );
			for [ &mut self.land, &mut self.hero.entity ].each |ent|	{
				let d = &mut ent.data;
				self.view.cam.fill_data( d );
				d.insert( ~"u_LightPos",	engine::shade::UniFloatVec(light_pos) );
				let world = ent.node.world_space().to_matrix();
				d.insert( ~"u_World",		engine::shade::UniMatrix(false,world) );
			}
		}
		let c_land = tech.process( &self.land, copy output, ct, lg );
		let c_hero = tech.process( &self.hero.entity, copy output, ct, lg );
		let (fbo,pmap,_) = output;
		let (vao,_,_) = self.land.input;
		let c_grid = self.grid.call( fbo, copy pmap, vao );
		ct.flush( ~[c_land,c_hero,c_grid] );
	}
	 fn debug_move( &self, _rot : bool, _x : int, _y : int )	{
		//empty
	}
}


pub fn make_scene( ct : &mut engine::context::Context, aspect : float, lg : &engine::context::Log )-> Scene	{
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
			scene::Camera{
				node	: cam_node,
				proj	: projection::PerspectiveSym{
					vfov	: 45f32,
					aspect	: aspect as f32,
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
	let mat = @engine::draw::load_material(~"data/code/mat/phong");
	let vao = ct.create_vertex_array();
	// load battle landscape
	let mut battle_land = {
		let mesh = @engine::load::load_mesh( ~"data/mesh/battle-test.k3mesh", ct, lg );
		let node = @mut engine::space::Node{
			name	: ~"landscape",
			space	: engine::space::QuatSpace::identity(),
			parent	: None,
			actions	: ~[],
		};
		engine::draw::Entity{
			node	: node,
			input	: (vao, mesh, mesh.get_range()),
			data	: engine::shade::make_data(),
			modifier: @() as @engine::draw::Mod,
			material: mat,
		}
	};
	// load protagonist
	let hero =	{
		let mesh = @engine::load::load_mesh( ~"data/mesh/character.k3mesh", ct, lg );
		let arm_node = @mut engine::space::Node{
			name	: ~"armature",
			space	: engine::space::QuatSpace::identity(),
			parent	: None,
			actions	: ~[],
		};
		let skel = @mut engine::load::load_armature( ~"data/arm/character.k3arm", arm_node, lg );
		let node = @mut engine::space::Node{
			name	: ~"hero",
			space	: engine::space::QuatSpace::identity(),
			parent	: Some(arm_node),
			actions	: ~[],
		};
		let mut ent = engine::draw::Entity{
			node	: node,
			input	: (vao,mesh,mesh.get_range()),
			data	: engine::shade::make_data(),
			modifier: skel as @engine::draw::Mod,
			material: mat,
		};
		// load char texture
		let tex = engine::load::load_texture_2D( ct, &~"data/texture/diffuse.jpg", true );
		let s_opt = Some( engine::texture::Sampler::new(3u,1) );
		ent.data.insert( ~"t_Main", engine::shade::UniTexture(0u,tex,s_opt) );
		let utc = vec4::new(1f32,1f32,0f32,0f32);
		ent.data.insert( ~"u_Tex0Transform", engine::shade::UniFloatVec(utc) );
		// done
		Character{
			entity		: ent,
			skeleton	: skel,
			record		: skel.find_record(~"Idle").expect(~"Hero has to have Idle"),
			start_time	: engine::anim::get_time(),
		}
	};
	// load land texture
	let tex = engine::load::load_texture_2D( ct, &~"data/texture/SoilCracked0103_2_S.jpg", true );
	let s_opt = Some( engine::texture::Sampler::new(3u,1) );
	battle_land.data.insert( ~"t_Main", engine::shade::UniTexture(0u,tex,s_opt) );
	let utc = vec4::new(10f32,10f32,0f32,0f32);
	battle_land.data.insert( ~"u_Tex0Transform", engine::shade::UniFloatVec(utc) );
	// create grid
	let grid = grid::Grid::create( ct, 10u, lg );
	grid.init( &mut ct.texture );
	{	// move hero
		let mut sp = hero.entity.node.space;
		sp.position = grid.get_cell_center(7u,2u);
		sp.position.z = 1.3f32;
		sp.orientation = quat::new( 0.707f32, 0f32, 0f32, 0.707f32 );
	}
	// done
	Scene{
		view	: view,
		land	: battle_land,
		grid	: grid,
		hero	: hero,
	}
}
