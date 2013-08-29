extern mod glfw;
extern mod lmath;
extern mod engine;
extern mod std;

use engine::{gr_low,gr_mid};
use engine::gr_low::context::ProxyState;

use input;
use scene;


enum Screen	{
	ScreenIntro,
	ScreenChar,
	ScreenBattle,
	ScreenWorld,
	ScreenDeath,
}


struct Game	{
	context		: gr_low::context::Context,
	audio		: engine::audio::Context,
	journal		: engine::journal::Log,
	sound_source: @mut engine::audio::Source,
	frames		: uint,
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
	fn create( el : &Elements, wid : uint, het : uint, lg : engine::journal::Log  )-> Game	{
		let mut ct = gr_low::context::create( wid, het );
		assert!( ct.sync_back() );
		// audio test
		let ac = engine::audio::Context::create();
		let buf = @engine::audio::load_wav( &ac, ~"data/sound/stereol.wav", &lg );
		let src = @mut ac.create_source();
		src.bind(buf);
		//src.play();
		// create a forward light technique
		let tech = gr_mid::draw::load_technique( ~"data/code/tech/forward/light" );
		let pmap = gr_mid::call::PlaneMap::new_simple( ~"o_Color", gr_low::frame::TarEmpty );
		let out = gr_mid::call::Output::new( ct.default_frame_buffer, pmap );
		// done
		ct.check(~"init");
		let intro = scene::intro::Scene{ active:false };
		let editor = scene::chared::make_scene( el, &mut ct, &lg );
		let battle = scene::battle::make_scene( &mut ct, &lg );
		Game{ context:ct, audio:ac, journal:lg,
			sound_source:src, frames:0u,
			technique:tech, output:out,
			s_intro:intro, s_editor:editor, s_battle:battle,
			screen:ScreenChar,
		}
	}

	fn update( &mut self, input : &input::State )-> bool	{
		let aspect = self.output.area.aspect();
		match self.screen	{
			ScreenChar		=> self.s_editor.update( input, &self.journal ),
			ScreenBattle	=> self.s_battle.update( input, &mut self.context.texture, aspect ),
			_ => true
		}
	}

	fn on_input( &mut self, event : input::Event )	{
		match self.screen	{
			ScreenChar	=> self.s_editor.on_input( &event ),
			_	=> ()
		}
	}

	fn render( &mut self, el : &Elements )-> bool	{
		match self.screen	{
			ScreenIntro	=> (),
			ScreenChar	=> self.s_editor.render( el, &mut self.context, &self.journal ),
			ScreenBattle	=> {
				// clear screen
				let cd = gr_mid::call::ClearData{
					color	:Some( gr_low::rast::Color::new(0x8080FFFF) ),
					depth	:Some( 1f ),
					stencil	:Some( 0u ),
				};
				let c0 = gr_mid::call::CallClear( copy self.output, cd, copy self.context.default_rast.mask );
				self.context.flush(~[c0]);
				// draw battle
				self.s_battle.render( &mut self.context, &self.technique, copy self.output, &self.journal );
			},
			_ => ()
		}
		// done
		self.frames += 1;
		self.context.check( ~"render" );
		true
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
		let config = scene::common::load_config::<Config>( ~"data/config.json" );
		let lg = engine::journal::Log::create( copy config.journal.path, config.journal.depth );
		lg.add(~"--- Claymore ---");

		glfw::ml::window_hint( glfw::RESIZABLE, 0 );
		glfw::ml::window_hint( glfw::OPENGL_DEBUG_CONTEXT, if config.GL.debug {1} else {0} );
		glfw::ml::window_hint( glfw::CONTEXT_VERSION_MAJOR, config.GL.major as libc::c_int );
		glfw::ml::window_hint( glfw::CONTEXT_VERSION_MINOR, config.GL.minor as libc::c_int );
		if config.GL.core	{
			glfw::ml::window_hint( glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE );
			glfw::ml::window_hint( glfw::OPENGL_FORWARD_COMPAT, 1 );
		}

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
				let (cx,cy) = window.get_cursor_pos();
				let mut buttons = 0u;
				for [0u, ..8u].each() |&i|	{
					buttons |= (window.get_mouse_button(i as i32) << i) as uint;
				}
				input::State{
					time	: engine::anim::get_time(),
					focus	: true,	//FIXME
					mouse	: input::Mouse{
						x	:cx/(config.window.width as float),
						y	:cy/(config.window.height as float),
						buttons	: buttons,
					},
					keys	: ~[],	//FIXME
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
