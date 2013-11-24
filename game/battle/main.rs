extern mod cgmath;
extern mod engine;
extern mod gen_hud;
extern mod gen_scene;

use std;
use glfw;
use cgmath::{angle,projection};
use cgmath::angle::ToRad;
use cgmath::matrix::ToMat4;
use cgmath::point::*;
use cgmath::quaternion::*;
use cgmath::vector::*;
use engine::{anim,gr_low,gr_mid};
use engine::anim::Player;
use engine::gr_mid::draw::Mod;
use engine::space::Interpolate;

use input;
use hud = hud::main;
use debug = hud::debug;
use scene;
use battle::{field,grid,think};
use battle::field::Member;
use battle::grid::{DrawableGrid,TopologyGrid,GeometryGrid};


struct Motion	{
	destination	: engine::space::Space,
	last_update	: anim::float,
}

pub struct Character	{
	brain		: ~think::Brain<think::PlayerCommand,Character>,	//FIXME
	// info
	name		: ~str,
	key			: field::MemberKey,
	team		: field::Team,
	body		: field::Limb,
	// stats
	move_speed	: f32,
	// view
	entity		: engine::object::Entity,
	skeleton	: @mut engine::space::Armature,
	record		: @engine::space::ArmatureRecord,
	priv start_time	: anim::float,
	// state
	priv location	: grid::Location,
	priv orientation: grid::Orientation,
	priv elevation	: f32,
	priv motion		: Option<Motion>,
}

impl field::Member for Character	{
	fn get_name<'a>( &'a self )-> &'a str	{self.name.as_slice()}
	fn get_limbs<'a>( &'a self )-> &'a [(grid::Location,field::Limb)]	{&[ (self.location,self.body.clone()) ]}
	fn get_team( &self )-> field::Team	{self.team}
	fn is_busy( &self )-> bool	{ self.motion.is_some() }
	fn receive_damage( &mut self, damage : field::Health, _limb_key : field::LimbKey )-> field::DamageResult	{
		if self.body.health > damage	{
			self.body.health -= damage;
			field::DamageSome
		}else	{
			self.body.health = 0;
			field::DamageKill
		}
	}
}

impl Character	{
	pub fn update_view( &mut self, time : anim::float )	{
		let mut moment  = time - self.start_time;
		if moment>self.record.duration	{
			//self.record = self.skeleton.find_record(~"ArmatureAction").expect(~"character Idle not found");
			self.start_time = time;
			moment = 0.0;
		}
		self.skeleton.set_record( self.record, moment );
		self.skeleton.fill_data( &mut self.entity.data );
	}

	fn recompute_space( &self, grid : &grid::Grid )-> engine::space::Space	{
		grid.compute_space( self.location, self.orientation, self.elevation )
	}

	pub fn update_logic( @mut self, time : anim::float, field : &mut field::Field, grid : &grid::Grid )	{
		let (ref mut dest_opt, ref mut done) = match self.motion	{
			Some(ref mut mo)	=>	{
				let pos	= &self.skeleton.root.space.disp;
				let dest_vector = mo.destination.disp.sub_v( pos );
				let dest_len = dest_vector.length();
				let delta = (time - mo.last_update) as f32;
				let travel_dist = std::num::min( dest_len, delta * self.move_speed );
				let move_vector = dest_vector.mul_s( travel_dist/dest_len );
				mo.last_update = time;
				(Some(pos.add_v( &move_vector )), travel_dist == dest_len)
			},
			None	=> (None,false)
		};
		match dest_opt	{
			&Some(ref mut dest_pos)	=>	{
				let dest_loc = grid.point_cast( &Point::from_vec(dest_pos) );
				if dest_loc != self.location	{
					//print(format!( "Location {:s} -> {:s}\n", self.location.to_str(), dest_loc.to_str() ));
					match field.get_by_location( dest_loc, grid as &TopologyGrid )	{
						&field::CellEmpty	=>	{
							field.remove_member( self.key );
							self.spawn( dest_loc, field, grid );
						},
						&field::CellPart(_,_)	=>	{	//collide
							*dest_pos = self.recompute_space( grid ).disp;
							*done = true;
						},
						_	=> fail!("Unexpected path cell: {:s}", dest_loc.to_str())
					}
				}
				self.skeleton.root.space.disp = *dest_pos;
			}
			_	=> ()
		}
		if *done	{
			self.motion = None;
		}
	}

	pub fn spawn( @mut self, d : grid::Location, field : &mut field::Field, grid : &grid::Grid )	{
		self.location = d;
		self.key = field.add_member( self as @mut field::Member, grid as &TopologyGrid );
		self.skeleton.root.space = self.recompute_space( grid );
	}

	pub fn move( @mut self, d : grid::Location, time : anim::float, field : &mut field::Field, grid : &grid::Grid )	{
		if false	{	//instant?
			field.remove_member( self.key );
			self.spawn( d, field, grid );
		}else	{
			assert!( !self.is_busy() );
			let space = grid.compute_space( d, self.orientation, self.elevation );
			self.motion = Some(Motion{
				destination	: space,
				last_update	: time,
			});
		}
	}
}


struct Boss	{
	// info
	name		: ~str,
	key			: field::MemberKey,
	team		: field::Team,
	body		: field::Limb,
	// stats
	move_speed	: f32,
	turn_speed	: f32,
	// view
	entity		: engine::object::Entity,
	skeleton	: @mut engine::space::Armature,
	record		: @engine::space::ArmatureRecord,
	priv start_time	: anim::float,
	// state
	priv location	: grid::Location,
	priv orientation: grid::Orientation,
	priv elevation	: f32,
	priv motion		: Option<Motion>,
}

impl field::Member for Boss	{
	fn get_name<'a>( &'a self )-> &'a str	{self.name.as_slice()}
	fn get_limbs<'a>( &'a self )-> &'a [(grid::Location,field::Limb)]	{&[ (self.location,self.body.clone()) ]}
	fn get_team( &self )-> field::Team	{self.team}
	fn is_busy( &self )-> bool	{ self.motion.is_some() }
	fn receive_damage( &mut self, damage : field::Health, _limb_key : field::LimbKey )-> field::DamageResult	{
		if self.body.health > damage	{
			self.body.health -= damage;
			field::DamageSome
		}else	{
			self.body.health = 0;
			field::DamageKill
		}
	}
}

impl Boss	{
	pub fn update_view( &mut self, time : anim::float )	{
		let mut moment  = time - self.start_time;
		if moment>self.record.duration	{
			//self.record = self.skeleton.find_record(~"ArmatureAction").expect(~"character Idle not found");
			self.start_time = time;
			moment = 0.0;
		}
		self.skeleton.set_record( self.record, moment );
		self.skeleton.fill_data( &mut self.entity.data );
	}

	fn recompute_space( &self, grid : &grid::Grid )-> engine::space::Space	{
		grid.compute_space( self.location, self.orientation, self.elevation )
	}

	pub fn update_logic( @mut self, _time : anim::float, _field : &mut field::Field, _grid : &grid::Grid )	{
		//empty
	}

	pub fn spawn( @mut self, d : grid::Location, field : &mut field::Field, grid : &grid::Grid )	{
		self.location = d;
		self.key = field.add_member( self as @mut field::Member, grid as &TopologyGrid );
		self.skeleton.root.space = self.recompute_space( grid );
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
	pub fn move( &mut self, dir : int, time : anim::float )-> bool	{
		if dir!=0 && time > self.start_time + 0.5	{
			let l = self.points.len() as int;
			self.destination = (((self.destination as int) + dir + l) % l) as uint;
			self.source = Some( self.cam.node.space );
			self.start_time = time;
			true
		}else	{false}
	}

	pub fn update( &mut self, time : anim::float )	{
		match self.source	{
			Some(source)	=>	{
				let moment = (time - self.start_time) / self.trans_duration;
				let dst = self.points[ self.destination ];
				self.cam.node.space = if moment >= 1.0	{
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
	hero	: @mut Character,
	boss	: @mut Boss,
	cache	: gr_mid::draw::Cache,
	hud		: gen_hud::common::Screen,
	field_revision	: uint,
	loc_selected	: grid::Location,
}

impl Scene	{
	pub fn reset( &mut self, time : anim::float )	{
		// common
		self.grid.clear();
		self.field.clear();
		// hero
		self.hero.spawn( Point2::new(7,2), &mut self.field, &self.grid );
		self.hero.start_time = time;
		// boss
		self.boss.spawn( Point2::new(5,5), &mut self.field, &self.grid );
		self.boss.start_time = time;
	}

	fn update_matrices( &mut self, aspect : f32 )	{
		let light_pos	= Vec4::new( 4f32, 1f32, 6f32, 1f32 );
		let all_ents = ~[&mut self.land, &mut self.hero.entity, &mut self.boss.entity];
		for ent in all_ents.move_iter()	{
			let d = &mut ent.data;
			self.view.cam.fill_data( d, aspect );
			d.insert( ~"u_LightPos",	gr_low::shade::UniFloatVec(light_pos) );
			let world = ent.node.world_space().to_mat4();
			d.insert( ~"u_World",		gr_low::shade::UniMatrix(false,world) );
		}
	}

	fn ray_cast( &self, state : &input::State )-> grid::Location	{
		let m = [state.mouse[0] as f32, state.mouse[1] as f32];
		self.grid.ray_cast( &self.view.cam, state.aspect as f32, &m )
	}

	pub fn on_input( &mut self, event : &input::Event, state : &input::State )	{
		let hero_command = !self.hero.is_busy();
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
						if !self.hero.is_busy()	{
							self.hero.move( pos, state.time_game, &mut self.field, &self.grid );
						}
					},
					&field::CellPart(_mk,_)	=> (),	//attack
					_	=> (),	//ignore
				}
			},
			&input::EvRender(_)	=>	{
				let tv = state.time_view;
				self.grid.update( &self.view.cam, state.aspect as f32 );
				self.hero.update_view( tv );
				self.boss.update_view( tv );
				self.view.update( tv );
				let tg = state.time_game;
				self.hero.update_logic( tg, &mut self.field, &self.grid );
				self.boss.update_logic( tg, &mut self.field, &self.grid );
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

	pub fn render( &mut self, output : &gr_mid::call::Output, tech : &gr_mid::draw::Technique,
			gc : &mut gr_low::context::Context, hc : &hud::Context, lg : &engine::journal::Log )	{
		// update grid
		if self.field_revision != self.field.get_revision()	{
			self.grid.clear();
			self.field.fill_grid( self.grid.mut_cells() );
			if !self.hero.is_busy()	{
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
		let c_land = tech.process( &self.land,			output.clone(), rast, &mut self.cache, gc, lg );
		let c_hero = tech.process( &self.hero.entity,	output.clone(), rast, &mut self.cache, gc, lg );
		let c_boss = tech.process( &self.boss.entity,	output.clone(), rast, &mut self.cache, gc, lg );
		let c_grid = self.grid.draw( output.clone(), self.land.input.va );
		gc.flush( [c0,c_land,c_hero,c_boss,c_grid], lg );
		lg.add("=== HUD ===");
		let hud_calls = hc.draw_all( &self.hud, output );
		gc.flush( hud_calls, lg );
	}
	
	pub fn debug_move( &self, _rot : bool, _x : int, _y : int )	{
		//empty
	}

	pub fn make_debug_menu_item( &self )-> debug::MenuItem<Scene>	{
		debug::MenuItem	{
			name	: ~"battle",
			action	: debug::ActionList(~[
				debug::MenuItem	{
					name	: ~"battle-reset",
					action	: debug::ActionFun(|s:&mut Scene| {s.reset(0.0)}),
				},
				debug::MenuItem	{
					name	: ~"battle-test",
					action	: debug::ActionFun(|_| {}),
				},
			]),
		}
	}
}


pub fn create( gc : &mut gr_low::context::Context, hc : &mut hud::Context, fcon : &gr_mid::font::Context, lg : &engine::journal::Log )-> Scene	{
	// create view
	let view = 	{
		// create camera
		let cam =	{
			let cam_space = engine::space::make( 1.0,
				Quat::new( 0.802f32, 0.447f32, 0.198f32, 0.343f32 ),
				Vec3::new( 10f32, -10f32, 5f32 )
			);
			let cam_node = @mut engine::space::Node{
				name	: ~"cam",
				space	: cam_space,
				parent	: None,
				actions	: ~[],
			};
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
			let cs = cam.node.space;
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
	let hero =	@mut {
		let brain : ~think::Player<Character> = ~think::Player::new();
		let ent = scene.entities.exclude( &"Player" ).expect("No player found");
		let skel = *scene.context.armatures.get( &~"Armature" );
		// done
		Character{
			brain	: brain as ~think::Brain<think::PlayerCommand,Character>,
			name	: ~"Clare",
			key		: 0,
			team	: 0,
			body	: field::Limb{ key: (field::LimbBody,0), health: 100, node: ent.node },
			move_speed	: 5.0,
			entity		: ent,
			skeleton	: skel,
			record		: skel.find_record("ArmatureBossAction").expect("Hero has to have Idle"),
			start_time	: 0.0,
			location	: Point2::new(0i,0i),
			orientation	: 0,
			elevation	: 1.5,
			motion		: None,
		}
	};
	// load boss
	let boss =	@mut {
		let ent = scene.entities.exclude( &"Boss" ).expect("No player found");
		let skel = *scene.context.armatures.get( &~"ArmatureBoss" );
		// done
		Boss{
			name	: ~"Boss",
			key		: 0,
			team	: 1,
			body	: field::Limb{ key: (field::LimbBody,0), health: 300, node: ent.node },
			move_speed	: 1.0,
			turn_speed	: 1.0,
			entity		: ent,
			skeleton	: skel,
			record		: skel.find_record("ArmatureBossAction").expect("Boss has to have Idle"),
			start_time	: 0.0,
			location	: Point2::new(0i,0i),
			orientation	: 0,
			elevation	: 1.5,
			motion		: None,
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
