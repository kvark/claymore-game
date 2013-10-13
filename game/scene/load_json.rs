use std::{io,path};
use std::hashmap::HashMap;
use extra::json;
use extra::serialize::{Decoder,Decodable};

use cgmath::{angle,projection};
use cgmath::quaternion::*;
use cgmath::vector::*;

use engine;
use engine::{gr_low,gr_mid};
use scene::common;


pub fn load_config<T:Decodable<json::Decoder>>( path : &str )-> T	{
	let rd = match io::file_reader(&path::Path(path))	{
		Ok(reader)	=> reader,
		Err(e)		=> fail!( e.to_str() ),
	};
	match json::from_reader(rd)	{
		Ok(js)	=> Decodable::decode( &mut json::Decoder(js) ),
		Err(e)	=> fail!( e.to_str() ),
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Space

#[deriving(Decodable)]
struct SpaceInfo	{
	position	: (f32,f32,f32),
	orientation	: (f32,f32,f32,f32),
	scale		: f32,
}

impl SpaceInfo	{
	pub fn spawn( &self )-> engine::space::QuatSpace	{
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

pub type NodeRef = @mut engine::space::Node;
pub type NodeMap = HashMap<~str,NodeRef>;

#[deriving(Decodable)]
struct NodeInfo	{
	name		: ~str,
	space		: SpaceInfo,
	children	: ~[NodeInfo],
}

pub fn make_nodes( infos : &~[NodeInfo], par : Option<NodeRef>, map : &mut NodeMap )	{
	for inode in infos.iter()	{
		let node = @mut engine::space::Node{
			name	: inode.name.clone(),
			space	: inode.space.spawn(),
			parent	: par,
			actions	: ~[],	//TODO
		};
		map.insert( inode.name.clone(), node );
		make_nodes( &inode.children, Some(node), map );
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Entity

#[deriving(Decodable)]
pub struct EntityInfo	{
	node		: ~str,
	material	: ~str,
	mesh		: ~str,
	range		: (uint,uint),
	armature	: ~str,
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Material

fn color_to_vec( col : &gr_low::rast::Color )-> Vec4<f32>	{
	Vec4::new( col.r, col.g, col.b, col.a )
}

#[deriving(Decodable)]
struct TextureInfo	{
	name	: ~str,
	path	: ~str,
	filter	: uint,
	wrap	: int,
	scale	: (f32,f32,f32),	//temp!
	offset	: (f32,f32,f32),
}

pub struct ShaderParam	{
	name	: ~str,
	value	: gr_low::shade::Uniform,
}

impl<D:Decoder> Decodable<D> for ShaderParam	{
	fn decode( dec : &mut D )-> ShaderParam	{
		do dec.read_struct("param",0)	|d| {//TODO: check this
			let name : ~str		= d.read_struct_field("name",	0u, Decodable::decode );
			let kind : ~str		= d.read_struct_field("type",	1u, Decodable::decode );
			let value = match kind	{
				~"scalar"	=> {
					let v : float	= d.read_struct_field("value",	2u, Decodable::decode );
					gr_low::shade::UniFloat(v)
				},
				~"color"	=> {
					let c : uint	= d.read_struct_field("value",	2u, Decodable::decode );
					let v = color_to_vec( &gr_low::rast::Color::new(c) );
					gr_low::shade::UniFloatVec(v)
				},
				~"vec3"		=> {
					let (x,y,z) : (f32,f32,f32) = d.read_struct_field("value", 2u, Decodable::decode );
					gr_low::shade::UniFloatVec( Vec4::new(x,y,z,0f32) )
				},
				~"vec4"		=> {
					let (x,y,z,w) : (f32,f32,f32,f32) = d.read_struct_field("value", 2u, Decodable::decode );
					gr_low::shade::UniFloatVec( Vec4::new(x,y,z,w) )
				},
				_	=> fail!( "Unknown type: "+kind ),
			};
			ShaderParam{
				name	: name,
				value	: value,
			}
		}
	}
}


#[deriving(Decodable)]
struct MaterialInfo	{
	name	: ~str,
	kind	: ~str,
	data	: ~[ShaderParam],
	textures: ~[TextureInfo],
}

type TextureCache = HashMap<~str,@gr_low::texture::Texture>;
impl MaterialInfo	{
	fn fill_data( &self, data : &mut gr_low::shade::DataMap, cache : &TextureCache )	{
		for par in self.data.iter()	{
			data.insert( ~"u_"+par.name, par.value.clone() );
		}
		for (i,tinfo) in self.textures.iter().enumerate()	{
			let tex = *cache.get( &tinfo.path );
			let s_opt = Some(gr_low::texture::Sampler::new( tinfo.filter, tinfo.wrap ));
			data.insert( ~"t_"+tinfo.name, gr_low::shade::UniTexture(0,tex,s_opt) );
			let (sx,sy,_) = tinfo.scale;
			let (ox,oy,_) = tinfo.offset;
			let u_transform = Vec4::new(sx,sy,ox,oy);
			data.insert( fmt!("u_Tex%uTransform",i), gr_low::shade::UniFloatVec(u_transform) );
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Projector

#[deriving(Decodable)]
pub struct ProjectorInfo	{
	fov		: float,
	range	: (float,float),
}

impl ProjectorInfo	{
	pub fn spawn( &self )-> common::Projector	{
		let (r0,r1) = self.range;
		let vfov = angle::rad( self.fov as f32 );
		projection::PerspectiveFov{
			fovy	: vfov,
			aspect	: 1f32,
			near	: r0 as f32,
			far		: r1 as f32,
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Camera

#[deriving(Decodable)]
pub struct CameraInfo	{
	node	: ~str,
	proj	: ProjectorInfo,
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Light

#[deriving(Decodable)]
pub struct LightInfo	{
	node	: ~str,
	kind	: ~str,
	color	: (f32,f32,f32),
	distance: float,
	energy	: float,
	attenu	: (float,float),
	sphere	: bool,
	params	: ~[float],
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Scene

pub fn parse_group( context : &mut common::SceneContext,
		info_array	: &[EntityInfo],
		gc			: &mut gr_low::context::Context,
		opt_vao		: Option<@mut gr_low::buf::VertexArray>,
		lg			: &engine::journal::Log
		)-> common::EntityGroup	{
	let mut group = common::EntityGroup(~[]);
	for ient in info_array.iter()	{
		let root = match context.nodes.find( &ient.node )	{
			Some(n)	=> *n,
			None	=> fail!( ~"Node not found: " + ient.node )
		};
		let data = match context.mat_data.find( &ient.material )	{
			Some(d)	=> d.clone(),
			None	=> fail!( ~"Material data not found: " + ient.material )
		};
		let mat = match context.materials.find( &ient.material )	{
			Some(m)	=> *m,
			None	=> fail!( ~"Material not found: " + ient.material )
		};
		let skel = if ient.armature.is_empty()	{
			@()	as @gr_mid::draw::Mod
		}else	{
			*context.armatures.get(&ient.armature)	as @gr_mid::draw::Mod
		};
		let vao = match opt_vao	{
			Some(v) => v,
			None	=> gc.create_vertex_array(),
		};
		let mesh = context.query_mesh( &ient.mesh, gc, lg );
		let (r_min,r_max) = ient.range;
		let mut inp = gr_mid::call::Input::new( vao, mesh );
		inp.range = gr_mid::mesh::Range{
			start	:r_min,
			num		:r_max-r_min,
		};
		let ent = engine::object::Entity{
			node	: root,
			input	: inp,
			data	: data,
			modifier: skel,
			material: mat,
		};
		group.push(ent);
	}
	group
}


#[deriving(Decodable)]
pub struct SceneInfo	{
	materials	: ~[MaterialInfo],
	nodes		: ~[NodeInfo],
	entities	: ~[EntityInfo],
	cameras		: ~[CameraInfo],
	lights		: ~[LightInfo],
}


pub fn load_scene( path : &str, gc : &mut gr_low::context::Context,
		opt_vao : Option<@mut gr_low::buf::VertexArray>, lg : &engine::journal::Log )-> common::Scene	{
	lg.add( "Loading scene: " + path );
	let c0 = engine::load::get_time();
	let scene = load_config::<SceneInfo>( path + ".json" );
	let mat_config = load_config::<~[MaterialInfo]>( path + ".mat.json" );
	let c1 = engine::load::get_time();
	lg.add(fmt!( "\t[p] Parse config: %f", c1-c0 ));
	// materials
	let mut tex_cache		: HashMap<~str,@gr_low::texture::Texture>	= HashMap::new();
	let mut map_material	: HashMap<~str,@gr_mid::draw::Material>	= HashMap::new();
	let mut map_data		: HashMap<~str,gr_low::shade::DataMap>	= HashMap::new();
	for imat in scene.materials.iter()	{
		let mat = @gr_mid::draw::load_material( "data/code/mat/" + imat.kind );
		if !map_material.contains_key( &imat.name )	{
			lg.add( ~"\tStandard material: " + imat.name );
			map_material.insert( imat.name.clone(), mat );
		}
		for itex in imat.textures.iter()	{
			if !tex_cache.contains_key( &itex.path )	{
				let path = ~"data/texture/" + itex.path;
				let tex = match tex_cache.find(&path)	{
					Some(t) => *t,
					None	=> engine::load::load_texture_2D( gc, path, true ),
				};
				tex_cache.insert( itex.path.clone(), tex );
			}
		}
		let mut data = gr_low::shade::DataMap::new();
		imat.fill_data( &mut data, &tex_cache );
		map_data.insert( imat.name.clone(), data );
	}
	for imat in mat_config.iter()	{
		let mat = @gr_mid::draw::load_material( imat.kind.clone() );
		map_material.insert( imat.name.clone(), mat );
		lg.add( ~"\tCustom material: " + imat.name );
		for itex in imat.textures.iter()	{
			if !tex_cache.contains_key( &itex.path )	{
				let tex = engine::load::load_texture_2D( gc, itex.path, true );
				tex_cache.insert( itex.path.clone(), tex );
			}
		}
		let mut data = gr_low::shade::DataMap::new();
		imat.fill_data( &mut data, &tex_cache );
		map_data.insert( imat.name.clone(), data );
	}
	let c2 = engine::load::get_time();
	lg.add(fmt!( "\t[p] Materials: %f", c2-c1 ));	
	// nodes
	let mut map_node : HashMap<~str,@mut engine::space::Node> = HashMap::new();
	make_nodes( &scene.nodes, None, &mut map_node );
	let c3 = engine::load::get_time();
	lg.add(fmt!( "\t[p] Nodes: %f", c3-c2 ));
	// context
	let mut context = common::SceneContext{
		prefix		: path.to_owned(),
		materials	: map_material,
		mat_data	: map_data,
		textures	: tex_cache,
		nodes		: map_node,
		meshes		: HashMap::new(),
		armatures	: HashMap::new(),
		actions		: HashMap::new(),
	};
	// armatures
	context.read_armatures( path, lg );
	let c4 = engine::load::get_time();
	lg.add(fmt!( "\t[p] Armatures: %f", c4-c3 ));
	// entities
	let entity_group = parse_group( &mut context, scene.entities, gc, opt_vao, lg );
	// cameras
	let mut map_camera : HashMap<~str,@common::Camera> = HashMap::new();
	for icam in scene.cameras.iter()	{
		let root = *context.nodes.get( &icam.node );
		map_camera.insert( root.name.clone(),
			@common::Camera{ node:root,
				proj:icam.proj.spawn(),
				ear:engine::audio::Listener{ volume:0f },
			}
		);
	}
	// lights
	let mut map_light : HashMap<~str,@common::Light> = HashMap::new();
	for ilight in scene.lights.iter()	{
		let root = *context.nodes.get( &ilight.node );
		let (cr,cg,cb) = ilight.color;
		let col = gr_low::rast::Color{ r:cr, g:cg, b:cb, a:1f32 };
		let data = match ilight.kind	{
			~"POINT"=> common::LiPoint,
			~"SUN"	=> common::LiSun,
			~"SPOT"	=> common::LiSpot( angle::rad(ilight.params[0] as f32),
				ilight.params[1] ),
			~"HEMI"	=> common::LiHemi,
			~"AREA"	=> common::LiArea( Vec2::new(ilight.params[0] as f32, ilight.params[1] as f32),
				ilight.params[2] ),
			_	=> fail!( ~"Unknown light type: " + ilight.kind ),
		};
		let (a1,a2) = ilight.attenu;
		map_light.insert( root.name.clone(), @common::Light{
			node	: root,
			color	: col,
			attenu	: [1f32/ilight.energy as f32,a1 as f32,a2 as f32],
			distance: ilight.distance as f32,
			bounded	: ilight.sphere,
			kind	: data,
		});
	}
	let c5 = engine::load::get_time();
	lg.add(fmt!( "\t[p] Objects: %f", c5-c4 ));
	lg.add(fmt!( "\t[p] Total: %f", c5-c0 ));
	// done
	common::Scene{
		context		: context,
		entities	: entity_group,
		cameras		: map_camera,
		lights		: map_light,
	}
}
