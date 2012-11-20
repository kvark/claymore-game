extern mod glfw3;
extern mod glcore;
extern mod lmath;
extern mod stb_image;

extern mod engine;


struct Sample	{
	context		: engine::context::Context,
	program		: @engine::shade::Program,
	mut data	: engine::shade::DataMap,
	entity		: engine::draw::Entity,
	technique	: engine::draw::Technique,
	texture		: @engine::texture::Texture,
	mut frames	: uint,
}


fn init( wid : uint, het : uint ) -> Sample	{
	let ct = engine::context::create( wid, het );
	assert ct.sync_back();
	// load shaders
	let vert_shader = match io::read_whole_file_str(&path::Path(~"data/code/test.glslv"))	{
		Ok(text) => ct.create_shader( 'v', text ),
		Err(msg) => fail(msg)
	};
	let frag_shader = match io::read_whole_file_str(&path::Path(~"data/code/test.glslf"))	{
		Ok(text) => ct.create_shader( 'f', text ),
		Err(msg) => fail(msg)
	};
	let program = @ct.create_program( ~[vert_shader,frag_shader] );
	// create entity
	let entity = {
		let mesh = @engine::load::read_mesh( &engine::load::create_reader(~"data/jazz_dancing.k3mesh"), &ct );
		let material = @engine::draw::load_material(~"data/code/mat/phong_tangent");
		let node = @engine::space::Node{ name:~"girl", space:engine::space::identity(), parent:None };
		engine::draw::Entity{
			node	: node,
			vao		: @ct.create_vertex_array(),
			mesh	: mesh,
			range	: mesh.get_range(),
			mods	: ~[],
			material: material,
		}
	};
	/*let vdata = ~[-1f32,-1f32,0f32,0f32,1f32,0f32,1f32,-1f32,0f32];
	let buf = @ct.create_buffer_loaded( vdata );
	let mut mesh = ct.create_mesh( ~"dummy", ~"3", 3, 0 );
	mesh.attribs.insert( ~"a_Position", engine::mesh::Attribute{
		kind			: glcore::GL_FLOAT,
		count			: 3u,
		normalized		: false,
		interpolated	: true,
		buffer			: buf,
		stride			: 3u * sys::size_of::<f32>(),
		offset			: 0,
	});*/
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
	let mut tex : @engine::texture::Texture;
	match stb_image::image::load(~"data/texture/SexyFem_Texture.tga")	{
		Some(image) => {
			tex = @ct.create_texture( glcore::GL_TEXTURE_2D, image.width, image.height, 1, 0 );
			ct.texture.bind( tex );
			ct.texture.load_2D( tex, 0, glcore::GL_RGBA as glcore::GLint,
				glcore::GL_RGBA, glcore::GL_UNSIGNED_BYTE, image.data );
			ct.texture.wrap( tex, 0 );
			ct.texture.filter( tex, 2u );
		}
		None => { fail(~"Unable to load image"); }
	}
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
		params.insert( ~"t_Main",		engine::shade::UniTexture(0u,tex) );
		params.insert( ~"u_World",		engine::shade::UniMatrix(false,mx) );
		params.insert( ~"u_WorldQuat",	engine::shade::UniQuat( lmath::quaternion::Quat::identity::<f32>() ));
		params.insert( ~"u_ViewProj",	engine::shade::UniMatrix(false,mvp) );
		params.insert( ~"u_CameraPos",	engine::shade::UniFloatVec(u_cam_pos) );
		params.insert( ~"u_LightPos",	engine::shade::UniFloatVec(u_light_pos) );
	}
	// done
	ct.check(~"init");
	io::println( fmt!("init: program %u, mesh %s, texture %u",
		*program.handle as uint, entity.mesh.name, *tex.handle as uint)
	);
	Sample { context:ct, program:program, data:params, entity:entity, technique:tech, texture:tex, frames:0 }
}


fn render( s : &Sample ) ->bool	{
	if true {	// compute new rotation matrix
		let angle = (s.frames as f32) * 0.02f32;
		let sn = f32::sin(angle), cn = f32::cos(angle);
		let half_90 = f32::cos(3.141592f32 * 0.25f32);
		let qbase = lmath::quaternion::Quat::<f32>{ w:half_90, x:half_90, y:0f32, z:0f32 };
		let q = lmath::quaternion::Quat::<f32>{ w:cn, x:0f32, y:sn, z:0f32 };
		let model_space = engine::space::QuatSpace{
			position 	: lmath::vector::Vec3::<f32>{ x:0f32, y:-1.8f32, z:0f32 },
			orientation	: qbase.mul_q(&q),
			scale		: 2f32
		};
		let mx = model_space.to_matrix();
		s.data.insert( ~"u_World",		engine::shade::UniMatrix(false,mx) );
		s.data.insert( ~"u_WorldQuat",	engine::shade::UniQuat( model_space.orientation ) );
	}

	if true	{
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
	}else
	if true	{
		let cdata = engine::call::ClearData{
			color	:Some(engine::rast::Color{ r:0.5f32, g:0.5f32, b:1.0f32, a:1.0f32 }),
			depth	:Some( 1f ),
			stencil	:None
		};
		let t = &s.technique;
		let c0 = engine::call::CallClear( t.fbo, copy t.pmap,
			cdata, t.rast.scissor, t.rast.mask );
		let c1 = engine::call::CallDraw( t.fbo, copy t.pmap,
				s.entity.vao, s.entity.mesh, s.entity.range, s.program, copy s.data, t.rast );
		s.context.flush(~[c0,c1]);
	}else	{
		glcore::glClearColor( 0.5f32, 0.5f32, 1.0f32, 1.0f32 );
		glcore::glClearDepth( 1.0f64 );
		glcore::glClear( glcore::GL_COLOR_BUFFER_BIT | glcore::GL_DEPTH_BUFFER_BIT );
		glcore::glEnable( glcore::GL_DEPTH_TEST );
		glcore::glEnable( glcore::GL_CULL_FACE );

		//FIXME: no copy (each_const required)
		s.context.draw_mesh( s.entity.mesh, &s.entity.range, s.entity.vao, s.program, &copy s.data );
	}
	
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
	io::println("--- Claymore ---");
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
