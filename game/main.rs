extern mod glfw;
extern mod lmath;
extern mod engine;
extern mod std;

use engine::context::ProxyState;
//use battle;
use chared;
use scene;


enum Screen	{
	ScreenChar,
	//ScreenBattle,
	ScreenWorld,
	ScreenDeath,
}


struct Game	{
	context		: engine::context::Context,
	audio		: engine::audio::Context,
	journal		: engine::context::Log,
	sound_source: @mut engine::audio::Source,
	frames		: uint,
	technique	: engine::draw::Technique,
	output		: engine::call::DrawOutput,
	editor		: chared::Scene,
	//battle		: battle::Scene,
	screen		: Screen,
	time		: float,
}

#[auto_decode]
pub struct Elements	{
	character	: bool,
	shadow		: bool,
	lbuffer		: uint,
	environment	: bool,
	hud			: bool,
	hud_debug	: bool,
}

pub impl Game	{
	fn create( el : &Elements, wid : uint, het : uint, lg : engine::context::Log  )-> Game	{
		let mut ct = engine::context::create( wid, het );
		assert!( ct.sync_back() );
		// audio test
		let ac = engine::audio::Context::create();
		let buf = @engine::audio::load_wav( &ac, ~"data/sound/stereol.wav", &lg );
		let src = @mut ac.create_source();
		src.bind(buf);
		//src.play();
		// create a forward light technique
		let tech = engine::draw::load_technique( ~"data/code/tech/forward/light" );
		let out = {
			let pmap = engine::call::PlaneMap::new_simple( ~"o_Color", engine::frame::TarEmpty );
			let mut rast = copy ct.default_rast;
			rast.set_depth(~"<=",true);
			rast.prime.cull = true;
			(ct.default_frame_buffer, pmap, rast)
		};
		// done
		ct.check(~"init");
		let aspect = (wid as float) / (het as float);
		let editor = chared::make_scene( el, &mut ct, aspect, &lg );
		//let battle = battle::make_scene( &ct, aspect, &lg );
		Game{ context:ct, audio:ac, journal:lg,
			sound_source:src, frames:0u,
			technique:tech, output:out,
			editor:editor, //battle:battle,
			screen:ScreenChar, time:0f,
		}
	}

	fn update( &mut self, nx : float, ny : float, mouse_hit : bool, scroll : float )-> bool	{
		let dt = engine::anim::get_time() - self.time;
		self.time += dt;
		match self.screen	{
			ScreenChar		=> self.editor.update( dt, nx, ny, mouse_hit, scroll, &self.journal ),
			//ScreenBattle	=> self.battle.update( &self.context.texture, nx, ny, mouse_hit ),
			_ => true
		}
	}

	fn on_char( &mut self, key : char )	{
		//io::println(fmt!("Char %c", key));
		match self.screen	{
			ScreenChar	=> self.editor.on_char( key ),
			_	=> ()
		}
	}
	fn on_key_press( &mut self, key : int )	{
		match self.screen	{
			ScreenChar	=> self.editor.on_key_press( key ),
			_	=> ()
		}	
	}
	
	fn render( &mut self, el : &Elements )-> bool	{
		match self.screen	{
			ScreenChar => self.editor.render( el, &mut self.context, &self.journal ),
			/*ScreenBattle => {
				// clear screen
				let c0 =
					engine::call::ClearData{
						color	:Some( engine::rast::Color::new(0x8080FFFF) ),
						depth	:Some( 1f ),
						stencil	:Some( 0u ),
					}.gen_call( copy self.output );
				self.context.flush(~[c0]);
				// draw battle
				self.battle.render( &self.context, &self.technique, copy self.output, &self.journal );
			},*/
			_ => ()
		}
		// done
		self.frames += 1;
		self.context.cleanup( &self.journal );
		self.context.check( ~"render" );
		true
	}
	
	fn debug_move( &mut self, _rot : bool, _x : int, _y : int )	{
		//self.battle.debug_move( rot, x, y );
	}
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
	do glfw::set_error_callback() |_error,description|	{
		fail!(fmt!( "GLFW Error: %s", description ))
	}
	do glfw::spawn {
		let config = scene::load_config::<Config>( ~"data/config.json" );
		let lg = engine::context::Log::create( copy config.journal.path, config.journal.depth );
		lg.add(~"--- Claymore ---");

		glfw::ml::window_hint( glfw::RESIZABLE, 0 );
		glfw::ml::window_hint( glfw::OPENGL_DEBUG_CONTEXT, if config.GL.debug {1} else {0} );
		glfw::ml::window_hint( glfw::CONTEXT_VERSION_MAJOR, config.GL.major as libc::c_int );
		glfw::ml::window_hint( glfw::CONTEXT_VERSION_MINOR, config.GL.minor as libc::c_int );
		//glfw::ml::window_hint( glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE );
		//glfw::ml::window_hint( glfw::OPENGL_FORWARD_COMPAT, 1 );

		let mode = if config.window.fullscreen {
			glfw::FullScreen( glfw::get_primary_monitor() )
		}else {
			glfw::Windowed
		};
		let window = glfw::Window::create(
			config.window.width, config.window.height,
			config.window.title, mode ).get();

		//window.set_input_mode( glfw::CURSOR_MODE, glfw::CURSOR_CAPTURED as int );
		window.make_context_current();
		
		let game = @mut Game::create( &config.elements, config.window.width, config.window.height, lg );
		do window.set_char_callback()	|_win,key|	{
			game.on_char( key as char );
		}
		do window.set_key_callback() |_win,key,action|	{
			if action == glfw::PRESS	{
				game.on_key_press( key as int );
			}
		};
		
		loop	{
			glfw::poll_events();
			if window.should_close() || window.get_key(glfw::support::consts::KEY_ESCAPE)!=0	{
				window.destroy();
				break;
			}
			//let (_,scroll_y) = window.get_scroll_offset(); //FIXME
			let scroll_y = 0;
			let shift = window.get_key(glfw::KEY_LEFT_SHIFT)!=0;
			// debug keys
			if window.get_key(glfw::KEY_LEFT)!=0	{
				game.debug_move(shift,-1,0);
			}
			if window.get_key(glfw::KEY_RIGHT)!=0	{
				game.debug_move(shift,1,0);
			}
			if window.get_key(glfw::KEY_DOWN)!=0	{
				game.debug_move(shift,0,-1);
			}
			if window.get_key(glfw::KEY_UP)!=0	{
				game.debug_move(shift,0,1);
			}
			// mouse buttons
			let mouse_hit = window.get_mouse_button( glfw::MOUSE_BUTTON_LEFT )!=0;
			// camera rotation
			let _cam_dir = (window.get_key(glfw::KEY_E) - window.get_key(glfw::KEY_Q)) as int;
			// render
			let (cx,cy) = window.get_cursor_pos();
			let nx = (cx as float)/(config.window.width as float);
			let ny = (cy as float)/(config.window.height as float);
			if !game.update( nx, ny, mouse_hit, scroll_y as float )	{
				break
			}
			if !game.render( &config.elements )	{
				break;
			}
			window.swap_buffers();
		}
	}
}
