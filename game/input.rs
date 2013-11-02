use std;
use glfw;
use engine;

pub type Key = glfw::Key;

pub enum Event	{
	EvFocus(bool),
	EvCharacter(char),
	EvKeyboard(Key,bool),
	EvMouseMove(f32,f32),
	EvMouseClick(uint,bool),
	EvScroll(f32,f32),
	EvRender(uint),
}

impl std::to_str::ToStr for Event	{
	fn to_str( &self )-> ~str	{
		fn b2c( b:bool )->char	{if b {'+'} else {'-'}}
		match self	{
			&EvFocus(on)		=> format!( "focus({:c})", b2c(on) ),
			&EvCharacter(c)		=> format!( "char({:c})", c ),
			&EvKeyboard(k,p)	=> format!( "keyboard({:c},{:s})", b2c(p), k.to_str() ),
			&EvMouseMove(x,y)	=> format!( "mouse_move({:f},{:f})", x, y ),
			&EvMouseClick(k,p)	=> format!( "mouse_hit({:c},{:u})", b2c(p), k ),
			&EvScroll(x,y)		=> format!( "scroll({:f},{:f})", x, y ),
			&EvRender(id)		=> format!( "render({:u})", id ),
		}
	}
}


pub struct State	{
	time_game	: engine::anim::float,
	time_view	: engine::anim::float,
	focus	: bool,
	aspect	: f32,
	mouse	: [f32,..2],
	log		: engine::journal::Log,
}
