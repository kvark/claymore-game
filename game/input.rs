pub enum Event	{
	Focus(bool),
	Character(char),
	Keyboard(int,bool),
	MouseMove(float,float),
	MouseClick(uint,bool),
	Scroll(float,float),
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
	keys	: ~[int],
}