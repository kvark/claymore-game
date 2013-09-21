use core::util;

pub type Coordinate = [int,..2];

pub trait Unit	{
	fn get_name()-> ~str;
	fn get_parts()-> ~[Coordinate];
}


pub type Time = uint;

pub struct Line	{
	queue		: ~[~[@Unit]],
	base_time	: Time,
}

impl Line	{
	pub fn process( &mut self, moment : Time, fun : &fn(@Unit)->uint )	{
		assert!( moment >= self.base_time )
		let max_moment = self.base_time + self.queue.len();
		if moment >= max_moment	{
			return
		}
		let mut active : ~[@Unit] = ~[];
		util::swap( &mut active, &mut self.queue[moment-self.base_time] );
		for active.each |&unit|	{
			let new_moment = moment + fun(unit);
			if new_moment >= max_moment	{
				self.queue.grow( new_moment+1 - max_moment, &~[] );
			}
			self.queue[new_moment - self.base_time].push(unit);
		}
	}

	pub fn optimize( &mut self )	{
		while !self.queue.is_empty() && self.queue.head().is_empty()	{
			self.queue.shift();
			self.base_time += 1;
		}
	}
}
