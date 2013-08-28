extern mod cgmath;
extern mod engine;
extern mod lmath;
extern mod numeric;
extern mod std;

use core::hashmap::linear::LinearMap;
use core::managed;
use core::to_str::ToStr;
use std::json;
use std::serialize::{Decoder,Decodable};


use numeric::*;
use lmath::quat::*;
use lmath::vec::*;
use lmath::mat::*;
use lmath::projection::*;
use cgmath::projection::*;

use engine::{gr_low,gr_mid};
use engine::space::Space;


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

pub type Projector = PerspectiveSym<f32>;

#[auto_decode]
pub struct ProjectorInfo	{
	fov		: float,
	range	: (float,float),
}

pub impl ProjectorInfo	{
	fn spawn( &self, aspect : float )-> Projector	{
		let (r0,r1) = self.range;
		let vfov = self.fov as f32;
		PerspectiveSym{
			vfov	: vfov.degrees(),
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

pub impl Camera	{
	fn get_proj_matrix( &self )-> mat4	{
		//self.proj.to_mat4().unwrap()	//ICE on Rust-0.6
		let p = &self.proj;
		lmath::projection::perspective( p.vfov, p.aspect, p.near, p.far )
	}
	fn get_matrix( &self )-> mat4	{
		let proj = self.get_proj_matrix();
		//proj * self.node.world_space().invert().to_matrix()
		proj.mul_m( &self.node.world_space().invert().to_matrix() )
	}
	fn get_inverse_matrix( &self )-> mat4	{
		let proj = self.get_proj_matrix();
		//self.node.world_space().to_matrix() * proj.invert()
		self.node.world_space().to_matrix().mul_m( &proj.invert() )
	}
	fn get_view_vector( &self )-> vec3	{
		let v = vec3::new( 0f32,0f32,-1f32 );
		self.node.world_space().orientation.mul_v( &v )
	}
	fn get_up_vector( &self )-> vec3	{
		let v = vec3::new( 0f32,1f32,0f32 );
		self.node.world_space().orientation.mul_v( &v )
	}
	fn get_side_vector( &self )-> vec3	{
		let v = vec3::new( 1f32,0f32,0f32 );
		self.node.world_space().orientation.mul_v( &v )
	}
	fn fill_data( &self, data : &mut gr_low::shade::DataMap )	{
		let sw = self.node.world_space();
		data.insert( ~"u_ViewProj",		gr_low::shade::UniMatrix(false,self.get_matrix()) );
		data.insert( ~"u_CameraPos",	gr_low::shade::UniFloatVec(sw.get_pos_scale()) );
		data.insert( ~"u_CameraRot",	gr_low::shade::UniFloatVec(sw.get_orientation()) );
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
	LiSpot(f32,float),
	LiArea(Vec2<f32>,float),
}

pub struct Light	{
	node	: NodeRef,
	color	: gr_low::rast::Color,
	attenu	: [f32, ..3],
	distance: f32,
	bounded	: bool,
	kind	: LightKind,
}

pub type ProjectorBlend = (@Projection<f32>,float);

pub impl Light	{
	fn get_attenuation( &self )-> Vec4<f32>	{
		assert!( self.distance>0f32 );
		let kd = 1f32 / self.distance;
		vec4::new( 1f32 / self.attenu[0],
			self.attenu[1]*kd, self.attenu[2]*kd*kd,
			if self.bounded {kd} else {0f32}
			)
	}
	fn get_far_distance( &self, threshold : f32 )-> f32	{
		if self.bounded	{
			return self.distance
		}
		let kE = (1f32 - 1f32 / (self.attenu[0] * threshold));
		if self.attenu[2]>0f32 {
			assert!( self.attenu[1]>=0f32 );
			let D = self.attenu[1]*self.attenu[1] - 4f32*self.attenu[2]*kE;
			if D>=0f32	{
				0.5f32 * (f32::sqrt(D) - self.attenu[1]) / self.attenu[2]
			}else	{fail!( ~"Bad attenuation: " /*+ self.attenu.to_str()*/ )}
		}else if self.attenu[1]>0f32	{
			assert!( self.attenu[2] == 0f32 );
			-kE / self.attenu[1]
		}else	{
			0f32
		}
	}

	fn get_proj_blend( &self, near:f32, far:f32 )-> Option<(mat4,float)>	{
		match self.kind	{
			LiSpot(angle,blend)	=>	{
				let m = perspective( angle, 1f32, near, far );
				Some(( m, blend ))
			},
			LiArea(v2,blend)	=>	{
				let m = ortho( -v2.x, v2.x, -v2.y, v2.y, near, far );
				Some(( m, blend ))
			},
			_	=> None
		}
	}

	fn fill_data( &self, data : &mut gr_low::shade::DataMap, near : f32, far : f32 )	{
		let sw = self.node.world_space();
		let mut pos = vec4::new( sw.position.x, sw.position.y, sw.position.z, 1f32 );
		let col = vec4::new( self.color.r, self.color.g, self.color.b, self.color.a );
		let range = vec4::new( near, far, 0f32, 1f32/(far-near) );
		//io::println(fmt!("Light near:%f far:%f",near as float,far as float));
		match self.kind	{
			LiSun	=>	{ pos.w = 0f32; },
			_	=> ()
		}
		match self.get_proj_blend(near,far)	{
			Some(ref pair)	=>	{
				let &(mp,blend) = pair;
				//let ml = mp * self.node.world_space().invert().to_matrix();
				let ml = mp.mul_m( &self.node.world_space().invert().to_matrix() );
				data.insert( ~"u_LightProj",	gr_low::shade::UniMatrix(false,ml) );
				data.insert( ~"u_LightBlend",	gr_low::shade::UniFloat(blend) );
			},
			None	=> ()
		}
		data.insert( ~"u_LightPos",			gr_low::shade::UniFloatVec(pos) );
		data.insert( ~"u_LightColor",		gr_low::shade::UniFloatVec(col) );
		let vat = self.get_attenuation();
		data.insert( ~"u_LightAttenuation",	gr_low::shade::UniFloatVec(vat) );
		data.insert( ~"u_LightRange",		gr_low::shade::UniFloatVec(range) );
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

pub struct EntityGroup( ~[engine::object::Entity] );

pub impl EntityGroup	{
	fn divide( &mut self, name : &~str )-> EntityGroup	{
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
	//pub fn with<T>( &mut self, name : &~str, fun : fn(&mut engine::object::Entity)->T )-> Option<T>	{
	//	let opt_pos = do self.position() |ent|	{ent.node.name == *name};
	//	match opt_pos	{
	//		Some(p)	=> Some( fun(&mut self[p]) ),
	//		None	=> None,
	//	}
	//}
	pub fn change_detail( &mut self, detail : engine::object::Entity )-> Option<engine::object::Entity>	{
		let opt_pos = self.position( |ent|	{managed::mut_ptr_eq(ent.node,detail.node)} );
		self.push( detail );
		match opt_pos	{
			Some(pos)	=> Some( self.swap_remove(pos) ),
			None		=> None,
		}
	}
	pub fn swap_entity( &mut self, name : &~str, other : &mut EntityGroup )	{
		let opt_pos = other.position( |ent|	{ent.node.name == *name} );
		let e1 = other.swap_remove( opt_pos.expect(~"Remote entity not found: " + *name) );
		let e2 = self.change_detail( e1 ).expect(	~"Local entity not found: " + *name);
		other.push(e2);
	}
}

pub type Dict<T>		= LinearMap<~str,T>;

pub struct SceneContext	{
	prefix		: ~str,
	materials	: Dict<@gr_mid::draw::Material>,
	mat_data	: Dict<gr_low::shade::DataMap>,
	textures	: Dict<@gr_low::texture::Texture>,
	nodes		: Dict<NodeRef>,
	meshes		: Dict<@gr_mid::mesh::Mesh>,
	armatures	: Dict<@mut engine::space::Armature>,
}

pub impl SceneContext	{
	fn new( prefix : ~str )-> SceneContext	{
		SceneContext	{
			prefix		: prefix,
			materials	: LinearMap::new(),
			mat_data	: LinearMap::new(),
			textures	: LinearMap::new(),
			nodes		: LinearMap::new(),
			meshes		: LinearMap::new(),
			armatures	: LinearMap::new(),
		}
	}
	fn query_mesh( &mut self, mesh_name : &~str, gc : &mut gr_low::context::Context,
			lg : &engine::journal::Log )-> @gr_mid::mesh::Mesh	{
		match self.meshes.find(mesh_name)	{
			Some(m)	=> return *m,
			None	=> (),
		};
		let split = vec::build(|push|	{
			mesh_name.each_split_char('@', |word|	{
				push( word.to_owned() ); true
			});
		});
		let path = self.prefix + split[split.len()-1u] + ~".k3mesh";
		let mut rd = engine::load::Reader::create_std( path );
		let mut q_mesh = None::<@gr_mid::mesh::Mesh>;
		if split.len() > 1	{
			assert!( rd.enter() == ~"*mesh" );
			while rd.has_more()!=0u	{
				assert!( rd.enter() == ~"meta" );
				let name = rd.get_string();
				let mesh = @engine::load::read_mesh( &mut rd, gc, lg );
				rd.leave();
				let full_name = fmt!( "%s@%s", name, split[1] );
				if full_name == *mesh_name	{
					q_mesh = Some(mesh);
				}
				self.meshes.insert( full_name, mesh );
			}
			rd.leave();
			q_mesh.expect(fmt!( "Mesh '%s' not found in the collection", *mesh_name ))
		}else	{
			let mesh = @engine::load::read_mesh( &mut rd, gc, lg );
			self.meshes.insert( copy *mesh_name, mesh );
			mesh
		}
	}

	fn parse_group( &mut self, info_array : &[EntityInfo],
			gc			: &mut gr_low::context::Context,
			opt_vao		: Option<@mut gr_low::buf::VertexArray>,
			lg			: &engine::journal::Log
			)-> EntityGroup	{
		let mut group = EntityGroup(~[]);
		for info_array.each() |ient|	{
			let root = match self.nodes.find( &ient.node )	{
				Some(n)	=> *n,
				None	=> fail!( ~"Node not found: " + ient.node )
			};
			let data = match self.mat_data.find( &ient.material )	{
				Some(d)	=> copy *d,
				None	=> fail!( ~"Material data not found: " + ient.material )
			};
			let mat = match self.materials.find( &ient.material )	{
				Some(m)	=> *m,
				None	=> fail!( ~"Material not found: " + ient.material )
			};
			let skel = if ient.armature.is_empty()	{
				@()	as @gr_mid::draw::Mod
			}else	{
				*self.armatures.get(&ient.armature)	as @gr_mid::draw::Mod
			};
			let vao = match opt_vao	{
				Some(v) => v,
				None	=> gc.create_vertex_array(),
			};
			let mesh = self.query_mesh( &ient.mesh, gc, lg );
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
}

pub struct Scene	{
	context		: SceneContext,
	entities	: EntityGroup,
	cameras		: Dict<@Camera>,
	lights		: Dict<@Light>,
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
		opt_vao : Option<@mut gr_low::buf::VertexArray>, aspect : float,
		lg : &engine::journal::Log )-> Scene	{
	lg.add( ~"Loading scene: " + path );
	let scene = load_config::<SceneInfo>( path + ~".json" );
	let mat_config = load_config::<~[MaterialInfo]>( path + ~".mat.json" );
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
	// nodes
	let mut map_node : LinearMap<~str,@mut engine::space::Node> = LinearMap::new();
	make_nodes( &scene.nodes, None, &mut map_node );
	// armatures
	let mut map_armature = {
		let mut map : LinearMap<~str,@mut engine::space::Armature> = LinearMap::new();
		let mut rd = engine::load::Reader::create_std( path+".k3arm" );	
		assert!( rd.enter() == ~"*arm" );
		while rd.has_more()!=0u	{
			assert!( rd.enter() == ~"meta" );
			let name = rd.get_string();
			let node_name = rd.get_string();
			let dual_quat = rd.get_bool();
			let root = *map_node.get( &node_name );
			let arm = @mut engine::load::read_armature( &mut rd, root, dual_quat, lg );
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
		meshes		: LinearMap::new(),
		armatures	: map_armature,
	};
	// entities
	let entity_group = context.parse_group( scene.entities, gc, opt_vao, lg );
	// cameras
	let mut map_camera : LinearMap<~str,@Camera> = LinearMap::new();
	for scene.cameras.each() |icam|	{
		let root = *context.nodes.get( &icam.node );
		map_camera.insert( copy root.name,
			@Camera{ node:root,
				proj:icam.proj.spawn(aspect),
				ear:engine::audio::Listener{ volume:0f },
			}
		);
	}
	// lights
	let mut map_light : LinearMap<~str,@Light> = LinearMap::new();
	for scene.lights.each() |ilight|	{
		let root = *context.nodes.get( &ilight.node );
		let (cr,cg,cb) = ilight.color;
		let col = gr_low::rast::Color{ r:cr, g:cg, b:cb, a:1f32 };
		let data = match ilight.kind	{
			~"POINT"=> LiPoint,
			~"SUN"	=> LiSun,
			~"SPOT"	=> LiSpot( (ilight.params[0] as f32).degrees(),
				ilight.params[1] ),
			~"HEMI"	=> LiHemi,
			~"AREA"	=> LiArea( vec2::new(ilight.params[0] as f32, ilight.params[1] as f32),
				ilight.params[2] ),
			_	=> fail!( ~"Unknown light type: " + ilight.kind ),
		};
		let (a1,a2) = ilight.attenu;
		map_light.insert( copy root.name, @Light{
			node	: root,
			color	: col,
			attenu	: [1f/ilight.energy as f32,a1 as f32,a2 as f32],
			distance: ilight.distance as f32,
			bounded	: ilight.sphere,
			kind	: data,
		});
	}
	// done
	Scene{
		context		: context,
		entities	: entity_group,
		cameras		: map_camera,
		lights		: map_light,
	}
}
