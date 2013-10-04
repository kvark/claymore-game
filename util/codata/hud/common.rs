use std;

pub type Path = ~str;
pub type Vector = [uint, ..2]; //x,y
pub type Kerning = [int, ..2];
pub type Color = uint;


pub enum Element	{
	ElSpace(Vector),
	ElBox(Box),
	ElImage(Path),
	ElText(Text),
}

pub struct Child( ~str, Element );

pub struct Screen	{
	alpha	: float,
	root	: Box,
}

pub enum Align	{
	AlignHor,
	AlignVer,
}

pub enum Ground	{
	GroundNone,
	GroundSolid( Color ),
	GroundFrame( Color ),
	GroundImage( Path, [float, ..2] ),
}

pub struct Box	{
	align	: Align,
	ground	: Ground,
	children: ~[Child],
}

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


impl Eq for Font	{
	fn eq( &self, other : &Font )-> bool	{
		self.path == other.path &&
		self.size[0] == other.size[0] && self.size[1] == other.size[1] &&
		self.kern[0] == other.kern[0] && self.kern[1] == other.kern[1]
	}
}

impl IterBytes for Font	{
	fn iter_bytes( &self, lsb0 : bool, f : std::to_bytes::Cb )-> bool	{
		self.path.iter_bytes( lsb0, |x| f(x) ) &&
		self.size.iter_bytes( lsb0, |x| f(x) ) &&
		self.kern.iter_bytes( lsb0, |x| f(x) )
	}
}

impl Clone for Font	{
	fn clone( &self )-> Font	{
		Font	{
			path	: self.path.clone(),
			size	: [self.size[0],self.size[1]],
			kern	: [self.kern[0],self.kern[1]],
		}
	}
}