extern mod lmath;
pub type Matrix = lmath::matrix::mat4;
pub type Vector = lmath::vector::vec3;
pub type Quaternion = lmath::quaternion::quat4;
pub type Scale = f32;


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Quaternion Space								//

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


pub struct QuatSpace	{
	position	: Vector,
	orientation	: Quaternion,
	scale		: Scale,
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
		let q = self.orientation.conjugate();
		let s = 1f32 / self.scale;
		let p = q.mul_v(&self.position).mul_t(-s);
		QuatSpace{ position:p, orientation:q, scale:s }
	}
	pure fn to_matrix()-> Matrix	{
		//FIXME: remove transpose
		let m3 = self.orientation.to_Mat3().transpose();
		let mut m4 = m3.mul_t(self.scale).to_Mat4();
		m4.w.x = self.position.x;
		m4.w.y = self.position.y;
		m4.w.z = self.position.z;
		m4
	}
}

pub pure fn identity()-> QuatSpace	{
	QuatSpace{
		position	: lmath::vector::Vec3::new(0f32,0f32,0f32),
		orientation	: lmath::quaternion::Quat::new(1f32,0f32,0f32,0f32),
		scale		: 1f32,
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Interpolation & Pretty							//

pub trait Interpolate	{
	pure fn interpolate( other : &self, t : float )-> self;
}

impl Vector : Interpolate	{
	pure fn interpolate( other : &Vector, t : float )-> Vector	{
		let t1  = (1f-t) as f32, t2 = t as f32;
		self.mul_t(t1).add_v( &other.mul_t(t2) )
	}
}

impl Quaternion : Interpolate	{
	//FIXME: use slerp
	pure fn interpolate( other : &Quaternion, t : float )-> Quaternion	{
		let t1  = (1f-t) as f32, t2 = t as f32;
		self.mul_t(t1).add_q( &other.mul_t(t2) )
	}
}

impl Scale : Interpolate	{
	pure fn interpolate( other : &Scale, t : float )-> Scale	{
		let t1  = (1f-t) as f32, t2 = t as f32;
		self*t1 + other*t2
	}
}


pub trait Pretty	{
	pure fn to_string()-> ~str;
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
impl Quaternion : Pretty	{
	pure fn to_string()-> ~str	{
		fmt!( "(%f,%f,%f,%f)", self.x as float, self.y as float, self.z as float, self.w as float )
	}
}
impl QuatSpace : Pretty	{
	pure fn to_string()-> ~str	{
		fmt!( "{pos:%s,rot:%s,scale:%f}", self.position.to_string(),
			self.orientation.to_string(), self.scale as float )
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


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Node											//

enum NodeCurve	{
	NCuPos(		anim::Curve<Vector>		),
	NCuRotQuat(	anim::Curve<Quaternion>	),
	//NCuRotEuler(anim::Curve<lmath::vector::vec3>		),
	NCuScale(	anim::Curve<Scale>						),
}

type NodeRecord = anim::Record<NodeCurve>;

pub struct Node	{
	name		: ~str,
	mut space	: QuatSpace,	//FIXME: arbitrary space
	parent		: Option<@Node>,
	actions		: ~[@NodeRecord],	//FIXME mutable
}

impl Node	{
	pure fn world_space() -> QuatSpace	{
		match self.parent	{
			Some(p)	=> p.world_space() * self.space,
			None	=> self.space
		}
	}
	fn set_space( s : &QuatSpace )	{
		self.space = *s;
	}
}

impl Node : anim::Player<NodeCurve>	{
	pure fn find_record( name : ~str )-> Option<@NodeRecord>	{
		do self.actions.find() |a| {a.name==name}
	}
	fn set_record( a : &NodeRecord, time : float )	{
		for a.curves.each() |chan|	{
			match chan	{
				&NCuPos(c)		=> self.space.position		= c.sample(time),
				&NCuRotQuat(c)	=> self.space.orientation	= c.sample(time),
				&NCuScale(c)	=> self.space.scale			= c.sample(time),
			}
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Armature										//

pub struct Bone	{
	node			: @Node,
	bind_space		: QuatSpace,
	bind_pose_inv	: QuatSpace,
	mut transform	: QuatSpace,
	parent_id		: Option<uint>,
}

impl Bone	{
	fn reset()	{
		self.node.space = self.bind_space;
		self.transform = identity();
	}
}

pub enum ArmatureCurve	{
	ACuPos(		uint, anim::Curve<Vector>		),
	ACuRotQuat(	uint, anim::Curve<Quaternion>	),
	ACuScale(	uint, anim::Curve<Scale>						),
}

pub type ArmatureRecord = anim::Record<ArmatureCurve>;


pub struct Armature	{
	bones	: ~[Bone],
	code	: ~str,
	actions	: ~[@ArmatureRecord],	//FIXME mutable
	max_bones	: uint,
}


impl Armature : draw::Mod	{
	pure fn get_name()-> ~str	{ ~"Arm" }
	pure fn get_code()-> ~str	{ copy self.code }
	//FIXME: make pure when Rust allows
	//TODO: use float arrays
	fn fill_data( data : &mut shade::DataMap )	{
		assert self.bones.len() < self.max_bones;
		let id = identity();
		let mut pos = vec::with_capacity::<lmath::vector::vec4>( self.max_bones );
		let mut rot = vec::with_capacity::<lmath::vector::vec4>( self.max_bones );
		while pos.len() < self.max_bones	{
			let space = if pos.len()>0u && pos.len()<self.bones.len()	{
					let b = &self.bones[pos.len()-1u];
					b.node.world_space() * b.bind_pose_inv
				}else {id};
			pos.push( lmath::vector::Vec4::new(
				space.position.x, space.position.y, space.position.z,
				space.scale ));
			rot.push( lmath::vector::Vec4::new(
				space.orientation.x, space.orientation.y,
				space.orientation.z, space.orientation.w
				));
		}
		data.insert( ~"bone_pos[0]", shade::UniFloatVecArray(pos) );
		data.insert( ~"bone_rot[0]", shade::UniFloatVecArray(rot) );
	}
}


pure fn is_same_node( a: Option<@Node>, b : Option<@Node> )-> bool	{
	match (a,b)	{
		(Some(na),Some(nb))	=> box::ptr_eq(na,nb),
		(None,None)	=> true,
		(_,_)		=> false,
	}
}

impl Armature : anim::Player<ArmatureCurve>	{
	pure fn find_record( name : ~str )-> Option<@ArmatureRecord>	{
		do self.actions.find() |a| {a.name==name}
	}
	fn set_record( a : &ArmatureRecord, time : float )	{
		for a.curves.each() |chan|	{
			match chan	{
				&ACuPos(bi,c)		=> { let b = &self.bones[bi];
					b.node.space.position	= b.bind_space.transform( &c.sample(time) );
				},
				&ACuRotQuat(bi,c)	=> { let b = &self.bones[bi];
					b.node.space.orientation= b.bind_space.orientation.mul_q( &c.sample(time) );
				},
				&ACuScale(bi,c)		=> { let b = &self.bones[bi];
					b.node.space.scale		= b.bind_space.scale * c.sample(time);
				}
			}
		}
	}
}