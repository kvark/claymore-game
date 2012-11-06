extern mod glfw3;
extern mod glcore;
extern mod engine;


type handle = u32;

struct Sample	{
	program	: engine::shade::Program,
	buffer	: handle
}

fn genBuffer() -> handle	{
	let mut h = 0u32;
	unsafe	{
		glcore::glGenBuffers( 1, ptr::addr_of(&h) );
	}
	h
}

fn init() -> Sample	{
	// load shaders
	let vert_code = "attribute vec2 position; void main()	{ gl_Position = vec4(position,0.0,1.0); }";
	let frag_code = "void main()	{ gl_FragData[0] = vec4(1.0,0.0,0.0,1.0); }";
	let vert_shader = engine::shade::createShader( glcore::GL_VERTEX_SHADER, vert_code );
	let frag_shader = engine::shade::createShader( glcore::GL_FRAGMENT_SHADER, frag_code );
	let program = engine::shade::createProgram( ~[vert_shader,frag_shader] );
	// load buffers
	let vdata = [-1f32,-1f32,0f32,1f32,1f32,-1f32];
	let buf_handle = genBuffer();
	glcore::glBindBuffer( glcore::GL_ARRAY_BUFFER, buf_handle );
	unsafe	{
		glcore::glBufferData( glcore::GL_ARRAY_BUFFER,
			sys::size_of::<f32>()*vdata.len() as i64,
			vec::raw::to_ptr(vdata) as *libc::c_void,
			glcore::GL_STATIC_DRAW );
	}
	// done
	io::println( fmt!("Init: program %u, buffer %u",program.handle as uint,buf_handle as uint) );
	Sample {program:program, buffer:buf_handle}
}


fn render(s:&Sample) ->bool	{
	glcore::glClearColor( 0.5f32, 0.5f32, 1.0f32, 1.0f32 );
	glcore::glClearDepth( 1.0f64 );
	glcore::glClear( glcore::GL_COLOR_BUFFER_BIT | glcore::GL_DEPTH_BUFFER_BIT );
	
	s.program.bind();
	glcore::glBindBuffer( glcore::GL_ARRAY_BUFFER, s.buffer );
	glcore::glVertexAttribPointer( 0, 2, glcore::GL_FLOAT, glcore::GL_FALSE,
		sys::size_of::<f32>()*2u as i32, 0 as *libc::c_void );
	glcore::glEnableVertexAttribArray( 0 );
	glcore::glDrawArrays( glcore::GL_TRIANGLES, 0, 3 );
	
	let code = glcore::glGetError();
	if (code != 0)	{
		io::println( fmt!("GL Error: %d",code as int) );
		return false;
	}
	return true;
}


struct Buffer	{
	a:int
}
struct BufferBinding	{
	a:int
}
struct FramebufferBinding	{
	a:int
}
struct Context	{
	arrayBuffer			: BufferBinding,
	indexBuffer			: BufferBinding,
	framebufferTarget	: FramebufferBinding,
}


fn failGLFW( where: &static/str )	{
	let code = glfw3::get_error();
	io::println(~"Error: " + glfw3::error_string(code));
	glfw3::terminate();
	fail( fmt!("glfw%s() failed\n",where) );
}


fn main()	{
	io::println("--- Claymore ---");
	do task::task().sched_mode(task::PlatformThread).spawn {
		if (glfw3::init()==0)	{
			failGLFW("Init");
		}
	
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
