extern mod lmath;
pub type Matrix = lmath::matrix::mat4;
pub type Vector = lmath::vector::vec3;


pub trait Space	{
	pure fn transform( value : &Vector )-> Vector;
	pure fn mul( other : &self )-> self;
	pure fn invert()-> self;
	pure fn to_matrix()-> Matrix;
}

//FIXME: waiting for Brendan
/*impl Matrix : Space	{
	pure fn mul( other : &Matrix )-> Matrix	{ self.mul_m(other) }
	pure fn invert()-> Matrix	{
		match self.invert()	{
			Some(m)	=> m,
			None => fail("Unable to invert matrix")
		}
	}
	pure fn to_matrix()-> Matrix	{self}
}*/

pub struct QuatSpace	{
	position	: Vector,
	orientation	: lmath::quaternion::quat4,
	scale		: f32,
}

impl QuatSpace : Space	{
	pure fn transform( value : &Vector )-> Vector	{
		self.orientation.mul_v( value ).mul_t( self.scale ).add_v( &self.position )
	}
	pure fn mul( other : &QuatSpace )-> QuatSpace	{
		QuatSpace{
			position	: self.transform( &other.position ),
			orientation	: self.orientation.mul_q( &other.orientation ),
			scale		: self.scale * other.scale
		}
	}
	pure fn invert()-> QuatSpace	{
		let q = self.orientation.inverse();
		let s = 1f32 / self.scale;
		let p = q.mul_v(&self.position).mul_t(-s);
		QuatSpace{ position:p, orientation:q, scale:s }
	}
	pure fn to_matrix()-> Matrix	{
		let mut m = self.orientation.to_Mat4();
		m.x.x *= self.scale;	m.x.w = self.position.x;
		m.y.y *= self.scale;	m.y.w = self.position.y;
		m.z.z *= self.scale;	m.z.w = self.position.z;
		m
	}
}


struct Node	{
	name		: ~str,
	mut space	: QuatSpace,	//FIXME: arbitrary space
	parent		: Option<@Node>,
}

impl Node	{
	pure fn world_space() -> QuatSpace	{
		match self.parent	{
			Some(p)	=> p.world_space().mul( &self.space ),
			None	=> self.space
		}
	}
}