extern mod lmath;
pub type Matrix = lmath::matrix::mat4;
pub type Vector = lmath::vector::vec3;


pub trait Pretty	{
	pure fn to_string()-> ~str;
}

pub trait Space	{
	pure fn transform( value : &Vector )-> Vector;
	pure fn mul( other : &self )-> self;
	pure fn inverse()-> self;
	pure fn to_matrix()-> Matrix;
}


impl Matrix : Space	{
	pure fn transform( value : &Vector )-> Vector	{
		let v4 = lmath::vector::Vec4::new( value.x, value.y, value.z, 1f32 );
		let vt = self.mul_v(&v4);
		lmath::vector::Vec3::new( vt.x/vt.w, vt.y/vt.w, vt.z/vt.w )
	}
	pure fn mul( other : &Matrix )-> Matrix	{
		self.mul_m(other)
	}
	pure fn inverse()-> Matrix	{
		match self.invert()	{
			Some(m)	=> m,
			None => fail(~"Unable to invert matrix")
		}
	}
	pure fn to_matrix()-> Matrix	{self}
}


impl Vector : Pretty	{
	pure fn to_string()-> ~str	{
		fmt!( "(%f,%f,%f)", self.x as float, self.y as float, self.z as float )
	}
}
impl lmath::vector::vec4 : Pretty	{
	pure fn to_string()-> ~str	{
		fmt!( "(%f,%f,%f,%f)", self.x as float, self.y as float, self.z as float, self.w as float )
	}	
}

impl Matrix : Pretty	{
	pure fn to_string()-> ~str	{
		fmt!("/%s\\\n|%s|\n|%s|\n\\%s/",
			self.row(0).to_string(),
			self.row(1).to_string(),
			self.row(2).to_string(),
			self.row(3).to_string())
	}
}


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
	pure fn inverse()-> QuatSpace	{
		let q = self.orientation.inverse();
		let s = 1f32 / self.scale;
		let p = q.mul_v(&self.position).mul_t(-s);
		QuatSpace{ position:p, orientation:q, scale:s }
	}
	pure fn to_matrix()-> Matrix	{
		let m3 = self.orientation.to_Mat3();
		let mut m4 = m3.mul_t(self.scale).to_Mat4();
		m4.w.x = self.position.x;
		m4.w.y = self.position.y;
		m4.w.z = self.position.z;
		m4
	}
}


pub struct Node	{
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