extern mod lmath;

use lmath::vec::vec3::*;
use lmath::vec::vec4::*;
use lmath::quat::*;
use lmath::mat::mat4::*;

pub type Matrix = Mat4<f32>;
pub type Vector = Vec3<f32>;
pub type Quaternion = Quat<f32>;
pub type Scale = f32;


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Quaternion Space								//

pub trait Space	{
	pure fn transform( point : &Vector )-> Vector;
	pure fn rotate( vector : &Vector )-> Vector;
	pure fn mul( other : &self )-> self;
	pure fn invert()-> self;
	pure fn to_matrix()-> Matrix;
}


impl Matrix : Space	{
	pure fn transform( point : &Vector )-> Vector	{
		let v4 = Vec4::new( point.x, point.y, point.z, 1f32 );
		let vt = self.mul_v(&v4);
		Vec3::new( vt.x/vt.w, vt.y/vt.w, vt.z/vt.w )
	}
	pure fn rotate( vector : &Vector )-> Vector	{
		let v4 = Vec4::new( vector.x, vector.y, vector.z, 0f32 );
		let vt = self.mul_v(&v4);
		Vec3::new( vt.x, vt.y, vt.z ).normalize()
	}
	pure fn mul( other : &Matrix )-> Matrix	{
		self.mul_m(other)
	}
	pure fn invert()-> Matrix	{
		self.inverse().expect( ~"Unable to invert matrix" )
	}
	pure fn to_matrix()-> Matrix	{self}
}


pub struct QuatSpace	{
	position	: Vector,
	orientation	: Quaternion,
	scale		: Scale,
}

impl QuatSpace : Space	{
	pure fn transform( point : &Vector )-> Vector	{
		self.orientation.mul_v( point ).mul_t( self.scale ).add_v( &self.position )
	}
	pure fn rotate( vector : &Vector )-> Vector	{
		self.orientation.mul_v( vector )
	}
	pure fn mul( other : &QuatSpace )-> QuatSpace	{
		QuatSpace{
			position	: self.transform( &other.position ),
			orientation	: self.orientation.mul_q( &other.orientation ),
			scale		: self.scale * other.scale
		}
	}
	pure fn invert()-> QuatSpace	{
		let q = self.orientation.conjugate();
		let s = 1f32 / self.scale;
		let p = q.mul_v(&self.position).mul_t(-s);
		QuatSpace{ position:p, orientation:q, scale:s }
	}
	pure fn to_matrix()-> Matrix	{
		//FIXME: remove transpose
		let m3 = self.orientation.to_mat3().transpose();
		let mut m4 = m3.mul_t(self.scale).to_mat4();
		m4.w.x = self.position.x;
		m4.w.y = self.position.y;
		m4.w.z = self.position.z;
		m4
	}
}

impl QuatSpace	{
	pure fn get_pos_scale()-> lmath::gltypes::vec4	{
		Vec4::new( self.position.x, self.position.y, self.position.z, self.scale )
	}
	pure fn get_orientation()-> lmath::gltypes::vec4	{
		Vec4::new(
			self.orientation.v.x, self.orientation.v.y,
			self.orientation.v.z, self.orientation.s )	
	}
}

pub pure fn identity()-> QuatSpace	{
	QuatSpace{
		position	: Vec3::new(0f32,0f32,0f32),
		orientation	: Quat::identity::<f32>(),
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
		self*t1 + (*other)*t2
	}
}

impl QuatSpace : Interpolate	{
	pure fn interpolate( other : &QuatSpace, t : float )-> QuatSpace	{
		QuatSpace{
			position	: self.position.interpolate( &other.position, t ),
			orientation	: self.orientation.interpolate( &other.orientation, t ),
			scale		: self.scale.interpolate( &other.scale, t ),
		}
	}
}


pub trait Pretty	{
	pure fn to_string()-> ~str;
}

impl Vec3<f32> : Pretty	{
	pure fn to_string()-> ~str	{
		fmt!( "(%f,%f,%f)", self.x as float, self.y as float, self.z as float )
	}
}
impl Vec4<f32> : Pretty	{
	pure fn to_string()-> ~str	{
		fmt!( "(%f,%f,%f,%f)", self.x as float, self.y as float, self.z as float, self.w as float )
	}	
}
impl Quat<f32> : Pretty	{
	pure fn to_string()-> ~str	{
		fmt!( "(%f,%f,%f,%f)", self.s as float, self.v.x as float, self.v.y as float, self.v.z as float )
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
	fn mut_space(&self)-> &self/mut QuatSpace	{
		&mut self.space
	}
	pure fn is_under( name : &~str )-> bool	{
		self.name == *name || match self.parent	{
			Some(p) => p.is_under(name),
			None	=> false,
		}
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
	root	: @Node,
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
		let mut pos = vec::with_capacity::<lmath::gltypes::vec4>( self.max_bones );
		let mut rot = vec::with_capacity::<lmath::gltypes::vec4>( self.max_bones );
		let parent_inv = self.root.world_space().invert();
		while pos.len() < self.max_bones	{
			let space = if pos.len()>0u && pos.len()<self.bones.len()	{
					let b = &self.bones[pos.len()-1u];
					parent_inv * b.node.world_space() * b.bind_pose_inv
				}else {id};
			pos.push( space.get_pos_scale() );
			rot.push( space.get_orientation() );
		}
		data.insert( ~"bone_pos[0]", shade::UniFloatVecArray(pos) );
		data.insert( ~"bone_rot[0]", shade::UniFloatVecArray(rot) );
	}
}


pure fn is_same_node( a: Option<@Node>, b : Option<@Node> )-> bool	{
	match (a,b)	{
		(Some(na),Some(nb))	=> managed::ptr_eq(na,nb),
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