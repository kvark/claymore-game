extern mod lmath;
extern mod engine;

extern mod std;
use std::json;
use send_map::linear::LinearMap;
use std::serialization::{Deserializer,Deserializable};


pub fn load_config<T:Deserializable>( path : ~str )-> T	{
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
struct ArmatureInfo	{
	node	: NodeInfo,
	path	: ~str,
	dual_quat	: bool,
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Material

#[auto_deserialize]
struct TextureInfo	{
	name	: ~str,
	path	: ~str,
	wrap	: int,
	filter	: uint,
}

pub struct ShaderParam	{
	name	: ~str,
	value	: engine::shade::Uniform,
}
impl ShaderParam : Deserializable	{
	static fn deserialize<D:Deserializer>( &self, d : &D )-> ShaderParam	{
		let v = d.read_float();
		ShaderParam{
			name	: ~"",
			value	: engine::shade::UniFloat(v),
		}
	}
}


#[auto_deserialize]
struct MaterialInfo	{
	name		: ~str,
	code_path	: ~str,
	data		: ~[ShaderParam],
	textures	: ~[TextureInfo],
}

pure fn color_to_vec(col : &engine::rast::Color)-> lmath::vector::vec4	{
	lmath::vector::Vec4::new( col.r, col.g, col.b, col.a )
}


fn generate_material_data( minfo : &MaterialInfo, ct : &engine::context::Context) -> engine::shade::DataMap	{
	let mut data = engine::shade::create_data();
	for minfo.data.each() |par|	{
		data.insert( copy par.name, copy par.value );
	}
	for minfo.textures.each() |tinfo|	{
		let tex = @engine::load::load_texture_2D( ct, &tinfo.path, tinfo.wrap, tinfo.filter );
		data.insert( copy tinfo.name, engine::shade::UniTexture(0,tex) );
	}
	data
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

pub type EntityGroup = ~[engine::draw::Entity];
pub fn divide_group( group : &mut EntityGroup, name : &~str )->EntityGroup	{
	let mut i = 0u;
	let mut rez : EntityGroup = ~[];
	while i<group.len()	{
		if group[i].node.is_under(name)	{
			rez.push( group.swap_remove(i) );
		}else	{
			i += 1u;
		}
	}
	rez
}

pub struct Scene	{
	materials	: LinearMap<~str,@engine::draw::Material>,
	nodes		: LinearMap<~str,NodeRef>,
	armatures	: LinearMap<~str,@engine::space::Armature>,
	mut entities: EntityGroup,
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
	let mut map_material_data = LinearMap::<~str,engine::shade::DataMap>();
	for scene.materials.each() |imat|	{
		let mat = @engine::draw::load_material( copy imat.code_path );
		map_material.insert( copy imat.name, mat );
		map_material_data.insert( copy imat.name, generate_material_data(imat,gc) );
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
	let mut entity_group : EntityGroup = ~[];
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
			data	: map_material_data.get( &ient.material ),
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
		entity_group.push(ent)
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
		entities	: entity_group,
		cameras		: map_camera,
		lights		: map_light,
	}
}