use core::hashmap::linear::LinearMap;
use std::json;
use std::serialize::{Decoder,Decodable};

use numeric::*;
use lmath::quat::*;
use lmath::vec::*;
use cgmath::projection::*;

use engine;
use engine::{gr_low,gr_mid};
use scene::common;


pub fn load_config<T:Decodable<json::Decoder>>( path : ~str )-> T	{
	let rd = match io::file_reader(&path::Path(path))	{
		Ok(reader)	=> reader,
		Err(e)		=> fail!( e.to_str() ),
	};
	match json::from_reader(rd)	{
		Ok(js)	=> Decodable::decode( &json::Decoder(js) ),
		Err(e)	=> fail!( e.to_str() ),
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

pub impl SpaceInfo	{
	fn spawn( &self )-> engine::space::QuatSpace	{
		engine::space::QuatSpace{
			position : {
				let (x,y,z) = self.position;
				vec3::new(x,y,z)
			},
			orientation : {
				let (w,x,y,z) = self.orientation;
				quat::new(w,x,y,z).normalize()
			},
			scale : self.scale,
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Node

pub type NodeRef = @mut engine::space::Node;
pub type NodeMap = LinearMap<~str,NodeRef>;

#[auto_decode]
struct NodeInfo	{
	name		: ~str,
	space		: SpaceInfo,
	children	: ~[NodeInfo],
}

pub fn make_nodes( infos : &~[NodeInfo], par : Option<NodeRef>, map : &mut NodeMap )	{
	for infos.each() |inode|	{
		let node = @mut engine::space::Node{
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
pub struct EntityInfo	{
	node		: ~str,
	material	: ~str,
	mesh		: ~str,
	range		: (uint,uint),
	armature	: ~str,
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Material

fn color_to_vec( col : &gr_low::rast::Color )-> vec4	{
	vec4::new( col.r, col.g, col.b, col.a )
}

#[auto_decode]
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
	fn decode( d : &D )-> ShaderParam	{
		do d.read_struct("param",0)	{//TODO: check this
			let name : ~str		= d.read_field(~"name",		0u, || {Decodable::decode(d)} );
			let kind : ~str		= d.read_field(~"type",		1u, || {Decodable::decode(d)} );
			let value = match kind	{
				~"scalar"	=> {
					let v : float	= d.read_field(~"value",	2u, || {Decodable::decode(d)} );
					gr_low::shade::UniFloat(v)
				},
				~"color"	=> {
					let c : uint	= d.read_field(~"value",	2u, || {Decodable::decode(d)} );
					let v = color_to_vec( &gr_low::rast::Color::new(c) );
					gr_low::shade::UniFloatVec(v)
				},
				~"vec3"		=> {
					let (x,y,z) : (f32,f32,f32) = d.read_field(~"value", 2u, || {Decodable::decode(d)} );
					gr_low::shade::UniFloatVec( vec4::new(x,y,z,0f32) )
				},
				~"vec4"		=> {
					let (x,y,z,w) : (f32,f32,f32,f32) = d.read_field(~"value", 2u, || {Decodable::decode(d)} );
					gr_low::shade::UniFloatVec( vec4::new(x,y,z,w) )
				},
				_	=> fail!( ~"Unknown type: "+kind ),
			};
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

type TextureCache = LinearMap<~str,@gr_low::texture::Texture>;
impl MaterialInfo	{
	fn fill_data( &self, data : &mut gr_low::shade::DataMap, cache : &TextureCache )	{
		for self.data.each() |par|	{
			data.insert( ~"u_"+par.name, copy par.value );
		}
		for self.textures.eachi() |i,tinfo|	{
			let tex = *cache.get( &tinfo.path );
			let s_opt = Some(gr_low::texture::Sampler::new( tinfo.filter, tinfo.wrap ));
			data.insert( ~"t_"+tinfo.name, gr_low::shade::UniTexture(0,tex,s_opt) );
			let (sx,sy,_) = tinfo.scale, (ox,oy,_) = tinfo.offset;
			let u_transform = vec4::new(sx,sy,ox,oy);
			data.insert( fmt!("u_Tex%uTransform",i), gr_low::shade::UniFloatVec(u_transform) );
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Projector

#[auto_decode]
pub struct ProjectorInfo	{
	fov		: float,
	range	: (float,float),
}

pub impl ProjectorInfo	{
	fn spawn( &self )-> common::Projector	{
		let (r0,r1) = self.range;
		let vfov = self.fov as f32;
		PerspectiveSym{
			vfov	: vfov.degrees(),
			aspect	: 1f32,
			near	: r0 as f32,
			far		: r1 as f32,
		}
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Camera

#[auto_decode]
pub struct CameraInfo	{
	node	: ~str,
	proj	: ProjectorInfo,
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Light

#[auto_decode]
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
	for info_array.each() |ient|	{
		let root = match context.nodes.find( &ient.node )	{
			Some(n)	=> *n,
			None	=> fail!( ~"Node not found: " + ient.node )
		};
		let data = match context.mat_data.find( &ient.material )	{
			Some(d)	=> copy *d,
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
		let mut in = gr_mid::call::Input::new( vao, mesh );
		in.range = gr_mid::mesh::Range{
			start	:r_min,
			num		:r_max-r_min,
		};
		let ent = engine::object::Entity{
			node	: root,
			input	: in,
			data	: data,
			modifier: skel,
			material: mat,
		};
		group.push(ent);
	}
	group
}


#[auto_decode]
pub struct SceneInfo	{
	materials	: ~[MaterialInfo],
	nodes		: ~[NodeInfo],
	entities	: ~[EntityInfo],
	cameras		: ~[CameraInfo],
	lights		: ~[LightInfo],
}


pub fn load_scene( path : ~str, gc : &mut gr_low::context::Context,
		opt_vao : Option<@mut gr_low::buf::VertexArray>, lg : &engine::journal::Log )-> common::Scene	{
	lg.add( ~"Loading scene: " + path );
	let c0 = engine::anim::get_time();
	let scene = load_config::<SceneInfo>( path + ~".json" );
	let mat_config = load_config::<~[MaterialInfo]>( path + ~".mat.json" );
	let mut c1 = engine::anim::get_time();
	lg.add(fmt!( "\t[p] Parse config: %f", c1-c0 ));
	// materials
	let mut tex_cache		: LinearMap<~str,@gr_low::texture::Texture>	= LinearMap::new();
	let mut map_material	: LinearMap<~str,@gr_mid::draw::Material>	= LinearMap::new();
	let mut map_data		: LinearMap<~str,gr_low::shade::DataMap>	= LinearMap::new();
	for scene.materials.each() |imat|	{
		let mat = @gr_mid::draw::load_material( ~"data/code/mat/" + imat.kind );
		if !map_material.contains_key( &imat.name )	{
			lg.add( ~"\tStandard material: " + imat.name );
			map_material.insert( copy imat.name, mat );
		}
		for imat.textures.each() |itex|	{
			if !tex_cache.contains_key( &itex.path )	{
				let path = ~"data/texture/" + itex.path;
				let tex = match tex_cache.find(&path)	{
					Some(t) => *t,
					None	=> engine::load::load_texture_2D( gc, &path, true ),
				};
				tex_cache.insert( copy itex.path, tex );
			}
		}
		let mut data = gr_low::shade::make_data();
		imat.fill_data( &mut data, &tex_cache );
		map_data.insert( copy imat.name, data );
	}
	for mat_config.each() |imat|	{
		let mat = @gr_mid::draw::load_material( copy imat.kind );
		map_material.insert( copy imat.name, mat );
		lg.add( ~"\tCustom material: " + imat.name );
		for imat.textures.each() |itex|	{
			if !tex_cache.contains_key( &itex.path )	{
				let tex = engine::load::load_texture_2D( gc, &itex.path, true );
				tex_cache.insert( copy itex.path, tex );
			}
		}
		let mut data = gr_low::shade::make_data();
		imat.fill_data( &mut data, &tex_cache );
		map_data.insert( copy imat.name, data );
	}
	let c2 = engine::anim::get_time();
	lg.add(fmt!( "\t[p] Materials: %f", c2-c1 ));	
	// nodes
	let mut map_node : LinearMap<~str,@mut engine::space::Node> = LinearMap::new();
	make_nodes( &scene.nodes, None, &mut map_node );
	let c3 = engine::anim::get_time();
	lg.add(fmt!( "\t[p] Nodes: %f", c3-c2 ));
	// context
	let mut context = common::SceneContext{
		prefix		: copy path,
		materials	: map_material,
		mat_data	: map_data,
		textures	: tex_cache,
		nodes		: map_node,
		meshes		: LinearMap::new(),
		armatures	: LinearMap::new(),
	};
	// armatures
	context.read_armatures( &path, lg );
	let c4 = engine::anim::get_time();
	lg.add(fmt!( "\t[p] Armatures: %f", c4-c3 ));
	// entities
	let entity_group = parse_group( &mut context, scene.entities, gc, opt_vao, lg );
	// cameras
	let mut map_camera : LinearMap<~str,@common::Camera> = LinearMap::new();
	for scene.cameras.each() |icam|	{
		let root = *context.nodes.get( &icam.node );
		map_camera.insert( copy root.name,
			@common::Camera{ node:root,
				proj:icam.proj.spawn(),
				ear:engine::audio::Listener{ volume:0f },
			}
		);
	}
	// lights
	let mut map_light : LinearMap<~str,@common::Light> = LinearMap::new();
	for scene.lights.each() |ilight|	{
		let root = *context.nodes.get( &ilight.node );
		let (cr,cg,cb) = ilight.color;
		let col = gr_low::rast::Color{ r:cr, g:cg, b:cb, a:1f32 };
		let data = match ilight.kind	{
			~"POINT"=> common::LiPoint,
			~"SUN"	=> common::LiSun,
			~"SPOT"	=> common::LiSpot( (ilight.params[0] as f32).degrees(),
				ilight.params[1] ),
			~"HEMI"	=> common::LiHemi,
			~"AREA"	=> common::LiArea( vec2::new(ilight.params[0] as f32, ilight.params[1] as f32),
				ilight.params[2] ),
			_	=> fail!( ~"Unknown light type: " + ilight.kind ),
		};
		let (a1,a2) = ilight.attenu;
		map_light.insert( copy root.name, @common::Light{
			node	: root,
			color	: col,
			attenu	: [1f/ilight.energy as f32,a1 as f32,a2 as f32],
			distance: ilight.distance as f32,
			bounded	: ilight.sphere,
			kind	: data,
		});
	}
	let c5 = engine::anim::get_time();
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
