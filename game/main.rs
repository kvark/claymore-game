extern mod glfw3;
extern mod glcore;
use libc::*;


fn render() ->bool	{
	glcore::glClearColor( 0.5 as c_float, 0.5 as c_float, 1.0 as c_float, 1.0 as c_float );
	glcore::glClearDepth( 1.0 as c_double );
	glcore::glClear( glcore::GL_COLOR_BUFFER_BIT | glcore::GL_DEPTH_BUFFER_BIT );
	
	let code = glcore::glGetError();
	if (code != 0)	{
		io::println( fmt!("GL Error: %d",code as int) );
		return false;
	}
	return true;
}

fn failGLFW( where: &static/str )	{
	let code = glfw3::get_error();
	io::println(~"Error: " + glfw3::error_string(code));
	glfw3::terminate();
	fail( fmt!("glfw%s() failed\n",where) );
}


fn main()	{
	do task::task().sched_mode(task::PlatformThread).spawn {
		if (glfw3::init()==0)	{
			failGLFW("Init");
		}
	
		let mut window = glfw3::create_window(800,600,glfw3::WINDOWED,"Claymore");
		if (ptr::is_null(window.ptr))	{
			failGLFW("OpenWindow");
		}
	
		window.set_title(~"Claymore");
		window.make_context_current();
		
		loop	{
			glfw3::poll_events();
			let isClosed = window.get_param(glfw3::CLOSE_REQUESTED)!=0;
			if (window.get_key(glfw3::KEY_ESC)!=0 || isClosed)	{
				glfw3::destroy_window(&mut window);
				break;
			}
			if (!render())	{
				break;
			}
			window.swap_buffers();
		}
	
		glfw3::terminate();
	}
}
