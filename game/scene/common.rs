extern mod cgmath;
extern mod engine;
extern mod lmath;
extern mod numeric;
extern mod std;

use core::hashmap::linear::LinearMap;
use core::managed;

use lmath::vec::*;
use lmath::mat::*;
use lmath::projection::*;
use cgmath::projection::*;

use engine::{gr_low,gr_mid};
use engine::space::Space;


//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Camera

pub type NodeRef = @mut engine::space::Node;

pub type Projector = PerspectiveSym<f32>;

pub struct Camera	{
	node	: NodeRef,
	proj	: Projector,
	ear		: engine::audio::Listener,
}

pub impl Camera	{
	fn get_proj_matrix( &self, aspect : f32 )-> mat4	{
		//self.proj.to_mat4().unwrap()	//ICE on Rust-0.6
		let p = &self.proj;
		lmath::projection::perspective( p.vfov, aspect, p.near, p.far )
	}
	fn get_matrix( &self, aspect : f32 )-> mat4	{
		let proj = self.get_proj_matrix( aspect );
		//proj * self.node.world_space().invert().to_matrix()
		proj.mul_m( &self.node.world_space().invert().to_matrix() )
	}
	fn get_inverse_matrix( &self, aspect : f32 )-> mat4	{
		let proj = self.get_proj_matrix( aspect );
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
	fn fill_data( &self, data : &mut gr_low::shade::DataMap, aspect : f32 )	{
		let sw = self.node.world_space();
		let pm = self.get_matrix(aspect);
		data.insert( ~"u_ViewProj",		gr_low::shade::UniMatrix(false,pm) );
		data.insert( ~"u_CameraPos",	gr_low::shade::UniFloatVec(sw.get_pos_scale()) );
		data.insert( ~"u_CameraRot",	gr_low::shade::UniFloatVec(sw.get_orientation()) );
	}
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

	fn read_armatures( &mut self, path : &~str, lg : &engine::journal::Log )	{
		let mut rd = engine::load::Reader::create_std( *path + ".k3arm" );	
		assert!( rd.enter() == ~"*arm" );
		while rd.has_more()!=0u	{
			assert!( rd.enter() == ~"meta" );
			let name = rd.get_string();
			let node_name = rd.get_string();
			let dual_quat = rd.get_bool();
			let root = match self.nodes.find( &node_name )	{
				Some(n)	=> *n,
				None	=> @mut engine::space::Node::new( copy node_name )
			};
			let arm = @mut engine::load::read_armature( &mut rd, root, dual_quat, lg );
			self.armatures.insert( name, arm );
			rd.leave();
		}
		rd.leave();
	}
}

pub struct Scene	{
	context		: SceneContext,
	entities	: EntityGroup,
	cameras		: Dict<@Camera>,
	lights		: Dict<@Light>,
}
