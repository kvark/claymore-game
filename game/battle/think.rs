extern mod cgmath;
extern mod engine;

use engine::anim;
use battle::{grid,field};


pub trait Motion	{
	fn update( &mut self, @mut field::Member, anim::float, &mut field::Field, &grid::Grid )-> bool;
	fn interrupt( &mut self );
}

pub trait Brain<M : field::Member>	{
	fn check( &mut self, grid::Location, &grid::Grid, &field::Field )-> bool;
	fn decide( &mut self, &M, &grid::Grid, &field::Field )-> ~Motion;
}

pub mod motion	{
	use engine::anim;
	use battle::{grid,field};
	use battle::think::Motion;

	pub struct Idle();
	impl Motion for Idle	{
		fn update( &mut self, _m : @mut field::Member, _time : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> bool	{true}
		fn interrupt( &mut self )	{}
	}

	pub struct Move	{
		destination	: grid::Location,
	}
	impl Motion for Move	{
		fn update( &mut self, _m : @mut field::Member, _time : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> bool	{
			true//TODO
		}
		fn interrupt( &mut self )	{
			//TODO
		}
	}

	pub struct Attack	{
		destination	: grid::Location,
	}
	impl Motion for Attack	{
		fn update( &mut self, _m : @mut field::Member, _time : anim::float, _field: &mut field::Field, _grid : &grid::Grid )-> bool	{
			true//TODO
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

impl<M : field::Member> Brain<M> for Player<M>	{
	fn check( &mut self, _loc : grid::Location, _grid : &grid::Grid, _field : &field::Field )-> bool	{
		self.do_cancel
	}
	
	fn decide( &mut self, member : &M, grid : &grid::Grid, field : &field::Field )-> ~Motion	{
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
