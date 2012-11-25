extern mod lmath;
extern mod engine;


pub struct Character	{
	entity		: engine::draw::Entity,
	skeleton	: @engine::space::Armature,
	mut record	: @engine::space::ArmatureRecord,
	priv mut start_time	: float,
}

impl Character	{
	fn update()	{
		let time = engine::anim::get_time();
		let mut moment  = time - self.start_time;
		if moment>self.record.duration	{
			self.record = self.skeleton.find_record(~"Idle").expect(~"!");
			self.start_time = time;
			moment = 0f;
		}
		self.skeleton.set_record( self.record, moment );
		let mut d2 = engine::shade::create_data();
		self.skeleton.fill_data( &mut d2 );
		for d2.each() |name,val|	{
			self.entity.set_data( copy *name, copy *val );
		}
	}
}


pub struct Scene	{
	cam		: main::Camera,
	land	: engine::draw::Entity,
	grid	: grid::Grid,
	hero	: Character,
}

impl Scene	{
	pub fn update( tb : &engine::texture::Binding, nx : float, ny : float )-> bool	{
		self.hero.update();
		self.grid.update( tb, &self.cam, nx, ny )
	}
	pub fn render( ct : &engine::context::Context, tech : &engine::draw::Technique )	{
		{// update matrices
			let view_proj	= self.cam.get_matrix();
			let cam_pos		= self.cam.get_pos_vec4();
			let light_pos	= lmath::vector::Vec4::new( 4f32, 1f32, 6f32, 1f32 );
			for [ &self.land, &self.hero.entity ].each |ent|	{
				ent.set_data( ~"u_ViewProj", 	engine::shade::UniMatrix(false,view_proj) );
				ent.set_data( ~"u_CameraPos",	engine::shade::UniFloatVec(cam_pos) );
				ent.set_data( ~"u_LightPos",	engine::shade::UniFloatVec(light_pos) );
				let world = ent.node.world_space().to_matrix();
				ent.set_data( ~"u_World",		engine::shade::UniMatrix(false,world) );
			}
		}
		let c_land = tech.process( &self.land, ct );
		let c_hero = tech.process( &self.hero.entity, ct );
		let c_grid = self.grid.call( tech.fbo, copy tech.pmap, self.land.vao );
		ct.flush( ~[c_land,c_hero,c_grid] );
	}
	pub fn debug_move( rot : bool, x : int, y : int )	{
		let mut s = self.cam.node.space;
		if rot	{
			const mul : f32 = 0.02f32;
			let a1 = (x as f32)*mul, a2 = (y as f32)*mul;
			let c1 = f32::cos(a1), c2 = f32::cos(a2);
			let s1 = f32::sin(a1), s2 = f32::sin(a2);
			let q1 = lmath::quaternion::Quat::new( c1, 0f32, -s1, 0f32 );
			let q2 = lmath::quaternion::Quat::new( c2, s2, 0f32, 0f32 );
			let q3 = s.orientation.mul_q( &q1.mul_q( &q2 ) );
			s.orientation = q3.mul_t( 1f32 / q3.length() );
		}else	{
			s.position.x += (x as f32) * 0.1f32;
			s.position.y += (y as f32) * 0.1f32;
		}
		self.cam.node.set_space(&s);
	}
}


pub fn make_battle( ct : &engine::context::Context, aspect : float )-> Scene	{
	// create camera
	let cam = 	{
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
		ent.set_data( ~"t_Main", engine::shade::UniTexture(0u,tex) );
		let utc = lmath::vector::Vec4::new(1f32,1f32,0f32,0f32);
		ent.set_data( ~"u_TexTransform", engine::shade::UniFloatVec(utc) );
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
	battle_land.set_data( ~"t_Main", engine::shade::UniTexture(0u,tex) );
	let utc = lmath::vector::Vec4::new(10f32,10f32,0f32,0f32);
	battle_land.set_data( ~"u_TexTransform", engine::shade::UniFloatVec(utc) );
	// create grid
	let grid = grid::make_grid( ct, 10u );
	grid.init( &ct.texture );
	{	// move hero
		let mut sp = engine::space::identity();
		sp.position = grid.get_cell_center(7u,2u);
		sp.position.z = 1.3f32;
		sp.orientation = lmath::quaternion::Quat::new( 0.707f32, 0f32, 0f32, 0.707f32 );
		hero.entity.node.set_space(&sp);
	}
	// done
	Scene{
		cam		: cam,
		land	: battle_land,
		grid	: grid,
		hero	: hero,
	}
}