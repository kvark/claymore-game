extern mod glfw3;
extern mod glcore;
extern mod lmath;
extern mod stb_image;

extern mod engine;


struct Camera	{
	node	: @engine::space::Node,
	proj	: lmath::matrix::mat4,
}

impl Camera	{
	pure fn gen_matrix()-> lmath::matrix::mat4	{
		self.proj * self.node.world_space().inverse().to_matrix()
	}
	pure fn gen_pos()-> lmath::vector::vec4	{
		let v = self.node.world_space().position;
		lmath::vector::Vec4::new( v.x, v.y, v.z, 0f32 )
	}
}

struct BattleScene	{
	cam		: Camera,
	land	: engine::draw::Entity,
}

struct Game	{
	context		: engine::context::Context,
	mut frames	: uint,
	technique	: engine::draw::Technique,
	battle		: BattleScene,
}


impl Game	{
	fn render() ->bool	{
		let mut queue : ~[engine::call::Call] = ~[];
		// clear screen
		queue.push( self.technique.gen_clear(
			engine::call::ClearData{
				color	:Some( engine::rast::make_color(0x8080FFFF) ),
				depth	:Some( 1f ),
				stencil	:Some( 0u ),
			})
		);
		{// update matrices
			let view_proj	= self.battle.cam.gen_matrix();
			let cam_pos		= self.battle.cam.gen_pos();
			let light_pos	= lmath::vector::Vec4::new( 4f32, 1f32, 6f32, 0f32 );
			for [&self.battle.land].each |ent|	{
				ent.set_data( ~"u_ViewProj", 	engine::shade::UniMatrix(false,view_proj) );
				ent.set_data( ~"u_CameraPos",	engine::shade::UniFloatVec(cam_pos) );
				ent.set_data( ~"u_LightPos",	engine::shade::UniFloatVec(light_pos) );
				let world = ent.node.world_space().to_matrix();
				ent.set_data( ~"u_World",		engine::shade::UniMatrix(false,world) );
			}
		}
		// draw land
		queue.push( self.technique.process( &self.battle.land, &self.context ) );
		// execute
		self.context.flush(queue);
		// done
		self.frames += 1;
		self.context.cleanup();
		self.context.check(~"render");
		true
	}
	fn debug_move( x : int, y : int )	{
		let mut s = self.battle.cam.node.space;
		s.position.x += (x as f32) * 0.1f32;
		s.position.y += (y as f32) * 0.1f32;
		self.battle.cam.node.set_space(&s);
	}
}


fn make_game( wid : uint, het : uint )-> Game	{
	let ct = engine::context::create( wid, het );
	assert ct.sync_back();
	// create camera
	let cam = 	{
		let aspect = (wid as float) / (het as float);
		let cam_space = engine::space::QuatSpace{
			position 	: lmath::vector::Vec3::new( 7.5f32, -6.5f32, 5f32 ),
			orientation	: lmath::quaternion::Quat::new( -1f32, 0f32, 0f32, 0f32 ),
			//orientation	: lmath::quaternion::Quat::new( 0.782f32, 0.482f32, 0.213f32, 0.334f32 ),
			//position 	: lmath::vector::Vec3::new( 0f32, 0f32, 5f32 ),
			//orientation	: lmath::quaternion::Quat::new( 1f32, 0f32, 0f32, 0f32 ),
			scale		: 1f32
		};
		let cam_node = @engine::space::Node{
			name	: ~"cam",
			space	: cam_space,
			parent	: None,
			actions	: ~[],
		};
		io::println( cam_node.world_space().inverse().to_string() );
		let projection = lmath::funs::projection::perspective::<f32>( 50f, aspect, 1f, 20f );
		Camera{
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
			&ct );
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
	// load land texture
	let tex = match stb_image::image::load(~"data/texture/diffuse.jpg")	{
		Some(image) => {
			let t = @ct.create_texture( glcore::GL_TEXTURE_2D, image.width, image.height, 1, 0 );
			ct.texture.bind( t );
			ct.texture.load_2D( t, 0, glcore::GL_RGBA as glcore::GLint,
				glcore::GL_RGBA, glcore::GL_UNSIGNED_BYTE, image.data );
			ct.texture.wrap( t, 0 );
			ct.texture.filter( t, 2u );
			t
		}
		None => { fail(~"Unable to load image"); }
	};
	battle_land.set_data( ~"t_Main",	engine::shade::UniTexture(0u,tex) );
	// create omni1 technique
	let tech = {
		let pmap = engine::call::create_plane_map( ~"o_Color", engine::frame::TarEmpty );
		let mut rast = engine::rast::create_rast(0,0);
		rast.depth.test = true;
		rast.prime.cull = true;
		let cache = @mut engine::draw::create_cache();
		engine::draw::load_technique( ~"data/code/tech/omni1",
			ct.default_frame_buffer, &pmap, &rast, cache)
	};
	// done
	ct.check(~"init");
	Game{ context:ct, frames:0u, technique:tech,
		battle:BattleScene{
			cam		: cam,
			land	: battle_land,
		}}
}


fn fail_GLFW( where: &static/str )	{
	let code = glfw3::get_error();
	io::println(~"GLFW error: " + glfw3::error_string(code));
	glfw3::terminate();
	fail( fmt!("glfw%s() failed\n",where) );
}


fn main()	{
	io::println("--- Claymore ---");
	do task::task().sched_mode(task::PlatformThread).spawn {
		if (glfw3::init()==0)	{
			fail_GLFW("Init");
		}

		//glfw3::window_hint( glfw3::OPENGL_DEBUG_CONTEXT, 1 );
		glfw3::window_hint( glfw3::WINDOW_RESIZABLE, 0 );
        glfw3::window_hint( glfw3::OPENGL_VERSION_MAJOR, 3 );
        glfw3::window_hint( glfw3::OPENGL_VERSION_MINOR, 2 );
		glfw3::window_hint( glfw3::OPENGL_PROFILE, glfw3::OPENGL_CORE_PROFILE );
        glfw3::window_hint( glfw3::OPENGL_FORWARD_COMPAT, 1 );
	
		let wid = 800u, het = 600u;
		let mut window = glfw3::create_window( wid, het, glfw3::WINDOWED, "Claymore" );
		if (ptr::is_null(window.ptr))	{
			fail_GLFW("OpenWindow");
		}
	
		window.make_context_current();
		let game = make_game( wid, het );
		
		loop	{
			glfw3::poll_events();
			let isClosed = window.get_param(glfw3::CLOSE_REQUESTED)!=0;
			if window.get_key(glfw3::KEY_ESC)!=0 || isClosed	{
				glfw3::destroy_window(&mut window);
				break;
			}
			// debug keys
			if window.get_key(glfw3::KEY_LEFT)!=0	{
				game.debug_move(-1,0);
			}
			if window.get_key(glfw3::KEY_RIGHT)!=0	{
				game.debug_move(1,0);
			}
			if window.get_key(glfw3::KEY_DOWN)!=0	{
				game.debug_move(0,-1);
			}
			if window.get_key(glfw3::KEY_UP)!=0	{
				game.debug_move(0,1);
			}
			// render
			if !game.render()	{
				break;
			}
			window.swap_buffers();
		}
	
		glfw3::terminate();
	}
}
