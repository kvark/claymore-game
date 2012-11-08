extern mod glfw3;
extern mod glcore;
extern mod engine;
use send_map::linear::LinearMap;


struct Sample	{
	ct			: engine::context::Context,	
	program		: engine::shade::Program,
	mut data	: engine::shade::DataMap,
	buffer		: engine::buf::Object,
}


fn init() -> Sample	{
	let ct = engine::context::create();
	//assert ct.sync_back();
	// load shaders
	let vert_code = "attribute vec2 position; void main()	{ gl_Position = vec4(position,0.0,1.0); }";
	let frag_code = "uniform float color; void main()	{ gl_FragData[0] = vec4(color,0.0,0.0,1.0); }";
	let vert_shader = ct.create_shader( glcore::GL_VERTEX_SHADER, vert_code );
	let frag_shader = ct.create_shader( glcore::GL_FRAGMENT_SHADER, frag_code );
	let program = ct.create_program( ~[vert_shader,frag_shader] );
	if true {
		let name = ~"color";
		let uni = program.params.get(&name).value;
		let mut my_val:float;
		unsafe	{ my_val = cast::transmute(&uni); }
		io::println( fmt!("Initial '%s' value: %f",name,my_val) );
	}
	// load buffers
	let vdata = ~[-1f32,-1f32,0f32,1f32,1f32,-1f32];
	let buf = ct.create_buffer_loaded( vdata );
	// done
	ct.check(~"init");
	io::println( fmt!("init: program %u, buffer %u",program.handle as uint,buf.handle as uint) );
	Sample { ct:ct, program:program, data:engine::shade::create_data(), buffer:buf }
}


fn render( s : &Sample ) ->bool	{
	glcore::glClearColor( 0.5f32, 0.5f32, 1.0f32, 1.0f32 );
	glcore::glClearDepth( 1.0f64 );
	glcore::glClear( glcore::GL_COLOR_BUFFER_BIT | glcore::GL_DEPTH_BUFFER_BIT );
	
	s.data.insert( ~"color", @1f as engine::shade::Uniform );
	s.ct.bind_program( &s.program, &const s.data );
	s.ct.buffer_array.bind( &s.buffer );
	glcore::glVertexAttribPointer( 0, 2, glcore::GL_FLOAT, glcore::GL_FALSE,
		sys::size_of::<f32>()*2u as i32, 0 as *libc::c_void );
	glcore::glEnableVertexAttribArray( 0 );
	glcore::glDrawArrays( glcore::GL_TRIANGLES, 0, 3 );
	
	s.ct.check(~"render");
	return true;
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
