use std;
use glfw;
use engine;

pub type Key = glfw::Key;

pub enum Event	{
	EvFocus(bool),
	EvCharacter(char),
	EvKeyboard(Key,bool),
	EvMouseMove(float,float),
	EvMouseClick(uint,bool),
	EvScroll(float,float),
	EvRender(uint),
}

impl std::to_str::ToStr for Event	{
	fn to_str( &self )-> ~str	{
		fn b2c( b:bool )->char	{if b {'+'} else {'-'}}
		match self	{
			&EvFocus(on)		=> fmt!( "focus(%c)", b2c(on) ),
			&EvCharacter(c)		=> fmt!( "char(%c)", c ),
			&EvKeyboard(k,p)	=> fmt!( "keyboard(%c,%s)", b2c(p), k.to_str() ),
			&EvMouseMove(x,y)	=> fmt!( "mouse_move(%f,%f)", x, y ),
			&EvMouseClick(k,p)	=> fmt!( "mouse_hit(%c,%u)", b2c(p), k ),
			&EvScroll(x,y)		=> fmt!( "scroll(%f,%f)", x, y ),
			&EvRender(id)		=> fmt!( "render(%u)", id ),
		}
	}
}


pub struct State	{
	time_game	: float,
	time_view	: float,
	focus	: bool,
	aspect	: float,
	mouse	: [float,..2],
	log		: engine::journal::Log,
}
