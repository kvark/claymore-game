extern mod cgmath;
extern mod engine;

use std::to_str::ToStr;
use engine::anim;
use battle::{grid,field,main,motion,unit};
use cgmath::point::Point;

pub enum MotionStatus	{
	StatusDone,
	StatusCanInterrupt,
	StatusBusy,
}

pub type MotionPtr<M> = ~Motion:'static<M>;
pub type MotionStdPtr = MotionPtr<unit::Standard>;

pub trait Motion<M: main::Member> : ToStr	{
	//fn start()	//custom interface
	fn update( &mut self, field::MemberKey, &mut M, anim::float, &mut field::Field, &grid::Grid )-> MotionStatus;
	fn stop( &mut self );
}

pub trait Brain<M : unit::Unit>	{
	fn check( &mut self, field::MemberKey, &M, &field::Field, &grid::Grid )-> bool;
	fn decide( &mut self, field::MemberKey, &M, &field::Field, &grid::Grid )-> MotionPtr<M>;
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

impl Brain<unit::Standard> for Player<unit::Standard>	{
	fn check( &mut self, _mkey: field::MemberKey, _member: &unit::Standard, _field: &field::Field, _grid: &grid::Grid)-> bool	{
		self.do_cancel
	}
	
	fn decide( &mut self, mkey: field::MemberKey, member: &unit::Standard, field: &field::Field, grid: &grid::Grid )-> MotionStdPtr	{
		self.do_cancel = false;
		if self.do_click	{
			self.do_click = false;
			let topo = grid as &grid::TopologyGrid;
			let link = field.get_member(mkey);
			let loc = link.location;
			match field.get_by_location( self.target, topo )	{
				&field::CellEmpty	=> ~motion::Move{
						destinations: ~[self.target],
						orientation	: topo.approximate_orientation( loc, self.target ),
						elevation	: member.elevation,
						speed		: member.move_speed,
					} as MotionStdPtr,
				&field::CellPart(key,_)	=>	{
					if field.get_member(key).team != link.team	{
						let my_damage = 1;
						~motion::Attack{
							destination	: self.target,
							damage		: my_damage,
							} as MotionStdPtr
					}else if key == mkey && loc != self.target	{
						~motion::Move{
							destinations: ~[self.target],
							orientation	: topo.approximate_orientation( loc, self.target ),
							elevation	: member.elevation,
							speed		: member.move_speed,
							} as MotionStdPtr
					}else	{
						~motion::Dummy as MotionStdPtr
					}
				},
				_	=> ~motion::Dummy as MotionStdPtr,
			}
		}else	{
			motion::Idle::new( &member.skeleton, ~"ArmatureBossAction" ).to_ptr()
		}
	}
}

pub struct Monster<M>	{
	target_key	: Option<field::MemberKey>,
}

impl<M> Monster<M>	{
	pub fn new()-> Monster<M>	{
		Monster{
			target_key	: None,
		}
	}
}

impl<M: main::Member> Brain<M> for Monster<M>	{
	fn check( &mut self, _mkey: field::MemberKey, _member: &M, _field: &field::Field, _grid: &grid::Grid )-> bool	{
		self.target_key.is_none()
	}
	fn decide( &mut self, mkey: field::MemberKey, _member: &M, field: &field::Field, grid: &grid::Grid )-> MotionPtr<M>	{
		let (self_pos,self_team) =	{
			let link = field.get_member(mkey);
			(link.location, link.team)
		};
		let is_valid = match self.target_key	{
			Some(key)	=>	{
				let team = field.get_member(key).team;
				team != self_team && team != field::team_dead
			},
			None	=> false,
		};
		if !is_valid	{
			self.target_key = None;
			field.each_member(|key,link|	{
				if self.target_key.is_none() && link.team!=self_team {
					self.target_key = Some(key);
				}
			});
			//print!("Chosen target key: {:?}\n", self.target_key);
		}
		if self.target_key.is_none()	{
			return ~motion::Dummy as MotionPtr<M>
		}
		let target_pos = {
			let link = field.get_member( self.target_key.unwrap() );
			link.location
		};
		let topo = grid as &grid::TopologyGrid;
		let neighbors = topo.get_neighbors( target_pos );
		let mut min_dist = topo.get_index_size() as int;
		let mut best_pos = target_pos;
		for &new_pos in neighbors.iter()	{
			let access = match field.get_by_location( new_pos, topo )	{
				&field::CellEmpty		=> true,
				&field::CellPart(key,_)	=> key==mkey,
				_						=> false,
			};
			if access	{
				let diff = new_pos.sub_p( &self_pos );
				let dist = diff.x*diff.x + diff.y*diff.y;
				if dist < min_dist	{
					best_pos = new_pos;
					min_dist = dist;
				}
			}
		}
		let my_elevation = 1.0f32;
		let my_speed = 1.0f32;
		let my_damage = 1u;
		if best_pos != target_pos	{
			~motion::Move{
				destinations: ~[best_pos],
				orientation	: topo.approximate_orientation( self_pos, best_pos ),
				elevation	: my_elevation,
				speed		: my_speed,
			} as MotionPtr<M>
		}else	{
			~motion::Attack{
				destination	: target_pos,
				damage		: my_damage,
			} as MotionPtr<M>
		}
	}
}

