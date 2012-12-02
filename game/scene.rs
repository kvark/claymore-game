extern mod lmath;
extern mod engine;

extern mod std;
use std::json;


pub fn load_config<T : std::serialization::Deserializable>( path : ~str )-> T	{
	let rd = match io::file_reader(&path::Path(path))	{
		Ok(reader)	=> reader,
		Err(e)		=> fail e.to_str(),
	};
	match json::Deserializer(rd)	{
		Ok(ref deser)	=> std::serialization::deserialize(deser),
		Err(e)			=> fail e.to_str(),
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Space

#[auto_deserialize]
struct SpaceInfo	{
	position	: (f32,f32,f32),
	orientation	: (f32,f32,f32,f32),
	scale		: f32,
}

impl SpaceInfo	{
	pure fn spawn()-> engine::space::QuatSpace	{
		engine::space::QuatSpace{
			position : {
				let (x,y,z) = self.position;
				lmath::vector::Vec3::new(x,y,z)
			},
			orientation : {
				let (w,x,y,z) = self.orientation;
				lmath::quaternion::Quat::new(w,x,y,z)
			},
			scale : self.scale,
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Node

#[auto_deserialize]
struct NodeInfo	{
	name		: ~str,
	space		: SpaceInfo,
	children	: ~[NodeInfo],
}

pub type NodeRef = @engine::space::Node;
pub type NodeMap = send_map::linear::LinearMap<~str,NodeRef>;

pub fn make_node( info : &NodeInfo, par : Option<NodeRef>, map : &mut NodeMap )->NodeRef	{
	let node = @engine::space::Node{
		name	: copy info.name,
		parent	: par,
		space	: info.space.spawn(),
		actions	: ~[],	//TODO
	};
	map.insert( copy info.name, node );
	for info.children.each() |nc|	{
		make_node( nc, Some(node), map );
	}
	node
}

priv fn load_node( path : ~str )-> (NodeRef,NodeMap) 	{
	let mut map = send_map::linear::LinearMap::<~str,NodeRef>();
	let node_info = load_config::<NodeInfo>(path);
	let node = make_node( &node_info, None, &mut map );
	(node,map)
}


#[auto_deserialize]
struct EntityInfo	{
	node		: ~str,
	mesh_path	: ~str,
	range		: (uint,uint),
	skel_path	: ~str,
	material	: ~str,
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Texture,Material,Armature

#[auto_deserialize]
struct TextureInfo	{
	name	: ~str,
	path	: ~str,
	wrap	: int,
	filter	: uint,
}

#[auto_deserialize]
struct MaterialInfo	{
	name		: ~str,
	code_path	: ~str,
	textures	: ~[TextureInfo],
}

#[auto_deserialize]
struct ArmatureInfo	{
	node	: NodeInfo,
	path	: ~str,
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Projector

pub struct Projector	{
	fov_x	: float,
	fov_y	: float,
	r_near	: float,
	r_far	: float,
}

impl Projector	{
	pure fn to_matrix() -> lmath::matrix::mat4	{
		lmath::funs::projection::perspective::<f32>(
			self.fov_y,
			self.fov_x / self.fov_y,
			self.r_near, self.r_far )
	}
}

#[auto_deserialize]
pub struct ProjectorInfo	{
	fov		: float,
	range	: (float,float),
}

impl ProjectorInfo	{
	pure fn spawn( aspect : float )-> Projector	{
		let (near,far) = self.range;
		Projector{
			fov_x	: aspect * self.fov,
			fov_y	: self.fov,
			r_near	: near,
			r_far	: far,
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Camera

pub struct Camera	{
	node	: NodeRef,
	proj	: Projector,
	ear		: engine::audio::Listener,
}

#[auto_deserialize]
pub struct CameraInfo	{
	node	: NodeInfo,
	proj	: ProjectorInfo,
}

impl Camera	{
	pure fn get_matrix()-> lmath::matrix::mat4	{
		let proj = self.proj.to_matrix();
		proj * self.node.world_space().inverse().to_matrix()
	}
	pure fn get_pos_vec4()-> lmath::vector::vec4	{
		let v = self.node.world_space().position;
		lmath::vector::Vec4::new( v.x, v.y, v.z, 0f32 )
	}
	pure fn get_view_vector()-> lmath::vector::vec3	{
		let v = lmath::vector::Vec3::new( 0f32,0f32,-1f32 );
		self.node.world_space().orientation.mul_v( &v )
	}
	pure fn get_up_vector()-> lmath::vector::vec3	{
		let v = lmath::vector::Vec3::new( 0f32,1f32,0f32 );
		self.node.world_space().orientation.mul_v( &v )
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Light

pub struct Light	{
	node	: NodeRef,
	proj	: Projector,
}

#[auto_deserialize]
pub struct LightInfo	{
	node	: NodeInfo,
	proj	: ProjectorInfo,
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Scene

#[auto_deserialize]
pub struct SceneInfo	{
	materials	: ~[MaterialInfo],
	entities	: ~[EntityInfo],
	skeletons	: ~[ArmatureInfo],
	cameras		: ~[CameraInfo],
	lights		: ~[LightInfo],
	dummies		: ~[NodeInfo],
}