extern mod cgmath;

use std;
use std::managed;

use cgmath::angle;
use cgmath::matrix::*;
use cgmath::quaternion::*;
use cgmath::transform::*;
use cgmath::vector::*;
use gr_low::shade;
use gr_mid::draw;

use anim;


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Quaternion Space								//

pub type Matrix = Mat4<f32>;
pub type Vector = Vec3<f32>;
pub type Scale = f32;
pub type Quaternion = Quat<f32>;
pub type Euler = Vec3<angle::Rad<f32>>;
pub type Space = Decomposed<Scale,Vector,Quaternion>;


pub fn make(scale: Scale, rot: Quaternion, disp: Vector)-> Space	{
	Decomposed{ scale: scale, rot: rot, disp: disp }
}
pub fn get_params(sp: &Space)-> (Vec4<f32>,Vec4<f32>)	{
	(sp.disp.extend( sp.scale ),
	sp.rot.v.extend( sp.rot.s ))
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Interpolation									//

pub trait Interpolate	{
	fn interpolate( &self, &Self, f32 )-> Self;
}

impl Interpolate for Vector	{
	fn interpolate( &self, other : &Vector, t : f32 )-> Vector	{
		self.mul_s(1.0-t).add_v( &other.mul_s(t) )
	}
}

impl Interpolate for Quaternion	{
	//FIXME: use slerp
	fn interpolate( &self, other : &Quaternion, t : f32 )-> Quaternion	{
		self.mul_s(1.0-t).add_q( &other.mul_s(t) )
	}
}

impl Interpolate for Scale	{
	fn interpolate( &self, other : &Scale, t : f32 )-> Scale	{
		self*(1.0-t) + (*other)*t
	}
}

impl Interpolate for Space	{
	fn interpolate( &self, other : &Space, t : f32 )-> Space	{
		Decomposed{
			scale	: self.scale.interpolate( &other.scale, t ),
			rot		: self.rot.interpolate( &other.rot, t ),
			disp	: self.disp.interpolate( &other.disp, t ),
		}
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Node											//

enum NodeCurve	{
	NCuPos(		~anim::Curve<Vector>		),
	NCuRotQuat(	~anim::Curve<Quaternion>	),
//	NCuRotEuler(~anim::Curve<Euler>			),
	NCuScale(	~anim::Curve<Scale>			),
}

type NodeRecord = anim::Record<NodeCurve>;

pub struct Node	{
	name	: ~str,
	space	: Space,
	parent	: Option<@mut Node>,
	actions	: ~[@NodeRecord],
}

impl Node	{
	pub fn new( name : ~str )-> Node	{
		Node	{
			name	: name,
			space	: Transform::identity(),
			parent	: None,
			actions	: ~[],
		}
	}
	pub fn world_space( &self ) -> Space	{
		match self.parent	{
			Some(p)	=> p.world_space().concat( &self.space ),
			None	=> self.space
		}
	}
	pub fn is_under( &self, name : &str )-> bool	{
		std::str::eq_slice(name,self.name) || match self.parent	{
			Some(p)	=> p.is_under(name),
			None	=> false,
		}
	}
}

impl anim::Player<NodeCurve> for Node	{
	fn find_record( &self, name : &str )-> Option<@NodeRecord>	{
		match self.actions.iter().find(|r| std::str::eq_slice(r.name,name))	{
			Some(rec)	=> Some(*rec),
			None		=> None,
		}
	}
	fn set_record( &mut self, a : &NodeRecord, time : anim::float )	{
		for chan in a.curves.iter()	{
			match chan	{
				&NCuPos(ref c)		=> self.space.disp	= c.sample(time),
				&NCuRotQuat(ref c)	=> self.space.rot	= c.sample(time),
				&NCuScale(ref c)	=> self.space.scale	= c.sample(time),
			}
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Armature										//

pub struct Bone	{
	node			: @mut Node,
	bind_space		: Space,
	bind_pose_inv	: Space,
	transform		: Space,
	parent_id		: Option<uint>,
}

impl Bone	{
	pub fn reset( &mut self )	{
		self.node.space = self.bind_space;
		self.transform = Transform::identity();
	}
}

pub enum ArmatureCurve	{
	ACuPos(		uint, ~anim::Curve<Vector>		),
	ACuRotQuat(	uint, ~anim::Curve<Quaternion>	),
	ACuScale(	uint, ~anim::Curve<Scale>		),
}

pub type ArmatureRecord = anim::Record<ArmatureCurve>;


pub struct Armature	{
	root	: @mut Node,
	bones	: ~[Bone],
	code	: ~str,
	actions	: ~[@ArmatureRecord],
	max_bones	: uint,
}

impl Armature	{
	pub fn change_root( &mut self, root : @mut Node )	{
		for bone in self.bones.iter()	{
			if bone.parent_id.is_none()	{
				bone.node.parent = Some(root);
			}
		}
		self.root = root;
	}
}

static armature_name : &'static str = &"Arm";

impl draw::Mod for Armature	{
	fn get_name<'a>( &'a self )-> &'a str	{ armature_name }
	fn get_code<'a>( &'a self )-> &'a str	{ self.code.as_slice() }
	//TODO: use float arrays
	fn fill_data( &self, data : &mut shade::DataMap )	{
		assert!( self.bones.len() < self.max_bones );
		let id = Transform::identity();
		let mut pos : ~[Vec4<f32>] = std::vec::with_capacity( self.max_bones );
		let mut rot : ~[Vec4<f32>] = std::vec::with_capacity( self.max_bones );
		let parent_inv = self.root.world_space().invert().expect(format!(
			"Uninvertable armature {:s} root space detected", self.root.name ));
		while pos.len() < self.max_bones	{
			let space = if pos.len()>0u && pos.len()<self.bones.len()	{
					let b = &self.bones[pos.len()-1u];
					parent_inv.concat( &b.node.world_space() ).concat( &b.bind_pose_inv )
				}else {id};
			let (p0,p1) = get_params( &space );
			pos.push( p0 );
			rot.push( p1 );
		}
		data.insert( ~"bone_pos[0]", shade::UniFloatVecArray(pos) );
		data.insert( ~"bone_rot[0]", shade::UniFloatVecArray(rot) );
	}
}


pub fn is_same_node( a: Option<@Node>, b : Option<@Node> )-> bool	{
	match (a,b)	{
		(Some(na),Some(nb))	=> managed::ptr_eq(na,nb),
		(None,None)	=> true,
		(_,_)		=> false,
	}
}

impl anim::Player<ArmatureCurve> for Armature	{
	fn find_record( &self, name : &str )-> Option<@ArmatureRecord>	{
		match self.actions.iter().find(|r| std::str::eq_slice(r.name,name))	{
			Some(rec)	=> Some(*rec),
			None		=> None,
		}
	}
	fn set_record( &mut self, a : &ArmatureRecord, time : anim::float )	{
		for chan in a.curves.iter()	{
			match chan	{
				&ACuPos(bi,ref c)		=> {
					let b = &self.bones[bi];	let v = c.sample(time);
					b.node.space.disp	= b.bind_space.transform_as_point( &v );
				},
				&ACuRotQuat(bi,ref c)	=> {
					let b = &self.bones[bi];	let q = c.sample(time);
					b.node.space.rot	= b.bind_space.rot.mul_q( &q );
				},
				&ACuScale(bi,ref c)		=> {
					let b = &self.bones[bi];	let s = c.sample(time);
					b.node.space.scale	= b.bind_space.scale * s;
				},
			}
		}
	}
}
