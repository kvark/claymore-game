use core;	//for IterBytes

pub type Path = ~str;
pub type Vector = [uint, ..2]; //x,y
pub type Kerning = [int, ..2];
pub type Color = uint;

pub enum Relation	{
	RelParent,
	RelHead,
	RelTail,
}

pub type Anchor = [i8, ..2]; //-1,0,+1 for x,y
pub type Alignment = (Anchor,Relation,Anchor); //this, rel, other

pub enum Element	{
	ElSpace(Vector),
	ElFrame(Frame),
	ElImage(Path),
	ElText(Text),
}

pub struct Child	{
	name	: ~str,
	align	: Alignment,
	element	: Element,
}

pub struct Screen	{
	alpha	: float,
	children: ~[Child],
}

pub struct Ground	{
	path	: Path,
	center	: [float, ..2],
}

pub struct Frame	{
	margin	: [Vector, ..2],
	ground	: Option<Ground>,
	children: ~[Child],
}

//#[deriving(Eq)]
#[deriving(IterBytes)]
pub struct Font	{
	path	: ~str,
	size	: Vector,
	kern	: Kerning,
}

pub struct Text	{
	value	: ~str,
	font	: Font,
	color	: Color,
	bound	: Vector,
	edit	: bool,
}

impl core::cmp::Eq for Font	{
	fn eq( &self, other : &Font )-> bool	{
		self.path == other.path && self.size == other.size && self.kern == other.kern
	}
	fn ne( &self, other : &Font )-> bool	{
		!self.eq( other )
	}
}