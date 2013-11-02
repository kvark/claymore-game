extern mod glfw;
extern mod cgmath;
extern mod engine;
extern mod std;

use engine::{anim,gr_low,gr_mid};
use engine::gr_low::context::ProxyState;

use hud;
use input;
use logic::Logic;
use scene::load_json;


struct Journal	{
	main	: engine::journal::Log,
	load	: engine::journal::Log,
	render	: engine::journal::Log,
	input	: engine::journal::Log,
}

struct Time	{
	moment	: anim::float,
	render	: engine::anim::Timer,
	animate	: engine::anim::Timer,
}

impl Time	{
	pub fn new()-> Time	{
		Time	{
			moment	: glfw::get_time(),
			render	: engine::anim::Timer::new(),
			animate	: engine::anim::Timer::new(),
		}
	}
	pub fn update( &mut self )	{
		let d = glfw::get_time() - self.moment;
		self.render.update( d );
		self.animate.update( d );
		self.moment += d;
	}
}


struct Game	{
	gr_context	: gr_low::context::Context,
	aud_context	: engine::audio::Context,
	font_context: gr_mid::font::Context,
	hud_context	: hud::main::Context,
	journal		: Journal,
	frames		: uint,
	call_count	: uint,
	logic 		: Logic,
	time 		: Time,
	debug_menu	: hud::debug::Menu<Logic>,
}

local_data_key!(game_singleton : @mut Game)


#[deriving(Decodable)]
pub struct Elements	{
	character	: bool,
	shadow		: bool,
	lbuffer		: uint,
	environment	: bool,
	hud			: bool,
	hud_debug	: bool,
}

impl Game	{
	pub fn create( el : &Elements, wid : uint, het : uint, ns : uint, journal : Journal  )-> Game	{
		let mut gcon = gr_low::context::create( glfw::get_proc_address, wid, het, ns );
		assert!( gcon.sync_back() );
		// audio test
		let acon = engine::audio::Context::create( "" );
		if false	{
			let buf = @engine::audio::load_wav( &acon, "data/sound/stereol.wav", &journal.load );
			let src = @mut acon.create_source();
			src.bind(buf);
		}
		//src.play();
		// create hud
		let fcon = gr_mid::font::Context::create();
		let mut hcon = hud::main::Context::create( &mut gcon, &journal.load );
		// logic
		let mut logic = Logic::create( el, &mut gcon, &fcon, &mut hcon, &journal.load );
		logic.reset( 0.0 );
		// debug menu
		let menu : hud::debug::Menu<Logic> = logic.create_debug_menu();
		menu.preload( &mut gcon, &fcon, &mut hcon, &journal.load );
		// done
		gcon.check("init");
		Game{
			gr_context	: gcon,
			aud_context	: acon,
			font_context: fcon,
			hud_context	: hcon,
			journal		: journal,
			frames:0u, call_count:0u,
			logic		: logic,
			time		: Time::new(),
			debug_menu	: menu,
		}
	}

	pub fn get()-> @mut Game	{
		std::local_data::get( game_singleton, |opt| *opt.expect("Your game is dead") )
	}

	pub fn on_input( &mut self, win : &glfw::Window, event : input::Event )	{
		self.time.update();
		self.journal.input.add( event.to_str() );
		let (px,py) = win.get_cursor_pos();
		let (sw,sh) = win.get_size();
		let state = input::State{
			time_game	: self.time.animate.time,
			time_view	: self.time.render.time,
			focus	: win.is_visible(),
			aspect	: (sw as f32) / (sh as f32),
			mouse	: [(px / (sw as f64)) as f32, (py / (sh as f64)) as f32],
			log		: self.journal.main.fork( ~"update" ),
		};
		self.logic.on_input( &event, &state, &mut self.debug_menu );
	}

	pub fn render( &mut self, el : &Elements )-> bool	{
		// scene
		self.logic.render( el, &mut self.gr_context, &self.hud_context,
			&self.debug_menu, &self.journal.render );
		// submit
		self.call_count = self.gr_context.call_count;
		self.gr_context.call_count = 0;
		self.frames += 1;
		self.gr_context.check( "render" );
		// exit if logging draw calls
		!self.journal.render.enable
	}
}


#[deriving(Decodable)]
struct ConfigWindow	{
	title:~str, width:uint, height:uint, samples:uint, fullscreen:bool,
}
#[deriving(Decodable)]
struct ConfigGL	{
	major:uint, minor:uint, core:bool, debug:bool,
}
#[deriving(Decodable)]
struct ConfigLog	{
	path:~str, load:bool, render:bool, input:bool,
}
#[deriving(Decodable)]
struct Config	{
	window	: ConfigWindow,
	GL		: ConfigGL,
	journal	: ConfigLog,
	elements: Elements,
}

#[main]
pub fn main()	{
	glfw::set_error_callback( |_error,description|	{
		fail!("GLFW Error: {:s}", description)
	});
	do glfw::start {
		let config = load_json::load_config::<Config>( "data/config.json" );
		let lg = engine::journal::Log::create( config.journal.path.clone() );
		lg.add("--- Claymore ---");
		let mut journal = Journal	{
			load	: lg.fork( ~"Load" ),
			render	: lg.fork( ~"Render" ),
			input	: lg.fork( ~"Input" ),
			main	: lg,
		};
		journal.load.enable		= config.journal.load;
		journal.render.enable	= config.journal.render;
		journal.input.enable	= config.journal.input;

		glfw::window_hint::resizable( false );
		glfw::window_hint::opengl_debug_context( config.GL.debug );
		glfw::window_hint::context_version_major( config.GL.major );
		glfw::window_hint::context_version_minor( config.GL.minor );
		if config.GL.core	{
			glfw::window_hint::opengl_profile( glfw::OpenGlCoreProfile );
			glfw::window_hint::opengl_forward_compat( true );
		}

		let cw = &config.window;
		let monitor = match glfw::Monitor::get_primary()	{
			Ok(m)	=> m,
			Err(e)	=> fail!( "Monitoor::get_primary failed: {:s}", e.to_str() )
		};
		let mode = if cw.fullscreen {
			glfw::FullScreen( monitor )
		}else {
			glfw::Windowed
		};
		assert_eq!( cw.samples, 0 );
		let window = match glfw::Window::create( cw.width, cw.height, cw.title, mode )	{
			Some(w)	=> w,
			None	=> fail!( "Window::create failed" )
		};

		//window.set_input_mode( glfw::CURSOR_MODE, glfw::CURSOR_CAPTURED as int );
		window.make_context_current();
		let game = @mut Game::create( &config.elements, cw.width, cw.height, cw.samples, journal );
		std::local_data::set( game_singleton, game );

		// init callbacks
		window.set_iconify_callback( |win,done|	{
			Game::get().on_input( win, input::EvFocus(!done) );
		});
		window.set_focus_callback( |win,done|	{
			Game::get().on_input( win, input::EvFocus(done) );
		});
		window.set_char_callback( |win,key|	{
			Game::get().on_input( win, input::EvCharacter( key ));
		});
		window.set_key_callback( |win,key,_scan,action,_mods|	{
			Game::get().on_input( win, input::EvKeyboard( key, action == glfw::Press ));
		});
		window.set_cursor_pos_callback( |win,posx,posy|	{
			Game::get().on_input( win, input::EvMouseMove( posx as f32, posy as f32 ));
		});
		window.set_mouse_button_callback( |win,button,action,_mods|	{
			Game::get().on_input( win, input::EvMouseClick( button as uint, action == glfw::Press ));
		});
		window.set_scroll_callback( |win,floatx,floaty|	{
			Game::get().on_input( win, input::EvScroll( floatx as f32, floaty as f32 ));
		});
		
		loop	{
			glfw::poll_events();
			if window.should_close() || window.get_key(glfw::KeyEscape) == glfw::Press	{
				window.close();
				break;
			}
			//TODO: update on a higher frequency than render
			game.on_input( &window, input::EvRender( game.frames ));
			// render
			if !game.render( &config.elements )	{
				break;
			}
			window.swap_buffers();
		}
	}
}
