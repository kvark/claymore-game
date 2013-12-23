extern mod cgmath;
extern mod engine;

use engine::anim;
use battle::{grid,field};

pub enum MotionStatus	{
	StatusDone,
	StatusCanInterrupt,
	StatusBusy,
}

pub trait Motion	{
	fn get_name<'a>( &'a self )-> &'a str;	//TODO: ToStr
	fn update( &mut self, &mut field::Member, anim::float, &mut field::Field, &grid::Grid )-> MotionStatus;
	fn interrupt( &mut self );
}

pub trait Brain<M : field::Member>	{
	fn check( &mut self, &M, &field::Field, &grid::Grid )-> bool;
	fn decide( &mut self, &M, &field::Field, &grid::Grid )-> ~Motion;
}

pub mod motion	{
	use engine::anim;
	use battle::{grid,field};
	use battle::think;

	pub struct Idle();
	impl think::Motion for Idle	{
		fn get_name<'a>( &'a self )-> &'a str	{ "Idle" }
		fn update( &mut self, _m : &mut field::Member, _time : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> think::MotionStatus	{
			think::StatusCanInterrupt
		}
		fn interrupt( &mut self )	{}
	}

	pub struct Move	{
		destination	: grid::Location,
	}
	impl think::Motion for Move	{
		fn get_name<'a>( &'a self )-> &'a str	{ "Move" }
		fn update( &mut self, _m : &mut field::Member, _time : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> think::MotionStatus	{
			think::StatusDone//TODO
		}
		fn interrupt( &mut self )	{
			//TODO
		}
	}

	pub struct Attack	{
		destination	: grid::Location,
	}
	impl think::Motion for Attack	{
		fn get_name<'a>( &'a self )-> &'a str	{ "Attack" }
		fn update( &mut self, _m : &mut field::Member, _time : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> think::MotionStatus	{
			think::StatusDone//TODO
		}
		fn interrupt( &mut self )	{
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

impl<M: field::Member> Brain<M> for Player<M>	{
	fn check( &mut self, _member : &M, _field : &field::Field, _grid : &grid::Grid)-> bool	{
		self.do_cancel
	}
	
	fn decide( &mut self, member : &M, field : &field::Field, grid : &grid::Grid )-> ~Motion	{
		self.do_cancel = false;
		if self.do_click	{
			self.do_click = false;
			let mk = match field.get_by_location( self.target, grid as &grid::TopologyGrid )	{
				&field::CellEmpty	=> return ~motion::Move{ destination: self.target } as ~Motion,
				&field::CellPart(key,_)	=> key,
				_	=> return ~motion::Idle as ~Motion,
			};
			match field.with_member( mk, |m| m.get_team() )	{
				Some(team) if team != member.get_team()	=>	{
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

