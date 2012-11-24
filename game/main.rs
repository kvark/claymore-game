extern mod glfw3;
extern mod glcore;
extern mod lmath;
extern mod stb_image;

extern mod engine;


struct Game	{
	context		: engine::context::Context,
	mut frames	: uint,
}


impl Game	{
	fn render() ->bool	{
		let c0 = engine::call::CallClear(
			self.context.default_frame_buffer,
			engine::call::create_plane_map( ~"main", engine::frame::TarEmpty ),
			engine::call::ClearData{
				color	:Some( engine::rast::make_color(0x8080FFFF) ),
				depth	:Some( 1f ),
				stencil	:Some(0u)
			},
			engine::rast::Scissor{ test:false, area:self.context.rast.scissor.area },
			self.context.rast.mask
			);

		self.context.flush(~[c0]);

		self.frames += 1;
		self.context.cleanup();
		self.context.check(~"render");
		true
	}
}


fn make_game( wid : uint, het : uint )-> Game	{
	let ct = engine::context::create( wid, het );
	assert ct.sync_back();
	// done
	ct.check(~"init");
	Game{ context:ct, frames:0u }
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
			if (window.get_key(glfw3::KEY_ESC)!=0 || isClosed)	{
				glfw3::destroy_window(&mut window);
				break;
			}
			if (!game.render())	{
				break;
			}
			window.swap_buffers();
		}
	
		glfw3::terminate();
	}
}
