extern mod cgmath;
extern mod engine;

use engine::anim;
use battle::{grid,field,main};

pub enum MotionStatus	{
	StatusDone,
	StatusCanInterrupt,
	StatusBusy,
}

pub trait Motion	{
	fn get_name<'a>( &'a self )-> &'a str;	//TODO: ToStr
	//fn start()	//custom interface
	fn update( &mut self, &mut field::Member, anim::float, &mut field::Field, &grid::Grid )-> MotionStatus;
	fn stop( &mut self );
}

pub trait Brain<M : field::Member>	{
	fn check( &mut self, &M, &field::Field, &grid::Grid )-> bool;
	fn decide( &mut self, &M, &field::Field, &grid::Grid )-> ~Motion;
}

pub mod motion	{
	use engine::{anim,space};
	use battle::{grid,field};
	use battle::think;

	pub struct Idle();
	impl think::Motion for Idle	{
		fn get_name<'a>( &'a self )-> &'a str	{ "Idle" }
		fn update( &mut self, _m : &mut field::Member, _delta : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> think::MotionStatus	{
			think::StatusCanInterrupt
		}
		fn stop( &mut self )	{}
	}

	pub struct Move	{
		destinations: ~[grid::Location],
		orientation	: grid::Orientation,
		speed		: f32,
	}
	impl think::Motion for Move	{
		fn get_name<'a>( &'a self )-> &'a str	{ "Move" }
		fn update( &mut self, _m : &mut field::Member, full_delta : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> think::MotionStatus	{
			let mut delta = full_delta as f32;
			while delta > 0.0	{
				let _dest_loc = self.destinations[0];
			}
			/*let pos	= &self.root.space.disp;
			let dest_vector = self.destination.disp.sub_v( pos );
			let dest_len = dest_vector.length();
			let delta = (time - mo.last_update) as f32;
			let travel_dist = std::num::min( dest_len, delta * self.speed );
			let move_vector = dest_vector.mul_s( travel_dist/dest_len );
			mo.last_update = time;
			(Some(pos.add_v( &move_vector )), travel_dist == dest_len)
		
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
			}*/
			think::StatusDone//TODO
		}
		fn stop( &mut self )	{
			//TODO
		}
	}

	pub struct Attack	{
		destination	: grid::Location,
	}
	impl think::Motion for Attack	{
		fn get_name<'a>( &'a self )-> &'a str	{ "Attack" }
		fn update( &mut self, _m : &mut field::Member, _delta : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> think::MotionStatus	{
			think::StatusDone//TODO
		}
		fn stop( &mut self )	{
			//TODO
		}
	}
}


pub enum PlayerCommand	{
	PcomNone,
	PcomWait,
	PcomMove(grid::Location),
	PcomAttack(grid::Location),
}

pub struct Player<M>	{
	do_cancel	: bool,
	do_click	: bool,
	target		: grid::Location,
}

impl<M> Player<M>	{
	pub fn new()-> Player<M>	{
		Player{
			do_cancel	: false,
			do_click	: false,
			target		: cgmath::point::Point2::new(0,0),
		}
	}
}

impl Brain<main::Character> for Player<main::Character>	{
	fn check( &mut self, _member : &main::Character, _field : &field::Field, _grid : &grid::Grid)-> bool	{
		self.do_cancel
	}
	
	fn decide( &mut self, member : &main::Character, field : &field::Field, grid : &grid::Grid )-> ~Motion	{
		self.do_cancel = false;
		if self.do_click	{
			self.do_click = false;
			let topo = grid as &grid::TopologyGrid;
			let mk = match field.get_by_location( self.target, topo )	{
				&field::CellEmpty	=> return ~motion::Move{
						destinations: ~[self.target],
						orientation	: topo.approximate_orientation( member.location, self.target ),
						speed		: member.move_speed,
					} as ~Motion,
				&field::CellPart(key,_)	=> key,
				_	=> return ~motion::Idle as ~Motion,
			};
			match field.with_member( mk, |m| m.get_team() )	{
				Some(team) if team != (member as &field::Member).get_team()	=>	{
					return ~motion::Attack{ destination: self.target } as ~Motion
				}
				_	=> ()
			}
		}
		~motion::Idle as ~Motion
	}
}

pub struct Monster<M>	{
	dummy	: bool,	//TODO
}

impl<M> Monster<M>	{
	pub fn new()-> Monster<M>	{
		Monster{
			dummy	: true,
		}
	}
}

impl<M: field::Member> Brain<M> for Monster<M>	{
	fn check( &mut self, _member : &M, _field : &field::Field, _grid : &grid::Grid )-> bool	{
		false
	}
	fn decide( &mut self, _member : &M, _field : &field::Field, _grid : &grid::Grid )-> ~Motion	{
		~motion::Idle as ~Motion
	}
}

