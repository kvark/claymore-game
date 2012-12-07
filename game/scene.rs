extern mod lmath;
extern mod engine;

extern mod std;
use std::json;
use send_map::linear::LinearMap;
use std::serialization::{deserialize,Deserializer,Deserializable};


pub fn load_config<T:Deserializable>( path : ~str )-> T	{
	io::println( ~"Loading config: "+path );
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
	space		: SpaceInfo,
	children	: ~[NodeInfo],
}

pub fn make_nodes( infos : &~[NodeInfo], par : Option<NodeRef>, map : &mut NodeMap )	{
	for infos.each() |inode|	{
		let node = @engine::space::Node{
			name	: copy inode.name,
			space	: inode.space.spawn(),
			parent	: par,
			actions	: ~[],	//TODO
		};
		map.insert( copy inode.name, node );
		make_nodes( &inode.children, Some(node), map );
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Entity

#[auto_deserialize]
struct EntityInfo	{
	node		: ~str,
	material	: ~str,
	mesh		: ~str,
	range		: (uint,uint),
	armature	: ~str,
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Material

pure fn color_to_vec(col : &engine::rast::Color)-> lmath::vector::vec4	{
	lmath::vector::Vec4::new( col.r, col.g, col.b, col.a )
}

#[auto_deserialize]
struct TextureInfo	{
	name	: ~str,
	path	: ~str,
	filter	: uint,
	wrap	: int,
	scale	: (f32,f32),
	offset	: (f32,f32),
}

pub struct ShaderParam	{
	name	: ~str,
	value	: engine::shade::Uniform,
}
impl ShaderParam : Deserializable	{
	static fn deserialize<D:Deserializer>( &self, d : &D )-> ShaderParam	{
		do d.read_rec()	{
			let name : ~str		= d.read_field(~"name",		0u, || {deserialize(d)} );
			let kind : ~str		= d.read_field(~"type",		1u, || {deserialize(d)} );
			let value = if kind==~"scalar"	{
				let v : float	= d.read_field(~"value",	2u, || {deserialize(d)} );
				engine::shade::UniFloat(v)
			}else		if kind==~"color"	{
				let c : uint	= d.read_field(~"value",	2u, || {deserialize(d)} );
				let v = color_to_vec( &engine::rast::make_color(c) );
				engine::shade::UniFloatVec(v)
			}else		if kind==~"vec3"	{
				let (x,y,z) : (f32,f32,f32) = d.read_field(~"value", 2u, || {deserialize(d)} );
				engine::shade::UniFloatVec( lmath::vector::Vec4::new(x,y,z,0f32) )
			}else		if kind==~"vec4"	{
				let (x,y,z,w) : (f32,f32,f32,f32) = d.read_field(~"value", 2u, || {deserialize(d)} );
				engine::shade::UniFloatVec( lmath::vector::Vec4::new(x,y,z,w) )
			}else	{fail ~"Unknown type: "+kind};
			ShaderParam{
				name	: name,
				value	: value,
			}
		}
	}
}


#[auto_deserialize]
struct MaterialInfo	{
	name	: ~str,
	kind	: ~str,
	data	: ~[ShaderParam],
	textures: ~[TextureInfo],
}

type TextureCache = LinearMap<~str,@engine::texture::Texture>;
impl MaterialInfo	{
	fn fill_data( data : &mut engine::shade::DataMap, cache : &TextureCache )	{
		for self.data.each() |par|	{
			data.insert( ~"u"+par.name, copy par.value );
		}
		for self.textures.eachi() |i,tinfo|	{
			let tex = cache.get( &tinfo.name );
			let s_opt = Some(engine::texture::make_sampler( tinfo.filter, tinfo.wrap ));
			data.insert( ~"t_"+tinfo.name, engine::shade::UniTexture(0,tex,s_opt) );
			let (sx,sy) = tinfo.scale, (ox,oy) = tinfo.offset;
			let u_transform = lmath::vector::Vec4::new(sx,sy,ox,oy);
			data.insert( fmt!("u_Tex%uTransform",i), engine::shade::UniFloatVec(u_transform) );
		}
	}
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
	pub fn fill_data( data : &mut engine::shade::DataMap )	{
		data.insert( ~"u_ViewProj",		engine::shade::UniMatrix(false,self.get_matrix()) );
		data.insert( ~"u_CameraPos",	engine::shade::UniFloatVec(self.get_pos_vec4()) );
	}
}

#[auto_deserialize]
pub struct CameraInfo	{
	node	: ~str,
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
	node	: ~str,
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
	meshes		: LinearMap<~str,@engine::mesh::Mesh>,
	armatures	: LinearMap<~str,@engine::space::Armature>,
	mut entities: EntityGroup,
	cameras		: LinearMap<~str,Camera>,
	lights		: LinearMap<~str,Light>,
}

#[auto_deserialize]
pub struct SceneInfo	{
	materials	: ~[MaterialInfo],
	nodes		: ~[NodeInfo],
	entities	: ~[EntityInfo],
	cameras		: ~[CameraInfo],
	lights		: ~[LightInfo],
}


pub fn load_scene( path : ~str, gc : &engine::context::Context,
		opt_vao : Option<@engine::buf::VertexArray>, aspect : float )-> Scene	{
	let scene = load_config::<SceneInfo>( path+~".json" );
	// materials
	let mut tex_cache		= LinearMap::<~str,@engine::texture::Texture>();
	let mut map_material	= LinearMap::<~str,@engine::draw::Material>();
	for scene.materials.each() |imat|	{
		let mat = @engine::draw::load_material( ~"data/code/mat/"+imat.kind );
		map_material.insert( copy imat.name, mat );
		for imat.textures.each() |itex|	{
			if !tex_cache.contains_key( &itex.name )	{
				let path = ~"data/texture/" + itex.path;
				let tex = @engine::load::load_texture_2D( gc, &path, true );
				tex_cache.insert( copy itex.name, tex );
			}
		}
	}
	// nodes
	let mut map_node = LinearMap::<~str,@engine::space::Node>();
	make_nodes( &scene.nodes, None, &mut map_node );
	// meshes
	let mut map_mesh = {
		let mut map = LinearMap::<~str,@engine::mesh::Mesh>();
		let rd = engine::load::create_reader( path+".k3mesh" );
		assert rd.enter() == ~"*mesh";
		while rd.has_more()	{
			assert rd.enter() == ~"meta";
			let name = rd.get_string();
			let mesh = @engine::load::read_mesh( &rd, gc );
			map.insert( name, mesh );
			rd.leave();
		}
		rd.leave();
		map
	};
	// armatures
	let mut map_armature = {
		let mut map = LinearMap::<~str,@engine::space::Armature>();
		let rd = engine::load::create_reader( path+".k3arm" );	
		assert rd.enter() == ~"*arm";
		while rd.has_more()	{
			assert rd.enter() == ~"meta";
			let name = rd.get_string();
			let node_name = rd.get_string();
			let dual_quat = rd.get_bool();
			let root = map_node.get( &node_name );
			let arm = @engine::load::read_armature( &rd, root, dual_quat );
			map.insert( name, arm );
			rd.leave();
		}
		rd.leave();
		map
	};
	// entities
	let mut entity_group : EntityGroup = ~[];
	for scene.entities.each() |ient|	{
		let root = map_node.get( &ient.node );
		let mat = map_material.get( &ient.material );
		let skel = match map_armature.find(&ient.armature)	{
			Some(arm)	=> arm	as @engine::draw::Mod,
			None		=> ()	as @engine::draw::Mod,
		};
		let mut data = engine::shade::create_data();
		for scene.materials.each() |imat|	{
			if imat.name == ient.material	{
				imat.fill_data( &mut data, &tex_cache );
			}
		}
		let (r_min,r_max) = ient.range;
		let ent = engine::draw::Entity{
			node	: root,
			data	: data,
			vao		: match opt_vao	{
					Some(v) => v,
					None	=> @gc.create_vertex_array(),
				},
			mesh	: map_mesh.get( &ient.mesh ),
			range	: engine::mesh::Range{
				start	:r_min,
				num		:r_max-r_min,
				},
			modifier: skel,
			material: mat,
		};
		entity_group.push(ent)
	}
	// cameras
	let mut map_camera = LinearMap::<~str,Camera>();
	for scene.cameras.each() |icam|	{
		let root = map_node.get( &icam.node );
		map_camera.insert( copy root.name, Camera{
			node:root, proj:icam.proj.spawn(aspect),
			ear:engine::audio::Listener{ volume:0f },
		});
	}
	// lights
	let mut map_light = LinearMap::<~str,Light>();
	for scene.lights.each() |ilight|	{
		let root = map_node.get( &ilight.node );
		map_light.insert( copy root.name, Light{
			node:root, proj:ilight.proj.spawn(1f),
		});
	}
	// done
	Scene{
		materials	: map_material,
		nodes		: map_node,
		meshes		: map_mesh,
		armatures	: map_armature,
		entities	: entity_group,
		cameras		: map_camera,
		lights		: map_light,
	}
}