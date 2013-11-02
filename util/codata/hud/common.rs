use std;

pub type Path = ~str;
pub type Vector = [uint, ..2]; //x,y
pub type Kerning = [int, ..2];
pub type Color = uint;


pub enum Size	{
	SizeAbs(uint),
	SizeRel(f32),
}

pub enum Element	{
	ElSpace(Vector),
	ElBox(Size,Size,Box),
	ElImage(Path),
	ElText(Text),
}

pub struct Child( ~str, Element );

pub struct Screen	{
	alpha	: f32,
	root	: Box,
}

pub enum Align	{
	AlignHor,
	AlignVer,
}

pub enum Ground	{
	GroundNone,
	GroundSolid( Color ),
	GroundFrame( Color, f32 ),
	GroundImage( Path, [f32, ..2] ),
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


//====== Implementation ======//

impl Size	{
	pub fn apply( &self, size : uint )-> uint	{
		match self	{
			&SizeAbs(abs)	=> abs,
			&SizeRel(rel)	=> (rel * (size as f32)) as uint,
		}
	}
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