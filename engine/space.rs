extern mod cgmath;

use std;
use std::{cell,rc};

use cgmath::angle;
use cgmath::matrix::{Matrix,Mat4};
use cgmath::quaternion::{Quat};
use cgmath::transform::{Transform,Decomposed};
use cgmath::vector::{Vector,Vec3,Vec4};

use anim;


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Quaternion Space								//

pub type Matrix = Mat4<f32>;
pub type Vector = Vec3<f32>;
pub type Scale = f32;
pub type Quaternion = Quat<f32>;
pub type Euler = Vec3<angle::Rad<f32>>;
pub type Space = Decomposed<Scale,Vector,Quaternion>;


pub fn make( scale: Scale, rot: Quaternion, disp: Vector )-> Space	{
	Decomposed{ scale: scale, rot: rot, disp: disp }
}
pub fn get_params( sp: &Space )-> (Vec4<f32>,Vec4<f32>)	{
	(sp.disp.extend( sp.scale ),
	sp.rot.v.extend( sp.rot.s ))
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Interpolation									//

pub trait Interpolate	{
	fn interpolate( &self, &Self, f32 )-> Self;
}

impl Interpolate for Vector	{
	fn interpolate( &self, other: &Vector, t: f32 )-> Vector	{
		self.mul_s(1.0-t).add_v( &other.mul_s(t) )
	}
}

impl Interpolate for Quaternion	{
	//TODO: use slerp
	fn interpolate( &self, other: &Quaternion, t: f32 )-> Quaternion	{
		self.mul_s(1.0-t).add_q( &other.mul_s(t) )
	}
}

impl Interpolate for Scale	{
	fn interpolate( &self, other: &Scale, t: f32 )-> Scale	{
		self*(1.0-t) + (*other)*t
	}
}

impl Interpolate for Space	{
	fn interpolate( &self, other: &Space, t: f32 )-> Space	{
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

pub type NodeRecord		= anim::Record<NodeCurve>;
pub type NodeRecordPtr	= anim::RecordPtr<NodeCurve>;
pub type NodePtr = rc::Rc<cell::RefCell<Node>>;

pub struct Node	{
	name	: ~str,
	space	: Space,
	parent	: Option<NodePtr>,
	actions	: ~[NodeRecordPtr],
}

impl Node	{
	pub fn new( name: ~str )-> Node	{
		Node	{
			name	: name,
			space	: Transform::identity(),
			parent	: None,
			actions	: ~[],
		}
	}
	
	pub fn to_ptr( self )-> NodePtr	{
		rc::Rc::new(cell::RefCell::new( self ))
	}
	
	pub fn world_space( &self ) -> Space	{
		match self.parent	{
			Some(ref par)	=> par.borrow().with(|p|
				p.world_space().concat( &self.space )),
			None		=> self.space,
		}
	}
	
	pub fn is_under( &self, name: &str )-> bool	{
		std::str::eq_slice(name,self.name) || match self.parent	{
			Some(ref par)	=> par.borrow().with(|p| p.is_under(name)),
			None	=> false,
		}
	}
}

impl anim::Player<NodeCurve> for Node	{
	fn find_record( &self, name: &str )-> Option<NodeRecordPtr>	{
		self.actions.iter().find(|r| std::str::eq_slice(r.borrow().name,name)).map(|r| r.clone())
	}
	fn set_record( &mut self, a: &NodeRecord, time: anim::float )	{
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

pub struct ModArm	{
	code_reg:	rc::Rc<~str>,
	code_dq:	rc::Rc<~str>
}

pub struct Bone	{
	node			: NodePtr,
	bind_space		: Space,
	bind_pose_inv	: Space,
	transform		: Space,
	parent_id		: Option<uint>,
}

impl Bone	{
	pub fn reset( &mut self )	{
		self.node.borrow().with_mut(|n| n.space = self.bind_space);
		self.transform = Transform::identity();
	}
}

pub enum ArmatureCurve	{
	ACuPos(		uint, ~anim::Curve<Vector>		),
	ACuRotQuat(	uint, ~anim::Curve<Quaternion>	),
	ACuScale(	uint, ~anim::Curve<Scale>		),
}

pub type ArmatureRecord 	= anim::Record<ArmatureCurve>;
pub type ArmatureRecordPtr	= anim::RecordPtr<ArmatureCurve>;
pub type ArmaturePtr = rc::Rc<cell::RefCell<Armature>>;

pub struct Armature	{
	root	: NodePtr,
	bones	: ~[Bone],
	actions	: ~[ArmatureRecordPtr],
}

impl Armature	{
	pub fn to_ptr( self )-> ArmaturePtr	{
		rc::Rc::new(cell::RefCell::new( self ))
	}
	pub fn change_root( &mut self, root: NodePtr )	{
		for bone in self.bones.iter()	{
			if bone.parent_id.is_none()	{
				bone.node.borrow().with_mut(|n| n.parent = Some( root.clone() ));
			}
		}
		self.root = root;
	}
}

impl anim::Player<ArmatureCurve> for Armature	{
	fn find_record( &self, name: &str )-> Option<ArmatureRecordPtr>	{
		self.actions.iter().find(|r| std::str::eq_slice(r.borrow().name,name)).map(|r| r.clone())
	}
	fn set_record( &mut self, a: &ArmatureRecord, time: anim::float )	{
		for chan in a.curves.iter()	{
			match chan	{
				&ACuPos(bi,ref c)		=> {
					let b = &self.bones[bi];	let v = c.sample(time);
					b.node.borrow().with_mut( |n| n.space.disp	= b.bind_space.transform_as_point(&v) );
				},
				&ACuRotQuat(bi,ref c)	=> {
					let b = &self.bones[bi];	let q = c.sample(time);
					b.node.borrow().with_mut( |n| n.space.rot	= b.bind_space.rot.mul_q(&q) );
				},
				&ACuScale(bi,ref c)		=> {
					let b = &self.bones[bi];	let s = c.sample(time);
					b.node.borrow().with_mut( |n| n.space.scale	= b.bind_space.scale * s );
				},
			}
		}
	}
}

