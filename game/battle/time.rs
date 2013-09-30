use std::util;


pub type Coordinate = [int,..2];

pub trait Unit	{
	fn get_name()-> ~str;
	fn get_parts()-> ~[Coordinate];
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
			util::swap( &mut active, &mut self.queue[moment-self.base_time] );
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
