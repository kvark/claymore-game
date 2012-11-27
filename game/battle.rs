extern mod lmath;
extern mod engine;


pub struct Character	{
	entity		: engine::draw::Entity,
	skeleton	: @engine::space::Armature,
	mut record	: @engine::space::ArmatureRecord,
	priv mut start_time	: float,
}

impl Character	{
	fn update()-> bool	{
		let time = engine::anim::get_time();
		let mut moment  = time - self.start_time;
		if moment>self.record.duration	{
			self.record = self.skeleton.find_record(~"Idle").expect(~"!");
			self.start_time = time;
			moment = 0f;
		}
		self.skeleton.set_record( self.record, moment );
		self.skeleton.fill_data( self.entity.mut_data() );
		true
	}
}


pub struct View	{
	cam				: main::Camera,
	trans_duration	: float,
	points			: ~[engine::space::QuatSpace],
	mut source		: Option<engine::space::QuatSpace>,
	mut destination	: uint,
	mut start_time	: float,
}

impl View	{
	pub fn update( dir : int )-> bool	{
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
				let dst = &self.points[ self.destination ];
				*self.cam.node.mut_space() = if moment >= 1f	{
						self.source = None;
						*dst
					}else	{
						source.interpolate( dst, moment )
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

impl Scene	{
	pub fn update( tb : &engine::texture::Binding, nx : float, ny : float, mouse_hit : bool, cam_dir : int )-> bool	{
		let (i,j,ok) = self.grid.update( tb, &self.view.cam, nx, ny );
		if mouse_hit && self.grid.get_rectangle().contains(i,j)	{
			let sp = self.hero.entity.node.mut_space();
			sp.position = self.grid.get_cell_center(i,j);
			sp.position.z = 1.3f32;
			sp.orientation = lmath::quaternion::Quat::new( 0.707f32, 0f32, 0f32, 0.707f32 );
		}
		self.hero.update() && self.view.update( cam_dir ) && ok
	}
	pub fn render( ct : &engine::context::Context, tech : &engine::draw::Technique )	{
		{// update matrices
			let view_proj	= self.view.cam.get_matrix();
			let cam_pos		= self.view.cam.get_pos_vec4();
			let light_pos	= lmath::vector::Vec4::new( 4f32, 1f32, 6f32, 1f32 );
			for [ &self.land, &self.hero.entity ].each |ent|	{
				let d = ent.mut_data();
				d.insert( ~"u_ViewProj", 	engine::shade::UniMatrix(false,view_proj) );
				d.insert( ~"u_CameraPos",	engine::shade::UniFloatVec(cam_pos) );
				d.insert( ~"u_LightPos",	engine::shade::UniFloatVec(light_pos) );
				let world = ent.node.world_space().to_matrix();
				d.insert( ~"u_World",		engine::shade::UniMatrix(false,world) );
			}
		}
		let c_land = tech.process( &self.land, ct );
		let c_hero = tech.process( &self.hero.entity, ct );
		let c_grid = self.grid.call( tech.fbo, copy tech.pmap, self.land.vao );
		ct.flush( ~[c_land,c_hero,c_grid] );
	}
	pub fn debug_move( _rot : bool, _x : int, _y : int )	{
		//empty
	}
}


pub fn make_battle( ct : &engine::context::Context, aspect : float )-> Scene	{
	// create view
	let view = 	{
		// create camera
		let cam =	{
			let cam_space = engine::space::QuatSpace{
				position 	: lmath::vector::Vec3::new( 10f32, -10f32, 5f32 ),
				orientation	: lmath::quaternion::Quat::new( 0.802f32, 0.447f32, 0.198f32, 0.343f32 ),
				scale		: 1f32
			};
			let cam_node = @engine::space::Node{
				name	: ~"cam",
				space	: cam_space,
				parent	: None,
				actions	: ~[],
			};
			let projection = lmath::funs::projection::perspective::<f32>( 45f, aspect, 1f, 25f );
			main::Camera{
				node:cam_node,
				proj:projection,
			}
		};
		let points = do vec::from_fn(4) |i|	{
			let angle = (i as f32) * 0.25f32 * f32::consts::pi;
			let q = lmath::quaternion::Quat::new( f32::cos(angle), 0f32, 0f32, f32::sin(angle) );
			engine::space::QuatSpace{
				position	: q.mul_v( &cam.node.space.position ),
				orientation	: q.mul_q( &cam.node.space.orientation ),
				scale		: cam.node.space.scale,
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
	let vao = @ct.create_vertex_array();
	// load battle landscape
	let battle_land = {
		let mesh = @engine::load::read_mesh(
			&engine::load::create_reader(~"data/battle-test.k3mesh"),
			ct );
		let node = @engine::space::Node{
			name	: ~"landscape",
			space	: engine::space::identity(),
			parent	: None,
			actions	: ~[],
		};
		engine::draw::Entity{
			node	: node,
			data	: engine::shade::create_data(),
			vao		: vao,
			mesh	: mesh,
			range	: mesh.get_range(),
			modifier: @() as @engine::draw::Mod,
			material: mat,
		}
	};
	// load protagonist
	let hero =	{
		let mesh = @engine::load::read_mesh(
			&engine::load::create_reader(~"data/character.k3mesh"),
			ct );
		let node = @engine::space::Node{
			name	: ~"hero",
			space	: engine::space::identity(),
			parent	: None,
			actions	: ~[],
		};
		let skel = @engine::load::read_armature(
			&engine::load::create_reader(~"data/character.k3arm"),
			false );
		let ent = engine::draw::Entity{
			node	: node,
			data	: engine::shade::create_data(),
			vao		: vao,
			mesh	: mesh,
			range	: mesh.get_range(),
			modifier: skel as @engine::draw::Mod,
			material: mat,
		};
		// load char texture
		let tex = @engine::load::load_texture_2D( ct, ~"data/texture/diffuse.jpg", 1, 3u );
		ent.mut_data().insert( ~"t_Main", engine::shade::UniTexture(0u,tex) );
		let utc = lmath::vector::Vec4::new(1f32,1f32,0f32,0f32);
		ent.mut_data().insert( ~"u_TexTransform", engine::shade::UniFloatVec(utc) );
		// done
		Character{
			entity		: ent,
			skeleton	: skel,
			record		: skel.find_record(~"Idle").expect(~"Hero has to have Idle"),
			start_time	: engine::anim::get_time(),
		}
	};
	// load land texture
	let tex = @engine::load::load_texture_2D( ct, ~"data/texture/SoilCracked0103_2_S.jpg", 1, 3u );
	battle_land.mut_data().insert( ~"t_Main", engine::shade::UniTexture(0u,tex) );
	let utc = lmath::vector::Vec4::new(10f32,10f32,0f32,0f32);
	battle_land.mut_data().insert( ~"u_TexTransform", engine::shade::UniFloatVec(utc) );
	// create grid
	let grid = grid::make_grid( ct, 10u );
	grid.init( &ct.texture );
	{	// move hero
		let sp = hero.entity.node.mut_space();
		sp.position = grid.get_cell_center(7u,2u);
		sp.position.z = 1.3f32;
		sp.orientation = lmath::quaternion::Quat::new( 0.707f32, 0f32, 0f32, 0.707f32 );
	}
	// done
	Scene{
		view	: view,
		land	: battle_land,
		grid	: grid,
		hero	: hero,
	}
}