extern mod lmath;
extern mod engine;

extern mod std;
use std::json;
use send_map::linear::LinearMap;


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

pub type NodeRef = @engine::space::Node;
pub type NodeMap = LinearMap<~str,NodeRef>;

#[auto_deserialize]
struct NodeInfo	{
	name		: ~str,
	parent		: ~str,
	space		: SpaceInfo,
}

pub fn make_node( info : &NodeInfo, map : &mut NodeMap )-> NodeRef	{
	let node = @engine::space::Node{
		name	: copy info.name,
		space	: info.space.spawn(),
		parent	: map.find( &info.parent ),
		actions	: ~[],	//TODO
	};
	map.insert( copy info.name, node );
	node
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Entity, Texture, Material, Armature

#[auto_deserialize]
struct EntityInfo	{
	node		: NodeInfo,
	material	: ~str,
	mesh_path	: ~str,
	range		: (uint,uint),
	has_armature: bool,
}

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
	dual_quat	: bool,
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

#[auto_deserialize]
pub struct CameraInfo	{
	node	: NodeInfo,
	proj	: ProjectorInfo,
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

pub struct Scene	{
	materials	: LinearMap<~str,@engine::draw::Material>,
	nodes		: LinearMap<~str,NodeRef>,
	armatures	: LinearMap<~str,@engine::space::Armature>,
	entities	: LinearMap<~str,engine::draw::Entity>,
	cameras		: LinearMap<~str,Camera>,
	lights		: LinearMap<~str,Light>,
}

#[auto_deserialize]
pub struct SceneInfo	{
	materials	: ~[MaterialInfo],
	dummies		: ~[NodeInfo],
	armatures	: ~[ArmatureInfo],
	entities	: ~[EntityInfo],
	cameras		: ~[CameraInfo],
	lights		: ~[LightInfo],
}


pub fn load_scene( path : ~str, gc : &engine::context::Context,
		opt_vao : Option<@engine::buf::VertexArray>, aspect : float )-> Scene	{
	let scene = load_config::<SceneInfo>( path );
	// materials
	let mut map_material = LinearMap::<~str,@engine::draw::Material>();
	for scene.materials.each() |imat|	{
		let mat = @engine::draw::load_material( copy imat.code_path );
		map_material.insert( copy imat.name, mat );
	}
	// nodes
	let mut map_node = LinearMap::<~str,@engine::space::Node>();
	for scene.dummies.each() |idummy|	{
		make_node( idummy, &mut map_node );
	}
	// armatures
	let mut map_armature = LinearMap::<~str,@engine::space::Armature>();
	for scene.armatures.each() |iarm|	{
		let root = make_node( &iarm.node, &mut map_node );
		let arm = @engine::load::read_armature(
			&engine::load::create_reader( copy iarm.path ),
			root, iarm.dual_quat );
		map_armature.insert( copy root.name, arm );
	}
	// entities
	let mut map_entity = LinearMap::<~str,engine::draw::Entity>();
	for scene.entities.each() |ient|	{
		let root = make_node( &ient.node, &mut map_node );
		let mat = map_material.get( &ient.material );
		let skel = if ient.has_armature	{
			map_armature.get(&ient.node.parent) as @engine::draw::Mod
		}else	{
			@() as @engine::draw::Mod
		};
		let (r_min,r_max) = ient.range;
		let ent = engine::draw::Entity{
			node	: root,
			data	: engine::shade::create_data(),
			vao		: match opt_vao	{
					Some(v) => v,
					None	=> @gc.create_vertex_array(),
				},
			mesh	: @engine::load::read_mesh(
				&engine::load::create_reader( copy ient.mesh_path ),
				gc ),
			range	: engine::mesh::Range{
				start	:r_min,
				num		:r_min-r_max,
				},
			modifier: skel,
			material: mat,
		};
		map_entity.insert( copy root.name, ent );
	}
	// cameras
	let mut map_camera = LinearMap::<~str,Camera>();
	for scene.cameras.each() |icam|	{
		let root = make_node( &icam.node, &mut map_node );
		map_camera.insert( copy root.name, Camera{
			node:root, proj:icam.proj.spawn(aspect),
			ear:engine::audio::Listener{ volume:0f },
		});
	}
	// lights
	let mut map_light = LinearMap::<~str,Light>();
	for scene.lights.each() |ilight|	{
		let root = make_node( &ilight.node, &mut map_node );
		map_light.insert( copy root.name, Light{
			node:root, proj:ilight.proj.spawn(1f),
		});
	}
	// done
	Scene{
		materials	: map_material,
		nodes		: map_node,
		armatures	: map_armature,
		entities	: map_entity,
		cameras		: map_camera,
		lights		: map_light,
	}
}