extern mod cgmath;
extern mod engine;
extern mod gen_hud;
extern mod gen_scene;

use std;
use glfw;
use cgmath::{angle,projection};
use cgmath::angle::ToRad;
use cgmath::matrix::ToMat4;
use cgmath::point::{Point2};
use cgmath::quaternion::{Quat};
use cgmath::vector::{Vec3,Vec4};
use engine::{anim,gr_low,gr_mid};
use engine::anim::Player;
use engine::gr_mid::draw::Mod;
use engine::space::Interpolate;

use input;
use hud = hud::main;
use debug = hud::debug;
use scene;
use battle::{field,grid,motion,think,unit};
use battle::grid::{DrawableGrid,TopologyGrid,GeometryGrid};


pub trait Prop	{
	fn get_root( &self )-> engine::space::NodePtr;
	fn get_elevation( &self )-> f32;
}

pub trait Member : unit::Unit + Prop	{}

impl Prop for unit::Standard	{
	fn get_root( &self )-> engine::space::NodePtr	{
		self.skeleton.borrow().with(|s| s.root.clone())
	}
	fn get_elevation( &self )-> f32	{
		self.elevation
	}
}

impl Member for unit::Standard	{}


pub struct CharBundle<M,B>	{
	brain		: ~B,
	member		: M,
	member_key	: field::MemberKey,
	motion		: think::MotionPtr,
	last_update	: anim::float,
	priv available	: bool,
}

impl<M: Member + 'static, B: think::Brain<M>> CharBundle<M,B>	{
	pub fn update( &mut self, time: anim::float, field: &mut field::Field, grid: &grid::Grid, lg: &engine::journal::Log )	{
		let delta = time - self.last_update;
		self.last_update = time;
		let new = match	self.motion.update( &mut self.member, delta, field, grid )	{
			think::StatusDone	=> 1,
			think::StatusCanInterrupt	=>
				if self.brain.check( &self.member, field, grid )	{
					self.motion.stop();
					lg.add(format!( "{:s}: interrupts", self.member.get_name() ));
					1
				}else {0},
			think::StatusBusy	=> -1,
		};
		self.available = new==0;
		if new>0	{
			self.motion = self.brain.decide( &self.member, field, grid );
			lg.add(format!( "{:s}: new motion {:s}", self.member.get_name(), self.motion.get_name() ));
		}
	}
	
	pub fn is_waiting( &self )-> bool	{
		self.available
	}
	
	pub fn spawn( &mut self, team: field::Team, d: grid::Location, field: &mut field::Field, grid: &grid::Grid )	{
		let link = field::Link	{
			name		: self.member.get_name().to_owned(),
			team		: team,
			location	: d,
			orientation	: 0,
		};
		let sp = grid.compute_space( d, 0, self.member.get_elevation() );
		let root = self.member.get_root();
		root.borrow().with_mut(|n| {n.space = sp;} );
		self.member_key = field.add_member( link, &self.member, grid as &TopologyGrid );
	}
}


impl unit::Standard	{
	pub fn update_view( &mut self, _time: anim::float )	{
		/*	FIXME
		let mut moment  = time - self.start_time;
		if moment>self.record.duration	{
			//self.record = self.skeleton.find_record(~"ArmatureAction").expect(~"character Idle not found");
			self.start_time = time;
			moment = 0.0;
		}
		self.skeleton.set_record( self.record, moment );
		*/
		self.entity.update_mod();
	}
}


pub struct View	{
	cam				: scene::common::Camera,
	trans_duration	: anim::float,
	points			: ~[engine::space::Space],
	source			: Option<engine::space::Space>,
	destination		: uint,
	start_time		: anim::float,
}

impl View	{
	pub fn move( &mut self, dir: int, time: anim::float )-> bool	{
		if dir!=0 && time > self.start_time + 0.5	{
			let l = self.points.len() as int;
			self.destination = (((self.destination as int) + dir + l) % l) as uint;
			self.source = Some( self.cam.node.borrow().borrow().get().space );
			self.start_time = time;
			true
		}else	{false}
	}

	pub fn update( &mut self, time: anim::float )	{
		match self.source	{
			Some(source)	=>	{
				let moment = (time - self.start_time) / self.trans_duration;
				let dst = self.points[ self.destination ];
				self.cam.node.borrow().borrow_mut().get().space = if moment >= 1.0	{
						self.source = None;
						dst
					}else	{
						source.interpolate( &dst, moment as f32 )
					};
			},
			None	=> ()
		}
	}
}


pub struct Scene	{
	view	: View,
	land	: engine::object::Entity,
	grid	: grid::Grid,
	field	: field::Field,
	hero	: CharBundle<unit::Standard,think::Player<unit::Standard>>,
	boss	: CharBundle<unit::Standard,think::Monster<unit::Standard>>,
	cache	: gr_mid::draw::Cache,
	hud		: gen_hud::common::Screen,
	field_revision	: uint,
	loc_selected	: grid::Location,
}

impl Scene	{
	pub fn reset( &mut self, _time: anim::float )	{
		// common
		self.grid.clear();
		self.field.clear();
		// hero
		self.hero.spawn( 0, Point2::new(7,2), &mut self.field, &self.grid );
		// boss
		self.boss.spawn( 1, Point2::new(5,5), &mut self.field, &self.grid );
	}

	fn update_matrices( &mut self, aspect: f32 )	{
		let light_pos	= Vec4::new( 4f32, 1f32, 6f32, 1f32 );
		let all_ents = ~[&mut self.land, &mut self.hero.member.entity, &mut self.boss.member.entity];
		for ent in all_ents.move_iter()	{
			let d = &mut ent.data;
			self.view.cam.fill_data( d, aspect );
			d.set( ~"u_LightPos",	gr_low::shade::UniFloatVec(light_pos) );
			let world = ent.node.borrow().with( |n| n.world_space().to_mat4() );
			d.set( ~"u_World",		gr_low::shade::UniMatrix(false,world) );
		}
	}

	fn ray_cast( &self, state: &input::State )-> grid::Location	{
		let m = [state.mouse[0] as f32, state.mouse[1] as f32];
		self.grid.ray_cast( &self.view.cam, state.aspect as f32, &m )
	}

	pub fn on_input( &mut self, event: &input::Event, state: &input::State )	{
		let hero_command = self.hero.is_waiting();
		match event	{
			&input::EvKeyboard(key,press) if press	=> {
				// camera rotation
				match key	{
					glfw::KeyE		=> { self.view.move( 1, state.time_view ); },
					glfw::KeyQ		=> { self.view.move(-1, state.time_view ); },
					_	=> (),
				}
			},
			&input::EvMouseClick(key,press) if hero_command && key==0 && press	=> {
				let pos = self.ray_cast( state );
				match self.field.get_by_location( pos, &self.grid as &TopologyGrid )	{
					&field::CellEmpty	=>	{
						self.hero.brain.move( pos );
					},
					&field::CellPart(mk,_) =>	{
						let hero_key = self.hero.member_key;
						let hero_loc = self.field.get_member(hero_key).location;
						if pos!=hero_loc && mk==hero_key	{
							self.hero.brain.move( pos );
						}else	{}	//attack
					},
					_	=> (),	//ignore
				}
			},
			&input::EvRender(_)	=>	{
				let tv = state.time_view;
				self.grid.update( &self.view.cam, state.aspect as f32 );
				self.hero.member.update_view( tv );
				self.boss.member.update_view( tv );
				self.view.update( tv );
				//let tg = state.time_game;
				//self.hero.member.update_logic( tg, &mut self.field, &self.grid );
				//self.boss.member.update_logic( tg, &mut self.field, &self.grid );
				self.update_matrices( state.aspect as f32 );
				if hero_command	{
					let active = self.ray_cast( state );
					if active != self.loc_selected	{
						self.loc_selected = active;
						self.field_revision = 0;
					}
				}
			},
			_	=> (),
		}
	}
	
	pub fn update( &mut self, time: anim::float, lg: &engine::journal::Log )	{
		lg.add(format!( "Frame at {}", time  ));
		self.hero.update( time, &mut self.field, &self.grid, lg );
		self.boss.update( time, &mut self.field, &self.grid, lg );
	}

	pub fn render( &mut self, output: &gr_mid::call::Output, tech: &gr_mid::draw::Technique,
			gc: &mut gr_low::context::Context, hc: &hud::Context, lg: &engine::journal::Log )	{
		// update grid
		if self.field_revision != self.field.get_revision()	{
			self.grid.clear();
			self.field.fill_grid( self.grid.mut_cells() );
			if self.hero.is_waiting()	{
				self.grid.get_index(self.loc_selected).map(|index|	{
					match self.field.get_cell(index)	{
						&field::CellEmpty	=>	{
							self.grid.mut_cells()[index] = grid::CELL_ACTIVE;
						},
						_	=> ()	//attack
					}
				});
			}
			self.grid.upload( &mut gc.texture );
			self.field_revision = self.field.get_revision();
		}
		// clear screen
		let cd = gr_mid::call::ClearData{
			color	:Some( gr_low::rast::Color::new(0x8080FFFF) ),
			depth	:Some( 1.0 ),
			stencil	:Some( 0u32 ),
		};
		let c0 = gr_mid::call::CallClear( cd, output.clone(), gc.default_rast.mask );
		lg.add("=== Battle scene ===");
		let mut rast = gc.default_rast;
		rast.set_depth( "<=", true );
		rast.prime.cull = true;
		let c_land = tech.process( &self.land,					output.clone(), rast, &mut self.cache, gc, lg );
		let c_hero = tech.process( &self.hero.member.entity,	output.clone(), rast, &mut self.cache, gc, lg );
		let c_boss = tech.process( &self.boss.member.entity,	output.clone(), rast, &mut self.cache, gc, lg );
		let c_grid = self.grid.draw( output.clone(), self.land.input.va.clone() );
		gc.flush( ~[c0,c_land,c_hero,c_boss,c_grid], lg );
		lg.add("=== HUD ===");
		let hud_calls = hc.draw_all( &self.hud, output );
		gc.flush( hud_calls, lg );
	}
	
	pub fn debug_move( &self, _rot: bool, _x: int, _y: int )	{
		//empty
	}

	pub fn make_debug_menu_item( &self )-> debug::MenuItem<Scene>	{
		debug::MenuItem	{
			name	: ~"battle",
			action	: debug::ActionList(~[
				debug::MenuItem	{
					name	: ~"battle-reset",
					action	: do debug::ActionFun |s:&mut Scene| {s.reset(0.0)},
				},
				debug::MenuItem	{
					name	: ~"battle-test",
					action	: do debug::ActionFun |_| {},
				},
			]),
		}
	}
}


pub fn create( gc: &mut gr_low::context::Context, hc: &mut hud::Context,
		fcon: &gr_mid::font::Context, lg: &engine::journal::Log )-> Scene	{
	// create view
	let view = 	{
		// create camera
		let cam =	{
			let cam_space = engine::space::make( 1.0,
				Quat::new( 0.802f32, 0.447f32, 0.198f32, 0.343f32 ),
				Vec3::new( 10f32, -10f32, 5f32 )
			);
			let cam_node = engine::space::Node{
				name	: ~"cam",
				space	: cam_space,
				parent	: None,
				actions	: ~[],
			}.to_ptr();
			scene::common::Camera{
				node	: cam_node,
				proj	: projection::PerspectiveFov{
					fovy	: angle::deg(45f32).to_rad(),
					aspect	: 1f32,
					near	: 1f32,
					far		: 25f32,
				},
				ear		: engine::audio::Listener{ volume:0.0 },
			}
		};
		let points = std::vec::from_fn(4, |i|	{
			let axis = Vec3::new( 0f32, 0f32, 1f32 );
			let angle = angle::deg( (i as f32) * 180f32 / 4f32 );
			let q = Quat::from_axis_angle( &axis, angle.to_rad() );
			let cs = cam.node.borrow().borrow().get().space;
			engine::space::make( cs.scale,
				q.mul_q( &cs.rot ),
				q.mul_v( &cs.disp ))
		});
		View{
			cam	: cam,
			trans_duration	: 2.0,
			points			: points,
			source			: None,
			destination		: 0,
			start_time		: 0.0,
		}
	};
	// load battle landscape
	let iscene = gen_scene::battle::main::load();
	let vao = gc.create_vertex_array();
	let mut scene = scene::load::parse( "data/scene/battle-test", &iscene, [], gc, Some(vao), lg );
	let battle_land = scene.entities.exclude( &"Plane" ).expect("No ground found");
	// load protagonist
	let hero = {
		let ent = scene.entities.exclude( &"Player" ).expect("No player found");
		let skel = scene.context.armatures.get( &~"Armature" ).clone();
		let rec = skel.borrow().with(|s| s.find_record("ArmatureBossAction")).
			expect("Hero has to have Idle");
		let mem = unit::Standard{
			name	: ~"Clare",
			body	: unit::Limb{ health: 100, node: ent.node.clone() },
			move_speed	: 5.0,
			turn_speed	: 5.0,
			entity		: ent,
			skeleton	: skel,
			record		: rec,
			elevation	: 1.5,
		};
		let brain : ~think::Player<unit::Standard> = ~think::Player::new();
		CharBundle	{
			brain		: brain,
			member		: mem,
			member_key	: 0,
			motion		: ~motion::Dummy as think::MotionPtr,
			last_update	: 0.0,
			available	: false,
		}
	};
	// load boss
	let boss = {
		let ent = scene.entities.exclude( &"Boss" ).expect("No player found");
		let skel = scene.context.armatures.get( &~"ArmatureBoss" ).clone();
		let rec = skel.borrow().with(|s| s.find_record("ArmatureBossAction")).
			expect("Boss has to have Idle");
		let mem = unit::Standard{
			name	: ~"Boss",
			body	: unit::Limb{ health: 300, node: ent.node.clone() },
			move_speed	: 1.0,
			turn_speed	: 1.0,
			entity		: ent,
			skeleton	: skel,
			record		: rec,
			elevation	: 1.5,
		};
		let brain : ~think::Monster<unit::Standard> = ~think::Monster::new();
		CharBundle	{
			brain		: brain,
			member		: mem,
			member_key	: 0,
			motion		: ~motion::Dummy as think::MotionPtr,
			last_update	: 0.0,
			available	: false,
		}
	};
	// create grid
	let mut grid = grid::Grid::create( gc, 10u, lg );
	grid.init( &mut gc.texture );
	// create field
	let field = field::Field::new( grid.get_index_size() );
	// create hud
	let hud = gen_hud::battle::load();
	hc.preload( hud.root.children, gc, fcon, lg );
	// done
	Scene{
		view	: view,
		land	: battle_land,
		grid	: grid,
		field	: field,
		hero	: hero,
		boss	: boss,
		cache	: gr_mid::draw::make_cache(),
		hud		: hud,
		field_revision	: 0,
		loc_selected	: Point2::new(0i,0i),
	}
}
