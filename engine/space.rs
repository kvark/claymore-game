extern mod cgmath;

use std;
use std::managed;
use std::to_str::ToStr;

use cgmath::angle;
use cgmath::matrix::*;
use cgmath::quaternion::*;
use cgmath::transform::*;
use cgmath::vector::*;

use gr_low::shade;
use gr_mid::draw;
use anim;

pub type Matrix = Mat4<f32>;
pub type Vector = Vec3<f32>;
pub type Scale = f32;
pub type Quaternion = Quat<f32>;
pub type Euler = Vec3<angle::Rad<f32>>;
pub type Space2 = Transform3D<f32>;


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Quaternion Space								//


pub trait Space	{
	fn transform( &self, point : &Vector )-> Vector;
	fn concat( &self, other : &Self )-> Self;
	fn rotate( &self, vector : &Vector )-> Vector;
	fn inverted( &self )-> Self;
	fn to_matrix( &self )-> Matrix;
}


impl Space for Matrix	{
	fn transform( &self, point : &Vector )-> Vector	{
		let v4 = Vec4::new( point.x, point.y, point.z, 1f32 );
		let vt = self.mul_v(&v4);
		Vec3::new( vt.x/vt.w, vt.y/vt.w, vt.z/vt.w )
	}
	fn concat( &self, other : &Matrix )-> Matrix	{
		self.mul_m(other)
	}
	fn rotate( &self, vector : &Vector )-> Vector	{
		let v4 = Vec4::new( vector.x, vector.y, vector.z, 0f32 );
		let vt = self.mul_v(&v4);
		Vec3::new( vt.x, vt.y, vt.z ).normalize()
	}
	fn inverted( &self )-> Matrix	{
		self.invert().expect("Unable to invert matrix")
	}
	fn to_matrix( &self )-> Matrix	{*self}
}


pub struct QuatSpace	{
	position	: Vector,
	orientation	: Quaternion,
	scale		: Scale,
}

impl Space for QuatSpace	{
	fn transform( &self, point : &Vector )-> Vector	{
		self.orientation.mul_v( point ).mul_s( self.scale ).add_v( &self.position )
	}
	fn concat( &self, other : &QuatSpace )-> QuatSpace	{
		QuatSpace{
			position	: self.transform( &other.position ),
			orientation	: self.orientation.mul_q( &other.orientation ),
			scale		: self.scale * other.scale
		}
	}
	fn rotate( &self, vector : &Vector )-> Vector	{
		self.orientation.mul_v( vector )
	}
	fn inverted( &self )-> QuatSpace	{
		let q = self.orientation.conjugate();
		let s = 1f32 / self.scale;
		let p = q.mul_v(&self.position).mul_s(-s);
		QuatSpace{ position:p, orientation:q, scale:s }
	}
	fn to_matrix( &self )-> Matrix	{
		let m3 = self.orientation.to_mat3();
		let mut m4 = m3.mul_s(self.scale).to_mat4();
		m4.w.x = self.position.x;
		m4.w.y = self.position.y;
		m4.w.z = self.position.z;
		m4
	}
}

impl QuatSpace	{
	pub fn identity()-> QuatSpace	{
		QuatSpace{
			position	: Vec3::new(0f32,0f32,0f32),
			orientation	: Quat::identity(),
			scale		: 1f32,
		}
	}
	pub fn get_pos_scale( &self )-> Vec4<f32>	{
		Vec4::new( self.position.x, self.position.y, self.position.z, self.scale )
	}
	pub fn get_orientation( &self )-> Vec4<f32>	{
		Vec4::new(
			self.orientation.v.x, self.orientation.v.y,
			self.orientation.v.z, self.orientation.s )	
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Interpolation									//

pub trait Interpolate	{
	fn interpolate( &self, other : &Self, t : f32 )-> Self;
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

impl Interpolate for QuatSpace	{
	fn interpolate( &self, other : &QuatSpace, t : f32 )-> QuatSpace	{
		QuatSpace{
			position	: self.position.interpolate( &other.position, t ),
			orientation	: self.orientation.interpolate( &other.orientation, t ),
			scale		: self.scale.interpolate( &other.scale, t ),
		}
	}
}


impl ToStr for QuatSpace	{
	fn to_str( &self )-> ~str	{
		format!( "(pos:{},rot:{},scale:{:f})",
			"","",
			//self.position,
			//self.orientation,
			self.scale )
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
	space	: QuatSpace,	//FIXME: arbitrary space
	parent	: Option<@mut Node>,
	actions	: ~[@NodeRecord],
}

impl Node	{
	pub fn new( name : ~str )-> Node	{
		Node	{
			name	: name,
			space	: QuatSpace::identity(),
			parent	: None,
			actions	: ~[],
		}
	}
	pub fn world_space( &self ) -> QuatSpace	{
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
				&NCuPos(ref c)		=> self.space.position		= c.sample(time),
				&NCuRotQuat(ref c)	=> self.space.orientation	= c.sample(time),
//				&NCuRotEuler(ref c)	=> self.space.orientation	= c.sample(time).to_quat(),
				&NCuScale(ref c)	=> self.space.scale			= c.sample(time),
			}
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Armature										//

pub struct Bone	{
	node			: @mut Node,
	bind_space		: QuatSpace,
	bind_pose_inv	: QuatSpace,
	transform		: QuatSpace,
	parent_id		: Option<uint>,
}

impl Bone	{
	pub fn reset( &mut self )	{
		self.node.space = self.bind_space;
		self.transform = QuatSpace::identity();
	}
}

pub enum ArmatureCurve	{
	ACuPos(		uint, ~anim::Curve<Vector>		),
	ACuRotQuat(	uint, ~anim::Curve<Quaternion>	),
//	ACuRotEuler(uint, ~anim::Curve<Euler>		),
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
		let id = QuatSpace::identity();
		let mut pos : ~[Vec4<f32>] = std::vec::with_capacity( self.max_bones );
		let mut rot : ~[Vec4<f32>] = std::vec::with_capacity( self.max_bones );
		let parent_inv = self.root.world_space().inverted();
		while pos.len() < self.max_bones	{
			let space = if pos.len()>0u && pos.len()<self.bones.len()	{
					let b = &self.bones[pos.len()-1u];
					parent_inv.concat( &b.node.world_space() ).concat( &b.bind_pose_inv )
				}else {id};
			pos.push( space.get_pos_scale() );
			rot.push( space.get_orientation() );
		}
		data.insert( ~"bone_pos[0]", shade::UniFloatVecArray(pos) );
		data.insert( ~"bone_rot[0]", shade::UniFloatVecArray(rot) );
	}
}


fn is_same_node( a: Option<@Node>, b : Option<@Node> )-> bool	{
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
					let b = &self.bones[bi];	let p = c.sample(time);
					b.node.space.position	= b.bind_space.transform( &p );
				},
				&ACuRotQuat(bi,ref c)	=> {
					let b = &self.bones[bi];	let q = c.sample(time);
					b.node.space.orientation= b.bind_space.orientation.mul_q( &q );
				},
				&ACuScale(bi,ref c)		=> {
					let b = &self.bones[bi];	let s = c.sample(time);
					b.node.space.scale		= b.bind_space.scale * s;
				},
			}
		}
	}
}
