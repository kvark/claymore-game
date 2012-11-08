extern mod glfw3;
extern mod glcore;
extern mod engine;
extern mod stb_image;


struct Sample	{
	ct			: engine::context::Context,	
	program		: engine::shade::Program,
	mut data	: engine::shade::DataMap,
	buffer		: engine::buf::Object,
	texture		: engine::texture::Texture,
}


fn init() -> Sample	{
	let ct = engine::context::create();
	//assert ct.sync_back();
	// load shaders
	let vert_code = "#version 150 core
		in vec2 position;
		out vec2 texCoords;
		void main()	{
			texCoords = vec2(0.5,-0.5)*position + 0.5;
			gl_Position = vec4(position,0.0,1.0);
		}";
	let frag_code = "#version 150 core
		uniform float color=0.5;
		uniform sampler2D image;
		in vec2 texCoords;
		out vec4 result;
		void main()	{
			result = texture(image,texCoords);
		}";
	let vert_shader = ct.create_shader( glcore::GL_VERTEX_SHADER, vert_code );
	let frag_shader = ct.create_shader( glcore::GL_FRAGMENT_SHADER, frag_code );
	let program = ct.create_program( ~[vert_shader,frag_shader] );
	// load buffers
	let vdata = ~[-1f32,-1f32,0f32,1f32,1f32,-1f32];
	let buf = ct.create_buffer_loaded( vdata );
	// load texture
	let mut tex : engine::texture::Texture;
	match stb_image::image::load(~"data/GpuPro3.jpeg")	{
		Some(image) => {
			 tex = ct.create_texture( glcore::GL_TEXTURE_2D, image.width, image.height, 1 );
			 ct.texture.bind( 0u, &tex );
			 ct.texture.load_2D( &tex, 0, glcore::GL_RGBA as glcore::GLint,
			 	glcore::GL_RGBA, glcore::GL_UNSIGNED_BYTE, image.data );
			 ct.texture.wrap( &tex, 0 );
			 ct.texture.filter( &tex, 2u );
		}
		None => { fail(~"Unable to load image"); }
	}
	// done
	ct.check(~"init");
	io::println( fmt!("init: program %u, buffer %u, texture %u",
		*program.handle as uint,*buf.handle as uint, *tex.handle as uint)
	);
	Sample { ct:ct, program:program, data:engine::shade::create_data(), buffer:buf, texture:tex }
}


fn render( s : &Sample ) ->bool	{
	glcore::glClearColor( 0.5f32, 0.5f32, 1.0f32, 1.0f32 );
	glcore::glClearDepth( 1.0f64 );
	glcore::glClear( glcore::GL_COLOR_BUFFER_BIT | glcore::GL_DEPTH_BUFFER_BIT );
	
	s.ct.texture.bind( 0u, &s.texture );
	s.data.insert( ~"color", @1f	as engine::shade::Uniform );
	s.data.insert( ~"image", @0		as engine::shade::Uniform );
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
