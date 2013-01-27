extern mod glfw3;
extern mod lmath;
extern mod engine;
extern mod std;

use lmath::quat::*;
use lmath::vec::vec3::*;
use std::json;


enum Screen	{
	ScreenChar,
	ScreenBattle,
	ScreenWorld,
	ScreenDeath,
}


struct Game	{
	context		: engine::context::Context,
	audio		: engine::audio::Context,
	journal		: engine::context::Log,
	sound_source: @engine::audio::Source,
	mut frames	: uint,
	technique	: engine::draw::Technique,
	editor		: chared::Scene,
	battle		: battle::Scene,
	mut screen	: Screen,
	mut time	: float,
}

#[auto_decode]
pub struct Elements	{
	character	: bool,
	shadow		: bool,
	environment	: bool,
	hud			: bool,
	hud_debug	: bool,
}

impl Game	{
	fn update( nx : float, ny : float, mouse_hit : bool, scroll : float )-> bool	{
		let dt = engine::anim::get_time() - self.time;
		self.time += dt;
		match self.screen	{
			ScreenChar		=> self.editor.update( dt, nx, ny, mouse_hit, scroll, &self.journal ),
			ScreenBattle	=> self.battle.update( &self.context.texture, nx, ny, mouse_hit ),
			_ => true
		}
	}
	fn render( el : &Elements, press_key : Option<char> )-> bool	{
		match self.screen	{
			ScreenChar => self.editor.render( el, press_key, &self.context, &self.journal ),
			ScreenBattle => {
				// clear screen
				let c0 = self.technique.gen_clear(
					engine::call::ClearData{
						color	:Some( engine::rast::make_color(0x8080FFFF) ),
						depth	:Some( 1f ),
						stencil	:Some( 0u ),
					}
				);
				self.context.flush(~[c0]);
				// draw battle
				self.battle.render( &self.context, &self.technique, &self.journal );
			},
			_ => ()
		}
		// done
		self.frames += 1;
		self.context.cleanup();
		self.context.check(~"render");
		true
	}
	fn debug_move( rot : bool, x : int, y : int )	{
		self.battle.debug_move( rot, x, y );
	}
}


fn create_game( wid : uint, het : uint, lg : engine::context::Log  )-> Game	{
	let ct = engine::context::create( wid, het );
	assert ct.sync_back();
	// audio test
	let ac = engine::audio::create_context();
	let buf = @engine::audio::load_wav( &ac, ~"data/sound/stereol.wav", &lg );
	let src = @ac.create_source();
	src.bind(buf);
	//src.play();
	// create a forward light technique
	let tech = {
		let pmap = engine::call::make_pmap_simple( ~"o_Color", engine::frame::TarEmpty );
		let mut rast = copy ct.default_rast;
		rast.set_depth(~"<=",true);
		rast.prime.cull = true;
		let cache = @mut engine::draw::create_cache();
		engine::draw::load_technique( ~"main", ~"data/code/tech/forward/light",
			(ct.default_frame_buffer, pmap, rast), cache)
	};
	// done
	ct.check(~"init");
	let aspect = (wid as float) / (het as float);
	let editor = chared::make_scene( &ct, aspect, &lg );
	let battle = battle::make_scene( &ct, aspect, &lg );
	Game{ context:ct, audio:ac, journal:lg,
		sound_source:src,
		frames:0u, technique:tech,
		editor:editor, battle:battle,
		screen:ScreenChar, time:0f,
	}
}


fn fail_GLFW( where: &static/str ) -> !	{
	//let code = glfw3::get_error();
	//io::println(~"GLFW error: " + glfw3::error_string(code));
	glfw3::terminate();
	fail fmt!("glfw%s() failed\n",where)
}


#[auto_decode]
struct ConfigWindow	{
	title:~str, width:uint, height:uint, samples:uint, fullscreen:bool,
}
#[auto_decode]
struct ConfigGL	{
	major:uint, minor:uint, debug:bool,
}
#[auto_decode]
struct Log	{
	path:~str, depth:uint,
}
#[auto_decode]
struct Config	{
	window	: ConfigWindow,
	GL		: ConfigGL,
	journal	: Log,
	elements: Elements,
}


fn main()	{
	do task::task().sched_mode(task::PlatformThread).spawn {
		if (glfw3::init()==0)	{
			fail_GLFW("Init");
		}

		let config = scene::load_config::<Config>( ~"data/config.json" );
		let lg = engine::context::create_log( copy config.journal.path, config.journal.depth );
		lg.add(~"--- Claymore ---");

		glfw3::window_hint( glfw3::RESIZABLE, 0 );
		glfw3::window_hint( glfw3::OPENGL_DEBUG_CONTEXT, if config.GL.debug {1} else {0} );
		glfw3::window_hint( glfw3::CONTEXT_VERSION_MAJOR, config.GL.major as libc::c_int );
		glfw3::window_hint( glfw3::CONTEXT_VERSION_MINOR, config.GL.minor as libc::c_int );
		glfw3::window_hint( glfw3::OPENGL_PROFILE, glfw3::OPENGL_CORE_PROFILE );
		glfw3::window_hint( glfw3::OPENGL_FORWARD_COMPAT, 1 );

		let mode = if config.window.fullscreen {
			glfw3::FullScreen( glfw3::get_primary_monitor() )
		}else {
			glfw3::Windowed
		};
		let window = glfw3::Window::create(
			config.window.width, config.window.height,
			config.window.title, mode );
		if (window.is_null())	{
			fail_GLFW("OpenWindow");
		}
	
		window.set_input_mode( glfw3::CURSOR_MODE, glfw3::CURSOR_CAPTURED as int );
		window.make_context_current();
		let game = create_game( config.window.width, config.window.height, lg );
		
		loop	{
			glfw3::poll_events();
			let isClosed = window.get_param(glfw3::SHOULD_CLOSE)!=0;
			if window.get_key(glfw3::KEY_ESC)!=0 || isClosed	{
				window.destroy();
				break;
			}
			let (_,scroll_y) = window.get_scroll_offset();
			let shift = window.get_key(glfw3::KEY_LEFT_SHIFT)!=0;
			// debug keys
			if window.get_key(glfw3::KEY_LEFT)!=0	{
				game.debug_move(shift,-1,0);
			}
			if window.get_key(glfw3::KEY_RIGHT)!=0	{
				game.debug_move(shift,1,0);
			}
			if window.get_key(glfw3::KEY_DOWN)!=0	{
				game.debug_move(shift,0,-1);
			}
			if window.get_key(glfw3::KEY_UP)!=0	{
				game.debug_move(shift,0,1);
			}
			// mouse buttons
			let mouse_hit = window.get_mouse_button( glfw3::MOUSE_BUTTON_LEFT )!=0;
			// camera rotation
			let _cam_dir = (window.get_key(glfw3::KEY_E) - window.get_key(glfw3::KEY_Q)) as int;
			let press_key = if window.get_key(glfw3::KEY_E) != 0 {Some('Y')} else {None};
			// render
			let (cx,cy) = window.get_cursor_pos();
			let nx = (cx as float)/(config.window.width as float);
			let ny = (cy as float)/(config.window.height as float);
			if !game.update( nx, ny, mouse_hit, scroll_y as float )	{
				break
			}
			if !game.render( &config.elements, press_key )	{
				break;
			}
			window.swap_buffers();
		}
	
		glfw3::terminate();
	}
}
