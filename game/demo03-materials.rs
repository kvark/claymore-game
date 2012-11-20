extern mod glfw3;
extern mod glcore;
extern mod lmath;
extern mod stb_image;

extern mod engine;


struct Sample	{
	context		: engine::context::Context,
	mut data	: engine::shade::DataMap,
	entity		: engine::draw::Entity,
	technique	: engine::draw::Technique,
	texture		: @engine::texture::Texture,
	mut frames	: uint,
}


fn init( wid : uint, het : uint ) -> Sample	{
	let ct = engine::context::create( wid, het );
	assert ct.sync_back();
	// create entity
	let entity = {
		let mesh = @engine::load::read_mesh( &engine::load::create_reader(~"data/demo03.k3mesh"), &ct );
		let material = @engine::draw::load_material(~"data/code/mat/phong_tangent");
		let node = @engine::space::Node{ name:~"b1", space:engine::space::identity(), parent:None };
		engine::draw::Entity{
			node	: node,
			vao		: @ct.create_vertex_array(),
			mesh	: mesh,
			range	: mesh.get_range(),
			mods	: ~[],
			material: material,
		}
	};
	// create technique
	let tech = {
		let pmap = engine::call::create_plane_map( ~"o_Color", engine::frame::TarEmpty );
		let mut rast = engine::rast::create_rast(0,0);
		rast.depth.test = true;
		rast.prime.cull = true;
		let cache = @mut engine::draw::create_cache();
		engine::draw::load_technique( ~"data/code/tech/omni1", ct.default_frame_buffer, &pmap, &rast, cache)
	};
	// load texture
	let t_diffuse =
		match stb_image::image::load(~"data/texture/diffuse.jpg")	{
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
	let t_normal =
		match stb_image::image::load(~"data/texture/normal.jpg")	{
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
	// init parameters
	let mut params = engine::shade::create_data();
	{
		// compute matrices
		let aspect = (wid as float) / (het as float);
		let cam_space = engine::space::QuatSpace{
			position 	: lmath::vector::Vec3::new( 0f32, 0f32, 5f32 ),
			orientation	: lmath::quaternion::Quat::identity::<f32>(),
			scale		: 1f32
		};
		let cam_node = engine::space::Node{ name:~"cam", space:cam_space, parent:None };
		let projection = lmath::funs::projection::perspective::<f32>( 45f, aspect, 1f, 10f );
		let mx = lmath::matrix::Mat4::identity::<f32>();
		let cam_world = cam_node.world_space();
		let mview = cam_world.inverse().to_matrix();
		let mvp = projection.mul_m( &mview );
		let cam_pos = cam_world.position;
		let u_cam_pos	= lmath::vector::Vec4::new( cam_pos.x, cam_pos.y, cam_pos.z, 0f32 );
		let u_light_pos	= lmath::vector::Vec4::new( 3f32, 3f32, 3f32, 0f32 );
		// push to params
		params.insert( ~"u_Color",		engine::shade::UniFloat(1f) );
		params.insert( ~"t_Main",		engine::shade::UniTexture(0u,t_diffuse) );
		params.insert( ~"t_Normal",		engine::shade::UniTexture(0u,t_normal) );
		params.insert( ~"u_World",		engine::shade::UniMatrix(false,mx) );
		params.insert( ~"u_WorldQuat",	engine::shade::UniQuat( lmath::quaternion::Quat::identity::<f32>() ));
		params.insert( ~"u_ViewProj",	engine::shade::UniMatrix(false,mvp) );
		params.insert( ~"u_CameraPos",	engine::shade::UniFloatVec(u_cam_pos) );
		params.insert( ~"u_LightPos",	engine::shade::UniFloatVec(u_light_pos) );
	}
	// done
	ct.check(~"init");
	io::println( fmt!("init: mesh %s, texture %u",
		entity.mesh.name, *t_diffuse.handle as uint)
	);
	Sample { context:ct, data:params, entity:entity, technique:tech, texture:t_diffuse, frames:0 }
}


fn render( s : &Sample ) ->bool	{
	if true {	// compute new rotation matrix
		let angle = (s.frames as f32) * 0.01f32;
		let sn = f32::sin(angle), cn = f32::cos(angle);
		let qbase = lmath::quaternion::Quat::<f32>{ w:cn, x:0f32, y:sn, z:0f32 };
		let model_space = engine::space::QuatSpace{
			position 	: lmath::vector::Vec3::<f32>{ x:0f32, y:0f32, z:0f32 },
			orientation	: qbase,
			scale		: 1.5f32
		};
		let mx = model_space.to_matrix();
		s.data.insert( ~"u_World",		engine::shade::UniMatrix(false,mx) );
		s.data.insert( ~"u_WorldQuat",	engine::shade::UniQuat( model_space.orientation ) );
	}
	let cdata = engine::call::ClearData{
		color	:Some(engine::rast::Color{ r:0.5f32, g:0.5f32, b:1.0f32, a:1.0f32 }),
		depth	:Some( 1f ),
		stencil	:None
	};
	let t = &s.technique;
	let c0 = engine::call::CallClear( t.fbo, copy t.pmap,
		cdata, t.rast.scissor, t.rast.mask );
	let c1 = t.process( &s.entity, &s.context, copy s.data );
	s.context.flush(~[c0,c1]);
	
	s.frames += 1;
	s.context.cleanup();
	s.context.check(~"render");
	true
}


fn failGLFW( where: &static/str )	{
	let code = glfw3::get_error();
	io::println(~"GLFW error: " + glfw3::error_string(code));
	glfw3::terminate();
	fail( fmt!("glfw%s() failed\n",where) );
}


fn main()	{
	io::println("--- Claymore demo 03: materials ---");
	do task::task().sched_mode(task::PlatformThread).spawn {
		if (glfw3::init()==0)	{
			failGLFW("Init");
		}

        glfw3::window_hint( glfw3::OPENGL_VERSION_MAJOR, 3 );
        glfw3::window_hint( glfw3::OPENGL_VERSION_MINOR, 2 );
		glfw3::window_hint( glfw3::OPENGL_PROFILE, glfw3::OPENGL_CORE_PROFILE );
        glfw3::window_hint( glfw3::OPENGL_FORWARD_COMPAT, 1 );
	
		let wid = 800u, het = 600u;
		let mut window = glfw3::create_window( wid, het, glfw3::WINDOWED, "Claymore" );
		if (ptr::is_null(window.ptr))	{
			failGLFW("OpenWindow");
		}
	
		window.make_context_current();
		let sample = init( wid, het );
		
		loop	{
			glfw3::poll_events();
			let isClosed = window.get_param(glfw3::CLOSE_REQUESTED)!=0;
			if (window.get_key(glfw3::KEY_ESC)!=0 || isClosed)	{
				glfw3::destroy_window(&mut window);
				break;
			}
			if (!render(&sample))	{
				break;
			}
			window.swap_buffers();
		}
	
		glfw3::terminate();
	}
}
