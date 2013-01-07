extern mod cgmath;
extern mod engine;
extern mod lmath;
extern mod numeric;
extern mod std;

use cgmath::projection::*;
use lmath::quat::*;
use lmath::vec::vec3::*;
use lmath::vec::vec4::*;
use numeric::types::*;
use send_map::linear::LinearMap;
use std::json;
use std::serialize::{Decoder,Decodable};


pub fn load_config<T:Decodable<json::Decoder>>( path : ~str )-> T	{
	let rd = match io::file_reader(&path::Path(path))	{
		Ok(reader)	=> reader,
		Err(e)		=> fail e.to_str(),
	};
	match json::from_reader(rd)	{
		Ok(js)	=> Decodable::decode( &json::Decoder(js) ),
		Err(e)	=> fail e.to_str(),
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Space

#[auto_decode]
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
				Vec3::new(x,y,z)
			},
			orientation : {
				let (w,x,y,z) = self.orientation;
				Quat::new(w,x,y,z).normalize()
			},
			scale : self.scale,
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Node

pub type NodeRef = @engine::space::Node;
pub type NodeMap = LinearMap<~str,NodeRef>;

#[auto_decode]
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

#[auto_decode]
struct EntityInfo	{
	node		: ~str,
	material	: ~str,
	mesh		: ~str,
	range		: (uint,uint),
	armature	: ~str,
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Material

pure fn color_to_vec(col : &engine::rast::Color)-> lmath::gltypes::vec4	{
	Vec4::new( col.r, col.g, col.b, col.a )
}

#[auto_decode]
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
impl<D:Decoder> ShaderParam : Decodable<D>	{
	static fn decode( &self, d : &D )-> ShaderParam	{
		do d.read_rec()	{
			let name : ~str		= d.read_field(~"name",		0u, || {Decodable::decode(d)} );
			let kind : ~str		= d.read_field(~"type",		1u, || {Decodable::decode(d)} );
			let value = if kind==~"scalar"	{
				let v : float	= d.read_field(~"value",	2u, || {Decodable::decode(d)} );
				engine::shade::UniFloat(v)
			}else		if kind==~"color"	{
				let c : uint	= d.read_field(~"value",	2u, || {Decodable::decode(d)} );
				let v = color_to_vec( &engine::rast::make_color(c) );
				engine::shade::UniFloatVec(v)
			}else		if kind==~"vec3"	{
				let (x,y,z) : (f32,f32,f32) = d.read_field(~"value", 2u, || {Decodable::decode(d)} );
				engine::shade::UniFloatVec( Vec4::new(x,y,z,0f32) )
			}else		if kind==~"vec4"	{
				let (x,y,z,w) : (f32,f32,f32,f32) = d.read_field(~"value", 2u, || {Decodable::decode(d)} );
				engine::shade::UniFloatVec( Vec4::new(x,y,z,w) )
			}else	{fail ~"Unknown type: "+kind};
			ShaderParam{
				name	: name,
				value	: value,
			}
		}
	}
}


#[auto_decode]
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
			data.insert( ~"u_"+par.name, copy par.value );
		}
		for self.textures.eachi() |i,tinfo|	{
			let tex = cache.get( &tinfo.path );
			let s_opt = Some(engine::texture::make_sampler( tinfo.filter, tinfo.wrap ));
			data.insert( ~"t_"+tinfo.name, engine::shade::UniTexture(0,tex,s_opt) );
			let (sx,sy) = tinfo.scale, (ox,oy) = tinfo.offset;
			let u_transform = Vec4::new(sx,sy,ox,oy);
			data.insert( fmt!("u_Tex%uTransform",i), engine::shade::UniFloatVec(u_transform) );
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Projector

pub type Projector = PerspectiveSym<f32,Degrees<f32>>;

#[auto_decode]
pub struct ProjectorInfo	{
	fov		: float,
	range	: (float,float),
}

impl ProjectorInfo	{
	pure fn spawn( aspect : float )-> Projector	{
		let (r0,r1) = self.range;
		let vfov = Radians( self.fov as f32 );
		PerspectiveSym{
			vfov	: vfov.to_degrees(),
			aspect	: aspect as f32,
			near	: r0 as f32,
			far		: r1 as f32,
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
	pure fn get_matrix()-> lmath::gltypes::mat4	{
		let proj = match self.proj.to_mat4()	{
			Ok(m)	=> m,
			Err(e)	=> fail ~"Camera projection fail: " + e.to_str()
		};
		proj * self.node.world_space().invert().to_matrix()
	}
	pure fn get_view_vector()-> lmath::gltypes::vec3	{
		let v = Vec3::new( 0f32,0f32,-1f32 );
		self.node.world_space().orientation.mul_v( &v )
	}
	pure fn get_up_vector()-> lmath::gltypes::vec3	{
		let v = Vec3::new( 0f32,1f32,0f32 );
		self.node.world_space().orientation.mul_v( &v )
	}
	pure fn get_side_vector()-> lmath::gltypes::vec3	{
		let v = Vec3::new( 1f32,0f32,0f32 );
		self.node.world_space().orientation.mul_v( &v )
	}
	pub fn fill_data( data : &mut engine::shade::DataMap )	{
		let sw = self.node.world_space();
		data.insert( ~"u_ViewProj",		engine::shade::UniMatrix(false,self.get_matrix()) );
		data.insert( ~"u_CameraPos",	engine::shade::UniFloatVec(sw.get_pos_scale()) );
		data.insert( ~"u_CameraRot",	engine::shade::UniFloatVec(sw.get_orientation()) );
	}
}

#[auto_decode]
pub struct CameraInfo	{
	node	: ~str,
	proj	: ProjectorInfo,
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Light

pub struct Light	{
	node	: NodeRef,
	proj	: Projector,
	infinite: bool,
}

impl Light	{
	pub fn get_matrix()-> lmath::gltypes::mat4	{
		let proj = match self.proj.to_mat4()	{
			Ok(m)	=> m,
			Err(e)	=> fail ~"Light projection fail: " + e.to_str()
		};
		proj * self.node.world_space().invert().to_matrix()	
	}
	pub fn fill_data( data : &mut engine::shade::DataMap )	{
		let sw = self.node.world_space();
		let pos = Vec4::new( sw.position.x, sw.position.y, sw.position.z,
			if self.infinite {0f32} else {1f32} );
		data.insert( ~"u_LightProj",	engine::shade::UniMatrix(false,self.get_matrix()) );
		data.insert( ~"u_LightPos",		engine::shade::UniFloatVec(pos) );
	}	
}

#[auto_decode]
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
	textures	: LinearMap<~str,@engine::texture::Texture>,
	nodes		: LinearMap<~str,NodeRef>,
	meshes		: LinearMap<~str,@engine::mesh::Mesh>,
	armatures	: LinearMap<~str,@engine::space::Armature>,
	mut entities: EntityGroup,
	cameras		: LinearMap<~str,Camera>,
	lights		: LinearMap<~str,Light>,
}

#[auto_decode]
pub struct SceneInfo	{
	materials	: ~[MaterialInfo],
	nodes		: ~[NodeInfo],
	entities	: ~[EntityInfo],
	cameras		: ~[CameraInfo],
	lights		: ~[LightInfo],
}


pub fn load_scene( path : ~str, gc : &engine::context::Context,
		opt_vao : Option<@engine::buf::VertexArray>, aspect : float,
		lg : &engine::context::Log )-> Scene	{
	lg.add( ~"Loading scene: " + path );
	let scene = load_config::<SceneInfo>( path + ~".json" );
	let mat_config = load_config::<~[MaterialInfo]>( path + ~".mat.json" );
	// materials
	let mut tex_cache		= LinearMap::<~str,@engine::texture::Texture>();
	let mut map_material	= LinearMap::<~str,@engine::draw::Material>();
	for mat_config.each() |imat|	{
		let mat = @engine::draw::load_material( copy imat.kind );
		map_material.insert( copy imat.name, mat );
		lg.add( ~"\tCustom material: " + imat.name );
		for imat.textures.each() |itex|	{
			if !tex_cache.contains_key( &itex.path )	{
				let tex = @engine::load::load_texture_2D( gc, &itex.path, true );
				tex_cache.insert( copy itex.path, tex );
			}
		}
	}
	for scene.materials.each() |imat|	{
		let mat = @engine::draw::load_material( ~"data/code/mat/" + imat.kind );
		if !map_material.contains_key( &imat.name )	{
			lg.add( ~"\tStandard material: " + imat.name );
			map_material.insert( copy imat.name, mat );
		}
		for imat.textures.each() |itex|	{
			if !tex_cache.contains_key( &itex.path )	{
				let path = ~"data/texture/" + itex.path;
				let tex = match tex_cache.find(&path)	{
					Some(t) => t,
					None	=> @engine::load::load_texture_2D( gc, &path, true ),
				};
				tex_cache.insert( copy itex.path, tex );
			}
		}
	}
	// nodes
	let mut map_node = LinearMap::<~str,@engine::space::Node>();
	make_nodes( &scene.nodes, None, &mut map_node );
	// meshes
	let mut map_mesh = {
		let mut map = LinearMap::<~str,@engine::mesh::Mesh>();
		let rd = engine::load::create_reader_std( path+".k3mesh" );
		assert rd.enter() == ~"*mesh";
		while rd.has_more()!=0u	{
			assert rd.enter() == ~"meta";
			let name = rd.get_string();
			let mesh = @engine::load::read_mesh( &rd, gc, lg );
			map.insert( name, mesh );
			rd.leave();
		}
		rd.leave();
		map
	};
	// armatures
	let mut map_armature = {
		let mut map = LinearMap::<~str,@engine::space::Armature>();
		let rd = engine::load::create_reader_std( path+".k3arm" );	
		assert rd.enter() == ~"*arm";
		while rd.has_more()!=0u	{
			assert rd.enter() == ~"meta";
			let name = rd.get_string();
			let node_name = rd.get_string();
			let dual_quat = rd.get_bool();
			let root = map_node.get( &node_name );
			let arm = @engine::load::read_armature( &rd, root, dual_quat, lg );
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
		let data = {
			let o1 = do vec::position(mat_config)		|mi|	{ mi.name==ient.material };
			let o2 = do vec::position(scene.materials)	|mi|	{ mi.name==ient.material };
			let imat = match (o1,o2)	{
				(Some(p1),_)	=> &mat_config[p1],
				(None,Some(p2))	=> &scene.materials[p2],
				_	=> fail ~"Entity material not found: " + ient.material
			};
			let mut d = engine::shade::make_data();
			imat.fill_data( &mut d, &tex_cache );
			d
		};
		let vao = match opt_vao	{
			Some(v) => v,
			None	=> @gc.create_vertex_array(),
		};
		let mesh = map_mesh.get( &ient.mesh );
		let (r_min,r_max) = ient.range;
		let range = engine::mesh::Range{
			start	:r_min,
			num		:r_max-r_min,
		};
		let ent = engine::draw::Entity{
			node	: root,
			input	: (vao,mesh,range),
			data	: data,
			modifier: skel,
			material: mat,
		};
		entity_group.push(ent)
	}
	// cameras
	let mut map_camera = LinearMap::<~str,Camera>();
	for scene.cameras.each() |icam|	{
		let root = map_node.get( &icam.node );
		map_camera.insert( copy root.name,
			Camera{ node:root,
				proj:icam.proj.spawn(aspect),
				ear:engine::audio::Listener{ volume:0f },
			}
		);
	}
	// lights
	let mut map_light = LinearMap::<~str,Light>();
	for scene.lights.each() |ilight|	{
		let root = map_node.get( &ilight.node );
		map_light.insert( copy root.name,
			Light{ node:root,
				proj:ilight.proj.spawn(1f),
				infinite:false,
			}
		);
	}
	// done
	Scene{
		materials	: map_material,
		textures	: tex_cache,
		nodes		: map_node,
		meshes		: map_mesh,
		armatures	: map_armature,
		entities	: entity_group,
		cameras		: map_camera,
		lights		: map_light,
	}
}