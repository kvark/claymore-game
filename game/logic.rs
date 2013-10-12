extern mod glfw;
extern mod engine;
extern mod gen_hud;

use engine::{gr_low,gr_mid,journal};

use hud;
use hud::debug;
use input;
use main;
use scene::{chared,intro};
use battle = battle::main;


enum Screen	{
	ScreenIntro,
	ScreenChar,
	ScreenBattle,
	ScreenWorld,
	ScreenDeath,
}


struct Logic	{
	screen		: Screen,
	s_intro		: intro::Scene,
	//s_editor	: chared::Scene,
	s_battle	: battle::Scene,
	output		: gr_mid::call::Output,
	technique	: gr_mid::draw::Technique,
}

impl Logic	{
	pub fn create( _el : &main::Elements, gcon : &mut gr_low::context::Context, fcon : &gr_mid::font::Context,
			hcon : &mut hud::main::Context, log : &journal::Log )-> Logic	{
		// output
		let pmap = gr_mid::call::PlaneMap::new_main( gcon, ~"o_Color" );
		let out = gr_mid::call::Output::new( gcon.default_frame_buffer, pmap );
		// create a forward light technique
		let tech = gr_mid::draw::load_technique( "data/code/tech/forward/light" );
		Logic	{
			screen		: ScreenBattle,
			s_intro		: intro::Scene{ active:false },
			//s_editor	: chared::create( el, gcon, fcon, log ),
			s_battle	: battle::create( gcon, hcon, fcon, log ),
			output		: out,
			technique	: tech,
		}
	}

	pub fn reset( &mut self )	{
		match self.screen	{
			ScreenBattle	=> self.s_battle.reset(),
			_ => ()
		}
	}

	pub fn update( &mut self, input : &input::State, _log : &journal::Log )-> bool	{
		let aspect = self.output.area.aspect();
		match self.screen	{
			//ScreenChar		=> self.s_editor.update( input, log ),
			ScreenBattle	=> self.s_battle.update( input, aspect ),
			_ => true
		}
	}

	fn on_debug_key( &mut self, key : glfw::Key, debug : &mut debug::Menu<Logic> )-> bool	{
		if !debug.is_active()	{
			return match key	{
				glfw::KeyM	=>	{
					debug.selection.push(0);
					true
				},
				_	=> false
			}
		}
		match key	{
			glfw::KeyM		=>
				debug.selection.clear(),
			glfw::KeyUp		=> {
				let last = debug.selection.mut_iter().last().
					expect("Debug menu: nothing is selected");
				if *last>0	{
					*last -= 1;
				}
			},
			glfw::KeyDown	=> {
				let menu_len =	{
					let (_,ref last_list) = debug.selection_list_iter().last().
						expect("Debug menu: no list found");
					last_list.len()
				};
				let last = debug.selection.mut_iter().last().
					expect("Debug menu: nothing is selected");
				if ((*last+1) as uint) < menu_len	{
					*last += 1;
				}
			},
			glfw::KeyEnter	=> 	{
				let extend = match debug.get_selected_item().action	{
					debug::ActionFun(ref fun)	=> { (*fun)(self); false }
					debug::ActionList(ref list) if !list.is_empty()	=> true,
					_	=> false,	//beep
				};
				if extend	{
					debug.selection.push(0);
				}
			},
			_	=> {return false}
		}
		true
	}

	pub fn on_input( &mut self, event : &input::Event, debug : &mut debug::Menu<Logic> )	{
		match self.screen	{
			//ScreenChar		=> self.s_editor.on_input( event ),
			ScreenBattle	=> self.s_battle.on_input( event ),
			_	=> ()
		}
		match event	{
			&input::Keyboard(key,press) if press	=>
				self.on_debug_key( key, debug ),
			_	=> false
		};
	}

	pub fn render( &mut self, _el : &main::Elements, gcon : &mut gr_low::context::Context,
			hcon : &hud::main::Context, debug : &debug::Menu<Logic>, log : &journal::Log )	{
		match self.screen	{
			ScreenIntro	=> (),
			//ScreenChar	=> self.s_editor.render( el, &self.output, gcon, log ),
			ScreenBattle	=> self.s_battle.render( &self.output, &self.technique, gcon, hcon, log ),
			_ => ()
		}
		// debug menu
		log.add("=== Debug Menu ===");
		let debug_hud = debug.build( 0.5 );
		let debug_calls = hcon.draw_all( &debug_hud, &self.output );
		gcon.flush( debug_calls, log );
	}
	
	pub fn debug_move( &mut self, rot : bool, x : int, y : int )	{
		self.s_battle.debug_move( rot, x, y );
	}

	pub fn create_debug_menu( &self )-> debug::Menu<Logic>	{
		debug::Menu	{
			root	: debug::MenuItem	{
				name	: ~"root",
				action	: debug::ActionList(~[
					debug::MenuItem	{
						name	: ~"battle-reset",
						action	: debug::ActionFun(|c:&mut Logic| {c.s_battle.reset()}),
					},
					debug::MenuItem	{
						name	: ~"test-2",
						action	: debug::ActionFun(|_| {}),
					},
				]),
			},
			selection	: ~[],
			font	: gen_hud::common::Font	{
				path	: ~"Vera.ttf",
				size	: [10,10],
				kern	: [0,-10],
			},
		}
	}
}
