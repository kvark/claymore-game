extern mod glfw;
extern mod cgmath;
extern mod engine;
extern mod std;

use engine::{gr_low,gr_mid};
use engine::gr_low::context::ProxyState;

use input;
use hud = hud::main;
use scene;
use scene::chared;
use battle = battle::main;


enum Screen	{
	ScreenIntro,
	ScreenChar,
	ScreenBattle,
	ScreenWorld,
	ScreenDeath,
}

struct Journal	{
	main	: engine::journal::Log,
	load	: engine::journal::Log,
	render	: engine::journal::Log,
}


struct Game	{
	gr_context	: gr_low::context::Context,
	aud_context	: engine::audio::Context,
	font_context: gr_mid::font::Context,
	hud_context	: hud::Context,
	journal		: Journal,
	frames		: uint,
	call_count	: uint,
	technique	: gr_mid::draw::Technique,
	output		: gr_mid::call::Output,
	s_intro		: scene::intro::Scene,
	s_editor	: chared::Scene,
	s_battle	: battle::Scene,
	screen		: Screen,
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
		let mut gcon = gr_low::context::create( wid, het, ns );
		assert!( gcon.sync_back() );
		// audio test
		let acon = engine::audio::Context::create( "" );
		if false	{
			let buf = @engine::audio::load_wav( &acon, "data/sound/stereol.wav", &journal.load );
			let src = @mut acon.create_source();
			src.bind(buf);
		}
		//src.play();
		// create a forward light technique
		let tech = gr_mid::draw::load_technique( "data/code/tech/forward/light" );
		let pmap = gr_mid::call::PlaneMap::new_main( &gcon, ~"o_Color" );
		let out = gr_mid::call::Output::new( gcon.default_frame_buffer, pmap );
		// create hud
		let fcon = gr_mid::font::Context::create();
		let mut hcon = hud::Context::create( &mut gcon, &journal.load );
		// done
		gcon.check("init");
		let intro = scene::intro::Scene{ active:false };
		let editor = chared::create( el, &mut gcon, &fcon, &journal.load );
		let battle = battle::create( &mut gcon, &mut hcon, &fcon, &journal.load );
		Game{
			gr_context	: gcon,
			aud_context	: acon,
			font_context: fcon,
			hud_context	: hcon,
			journal		: journal,
			frames:0u, call_count:0u,
			technique	: tech,
			output		: out,
			s_intro:intro, s_editor:editor, s_battle:battle,
			screen		: ScreenBattle,
		}
	}

	pub fn get()-> @mut Game	{
		std::local_data::get( game_singleton, |opt| *opt.expect("Your game is dead") )
	}

	pub fn reset( &mut self )	{
		match self.screen	{
			ScreenBattle	=> self.s_battle.reset(),
			_ => ()
		}
	}

	pub fn update( &mut self, input : &input::State )-> bool	{
		let aspect = self.output.area.aspect();
		match self.screen	{
			ScreenChar		=> self.s_editor.update( input, &self.journal.main ),
			ScreenBattle	=> self.s_battle.update( input, aspect ),
			_ => true
		}
	}

	pub fn on_input( &mut self, event : input::Event )	{
		self.journal.main.add( event.to_str() );
		match self.screen	{
			ScreenChar		=> self.s_editor.on_input( &event ),
			ScreenBattle	=> self.s_battle.on_input( &event ),
			_	=> ()
		}
	}

	pub fn render( &mut self, el : &Elements )-> bool	{
		match self.screen	{
			ScreenIntro	=> (),
			ScreenChar	=> self.s_editor.render( el, &mut self.gr_context, &self.journal.render ),
			ScreenBattle	=> self.s_battle.render( &mut self.gr_context, &self.hud_context,
				&self.technique, self.output.clone(), &self.journal.render ),	
			_ => ()
		}
		// submit
		self.call_count = self.gr_context.call_count;
		self.gr_context.call_count = 0;
		self.frames += 1;
		self.gr_context.check( "render" );
		// exit if logging draw calls
		!self.journal.render.enable
	}
	
	pub fn debug_move( &mut self, rot : bool, x : int, y : int )	{
		self.s_battle.debug_move( rot, x, y );
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
	path:~str, load:bool, render:bool,
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
		fail!("GLFW Error: %s", description)
	});
	do glfw::start {
		let config = scene::load_json::load_config::<Config>( "data/config.json" );
		let lg = engine::journal::Log::create( config.journal.path.clone() );
		lg.add(~"--- Claymore ---");
		let mut journal = Journal	{
			load	: lg.fork( ~"Load" ),
			render	: lg.fork( ~"Render" ),
			main	: lg,
		};
		journal.load.enable		= config.journal.load;
		journal.render.enable	= config.journal.render;

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
			Err(e)	=> fail!( "Monitoor::get_primary failed: %s", e.to_str() )
		};
		let mode = if cw.fullscreen {
			glfw::FullScreen( monitor )
		}else {
			glfw::Windowed
		};
		assert_eq!( cw.samples, 0 );
		let window = match glfw::Window::create( cw.width, cw.height, cw.title, mode )	{
			Ok(w)	=> w,
			Err(e)	=> fail!( "Window::create failed: %s", e.to_str() ),
		};

		//window.set_input_mode( glfw::CURSOR_MODE, glfw::CURSOR_CAPTURED as int );
		window.make_context_current();
		let game = @mut Game::create( &config.elements, cw.width, cw.height, cw.samples, journal );
		game.reset();

		std::local_data::set( game_singleton, game );

		// init callbacks
		window.set_iconify_callback( |_win,done|	{
			Game::get().on_input( input::Focus(!done) );
		});
		window.set_focus_callback( |_win,done|	{
			Game::get().on_input( input::Focus(done) );
		});
		window.set_char_callback( |_win,key|	{
			Game::get().on_input( input::Character( key ));
		});
		window.set_key_callback( |_win,key,_scan,action,_mods|	{
			Game::get().on_input( input::Keyboard( key, action == glfw::Press ));
		});
		window.set_cursor_pos_callback( |_win,posx,posy|	{
			Game::get().on_input( input::MouseMove( posx, posy ));
		});
		window.set_mouse_button_callback( |_win,button,action,_mods|	{
			Game::get().on_input( input::MouseClick( button as uint, action == glfw::Press ));
		});
		window.set_scroll_callback( |_win,floatx,floaty|	{
			Game::get().on_input( input::Scroll( floatx, floaty ));
		});
		
		loop	{
			glfw::poll_events();
			if window.should_close() || window.get_key(glfw::KeyEscape) == glfw::Press	{
				window.close();
				break;
			}
			// update
			let input = {
				let (px,py) = window.get_cursor_pos();
				let mut buttons = 0u;
				let all_buttons = [glfw::MouseButtonLeft,glfw::MouseButtonRight,glfw::MouseButtonMiddle];
				for (i,but) in all_buttons.iter().enumerate()	{
					if window.get_mouse_button(*but) == glfw::Press	{
						buttons |= (1<<i) as uint;
					}
				}
				input::State{
					time	: engine::anim::get_time(),
					focus	: window.is_visible(),
					mouse	: input::Mouse{
						x	: px / (cw.width as float),
						y	: py / (cw.height as float),
						buttons	: buttons,
					},
				}
			};
			//TODO: update on a higher frequency than render
			if !game.update( &input )	{
				break
			}
			// render
			if !game.render( &config.elements )	{
				break;
			}
			window.swap_buffers();
		}
	}
}
