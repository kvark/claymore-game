extern mod cgmath;
extern mod engine;
extern mod gen_hud;
extern mod gen_scene;

use std;
use glfw;
use cgmath::{angle,projection,rotation};
use cgmath::angle::ToRad;
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
use battle::grid;


pub struct Character	{
	entity		: engine::object::Entity,
	skeleton	: @mut engine::space::Armature,
	record		: @engine::space::ArmatureRecord,
	priv start_time	: float,
	coord		: grid::Coordinate,
}

impl Character	{
	pub fn update( &mut self )-> bool	{
		let time = engine::anim::get_time();
		let mut moment  = time - self.start_time;
		if moment>self.record.duration	{
			//self.record = self.skeleton.find_record(~"ArmatureAction").expect(~"character Idle not found");
			self.start_time = time;
			moment = 0f;
		}
		self.skeleton.set_record( self.record, moment );
		self.skeleton.fill_data( &mut self.entity.data );
		true
	}

	pub fn move( &mut self, grid : &mut grid::Grid, d : grid::Coordinate )	{
		grid.set_cell( self.coord, grid::CELL_EMPTY, None );
		grid.set_cell( d, grid::CELL_OCCUPIED, None );
		self.coord = d;
		let sp = &mut self.skeleton.root.space;
		sp.position = grid.get_cell_center( d );
		sp.position.z = 1.5f32;
		//sp.orientation = Quat::new( 0.707f32, 0f32, 0f32, 0.707f32 );
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
	pub fn move( &mut self, dir : int )-> bool	{
		let time = engine::anim::get_time();
		if dir!=0 && time > self.start_time + 0.5f	{
			let l = self.points.len() as int;
			self.destination = (((self.destination as int) + dir + l) % l) as uint;
			self.source = Some( self.cam.node.space );
			self.start_time = time;
			true
		}else	{false}
	}

	pub fn update( &mut self )-> bool	{
		let time = engine::anim::get_time();
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
		true
	}
}


pub struct Scene	{
	view	: View,
	land	: engine::object::Entity,
	grid	: grid::Grid,
	hero	: Character,
	boss	: Character,
	cache	: gr_mid::draw::Cache,
	hud		: gen_hud::common::Screen,
	debug	: debug::Menu,
}

impl Scene	{
	pub fn reset( &mut self )	{
		self.grid.reset();
		self.hero.move( &mut self.grid, [7,2] );
		self.boss.move( &mut self.grid, [5,5] );
		let time = engine::anim::get_time();
		self.hero.start_time = time;
		self.boss.start_time = time;
	}

	pub fn update( &mut self, input : &input::State, aspect : f32 )-> bool	{
		let ok = self.grid.update( &self.view.cam, aspect, input.mouse.x, input.mouse.y );
		self.hero.update() && self.boss.update() && self.view.update() && ok
	}

	pub fn on_input( &mut self, event : &input::Event )	{
		match event	{
			&input::Keyboard(key,press) if press	=> {
				// camera rotation
				let in_menu = self.debug.is_active();
				match key	{
					glfw::KeyE		=> { self.view.move(-1); },
					glfw::KeyQ		=> { self.view.move(1); },
					glfw::KeyM if in_menu		=>
						self.debug.selection.clear(),
					glfw::KeyM	=> 
						self.debug.selection.push(0),
					glfw::KeyUp if in_menu	=> {
						let last = self.debug.selection.mut_iter().last().
							expect("Debug menu: nothing is selected");
						if *last>0	{
							*last -= 1;
						}
					},
					glfw::KeyDown if in_menu	=> {
						let menu_len =	{
							let (_,ref last_list) = self.debug.selection_list_iter().last().
								expect("Debug menu: no list found");
							last_list.len()
						};
						let last = self.debug.selection.mut_iter().last().
							expect("Debug menu: nothing is selected");
						if ((*last+1) as uint) < menu_len	{
							*last += 1;
						}
					},
					glfw::KeyEnter if in_menu	=> 	{
						let extend = match self.debug.get_selected_item().action	{
							debug::ActionFun(ref fun)	=> {(*fun)(); false},
							debug::ActionList(ref list) if !list.is_empty()	=> true,
							_	=> false,	//beep
						};
						if extend	{
							self.debug.selection.push(0);
						}
					},
					_	=> ()
				};
				
			},
			&input::MouseClick(key,press) if key==0 && press	=> {
				match self.grid.selected	{
					Some(pos)	=>	{
						match self.grid.get_cell(pos)	{
							Some(col) if col==grid::CELL_ACTIVE	=>	{
								self.hero.move( &mut self.grid, pos );
							},
							_	=> ()	//beep
						}
					},
					None	=> ()	//beep
				}
			},
			_	=> (),
		}
	}

	pub fn render( &mut self, gc : &mut gr_low::context::Context, hc : &hud::Context,
			tech : &gr_mid::draw::Technique, output : gr_mid::call::Output, lg : &engine::journal::Log )	{
		// update grid
		self.grid.upload_dirty_cells( &mut gc.texture );
		{// update matrices
			let aspect = output.area.aspect();
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
		let c_grid = self.grid.call( output.fb, output.pmap.clone(), self.land.input.va );
		gc.flush( [c0,c_land,c_hero,c_boss,c_grid], lg );
		lg.add("=== HUD ===");
		let hud_calls = hc.draw_all( &self.hud, &output );
		gc.flush( hud_calls, lg );
		lg.add("=== Debug Menu ===");
		let debug_hud = self.debug.build( 0.5 );
		let debug_calls = hc.draw_all( &debug_hud, &output );
		gc.flush( debug_calls, lg );
	}
	
	pub fn debug_move( &self, _rot : bool, _x : int, _y : int )	{
		//empty
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
	let battle_land = scene.entities.exclude( &~"Plane" ).expect("No ground found");
	// load protagonist
	let hero =	{
		let ent = scene.entities.exclude( &~"Player" ).expect("No player found");
		let skel = *scene.context.armatures.get( &~"Armature" );
		// done
		Character{
			entity		: ent,
			skeleton	: skel,
			record		: skel.find_record("ArmatureBossAction").expect("Hero has to have Idle"),
			start_time	: 0f,
			coord		: [0,0],
		}
	};
	// load boss
	let boss =	{
		let ent = scene.entities.exclude( &~"Boss" ).expect("No player found");
		let skel = *scene.context.armatures.get( &~"ArmatureBoss" );
		// done
		Character{
			entity		: ent,
			skeleton	: skel,
			record		: skel.find_record("ArmatureBossAction").expect("Boss has to have Idle"),
			start_time	: 0f,
			coord		: [0,0],
		}
	};
	// create grid
	let mut grid = grid::Grid::create( gc, 10u, lg );
	grid.init( &mut gc.texture );
	let hud = gen_hud::battle::load();
	hc.preload( hud.root.children, gc, fcon, lg );
	// create debug menu
	let debug = debug::Menu	{
		root	: debug::MenuItem	{
			name	: ~"root",
			action	: debug::ActionList(~[
				debug::MenuItem	{
					name	: ~"test",
					action	: debug::ActionFun(|| {}),
				},
				debug::MenuItem	{
					name	: ~"test-2",
					action	: debug::ActionFun(|| {}),
				},
			]),
		},
		selection	: ~[0],
		font	: gen_hud::common::Font	{
			path	: ~"Vera.ttf",
			size	: [10,10],
			kern	: [0,-10],
		},
	};
	debug.preload( hc, fcon, gc, lg );
	// done
	Scene{
		view	: view,
		land	: battle_land,
		grid	: grid,
		hero	: hero,
		boss	: boss,
		cache	: gr_mid::draw::make_cache(),
		hud		: hud,
		debug	: debug,
	}
}
