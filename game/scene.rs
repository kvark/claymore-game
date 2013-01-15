extern mod cgmath;
extern mod engine;
extern mod lmath;
extern mod numeric;
extern mod std;

use cgmath::projection::*;
use lmath::quat::*;
use lmath::vec::vec2::*;
use lmath::vec::vec3::*;
use lmath::vec::vec4::*;
use lmath::mat::mat4::Mat4;
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
pub struct EntityInfo	{
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

pub enum LightKind	{
	LiPoint,
	LiHemi,
	LiSun,
	LiSpot(angle::Degrees<f32>,float),
	LiArea(Vec2<f32>,float),
}

pub struct Light	{
	node	: NodeRef,
	color	: engine::rast::Color,
	attenu	: Vec4<f32>,
	kind	: LightKind,
}

pub type ProjectorBlend = (@Projection<f32>,float);

impl Light	{
	pub pure fn get_far_distance( threshold : f32 )-> f32	{
		assert self.attenu.w == 0f32;
		let E = self.attenu.x, a1 = self.attenu.y, a2 = self.attenu.z;
		if a2>0f32 {
			assert a1>=0f32;
			let D = a1*a1 - 4f32*a2*(1f32 - E/threshold);
			if D>=0f32	{
				0.5f32 * (core::f32::sqrt(D) - a1) / a2
			}else	{fail ~"Bad attenuation: " + self.attenu.to_string()}
		}else if a1>0f32	{
			assert a2==0f32;
			(E/threshold - 1f32) / a1
		}else	{
			0f32
		}
	}
	pub fn get_proj_blend( near:f32, far:f32 )-> Option<(Result<Mat4<f32>,~str>,float)>	{
		match self.kind	{
			LiSpot(angle,blend)	=>	{
				let proj = PerspectiveSym{ vfov:angle, aspect:1f32, near:near, far:far };
				Some((proj.to_mat4(),blend))
			},
			LiArea(v2,blend)	=>	{
				let proj = Ortho{ left:-v2.x, right:v2.x,
					bottom:-v2.y, top:v2.y,
					near:near, far:far };
				Some((proj.to_mat4(),blend))
			},
			_	=> None
		}
	}
	pub fn fill_data( data : &mut engine::shade::DataMap, near : f32, far : f32 )	{
		let sw = self.node.world_space();
		let mut pos = Vec4::new( sw.position.x, sw.position.y, sw.position.z, 1f32 );
		let col = Vec4::new( self.color.r, self.color.g, self.color.b, self.color.a );
		//io::println(fmt!("Light near:%f far:%f",near as float,far as float));
		match self.kind	{
			LiSun	=>	{ pos.w = 0f32; },
			_	=> ()
		}
		match self.get_proj_blend(near,far)	{
			Some(ref pair)	=>	{
				let &(opt_mp,blend) = pair;
				let mp = match opt_mp	{
					Ok(m)	=> m,
					Err(e)	=> fail ~"Light projection fail: " + e.to_str()
				};
				let ml = mp * self.node.world_space().invert().to_matrix();
				data.insert( ~"u_LightProj",	engine::shade::UniMatrix(false,ml) );
				data.insert( ~"u_LightBlend",	engine::shade::UniFloat(blend) );
			},
			None	=> ()
		}
		data.insert( ~"u_LightPos",			engine::shade::UniFloatVec(pos) );
		data.insert( ~"u_LightColor",		engine::shade::UniFloatVec(col) );
		data.insert( ~"u_LightAttenuation",	engine::shade::UniFloatVec(self.attenu) );
	}	
}

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

pub enum EntityGroup = ~[engine::draw::Entity];
impl EntityGroup	{
	pub fn divide( &mut self, name : &~str )-> EntityGroup	{
		let mut i = 0u;
		let mut rez = EntityGroup(~[]);
		while i<self.len()	{
			if self[i].node.is_under(name)	{
				rez.push( self.swap_remove(i) );
			}else	{
				i += 1u;
			}
		}
		rez	
	}
	pub fn change_detail( &mut self, detail : engine::draw::Entity )-> Option<engine::draw::Entity>	{
		let position = do self.position() |ent|	{managed::ptr_eq(ent.node,detail.node)};
		self.push( detail );
		match position	{
			Some(pos)	=> Some( self.swap_remove(pos) ),
			None		=> None,
		}
	}
}

pub type Dict<T>		= LinearMap<~str,T>;

pub struct SceneContext	{
	prefix		: ~str,
	materials	: Dict<@engine::draw::Material>,
	mat_data	: Dict<engine::shade::DataMap>,
	textures	: Dict<@engine::texture::Texture>,
	nodes		: Dict<NodeRef>,
	meshes		: Dict<@engine::mesh::Mesh>,
	armatures	: Dict<@engine::space::Armature>,
}

impl SceneContext	{
	pub fn query_mesh( &mut self, mesh_name : &~str, gc : &engine::context::Context,
			lg : &engine::context::Log )-> @engine::mesh::Mesh	{
		if !self.meshes.contains_key(mesh_name)	{
			let split = mesh_name.split_char('@');
			let path = self.prefix + split[split.len()-1u] + ".k3mesh";
			let rd = engine::load::create_reader_std( path );
			if split.len() > 1	{
				assert rd.enter() == ~"*mesh";
				while rd.has_more()!=0u	{
					assert rd.enter() == ~"meta";
					let name = rd.get_string();
					let mesh = @engine::load::read_mesh( &rd, gc, lg );
					rd.leave();
					let full_name = fmt!( "%s@%s", name, split[1] );
					self.meshes.insert( full_name, mesh );
				}
				rd.leave();
			}else	{
				let mesh = @engine::load::read_mesh( &rd, gc, lg );
				self.meshes.insert( copy *mesh_name, mesh );
				return mesh
			}
		}
		self.meshes.get(mesh_name)
	}
	pub fn parse_group( &mut self, info_array : &[EntityInfo],
			gc			: &engine::context::Context,
			opt_vao		: Option<@engine::buf::VertexArray>,
			lg			: &engine::context::Log
			)-> EntityGroup	{
		let mut group = EntityGroup(~[]);
		for info_array.each() |ient|	{
			let root = self.nodes.get( &ient.node );
			let data = match self.mat_data.find_ref( &ient.material )	{
				Some(d)	=> copy *d,
				None	=> fail ~"Material data not found: " + ient.material
			};
			let mat = self.materials.get( &ient.material );
			let skel = if ient.armature.is_empty()	{
				()	as @engine::draw::Mod
			}else	{
				self.armatures.get(&ient.armature)	as @engine::draw::Mod
			};
			let vao = match opt_vao	{
				Some(v) => v,
				None	=> @gc.create_vertex_array(),
			};
			let mesh = self.query_mesh( &ient.mesh, gc, lg );
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
			group.push(ent);
		}
		group
	}
}

pub struct Scene	{
	context		: SceneContext,
	entities	: EntityGroup,
	cameras		: Dict<Camera>,
	lights		: Dict<Light>,
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
	let mut map_data		= LinearMap::<~str,engine::shade::DataMap>();
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
		let mut data = engine::shade::make_data();
		imat.fill_data( &mut data, &tex_cache );
		map_data.insert( copy imat.name, data );
	}
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
		let mut data = engine::shade::make_data();
		imat.fill_data( &mut data, &tex_cache );
		map_data.insert( copy imat.name, data );
	}
	// nodes
	let mut map_node = LinearMap::<~str,@engine::space::Node>();
	make_nodes( &scene.nodes, None, &mut map_node );
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
	// context
	let mut context = SceneContext{
		prefix		: copy path,
		materials	: map_material,
		mat_data	: map_data,
		textures	: tex_cache,
		nodes		: map_node,
		meshes		: LinearMap::<~str,@engine::mesh::Mesh>(),
		armatures	: map_armature,
	};
	// entities
	let entity_group = context.parse_group( scene.entities, gc, opt_vao, lg );
	// cameras
	let mut map_camera = LinearMap::<~str,Camera>();
	for scene.cameras.each() |icam|	{
		let root = context.nodes.get( &icam.node );
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
		let root = context.nodes.get( &ilight.node );
		let (cr,cg,cb) = ilight.color;
		let col = engine::rast::Color{ r:cr, g:cg, b:cb, a:1f32 };
		let data = match ilight.kind	{
			~"POINT"=> LiPoint,
			~"SUN"	=> LiSun,
			~"SPOT"	=> LiSpot( angle::Radians(ilight.params[0] as f32).to_degrees(),
				ilight.params[1] ),
			~"HEMI"	=> LiHemi,
			~"AREA"	=> LiArea( Vec2::new(ilight.params[0] as f32, ilight.params[1] as f32),
				ilight.params[2] ),
			_	=> fail ~"Unknown light type: " + ilight.kind
		};
		let (a1,a2) = ilight.attenu;
		assert ilight.distance>0f;
		let kd = 1f / ilight.distance;
		let attenu = Vec4::new( ilight.energy as f32,
			a1*kd as f32, a2*kd*kd as f32,
			if ilight.sphere {kd as f32} else {0f32}
			);
		map_light.insert( copy root.name,
			Light{ node:root,
				color:col,
				attenu:attenu,
				kind:data,
			}
		);
	}
	// done
	Scene{
		context		: context,
		entities	: entity_group,
		cameras		: map_camera,
		lights		: map_light,
	}
}