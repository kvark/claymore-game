extern mod glfw3;
extern mod glcore;
extern mod lmath;
extern mod stb_image;

extern mod engine;


struct Sample	{
	ct			: engine::context::Context,
	program		: @engine::shade::Program,
	mut data	: engine::shade::DataMap,
	mesh		: @engine::mesh::Mesh,
	va			: @engine::buf::VertexArray,
	texture		: @engine::texture::Texture,
	fbo			: @engine::frame::Buffer,
	mut frames	: uint,
}


fn init( wid : uint, het : uint ) -> Sample	{
	let ct = engine::context::create( wid, het );
	assert ct.sync_back();
	// load shaders
	let vert_shader = match io::read_whole_file_str(&path::Path(~"data/code/test.glslv"))	{
		Ok(text) => ct.create_shader( glcore::GL_VERTEX_SHADER, text ),
		Err(msg) => fail(msg)
	};
	let frag_shader = match io::read_whole_file_str(&path::Path(~"data/code/test.glslf"))	{
		Ok(text) => ct.create_shader( glcore::GL_FRAGMENT_SHADER, text ),
		Err(msg) => fail(msg)
	};
	let program = @ct.create_program( ~[vert_shader,frag_shader] );
	// load buffers and mesh
	let va = @ct.create_vertex_array();
	let mesh = @engine::load::read_mesh( &engine::load::create_reader(~"data/jazz_dancing.k3mesh"), &ct );
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
		let u_cam_pos = lmath::vector::Vec4::new( cam_pos.x, cam_pos.y, cam_pos.z, 0f32 );
		// push to params
		params.insert( ~"u_Color",		engine::shade::UniFloat(1f) );
		params.insert( ~"t_Image",		engine::shade::UniTexture(0u,tex) );
		params.insert( ~"u_World",		engine::shade::UniMatrix(false,mx) );
		params.insert( ~"u_ViewProj",	engine::shade::UniMatrix(false,mvp) );
		params.insert( ~"u_CamPos",		engine::shade::UniFloatVec(u_cam_pos) );
	}
	let fbo = @ct.create_frame_buffer_main();
	// done
	ct.check(~"init");
	io::println( fmt!("init: program %u, mesh %s, texture %u",
		*program.handle as uint, mesh.name, *tex.handle as uint)
	);
	Sample { ct:ct, program:program, data:params, mesh:mesh, va:va, texture:tex, frames:0, fbo:fbo }
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
		s.data.insert( ~"u_World", engine::shade::UniMatrix(false,mx) );
	}

	if true	{
		let mut pmap = send_map::linear::LinearMap::<~str,engine::frame::Target>();
		pmap.insert( ~"", engine::frame::TarEmpty );
		pmap.insert( ~"o_Color", engine::frame::TarEmpty );
		let cdata = engine::call::ClearData{
			color	:Some(engine::rast::Color{ r:0.5f32, g:0.5f32, b:1.0f32, a:1.0f32 }),
			depth	:Some( 1f32 ),
			stencil	:None
		};
		let mut rast = engine::rast::create_rast(0,0);
		rast.depth.test = true;
		rast.prime.cull = true;
		let c0 = engine::call::CallClear( s.fbo, copy pmap, cdata, rast.scissor, rast.mask );
		let c1 = engine::call::CallDraw( s.fbo, copy pmap, s.va, s.mesh, s.mesh.get_range(), s.program, copy s.data, rast );

//		glcore::glClearColor( 0.5f32, 0.5f32, 1.0f32, 1.0f32 );
//		glcore::glClearDepth( 1.0f64 );
//		glcore::glClear( glcore::GL_COLOR_BUFFER_BIT | glcore::GL_DEPTH_BUFFER_BIT );

		s.ct.flush(~[c0,c1]);
	}else	{
		glcore::glClearColor( 0.5f32, 0.5f32, 1.0f32, 1.0f32 );
		glcore::glClearDepth( 1.0f64 );
		glcore::glClear( glcore::GL_COLOR_BUFFER_BIT | glcore::GL_DEPTH_BUFFER_BIT );
		glcore::glEnable( glcore::GL_DEPTH_TEST );
		glcore::glEnable( glcore::GL_CULL_FACE );

		//FIXME: no copy (each_const required)
		s.ct.draw_mesh( s.mesh, &s.mesh.get_range(), s.va, s.program, &copy s.data );
	}
	
	s.frames += 1;
	s.ct.check(~"render");
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
