extern mod lmath;
pub type Matrix = lmath::matrix::mat4;
pub type Vector = lmath::vector::vec3;
pub type Quaternion = lmath::quaternion::quat4;
pub type Scale = f32;


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

pub pure fn identity()-> QuatSpace	{
	QuatSpace{
		position	: lmath::vector::Vec3::new(0f32,0f32,0f32),
		orientation	: lmath::quaternion::Quat::new(1f32,0f32,0f32,0f32),
		scale		: 1f32,
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
			Some(p)	=> p.world_space().mul( &self.space ),
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
	bind_pose		: QuatSpace,
	mut transform	: QuatSpace,
	parent_id		: Option<uint>,
}

impl Bone	{
	fn reset()	{
		self.node.space = self.bind_pose;
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
	node	: Option<@Node>,
	bones	: ~[Bone],
	code	: ~str,
	actions	: ~[@ArmatureRecord],	//FIXME mutable
	max_bones	: uint,
	mut last_updated	: uint,
}


impl Armature : draw::Mod	{
	pure fn get_name()-> ~str	{ ~"arm" }
	pure fn get_code()-> ~str	{ copy self.code }
	//FIXME: make pure when Rust allows
	//TODO: use float arrays
	fn fill_data( data : &mut shade::DataMap )	{
		assert self.bones.len() < self.max_bones;
		let mut id = identity();
		let mut i = 0u;
		while i<self.max_bones	{
			let space = if i==0u || i>self.bones.len() {&mut id}
				else { &mut self.bones[i-1u].node.space };
			let pos_scale = lmath::vector::Vec4::new(
				space.position.x, space.position.y, space.position.z,
				space.scale );
			data.insert( fmt!("bones[%u].pos",i), shade::UniFloatVec(pos_scale) );
			data.insert( fmt!("bones[%u].rot",i), shade::UniQuat(space.orientation) );
			i += 1u;
		}
	}
}


pure fn is_same_node( a: Option<@Node>, b : Option<@Node> )-> bool	{
	match (a,b)	{
		(Some(na),Some(nb))	=> box::ptr_eq(na,nb),
		(None,None)	=> true,
		(_,_)		=> false,
	}
}

impl Armature	{
	fn update()	{
		let mut cache_bind = vec::with_capacity::<QuatSpace>( self.bones.len() );
		for self.bones.each() |b|	{
			let bind_inv = b.bind_pose.inverse();
			b.transform = match b.parent_id	{
				Some(pid)	=>	{
					assert pid < cache_bind.len();
					assert is_same_node( b.node.parent, Some(self.bones[pid].node) );
					let pose = b.node.world_space();
					let bind_pose_inv =  bind_inv * cache_bind[pid];
					cache_bind.push( bind_pose_inv );
					pose.mul( &bind_pose_inv )
				},
				None	=>	{
					assert is_same_node( b.node.parent, self.node );
					cache_bind.push( bind_inv );
					b.node.space.mul( &bind_inv )
				}
			};
		}
		self.last_updated += 1u;
	}
}


impl Armature : anim::Player<ArmatureCurve>	{
	pure fn find_record( name : ~str )-> Option<@ArmatureRecord>	{
		do self.actions.find() |a| {a.name==name}
	}
	fn set_record( a : &ArmatureRecord, time : float )	{
		for a.curves.each() |chan|	{
			match chan	{
				&ACuPos(bi,c)		=> self.bones[bi].node.space.position		= c.sample(time),
				&ACuRotQuat(bi,c)	=> self.bones[bi].node.space.orientation	= c.sample(time),
				&ACuScale(bi,c)		=> self.bones[bi].node.space.scale			= c.sample(time),
			}
		}
	}
}