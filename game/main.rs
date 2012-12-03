extern mod glfw3;
extern mod lmath;
extern mod engine;

extern mod std;
use std::json;


enum Screen	{
	ScreenEntry,
	ScreenBattle,
	ScreenWorld,
	ScreenDeath,
}

struct Entry	{
	ok	: bool
}

fn make_entry( _ct : &engine::context::Context, _aspect : float )-> Entry	{
	let _info = scene::load_config::<scene::SceneInfo>( ~"data/object/scene.json" );
	Entry{ok:true}
}


struct Game	{
	context		: engine::context::Context,
	audio		: engine::audio::Context,
	mut frames	: uint,
	technique	: engine::draw::Technique,
	entry		: Entry,
	battle		: battle::Scene,
	mut screen	: Screen,
}

impl Game	{
	fn update( nx : float, ny : float, mouse_hit : bool, cam_dir : int )-> bool	{
		match self.screen	{
			ScreenEntry => {
				true
			},
			ScreenBattle => {
				self.battle.update( &self.context.texture, nx, ny, mouse_hit, cam_dir )
			},
			_ => true
		}
	}
	fn render()-> bool	{
		match self.screen	{
			ScreenEntry => {
				// clear screen
				let c0 = self.technique.gen_clear(
					engine::call::ClearData{
						color	:Some( engine::rast::make_color(0x8080FFFF) ),
						depth	:Some( 1f ),
						stencil	:Some( 0u ),
					}
				);
				self.context.flush(~[c0]);
			},
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
				self.battle.render( &self.context, &self.technique );
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


fn create_game( wid : uint, het : uint )-> Game	{
	let ct = engine::context::create( wid, het );
	assert ct.sync_back();
	let ac = engine::audio::create_context();
	// create a forward light technique
	let tech = {
		let pmap = engine::call::create_plane_map( ~"o_Color", engine::frame::TarEmpty );
		let mut rast = engine::rast::create_rast(0,0);
		rast.set_depth(~"<=",true);
		rast.prime.cull = true;
		let cache = @mut engine::draw::create_cache();
		engine::draw::load_technique( ~"data/code/tech/forward/light",
			ct.default_frame_buffer, &pmap, &rast, cache)
	};
	// done
	ct.check(~"init");
	let aspect = (wid as float) / (het as float);
	Game{ context:ct, audio:ac,
		frames:0u, technique:tech,
		entry:make_entry( &ct, aspect ),
		battle:battle::make_battle( &ct, aspect ),
		screen:ScreenEntry,
	}
}


fn fail_GLFW( where: &static/str ) -> !	{
	let code = glfw3::get_error();
	io::println(~"GLFW error: " + glfw3::error_string(code));
	glfw3::terminate();
	fail fmt!("glfw%s() failed\n",where)
}


#[auto_deserialize]
struct ConfigWindow	{
	title:~str, width:uint, height:uint, samples:uint, fullscreen:bool,
}
#[auto_deserialize]
struct ConfigGL	{
	major:uint, minor:uint, debug:bool,
}
#[auto_deserialize]
struct Config	{
	window	: ConfigWindow,
	GL		: ConfigGL,
}


fn main()	{
	io::println("--- Claymore ---");
	do task::task().sched_mode(task::PlatformThread).spawn {
		if (glfw3::init()==0)	{
			fail_GLFW("Init");
		}

		let config = scene::load_config::<Config>(~"data/config.json");

		glfw3::window_hint( glfw3::WINDOW_RESIZABLE, 0 );
		glfw3::window_hint( glfw3::OPENGL_DEBUG_CONTEXT, if config.GL.debug {1} else {0} );
		glfw3::window_hint( glfw3::OPENGL_VERSION_MAJOR, config.GL.major as libc::c_int );
		glfw3::window_hint( glfw3::OPENGL_VERSION_MINOR, config.GL.minor as libc::c_int );
		glfw3::window_hint( glfw3::OPENGL_PROFILE, glfw3::OPENGL_CORE_PROFILE );
		glfw3::window_hint( glfw3::OPENGL_FORWARD_COMPAT, 1 );

		let mut window = glfw3::create_window( config.window.width, config.window.height,
			if config.window.fullscreen {glfw3::FULLSCREEN} else {glfw3::WINDOWED},
			config.window.title );
		if (ptr::is_null(window.ptr))	{
			fail_GLFW("OpenWindow");
		}
	
		window.make_context_current();
		let game = create_game( config.window.width, config.window.height );
		
		loop	{
			glfw3::poll_events();
			let isClosed = window.get_param(glfw3::CLOSE_REQUESTED)!=0;
			if window.get_key(glfw3::KEY_ESC)!=0 || isClosed	{
				glfw3::destroy_window(&mut window);
				break;
			}
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
			let cam_dir = (window.get_key(glfw3::KEY_E) - window.get_key(glfw3::KEY_Q)) as int;
			// render
			let (cx,cy) = window.get_cursor_pos();
			let nx = (cx as float)/(config.window.width as float);
			let ny = (cy as float)/(config.window.height as float);
			if !game.update(nx,ny,mouse_hit,cam_dir)	{
				break
			}
			if !game.render()	{
				break;
			}
			window.swap_buffers();
		}
	
		glfw3::terminate();
	}
}
