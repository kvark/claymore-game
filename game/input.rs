use std;
use glfw;

pub enum Event	{
	Focus(bool),
	Character(char),
	Keyboard(glfw::Key,bool),
	MouseMove(float,float),
	MouseClick(uint,bool),
	Scroll(float,float),
}

impl std::to_str::ToStr for Event	{
	fn to_str( &self )-> ~str	{
		fn b2c( b:bool )->char	{if b {'+'} else {'-'}}
		match self	{
			&Focus(on)		=> fmt!( "focus(%c)", b2c(on) ),
			&Character(c)	=> fmt!( "char(%c)", c ),
			&Keyboard(k,p)	=> fmt!( "keyboard(%c,%s)", b2c(p), k.to_str() ),
			&MouseMove(x,y)	=> fmt!( "mouse_move(%f,%f)", x, y ),
			&MouseClick(k,p)=> fmt!( "mouse_hit(%c,%u)", b2c(p), k ),
			&Scroll(x,y)	=> fmt!( "scroll(%f,%f)", x, y ),
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

