extern mod glfw3;
extern mod glcore;
extern mod lmath;
extern mod stb_image;

extern mod engine;


struct Sample	{
	ct			: engine::context::Context,
	program		: engine::shade::Program,
	mut data	: engine::shade::DataMap,
	mesh		: engine::mesh::Mesh,
	texture		: @engine::texture::Texture,
}


fn init() -> Sample	{
	let ct = engine::context::create();
	assert ct.sync_back();
	// default VAO
	let vao = ct.create_vertex_array();
	ct.bind_vertex_array( vao );
	// load shaders
	let vert_shader = match io::read_whole_file_str(&path::Path(~"data/code/test.glslv"))	{
		Ok(text) => ct.create_shader( glcore::GL_VERTEX_SHADER, text ),
		Err(msg) => fail(msg)
	};
	let frag_shader = match io::read_whole_file_str(&path::Path(~"data/code/test.glslf"))	{
		Ok(text) => ct.create_shader( glcore::GL_FRAGMENT_SHADER, text ),
		Err(msg) => fail(msg)
	};
	let program = ct.create_program( ~[vert_shader,frag_shader] );
	// load buffers and mesh
	let vdata = ~[-1f32,-1f32,0f32,0f32,1f32,0f32,1f32,-1f32,0f32];
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
	});
	// load texture
	let mut tex : @engine::texture::Texture;
	match stb_image::image::load(~"data/GpuPro3.jpeg")	{
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
	let mx = lmath::matrix::Mat4::identity::<f32>();
	params.insert( ~"u_Color",		engine::shade::UniFloat(1f) );
	params.insert( ~"t_Image",		engine::shade::UniTexture(0u,tex) );
	params.insert( ~"u_World",		engine::shade::UniMatrix(false,mx) );
	params.insert( ~"u_ViewProj",	engine::shade::UniMatrix(false,mx) );
	// done
	ct.check(~"init");
	io::println( fmt!("init: program %u, buffer %u, texture %u",
		*program.handle as uint,*buf.handle as uint, *tex.handle as uint)
	);
	Sample { ct:ct, program:program, data:params, mesh:mesh, texture:tex }
}


fn render( s : &Sample ) ->bool	{
	glcore::glClearColor( 0.5f32, 0.5f32, 1.0f32, 1.0f32 );
	glcore::glClearDepth( 1.0f64 );
	glcore::glClear( glcore::GL_COLOR_BUFFER_BIT | glcore::GL_DEPTH_BUFFER_BIT );
	
	//FIXME: no copy (each_const required)
	s.ct.draw_mesh( &s.mesh, &s.program, &copy s.data );
	
	s.ct.check(~"render");
	return true;
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
	
		let mut window = glfw3::create_window( 800, 600, glfw3::WINDOWED, "Claymore" );
		if (ptr::is_null(window.ptr))	{
			failGLFW("OpenWindow");
		}
	
		window.set_title(~"Claymore");
		window.make_context_current();
		
		let sample = init();
		
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
