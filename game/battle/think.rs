extern mod cgmath;

use battle::{grid,field};


pub trait Brain<C, M : field::Member>	{
	fn on_move( &mut self, grid::Location, &grid::Grid, &field::Field )-> bool;
	fn on_idle( &mut self, &M, &grid::Grid, &field::Field )-> C;
}


pub trait Motion	{
	fn update( )-> bool;
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

impl<M : field::Member> Brain<PlayerCommand,M> for Player<M>	{
	fn on_move( &mut self, _loc : grid::Location, _grid : &grid::Grid, _field : &field::Field )-> bool	{
		self.do_cancel
	}
	
	fn on_idle( &mut self, member : &M, grid : &grid::Grid, field : &field::Field )-> PlayerCommand	{
		self.do_cancel = false;
		if self.do_click	{
			self.do_click = false;
			let mk = match field.get_by_location( self.target, grid as &grid::TopologyGrid )	{
				&field::CellEmpty	=> return PcomMove( self.target ),
				&field::CellPart(key,_)	=> key,
				_	=> return PcomNone,
			};
			match field.with_member( mk, |m| m.get_team() )	{
				Some(team) if team != member.get_team()	=>	{
					return PcomAttack( self.target )
				}
				_	=> ()
			}
		}
		PcomNone
	}
}
