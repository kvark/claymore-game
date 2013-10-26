extern mod cgmath;
extern mod engine;
extern mod gen_hud;
extern mod gen_scene;

use std;
use glfw;
use cgmath::{angle,projection,rotation};
use cgmath::angle::ToRad;
use cgmath::point::*;
use cgmath::quaternion::*;
use cgmath::vector::*;
use engine::anim::Player;
use engine::{gr_low,gr_mid};
use engine::gr_mid::draw::Mod;
use engine::space::{Interpolate,Space};

use input;
use hud = hud::main;
use debug = hud::debug;
use scene;
use battle::{field,grid};
use battle::field::Member;
use battle::grid::{DrawableGrid,TopologyGrid,GeometryGrid};


pub struct Character	{
	name		: ~str,
	health		: field::Health,
	parts		: ~[field::Offset],
	entity		: engine::object::Entity,
	skeleton	: @mut engine::space::Armature,
	record		: @engine::space::ArmatureRecord,
	priv start_time	: float,
	location	: grid::Location,
	orientation	: field::Orientation,
}

impl Character	{
	pub fn update_view( &mut self, time : float )	{
		let mut moment  = time - self.start_time;
		if moment>self.record.duration	{
			//self.record = self.skeleton.find_record(~"ArmatureAction").expect(~"character Idle not found");
			self.start_time = time;
			moment = 0f;
		}
		self.skeleton.set_record( self.record, moment );
		self.skeleton.fill_data( &mut self.entity.data );
	}

	pub fn update_logic( &mut self, _time : float, _field : &mut field::Field )	{
		//TODO
	}

	pub fn spawn( @mut self, d : grid::Location, field : &mut field::Field, grid : &grid::Grid )	{
		field.add_member( self as @mut field::Member, d, 0, grid as &TopologyGrid );
		self.location = d;
		self.skeleton.root.space = grid.compute_space( d, self.orientation, 1.5 );
	}

	pub fn move( @mut self, d : grid::Location, field : &mut field::Field, grid : &grid::Grid )	{
		field.remove_member( self.get_name() );
		self.spawn( d, field, grid );
	}
}

impl field::Member for Character	{
	fn get_name<'a>( &'a self )-> &'a str	{self.name.as_slice()}
	fn get_health( &self )-> field::Health	{self.health}
	fn get_parts<'a>( &'a self )-> &'a [grid::Offset]	{self.parts.as_slice()}
	fn is_busy( &self )-> bool	{ false }
	fn receive_damage( &mut self, damage : field::Health, part : Option<field::PartId> )-> field::DamageResult	{
		assert!( part.is_none() );
		if self.health > damage	{
			self.health -= damage;
			field::DamageSome
		}else	{
			self.health = 0;
			field::DamageKill
		}
	}
}


pub struct View	{
	cam				: scene::common::Camera,
	trans_duration	: float,
	points			: ~[engine::space::QuatSpace],
	source			: Option<engine::space::QuatSpace>,
	destination		: uint,
	start_time		: float,
}

impl View	{
	pub fn move( &mut self, dir : int, time : float )-> bool	{
		if dir!=0 && time > self.start_time + 0.5f	{
			let l = self.points.len() as int;
			self.destination = (((self.destination as int) + dir + l) % l) as uint;
			self.source = Some( self.cam.node.space );
			self.start_time = time;
			true
		}else	{false}
	}

	pub fn update( &mut self, time : float )	{
		match self.source	{
			Some(source)	=>	{
				let moment = (time - self.start_time) / self.trans_duration;
				let dst = self.points[ self.destination ];
				self.cam.node.space = if moment >= 1f	{
						self.source = None;
						dst
					}else	{
						source.interpolate( &dst, moment )
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
	boss	: @mut Character,
	cache	: gr_mid::draw::Cache,
	hud		: gen_hud::common::Screen,
	grid_dirty	: bool,
	loc_selected: grid::Location,
}

impl Scene	{
	pub fn reset( &mut self, time : float )	{
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
			let world = ent.node.world_space().to_matrix();
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
					glfw::KeyE		=> { self.view.move( 1, state.time ); },
					glfw::KeyQ		=> { self.view.move(-1, state.time ); },
					_	=> (),
				}
			},
			&input::EvMouseClick(key,press) if hero_command && key==0 && press	=> {
				let pos = self.ray_cast( state );
				match self.field.get_by_location( pos, &self.grid as &TopologyGrid )	{
					(Some(_),field::CellEmpty)	=> self.hero.move( pos, &mut self.field, &self.grid ),
					(Some(_),_)	=> (),	//attack
					_	=> (),	//ignore
				}
			},
			&input::EvRender(_)	=>	{
				let tv = state.time;	//FIXME
				self.grid.update( &self.view.cam, state.aspect as f32 );
				self.hero.update_view( tv );
				self.boss.update_view( tv );
				self.view.update( tv );
				let tl = state.time;	//FIXME
				self.hero.update_logic( tl, &mut self.field );
				self.boss.update_logic( tl, &mut self.field );
				self.update_matrices( state.aspect as f32 );
				let active = self.ray_cast( state );
				if active != self.loc_selected	{
					self.loc_selected = active;
					self.grid_dirty = true;
				}
			},
			_	=> (),
		}
	}

	pub fn render( &mut self, output : &gr_mid::call::Output, tech : &gr_mid::draw::Technique,
			gc : &mut gr_low::context::Context, hc : &hud::Context, lg : &engine::journal::Log )	{
		// update grid
		if self.grid_dirty	{
			self.grid.clear();
			self.field.fill_grid( self.grid.mut_cells() );
			match self.field.get_by_location( self.loc_selected, &self.grid as &TopologyGrid )	{
				(Some(index),field::CellEmpty)	=>	{
					//print(fmt!( "loc(%i,%i) index = %u\n", active[0], active[1], index ));
					self.grid.mut_cells()[index] = grid::CELL_ACTIVE
				},
				(Some(_),_)	=> (),	//attack animation
				_		=> ()
			}
			self.grid.upload( &mut gc.texture );
			self.grid_dirty = false;
		}
		// clear screen
		let cd = gr_mid::call::ClearData{
			color	:Some( gr_low::rast::Color::new(0x8080FFFF) ),
			depth	:Some( 1f ),
			stencil	:Some( 0u ),
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
					action	: debug::ActionFun(|s:&mut Scene| {s.reset(0f)}),
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
			let cam_space = engine::space::QuatSpace{
				position 	: Vec3::new( 10f32, -10f32, 5f32 ),
				orientation	: Quat::new( 0.802f32, 0.447f32, 0.198f32, 0.343f32 ),
				scale		: 1f32
			};
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
				ear		: engine::audio::Listener{ volume:0f },
			}
		};
		let points = std::vec::from_fn(4, |i|	{
			let axis = Vec3::new( 0f32, 0f32, 1f32 );
			let angle = angle::deg( (i as f32) * 180f32 / 4f32 );
			let q = rotation::AxisAngle::new( axis, angle ).to_quat();
			let cs = cam.node.space;
			engine::space::QuatSpace{
				position	: q.mul_v( &cs.position ),
				orientation	: q.mul_q( &cs.orientation ),
				scale		: cs.scale,
			}
		});
		View{
			cam	: cam,
			trans_duration	: 2f,
			points			: points,
			source			: None,
			destination		: 0,
			start_time		: 0f,
		}
	};
	// load battle landscape
	let iscene = gen_scene::battle::main::load();
	let vao = gc.create_vertex_array();
	let mut scene = scene::load::parse( "data/scene/battle-test", &iscene, [], gc, Some(vao), lg );
	let battle_land = scene.entities.exclude( &"Plane" ).expect("No ground found");
	// load protagonist
	let hero =	@mut {
		let ent = scene.entities.exclude( &"Player" ).expect("No player found");
		let skel = *scene.context.armatures.get( &~"Armature" );
		// done
		Character{
			name	: ~"Clare",
			health	: 100,
			parts	: ~[Vec2::new(0i,0i)],
			entity		: ent,
			skeleton	: skel,
			record		: skel.find_record("ArmatureBossAction").expect("Hero has to have Idle"),
			start_time	: 0f,
			location	: Point2::new(0i,0i),
			orientation	: 0,
		}
	};
	// load boss
	let boss =	@mut {
		let ent = scene.entities.exclude( &"Boss" ).expect("No player found");
		let skel = *scene.context.armatures.get( &~"ArmatureBoss" );
		// done
		Character{
			name	: ~"Boss",
			health	: 300,
			parts	: ~[Vec2::new(0i,0i)],
			entity		: ent,
			skeleton	: skel,
			record		: skel.find_record("ArmatureBossAction").expect("Boss has to have Idle"),
			start_time	: 0f,
			location	: Point2::new(0i,0i),
			orientation	: 0,
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
		grid_dirty	: true,
		loc_selected: Point2::new(0i,0i),
	}
}
