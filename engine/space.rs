extern mod lmath;

use core::managed;
use core::to_str::ToStr;
use core::str;

use lmath::vec::*;
use lmath::quat::*;
use lmath::mat::*;

use gr_low::shade;
use gr_mid::draw;
use anim;

pub type Matrix = Mat4<f32>;
pub type Vector = Vec3<f32>;
pub type Quaternion = Quat<f32>;
pub type Scale = f32;


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Quaternion Space								//

pub trait Space : ops::Mul<Space,Space>	{
	fn transform( &self, point : &Vector )-> Vector;
	fn rotate( &self, vector : &Vector )-> Vector;
	fn invert( &self )-> Self;
	fn to_matrix( &self )-> Matrix;
}


impl Space for Matrix	{
	fn transform( &self, point : &Vector )-> Vector	{
		let v4 = vec4::new( point.x, point.y, point.z, 1f32 );
		let vt = self.mul_v(&v4);
		vec3::new( vt.x/vt.w, vt.y/vt.w, vt.z/vt.w )
	}
	fn rotate( &self, vector : &Vector )-> Vector	{
		let v4 = vec4::new( vector.x, vector.y, vector.z, 0f32 );
		let vt = self.mul_v(&v4);
		vec3::new( vt.x, vt.y, vt.z ).normalize()
	}
	fn invert( &self )-> Matrix	{
		self.inverse().expect( ~"Unable to invert matrix" )
	}
	fn to_matrix( &self )-> Matrix	{*self}
}


pub struct QuatSpace	{
	position	: Vector,
	orientation	: Quaternion,
	scale		: Scale,
}

impl ops::Mul<QuatSpace,QuatSpace> for QuatSpace	{
	fn mul( &self, other : &QuatSpace )-> QuatSpace	{
		QuatSpace{
			position	: self.transform( &other.position ),
			orientation	: self.orientation.mul_q( &other.orientation ),
			scale		: self.scale * other.scale
		}
	}
}

impl Space for QuatSpace	{
	fn transform( &self, point : &Vector )-> Vector	{
		self.orientation.mul_v( point ).mul_t( self.scale ).add_v( &self.position )
	}
	fn rotate( &self, vector : &Vector )-> Vector	{
		self.orientation.mul_v( vector )
	}
	fn invert( &self )-> QuatSpace	{
		let q = self.orientation.conjugate();
		let s = 1f32 / self.scale;
		let p = q.mul_v(&self.position).mul_t(-s);
		QuatSpace{ position:p, orientation:q, scale:s }
	}
	fn to_matrix( &self )-> Matrix	{
		let m3 = self.orientation.to_mat3();
		let mut m4 = m3.mul_t(self.scale).to_mat4();
		m4.w.x = self.position.x;
		m4.w.y = self.position.y;
		m4.w.z = self.position.z;
		m4
	}
}

pub impl QuatSpace	{
	fn identity()-> QuatSpace	{
		QuatSpace{
			position	: vec3::new(0f32,0f32,0f32),
			orientation	: Quat::identity::<f32>(),
			scale		: 1f32,
		}
	}
	fn get_pos_scale( &self )-> vec4	{
		vec4::new( self.position.x, self.position.y, self.position.z, self.scale )
	}
	fn get_orientation( &self )-> vec4	{
		vec4::new(
			self.orientation.v.x, self.orientation.v.y,
			self.orientation.v.z, self.orientation.s )	
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Interpolation									//

pub trait Interpolate	{
	fn interpolate( &self, other : &Self, t : float )-> Self;
}

impl Interpolate for Vector	{
	fn interpolate( &self, other : &Vector, t : float )-> Vector	{
		let t1  = (1f-t) as f32, t2 = t as f32;
		self.mul_t(t1).add_v( &other.mul_t(t2) )
	}
}

impl Interpolate for Quaternion	{
	//FIXME: use slerp
	fn interpolate( &self, other : &Quaternion, t : float )-> Quaternion	{
		let t1  = (1f-t) as f32, t2 = t as f32;
		self.mul_t(t1).add_q( &other.mul_t(t2) )
	}
}

impl Interpolate for Scale	{
	fn interpolate( &self, other : &Scale, t : float )-> Scale	{
		let t1  = (1f-t) as f32, t2 = t as f32;
		self*t1 + (*other)*t2
	}
}

impl Interpolate for QuatSpace	{
	fn interpolate( &self, other : &QuatSpace, t : float )-> QuatSpace	{
		QuatSpace{
			position	: self.position.interpolate( &other.position, t ),
			orientation	: self.orientation.interpolate( &other.orientation, t ),
			scale		: self.scale.interpolate( &other.scale, t ),
		}
	}
}


impl ToStr for QuatSpace	{
	fn to_str( &self )-> ~str	{
		fmt!( "{pos:%s,rot:%s,scale:%f}",
			"","",
			//self.position.to_str(),
			//self.orientation.to_str(),
			self.scale as float )
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Node											//

enum NodeCurve	{
	NCuPos(		@anim::Curve<Vector>		),
	NCuRotQuat(	@anim::Curve<Quaternion>	),
//	NCuRotEuler(@anim::Curve<Vector>		),
	NCuScale(	@anim::Curve<Scale>			),
}

type NodeRecord = anim::Record<NodeCurve>;

pub struct Node	{
	name	: ~str,
	space	: QuatSpace,	//FIXME: arbitrary space
	parent	: Option<@mut Node>,
	actions	: ~[@NodeRecord],
}

pub impl Node	{
	fn new( name : ~str )-> Node	{
		Node	{
			name	: name,
			space	: QuatSpace::identity(),
			parent	: None,
			actions	: ~[],
		}
	}
	fn world_space( &self ) -> QuatSpace	{
		match self.parent	{
			Some(p)	=> p.world_space() * self.space,
			None	=> self.space
		}
	}
	fn is_under( &self, name : &~str )-> bool	{
		self.name == *name || match self.parent	{
			Some(p) => p.is_under(name),
			None	=> false,
		}
	}
}

impl anim::Player<NodeCurve> for Node	{
	fn find_record( &self, name : &str )-> Option<@NodeRecord>	{
		self.actions.find(|a| str::eq_slice(a.name,name))
	}
	fn set_record( &mut self, a : &NodeRecord, time : float )	{
		for a.curves.each() |chan|	{
			match chan	{
				&NCuPos(ref c)		=> self.space.position		= c.sample(time),
				&NCuRotQuat(ref c)	=> self.space.orientation	= c.sample(time),
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

pub impl Bone	{
	fn reset( &mut self )	{
		self.node.space = self.bind_space;
		self.transform = QuatSpace::identity();
	}
}

pub enum ArmatureCurve	{
	ACuPos(		uint, @anim::Curve<Vector>		),
	ACuRotQuat(	uint, @anim::Curve<Quaternion>	),
	ACuRotEuler(uint, @anim::Curve<Vector>		),
	ACuScale(	uint, @anim::Curve<Scale>		),
}

pub type ArmatureRecord = anim::Record<ArmatureCurve>;


pub struct Armature	{
	root	: @mut Node,
	bones	: ~[Bone],
	code	: ~str,
	actions	: ~[@ArmatureRecord],
	max_bones	: uint,
}

pub impl Armature	{
	fn change_root( &mut self, root : @mut Node )	{
		for self.bones.each() |bone|	{
			if bone.parent_id.is_none()	{
				bone.node.parent = Some(root);
			}
		}
		self.root = root;
	}
}

impl draw::Mod for Armature	{
	fn get_name( &self )-> ~str	{ ~"Arm" }
	fn get_code( &self )-> ~str	{ copy self.code }
	//TODO: use float arrays
	fn fill_data( &self, data : &mut shade::DataMap )	{
		assert!( self.bones.len() < self.max_bones );
		let id = QuatSpace::identity();
		let mut pos = vec::with_capacity::<vec4>( self.max_bones );
		let mut rot = vec::with_capacity::<vec4>( self.max_bones );
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


fn is_same_node( a: Option<@Node>, b : Option<@Node> )-> bool	{
	match (a,b)	{
		(Some(na),Some(nb))	=> managed::ptr_eq(na,nb),
		(None,None)	=> true,
		(_,_)		=> false,
	}
}

impl anim::Player<ArmatureCurve> for Armature	{
	fn find_record( &self, name : &str )-> Option<@ArmatureRecord>	{
		self.actions.find(|a| str::eq_slice(a.name,name))
	}
	fn set_record( &mut self, a : &ArmatureRecord, time : float )	{
		for a.curves.each() |chan|	{
			match chan	{
				&ACuPos(bi,ref c)		=> {
					let b = &self.bones[bi];	let p = c.sample(time);
					b.node.space.position	= b.bind_space.transform( &p );
				},
				&ACuRotQuat(bi,ref c)	=> {
					let b = &self.bones[bi];	let q = c.sample(time);
					b.node.space.orientation= b.bind_space.orientation.mul_q( &q );
				},
				&ACuRotEuler(bi,ref c)	=> {
					let b = &self.bones[bi];	let eul = c.sample(time);
					let q = quat::from_angle_xyz( eul.x, eul.y, eul.z );
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
