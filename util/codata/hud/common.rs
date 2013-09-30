use std;

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