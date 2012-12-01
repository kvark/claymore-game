extern mod glfw3;
extern mod lmath;

extern mod engine;


pure fn color_to_vec(col : &engine::rast::Color)-> lmath::vector::vec4	{
	lmath::vector::Vec4::new( col.r, col.g, col.b, col.a )
}

struct Bubble	{
	text	: ~str,
	mut data: engine::shade::DataMap,
	t_string: @engine::texture::Texture,
	t_bubble: @engine::texture::Texture,
}

struct BubbleManager	{
	font		: engine::font::Font,
	max_size	: (uint,uint),
	program		: @engine::shade::Program,
	t_bubble	: @engine::texture::Texture,
	seam		: (uint,uint),
}

impl BubbleManager	{
	fn spawn( ct : &engine::context::Context, pos:(f32,f32),
			text : ~str, kerning : float, color : uint )-> Bubble	{
		let texture = @self.font.bake( ct, text, self.max_size, kerning );
		// prepare data
		let mut data = engine::shade::create_data();
		let v_color = color_to_vec( &engine::rast::make_color(color) );
		let (wid,het) = ct.screen_size, (pos_x,pos_y) = pos;
		let transform = lmath::vector::Vec4::new(
			2f32 * (texture.width as f32) / (wid as f32),
			2f32 * (texture.height as f32)/ (het as f32),
			pos_x, pos_y);	//OPTION: absolute coords
		let (cx,cy) = self.seam;
		let param = lmath::vector::Vec4::new(
			(cx as f32) / (self.t_bubble.width as f32),
			(cy as f32) / (self.t_bubble.height as f32),
			(self.t_bubble.width	as f32) / (texture.width as f32),
			(self.t_bubble.height	as f32) / (texture.height as f32)
		);
		data.insert( ~"u_Transform",	engine::shade::UniFloatVec(transform)	);
		data.insert( ~"u_Color",		engine::shade::UniFloatVec(v_color)		);
		data.insert( ~"t_Text",			engine::shade::UniTexture(0u,texture)	);
		data.insert( ~"t_Bubble",		engine::shade::UniTexture(0u,self.t_bubble));
		data.insert( ~"u_Bubble",		engine::shade::UniFloatVec(param)		);
		// spawn
		Bubble{ text:text, data:data, t_string:texture, t_bubble:self.t_bubble }
	}
}


struct Sample	{
	context		: engine::context::Context,
	font_lib	: engine::font::Context,
	bman		: BubbleManager,
	bubbles		: ~[Bubble],
	vao			: @engine::buf::VertexArray,
	mesh		: @engine::mesh::Mesh,
	mut frames	: uint,
}


fn init( wid : uint, het : uint ) -> Sample	{
	let ct = engine::context::create( wid, het );
	assert ct.sync_back();
	// create text
	let fl = engine::font::create_context();
	let font = fl.load_font( "data/font/Vera.ttf", 0u, 20u, 20u );
	// done
	ct.check(~"init");
	let bman = BubbleManager{
		font	: font,
		max_size: (640u,400u),
		program	: @engine::load::load_program( &ct, ~"data/code/hud/text_bubble" ),
		t_bubble: @engine::load::load_texture_2D( &ct, ~"data/texture/text_bubble2.png", 0, 2u ),
		seam	: (32u,20u),
	};
	let b0 = bman.spawn( &ct, (-0.9f32,-0.8f32), ~"Hello, world!\nClaymore text demo is here!",
		-1f, 0x2020FFFF );
	let b1 = bman.spawn( &ct, (-0.4f32,0.4f32), fmt!(
		"There is a single bu bble texture in this demo,\nand the size is just %ux%u.\n%s",
		bman.t_bubble.width, bman.t_bubble.height,
		"It is drawn together with the text\nusing a very smart bubble shader."
		), -1f, 0xFF2020FF );
	let b2 = bman.spawn( &ct, (0.1f32,-0.5f32), ~"Please don't mind the very basic\nfont (Bitstream Vera).",
		-1f, 0x20FF20FF );
	Sample { context:ct, font_lib:fl,
		bman : bman,
		bubbles	: ~[b0,b1,b2],
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
	let mut rast = engine::rast::create_rast(0,0);
	rast.set_blend( ~"s+d", ~"Sa", ~"1-Sa" );

	let mut calls : ~[engine::call::Call] = ~[];
	calls.push( engine::call::CallClear(
		fbo, copy pmap, cdata, rast.scissor, rast.mask)
	);
	for s.bubbles.each() |b|	{
		calls.push( engine::call::CallDraw( fbo, copy pmap, s.vao,
			s.mesh, s.mesh.get_range(), s.bman.program, copy b.data, rast )
		);
	}
	s.context.flush( calls );
	
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
	io::println("--- Claymore demo 04: text ---");
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
