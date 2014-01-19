extern mod cgmath;
extern mod engine;

use engine::anim;
use battle::{grid,field,main,motion,unit};
use cgmath::point::Point;
use cgmath::vector::{Vector,Vec2};

pub enum MotionStatus	{
	StatusDone,
	StatusCanInterrupt,
	StatusBusy,
}

pub type MotionPtr = ~Motion:'static;

pub trait Motion	{
	fn get_name<'a>( &'a self )-> &'a str;	//TODO: ToStr
	//fn start()	//custom interface
	fn update( &mut self, &mut main::Member, anim::float, &mut field::Field, &grid::Grid )-> MotionStatus;
	fn stop( &mut self );
}

pub trait Brain<M : unit::Unit>	{
	fn check( &mut self, &M, &field::Field, &grid::Grid )-> bool;
	fn decide( &mut self, &M, &field::Field, &grid::Grid )-> MotionPtr;
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
	fn check( &mut self, _member: &unit::Standard, _field: &field::Field, _grid: &grid::Grid)-> bool	{
		self.do_cancel
	}
	
	fn decide( &mut self, member: &unit::Standard, field: &field::Field, grid: &grid::Grid )-> MotionPtr	{
		self.do_cancel = false;
		if self.do_click	{
			self.do_click = false;
			let topo = grid as &grid::TopologyGrid;
			let m_key : field::MemberKey = 0;
			let link = field.get_member(m_key);
			let loc = link.location;
			match field.get_by_location( self.target, topo )	{
				&field::CellEmpty	=> ~motion::Move{
						destinations: ~[self.target],
						location	: loc,
						orientation	: topo.approximate_orientation( loc, self.target ),
						elevation	: member.elevation,
						speed		: member.move_speed,
					} as MotionPtr,
				&field::CellPart(key,_)	=>	{
					if field.get_member(key).team != link.team	{
						let my_damage = 1;
						~motion::Attack{
							destination	: self.target,
							damage		: my_damage,
							} as MotionPtr
					}else if key == m_key && loc != self.target	{
						~motion::Move{
							destinations: ~[self.target],
							location	: loc,
							orientation	: topo.approximate_orientation( loc, self.target ),
							elevation	: member.elevation,
							speed		: member.move_speed,
							} as MotionPtr
					}else	{
						~motion::Dummy as MotionPtr
					}
				},
				_	=> ~motion::Dummy as MotionPtr,
			}
		}else	{
			motion::Idle::new( &member.skeleton, ~"ArmatureBossAction" ).to_ptr()
		}
	}
}

pub struct Monster<M>	{
	target_key	: field::MemberKey,
}

impl<M> Monster<M>	{
	pub fn new()-> Monster<M>	{
		Monster{
			target_key	: 0,
		}
	}
}

impl<M: main::Member> Brain<M> for Monster<M>	{
	fn check( &mut self, _member: &M, _field: &field::Field, _grid: &grid::Grid )-> bool	{
		self.target_key == 0
	}
	fn decide( &mut self, _member: &M, field: &field::Field, grid: &grid::Grid )-> MotionPtr	{
		let m_key : field::MemberKey = 0;
		let link = field.get_member(m_key);
		let is_valid = field.get_member( self.target_key ).team != link.team;
		if !is_valid	{
			self.target_key = 0;
			field.each_member(|key,_|	{
				//if self.target_key==0 && m.get_team() != member.get_team()	{
				//	self.target_key = key;
				//}
				if self.target_key==0 && key!=m_key	{
					self.target_key = key;
				}
			});
			print!("Chosen target key: {:?}\n", self.target_key);
		}
		if self.target_key == 0	{
			return ~motion::Dummy as MotionPtr
		}
		/*let target_pos = field.with_member( self.target_key, |m|	{
			let (pos, _) = m.get_limbs()[0];
			pos
		}).expect(format!( "Invalid target key: {:?}", self.target_key ));
		*/
		let self_pos = link.location;
		let target_pos = self_pos;
		let neighbors = [
			target_pos.add_v( &Vec2::new(1,0) ),
			target_pos.add_v( &Vec2::new(0,1) ),
			target_pos.add_v( &Vec2::new(-1,0) ),
			target_pos.add_v( &Vec2::new(0,-1) ),
			];
		let mut min_dist = grid.get_index_size() as int;
		let mut best_pos = target_pos;
		let topo = grid as &grid::TopologyGrid;
		for &new_pos in neighbors.iter()	{
			let access = match field.get_by_location( new_pos, topo )	{
				&field::CellEmpty		=> true,
				&field::CellPart(key,_)	=> key==m_key,
				_						=> false,
			};
			let diff = new_pos.sub_p( &self_pos );
			let dist = diff.x*diff.x + diff.y*diff.y;
			if access && dist < min_dist	{
				best_pos = new_pos;
				min_dist = dist;
			}
		}
		let my_elevation = 1.0f32;
		let my_speed = 1.0f32;
		let my_damage = 1u;
		if best_pos != target_pos	{
			~motion::Move{
				destinations: ~[best_pos],
				location	: self_pos,
				orientation	: topo.approximate_orientation( self_pos, best_pos ),
				elevation	: my_elevation,
				speed		: my_speed,
			} as MotionPtr
		}else	{
			~motion::Attack{
				destination	: target_pos,
				damage		: my_damage,
			} as MotionPtr
		}
	}
}

