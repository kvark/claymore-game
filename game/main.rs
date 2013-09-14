extern mod glfw;
extern mod lmath;
extern mod engine;
extern mod std;

use engine::{gr_low,gr_mid};
use engine::gr_low::context::ProxyState;

use input;
use hud_new;
use scene;


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
	hud_context	: hud_new::Context,
	journal		: Journal,
	frames		: uint,
	call_count	: uint,
	technique	: gr_mid::draw::Technique,
	output		: gr_mid::call::Output,
	s_intro		: scene::intro::Scene,
	s_editor	: scene::chared::Scene,
	s_battle	: scene::battle::Scene,
	screen		: Screen,
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
	fn create( el : &Elements, wid : uint, het : uint, ns : uint, journal : Journal  )-> Game	{
		let mut gcon = gr_low::context::create( wid, het, ns );
		assert!( gcon.sync_back() );
		// audio test
		let acon = engine::audio::Context::create();
		if false	{
			let buf = @engine::audio::load_wav( &acon, ~"data/sound/stereol.wav", &journal.load );
			let src = @mut acon.create_source();
			src.bind(buf);
		}
		//src.play();
		// create a forward light technique
		let tech = gr_mid::draw::load_technique( ~"data/code/tech/forward/light" );
		let pmap = gr_mid::call::PlaneMap::new_main( &gcon, ~"o_Color" );
		let out = gr_mid::call::Output::new( gcon.default_frame_buffer, pmap );
		// create hud
		let fcon = gr_mid::font::Context::create();
		let mut hcon = hud_new::Context::create( &mut gcon, &journal.load );
		// done
		gcon.check(~"init");
		let intro = scene::intro::Scene{ active:false };
		let editor = scene::chared::make_scene( el, &mut gcon, &fcon, &journal.load );
		let battle = scene::battle::make_scene( &mut gcon, &mut hcon, &fcon, &journal.load );
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

	fn update( &mut self, input : &input::State )-> bool	{
		let aspect = self.output.area.aspect();
		match self.screen	{
			ScreenChar		=> self.s_editor.update( input, &self.journal.main ),
			ScreenBattle	=> self.s_battle.update( input, &mut self.gr_context.texture, aspect ),
			_ => true
		}
	}

	fn on_input( &mut self, event : input::Event )	{
		self.journal.main.add( event.to_str() );
		match self.screen	{
			ScreenChar		=> self.s_editor.on_input( &event ),
			ScreenBattle	=> self.s_battle.on_input( &event ),
			_	=> ()
		}
	}

	fn render( &mut self, el : &Elements )-> bool	{
		match self.screen	{
			ScreenIntro	=> (),
			ScreenChar	=> self.s_editor.render( el, &mut self.gr_context, &self.journal.render ),
			ScreenBattle	=> self.s_battle.render( &mut self.gr_context, &self.hud_context,
				&self.technique, copy self.output, &self.journal.render ),	
			_ => ()
		}
		// submit
		self.call_count = self.gr_context.call_count;
		self.gr_context.call_count = 0;
		self.frames += 1;
		self.gr_context.check( ~"render" );
		// exit if logging draw calls
		!self.journal.render.enable
	}
	
	fn debug_move( &mut self, rot : bool, x : int, y : int )	{
		self.s_battle.debug_move( rot, x, y );
	}
}


#[auto_decode]
struct ConfigWindow	{
	title:~str, width:uint, height:uint, samples:uint, fullscreen:bool,
}
#[auto_decode]
struct ConfigGL	{
	major:uint, minor:uint, core:bool, debug:bool,
}
#[auto_decode]
struct ConfigLog	{
	path:~str, load:bool, render:bool,
}
#[auto_decode]
struct Config	{
	window	: ConfigWindow,
	GL		: ConfigGL,
	journal	: ConfigLog,
	elements: Elements,
}


fn main()	{
	do glfw::set_error_callback() |_error,description|	{
		fail!(fmt!( "GLFW Error: %s", description ))
	}
	do glfw::spawn {
		let config = scene::load_json::load_config::<Config>( ~"data/config.json" );
		let lg = engine::journal::Log::create( copy config.journal.path );
		lg.add(~"--- Claymore ---");
		let mut journal = Journal	{
			load	: lg.fork( ~"Load" ),
			render	: lg.fork( ~"Render" ),
			main	: lg,
		};
		journal.load.enable		= config.journal.load;
		journal.render.enable	= config.journal.render;

		glfw::ml::window_hint( glfw::RESIZABLE, 0 );
		glfw::ml::window_hint( glfw::OPENGL_DEBUG_CONTEXT, if config.GL.debug {1} else {0} );
		glfw::ml::window_hint( glfw::CONTEXT_VERSION_MAJOR, config.GL.major as libc::c_int );
		glfw::ml::window_hint( glfw::CONTEXT_VERSION_MINOR, config.GL.minor as libc::c_int );
		if config.GL.core	{
			glfw::ml::window_hint( glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE );
			glfw::ml::window_hint( glfw::OPENGL_FORWARD_COMPAT, 1 );
		}

		let cw = &config.window;
		let mode = if cw.fullscreen {
			glfw::FullScreen( glfw::get_primary_monitor() )
		}else {
			glfw::Windowed
		};
		assert_eq!( cw.samples, 0 );
		let window = match glfw::Window::create( cw.width, cw.height, cw.title, mode )	{
			Ok(w)	=> w,
			Err(e)	=> fail!(fmt!( "Window::create failed: %s", e )),
		};

		//window.set_input_mode( glfw::CURSOR_MODE, glfw::CURSOR_CAPTURED as int );
		window.make_context_current();
		let game = @mut Game::create( &config.elements, cw.width, cw.height, cw.samples, journal );

		// init callbacks
		window.set_iconify_callback( |_win,done|	{
			game.on_input( input::Focus(!done) );
		});
		window.set_focus_callback( |_win,done|	{
			game.on_input( input::Focus(done) );
		});
		window.set_char_callback( |_win,key|	{
			game.on_input( input::Character( key as char ));
		});
		window.set_key_callback( |_win,key,action|	{
			game.on_input( input::Keyboard( key as int, action == glfw::PRESS ));
		});
		window.set_cursor_pos_callback( |_win,posx,posy|	{
			game.on_input( input::MouseMove( posx, posy ));
		});
		window.set_mouse_button_callback( |_win,button,action|	{
			game.on_input( input::MouseClick( button as uint, action == glfw::PRESS ));
		});
		window.set_scroll_callback( |_win,floatx,floaty|	{
			game.on_input( input::Scroll( floatx, floaty ));
		});
		
		loop	{
			glfw::poll_events();
			if window.should_close() || window.get_key(glfw::support::consts::KEY_ESCAPE)!=0	{
				window.destroy();
				break;
			}
			// render
			let input = {
				let (px,py) = window.get_cursor_pos();
				let mut buttons = 0u;
				for [0u, ..8u].each() |&i|	{
					buttons |= (window.get_mouse_button(i as i32) << i) as uint;
				}
				input::State{
					time	: engine::anim::get_time(),
					focus	: window.get_param(glfw::support::consts::VISIBLE) != 0,
					mouse	: input::Mouse{
						x	:px / (cw.width as float),
						y	:py / (cw.height as float),
						buttons	: buttons,
					},
				}
			};
			//TODO: update on a higher frequency than render
			if !game.update( &input )	{
				break
			}
			if !game.render( &config.elements )	{
				break;
			}
			window.swap_buffers();
		}
	}
}
