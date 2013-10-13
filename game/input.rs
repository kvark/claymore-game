use std;
use glfw;

pub type Key = glfw::Key;

pub enum Event	{
	EvFocus(bool),
	EvCharacter(char),
	EvKeyboard(Key,bool),
	EvMouseMove(float,float),
	EvMouseClick(uint,bool),
	EvScroll(float,float),
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
		}
	}
}


pub struct Mouse	{
	x	: float,
	y	: float,
	buttons	: uint,
}

pub struct State	{
	time	: float,
	focus	: bool,
	mouse	: Mouse,
}
