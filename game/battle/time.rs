extern mod engine;

use std;


pub struct Position([int,..2]);
pub type PartId		= u8;
pub type Health		= uint;

impl Eq for Position	{
	fn eq( &self, other : &Position )-> bool	{
		self[0]==other[0] && self[1]==other[1]
	}
}

pub trait Unit	{
	fn get_name( &self )-> ~str;
	fn iter_parts<'a>( &self )-> std::vec::VecIterator<Position>;
	fn receive_damage( &mut self, damage : Health, part : PartId )-> bool;
}

struct Grid	{
	units		: ~[@mut Unit],
	obstacles	: ~[Position],
}

enum Cell	{
	CellEmpty,
	CellObstacle,
	CellDestructible( Health, @engine::anim::Act ),
	CellUnit( @mut Unit, PartId ),
}

impl Grid	{
	fn query( &self, p : Position )-> Cell	{
		if self.obstacles.contains(&p)	{
			return CellObstacle
		}
		for &unit in self.units.iter()	{
			for (i,&pos) in unit.iter_parts().enumerate()	{
				if pos == p	{
					return CellUnit( unit, i as PartId )
				}
			}
		}
		CellEmpty
	}
	fn apply_damage( &mut self, p : Position, damage : Health )-> bool	{
		let mut cell = self.query( p );
		let (hit,empty) = match cell	{
			CellEmpty		=> (false,false),
			CellObstacle	=> (true,false),
			CellDestructible( ref mut health, _action )	=>	{
				*health -= std::num::min( *health, damage );
				(true,*health==0)
			},
			CellUnit( unit, part )	=>	{
				let done = unit.receive_damage( damage, part );
				(true,done)
			},
		};
		if empty	{
			cell = CellEmpty;
		}
		hit
	}
}


pub type Phase = Option<@Unit>;
pub type Time = uint;

pub struct Line	{
	queue		: ~[~[@Unit]],
	base_time	: Time,
}

impl Line	{
	pub fn add( &mut self, moment : Time, unit : @Unit )	{
		let max_moment = self.base_time + self.queue.len();
		if moment >= max_moment	{
			//self.queue.grow( moment+1 - max_moment, &~[] );
		}
		self.queue[moment - self.base_time].push(unit);
	}

	pub fn process( &mut self, moment : Time )-> ~[@Unit]	{
		let max_moment = self.base_time + self.queue.len();
		if moment >= self.base_time && moment < max_moment	{
			let mut active : ~[@Unit] = ~[];
			std::util::swap( &mut active, &mut self.queue[moment-self.base_time] );
			active
		}else	{~[]}
	}

	pub fn optimize( &mut self )	{
		while !self.queue.is_empty() && self.queue.head().is_empty()	{
			self.queue.shift();
			self.base_time += 1;
		}
	}
}
