extern mod cgmath;
extern mod engine;
extern mod cgmath;
extern mod std;

use std::num;
use std::hashmap::HashMap;

use cgmath::{angle,projection};
use cgmath::vector::{Vector,Vec2,Vec3,Vec4};
use cgmath::matrix::{Matrix,Mat4,ToMat4};
use cgmath::transform::Transform;

use engine::{gr_low,gr_mid,space};


//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Camera

pub type NodeRef = engine::space::NodePtr;
pub type Projector = projection::PerspectiveFov<f32,angle::Rad<f32>>;

pub struct Camera	{
	node	: NodeRef,
	proj	: Projector,
	ear		: engine::audio::Listener,
}

impl Camera	{
	pub fn get_proj_matrix( &self, aspect : f32 )-> Mat4<f32>	{
		let mut proj = self.proj.clone();
		proj.aspect = aspect;
		proj.to_mat4()
	}
	pub fn get_matrix( &self, aspect : f32 )-> Mat4<f32>	{
		let proj = self.get_proj_matrix( aspect );
		let winv = self.node.borrow().with(|n|	{
			n.world_space().invert().expect(format!(
				"Unable to invert camera's world space: {:s}",
				n.name ))
			});
		proj.mul_m( &winv.to_mat4() )
	}
	pub fn get_inverse_matrix( &self, aspect : f32 )-> Mat4<f32>	{
		let proj = self.get_proj_matrix( aspect );
		let n = self.node.borrow().borrow();
		let pinv = proj.invert().expect(format!(
			"Unable to invert camera's projection matrix: {:s}",
			n.get().name ));
		n.get().world_space().to_mat4().mul_m( &pinv )
	}
	pub fn get_view_vector( &self )-> Vec3<f32>	{
		let v = Vec3::new( 0f32,0f32,-1f32 );
		self.node.borrow().with( |n| n.world_space().rot.mul_v(&v) )
	}
	pub fn get_up_vector( &self )-> Vec3<f32>	{
		let v = Vec3::new( 0f32,1f32,0f32 );
		self.node.borrow().with( |n| n.world_space().rot.mul_v(&v) )
	}
	pub fn get_side_vector( &self )-> Vec3<f32>	{
		let v = Vec3::new( 1f32,0f32,0f32 );
		self.node.borrow().with( |n| n.world_space().rot.mul_v(&v) )
	}
	pub fn fill_data( &self, data : &mut gr_low::shade::DataMap, aspect : f32 )	{
		let sw = self.node.borrow().with( |n| n.world_space() );
		let pm = self.get_matrix( aspect );
		let (p0,p1) = space::get_params( &sw );
		data.set( ~"u_ViewProj",	gr_low::shade::UniMatrix(false,pm) );
		data.set( ~"u_CameraPos",	gr_low::shade::UniFloatVec(p0) );
		data.set( ~"u_CameraRot",	gr_low::shade::UniFloatVec(p1) );
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Light

pub enum LightKind	{
	LiPoint,
	LiHemi,
	LiSun,
	LiSpot(angle::Rad<f32>,f32),
	LiArea(Vec2<f32>,f32),
}

pub struct Light	{
	node	: NodeRef,
	color	: gr_low::rast::Color,
	attenu	: [f32, ..3],
	distance: f32,
	bounded	: bool,
	kind	: LightKind,
}

impl Light	{
	pub fn get_attenuation( &self )-> Vec4<f32>	{
		assert!( self.distance>0f32 );
		let kd = 1f32 / self.distance;
		Vec4::new( 1f32 / self.attenu[0],
			self.attenu[1]*kd, self.attenu[2]*kd*kd,
			if self.bounded {kd} else {0f32}
			)
	}

	pub fn get_far_distance( &self, threshold : f32 )-> f32	{
		if self.bounded	{
			return self.distance
		}
		let kE = (1f32 - 1f32 / (self.attenu[0] * threshold));
		if self.attenu[2]>0f32 {
			assert!( self.attenu[1]>=0f32 );
			let D = self.attenu[1]*self.attenu[1] - 4f32*self.attenu[2]*kE;
			if D>=0f32	{
				0.5f32 * (num::sqrt(D) - self.attenu[1]) / self.attenu[2]
			}else	{fail!( ~"Bad attenuation: " /*+ self.attenu.to_str()*/ )}
		}else if self.attenu[1]>0f32	{
			assert!( self.attenu[2] == 0f32 );
			-kE / self.attenu[1]
		}else	{
			0f32
		}
	}

	pub fn get_proj_blend( &self, near:f32, far:f32 )-> Option<(Mat4<f32>,f32)>	{
		match self.kind	{
			LiSpot(angle,blend)	=>	{
				let m = projection::perspective( angle, 1f32, near, far );
				Some(( m, blend ))
			},
			LiArea(v2,blend)	=>	{
				let m = projection::ortho( -v2.x, v2.x, -v2.y, v2.y, near, far );
				Some(( m, blend ))
			},
			_	=> None
		}
	}

	pub fn fill_data( &self, data : &mut gr_low::shade::DataMap, near : f32, far : f32 )	{
		let sn = self.node.borrow().borrow();
		let sw = sn.get().world_space();
		let mut pos = sw.disp.extend( 1.0 );
		let col = Vec4::new( self.color.r, self.color.g, self.color.b, self.color.a );
		let range = Vec4::new( near, far, 0f32, 1f32/(far-near) );
		//io::println(format!("Light near:{:f} far:{:f}",near as float,far as float));
		match self.kind	{
			LiSun	=>	{ pos.w = 0f32; },
			_	=> ()
		}
		match self.get_proj_blend(near,far)	{
			Some(ref pair)	=>	{
				let &(mp,blend) = pair;
				let winv = sw.invert().expect(format!(
					"Unable to invert light's world space: {:s}",
					sn.get().name ));
				let ml = mp.mul_m( &winv.to_mat4() );
				data.set( ~"u_LightProj",	gr_low::shade::UniMatrix(false,ml) );
				data.set( ~"u_LightBlend",	gr_low::shade::UniFloat(blend) );
			},
			None	=> ()
		}
		data.set( ~"u_LightPos",		gr_low::shade::UniFloatVec(pos) );
		data.set( ~"u_LightColor",		gr_low::shade::UniFloatVec(col) );
		let vat = self.get_attenuation();
		data.set( ~"u_LightAttenuation",	gr_low::shade::UniFloatVec(vat) );
		data.set( ~"u_LightRange",		gr_low::shade::UniFloatVec(range) );
	}
}

//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - -//
//	Scene

pub struct EntityGroup( ~[engine::object::Entity] );

impl EntityGroup	{
	pub fn divide( &mut self, name : &str )-> EntityGroup	{
		let mut i = 0u;
		let mut rez = EntityGroup(~[]);
		while i<self.get().len()	{
			if self.get()[i].node.borrow().with(|n| n.is_under(name))	{
				rez.get_mut().push( self.get_mut().swap_remove(i) );
			}else	{
				i += 1u;
			}
		}
		rez	
	}
	
	pub fn get<'a>( &'a self )-> &'a ~[engine::object::Entity]    {
    	let &EntityGroup(ref list) = self;
		list
	}
	
	pub fn get_mut<'a>( &'a mut self )-> &'a mut ~[engine::object::Entity]    {
    	let &EntityGroup(ref mut list) = self;
		list
	}
	
	pub fn find_mut<'a,T>( &'a mut self, name : &str )-> Option<&'a mut engine::object::Entity>	{
		self.get_mut().mut_iter().find(|ent|	{
			ent.node.borrow().with( |n| std::str::eq_slice(n.name,name) )
		})
	}

	pub fn change_detail( &mut self, detail : engine::object::Entity )-> Option<engine::object::Entity>	{
		let opt_pos = self.get().iter().position(|ent| ent.node.ptr_eq( &detail.node ));
		self.get_mut().push( detail );
		opt_pos.map(|pos| { self.get_mut().swap_remove(pos) })
	}

	pub fn swap_entity( &mut self, name : &str, other : &mut EntityGroup )	{
		let opt_pos = other.get().iter().position(|ent|	{
			ent.node.borrow().with( |n| std::str::eq_slice(n.name,name) )
		});
		let e1 = other.get_mut().swap_remove(
			opt_pos.expect(format!( "Remote entity not found: {:s}", name ))
		);
		let e2 = self.change_detail( e1 ).expect(format!( "Local entity not found: {:s}", name ));
		other.get_mut().push(e2);
	}

	pub fn exclude( &mut self, name : &str )-> Option<engine::object::Entity>	{
		self.get().iter().position(|ent|	{
			ent.node.borrow().with( |n| std::str::eq_slice(n.name,name) )
		}).map(|pos| { self.get_mut().swap_remove(pos) })
	}
}

pub type Dict<T>		= HashMap<~str,T>;

pub struct SceneContext	{
	prefix		: ~str,
	materials	: Dict<@gr_mid::draw::Material>,
	mat_data	: Dict<gr_low::shade::DataMap>,
	textures	: Dict<@gr_low::texture::Texture>,
	nodes		: Dict<NodeRef>,
	meshes		: Dict<@gr_mid::mesh::Mesh>,
	armatures	: Dict<engine::space::ArmaturePtr>,
	actions		: Dict<@engine::space::ArmatureRecord>,
}

impl SceneContext	{
	pub fn new( prefix : ~str )-> SceneContext	{
		SceneContext	{
			prefix		: prefix,
			materials	: HashMap::new(),
			mat_data	: HashMap::new(),
			textures	: HashMap::new(),
			nodes		: HashMap::new(),
			meshes		: HashMap::new(),
			armatures	: HashMap::new(),
			actions		: HashMap::new(),
		}
	}

	pub fn query_mesh( &mut self, mesh_name : &~str, gc : &mut gr_low::context::Context,
			lg : &engine::journal::Log )-> @gr_mid::mesh::Mesh	{
		match self.meshes.find(mesh_name)	{
			Some(m)	=> return *m,
			None	=> (),
		};
		let split = mesh_name.split('@').map(|w| w.to_owned()).to_owned_vec();
		let path = format!( "{:s}/{:s}.k3mesh", self.prefix, split[split.len()-1u] );
		let mut rd = engine::load::Reader::create_std( path );
		let mut q_mesh = None::<@gr_mid::mesh::Mesh>;
		if split.len() > 1	{
			assert!( rd.enter() == ~"*mesh" );
			while rd.has_more()!=0u	{
				let mesh = @engine::load::read_mesh( &mut rd, gc, lg );
				let full_name = format!( "{:s}@{:s}", mesh.name, split[1] );
				if full_name == *mesh_name	{
					q_mesh = Some(mesh);
				}
				self.meshes.insert( full_name, mesh );
			}
			rd.leave();
			q_mesh.expect(format!( "Mesh '{:s}' not found in the collection", *mesh_name ))
		}else	{
			let mesh = @engine::load::read_mesh( &mut rd, gc, lg );
			self.meshes.insert( mesh_name.clone(), mesh );
			mesh
		}
	}

	pub fn query_action( &mut self, act_name : &~str, bones : &mut ~[engine::space::Bone], lg : &engine::journal::Log )-> @engine::space::ArmatureRecord	{
		match self.actions.find(act_name)	{
			Some(a)	=> return *a,
			None	=> (),
		};
		let split = act_name.split('@').map(|w| w.to_owned()).to_owned_vec();
		let path = format!( "{:s}/{:s}.k3act", self.prefix, split[split.len()-1u] );
		let mut rd = engine::load::Reader::create_std( path );
		let mut q_act = None::<@engine::space::ArmatureRecord>;
		if split.len() > 1	{
			assert!( rd.enter() == ~"*action" );
			while rd.has_more()!=0u	{
				let act = @engine::load::read_action( &mut rd, *bones, lg );
				let full_name = format!( "{:s}@{:s}", act.name, split[1] );
				if full_name == *act_name	{
					q_act = Some(act);
				}
				self.actions.insert( full_name, act );
			}
			rd.leave();
			q_act.expect(format!( "Action '{:s}' not found in the collection", *act_name ))
		}else	{
			let act = @engine::load::read_action( &mut rd, *bones, lg );
			self.actions.insert( act_name.clone(), act );
			act
		}
	}

	pub fn read_armatures( &mut self, path : &str, lg : &engine::journal::Log )	{
		let mut rd = engine::load::Reader::create_std( path + ".k3arm" );	
		assert!( rd.enter() == ~"*arm" );
		while rd.has_more()!=0u	{
			assert!( rd.enter() == ~"meta" );
			let name = rd.get_string();
			let node_name = rd.get_string();
			let dual_quat = rd.get_bool();
			let root = match self.nodes.find( &node_name )	{
				Some(n)	=> *n,
				None	=> engine::space::Node::new( node_name ).to_ptr(),
			};
			let arm = engine::load::read_armature( &mut rd, root, dual_quat, lg ).to_ptr();
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
