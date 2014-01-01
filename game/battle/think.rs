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
	fn update( &mut self, &mut main::Member, anim::float, &mut field::Field, &grid::Grid )-> MotionStatus;
	fn stop( &mut self );
}

pub trait Brain<M : field::Member>	{
	fn check( &mut self, &M, &field::Field, &grid::Grid )-> bool;
	fn decide( &mut self, &M, &field::Field, &grid::Grid )-> ~Motion;
}

pub mod motion	{
	use cgmath::point::*;
	use cgmath::vector::*;
	use engine::anim;
	use battle::{grid,field,main};
	use battle::think;

	pub struct Idle();
	impl think::Motion for Idle	{
		fn get_name<'a>( &'a self )-> &'a str	{ "Idle" }
		fn update( &mut self, _m : &mut main::Member, _delta : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> think::MotionStatus	{
			think::StatusCanInterrupt
		}
		fn stop( &mut self )	{}
	}

	pub struct Move	{
		destinations: ~[grid::Location],
		location	: grid::Location,
		orientation	: grid::Orientation,
		elevation	: f32,
		speed		: f32,
	}
	
	impl think::Motion for Move	{
		fn get_name<'a>( &'a self )-> &'a str	{ "Move" }
		
		fn update( &mut self, m: &mut main::Member, full_delta : anim::float, field: &mut field::Field, grid : &grid::Grid )-> think::MotionStatus	{
			let mut delta = full_delta as f32;
			let mut pos = Point::from_vec( &m.get_root().space.disp );
			while delta > 0.0 && !self.destinations.is_empty()	{
				let mut dest_pos = (grid as &grid::GeometryGrid).get_cell_center( self.destinations[0] );
				dest_pos.z = self.elevation;
				let dest_vector = dest_pos.sub_p( &pos );
				let dest_len = dest_vector.length();
				let time_left = dest_len / self.speed;
				pos = if delta >= time_left	{
					delta -= time_left;
					self.destinations.shift();
					dest_pos
				}else	{
					let move_vector = dest_vector.mul_s( delta * self.speed / dest_len );
					delta = 0.0;
					pos.add_v( &move_vector )
				};
			}
			let dest_loc = (grid as &grid::GeometryGrid).point_cast( &pos );
			let done : bool = if dest_loc != self.location	{
				//print(format!( "Location {:s} -> {:s}\n", self.location.to_str(), dest_loc.to_str() ));
				match field.get_by_location( dest_loc, grid as &grid::TopologyGrid )	{
					&field::CellEmpty	=>	{
						m.move( dest_loc, self.orientation );
						field.update_member( m.get_key(), grid as &grid::TopologyGrid ); 
						false
					},
					&field::CellPart(_,_)	=>	{	//collide
						pos = Point::from_vec( &(grid as &grid::GeometryGrid).compute_space(
							dest_loc, self.orientation, self.elevation ).disp);
						true
					},
					_	=> fail!("Unexpected path cell: {:s}", dest_loc.to_str())
				}
			}else {false};
			
			m.get_root().space.disp = pos.to_vec();
			
			if done || self.destinations.is_empty()	{
				think::StatusDone
			}else	{
				think::StatusCanInterrupt
			}
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
		fn update( &mut self, _m : &mut main::Member, _delta : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> think::MotionStatus	{
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

impl<M> Player<M>	{
	pub fn move( &mut self, target: grid::Location )	{
		self.do_cancel = true;
		self.do_click = true;
		self.target = target;
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
						location	: member.location,
						orientation	: topo.approximate_orientation( member.location, self.target ),
						elevation	: member.elevation,
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

