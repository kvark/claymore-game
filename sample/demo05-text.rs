extern mod glfw3;
extern mod lmath;

extern mod engine;


struct Sample	{
	context		: engine::context::Context,
	font_lib	: engine::font::Context,
	texture		: @engine::texture::Texture,
	program		: @engine::shade::Program,
	vao			: @engine::buf::VertexArray,
	mesh		: @engine::mesh::Mesh,
	mut frames	: uint,
}


fn init( wid : uint, het : uint ) -> Sample	{
	let ct = engine::context::create( wid, het );
	assert ct.sync_back();
	// create text
	let fl = engine::font::create_context();
	let font = fl.load_font( ~"data/font/AnnabelScript.ttf", 0u, 30u, 30u );
	// done
	ct.check(~"init");
	Sample { context:ct, font_lib:fl,
		texture	:@font.bake( &ct, ~"Hello, world!", 200u, 50u ),
		program	:@engine::load::load_program( &ct, ~"data/code/text" ),
		vao		:@ct.create_vertex_array(),
		mesh	:@engine::mesh::create_quad( &ct ),
		frames	:0 }
}


fn render( s : &Sample ) ->bool	{
	let cdata = engine::call::ClearData{
		color	:Some(engine::rast::make_color(0x8080FFFF)),
		depth	:Some( 1f ),
		stencil	:None
	};
	let fbo = s.context.default_frame_buffer;
	let pmap = engine::call::create_plane_map( ~"o_Color", engine::frame::TarEmpty );
	let rast = engine::rast::create_rast(0,0);

	let mut data = engine::shade::create_data();
	let transform = lmath::vector::Vec4::new( 1f32, 1f32, 0f32, 0f32 );
	data.insert( ~"u_Transform",	engine::shade::UniFloatVec(transform)	);
	data.insert( ~"t_Text",			engine::shade::UniTexture(0u,s.texture)	);

	let c0 = engine::call::CallClear(
		fbo, copy pmap, cdata, rast.scissor, rast.mask);
	let c1 = engine::call::CallDraw( fbo, copy pmap, s.vao,
		s.mesh, s.mesh.get_range(), s.program, copy data, rast );
	s.context.flush( ~[c0,c1] );
	
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
