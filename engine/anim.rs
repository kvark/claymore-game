use std;
use std::{cell,rc};

use space::Interpolate;

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Timers											//
pub type float = f64;

pub struct Timer	{
	time		: float,
	speed		: float,
	max_delta	: float,
	active		: bool,
}

impl Timer	{
	pub fn new()-> Timer	{
		Timer	{
			time		: 0.0,
			speed		: 1.0,
			max_delta	: 1.0,
			active		: true,
		}
	}
	pub fn update( &mut self, delta: float )	{
		if self.active	{
			assert!( delta >= 0.0 );
			let d = std::num::min( delta, self.max_delta );
			self.time += self.speed * d;
		}
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Curves											//

pub trait Curve<T>	{
	fn sample( &self, float )-> T;
}

pub struct KeySimple<T>	{
	t	: float,
	co	: T,
}

pub struct KeyBezier<T>	{
	t	: float,
	co	: T,
	hl	: T,
	hr	: T
}

impl<T:Clone + Interpolate> Curve<T> for ~[KeySimple<T>]	{
	fn sample( &self, time: float )-> T	{
		let mut i = self.len();
		while i>0u && self[i-1].t>time	{ i-=1;	}
		if i==self.len()	{
			self[i-1u].co.clone()
		}else if i==0u && self[0].t>time	{
			self[0].co.clone()
		}else	{
			let a = &self[i-1u];
			let b = &self[i];
			assert!( a.t < b.t );
			let t = (time - a.t) / (b.t - a.t);
			a.co.interpolate( &b.co, t as f32 ).clone()
		}
	}
}

impl<T:Interpolate> Curve<T> for ~[KeyBezier<T>]	{
	fn sample( &self, time: float )-> T	{
		let mut i = self.len();
		while i>0u && self[i-1].t>time	{ i-=1;	}
		if i==self.len()	{
			let a = &self[i-1u];
			a.co.interpolate( &a.hr, (time-a.t) as f32 )
		}else if i==0u && self[0].t>time	{
			let a = &self[0];
			a.co.interpolate( &a.hl, (a.t-time) as f32 )
		}else	{
			let a = &self[i-1u];
			let b = &self[i];
			assert!( a.t < b.t );
			let t = (time - a.t) / (b.t - a.t);
			let va = a.co.interpolate( &a.hr, t as f32 );
			let vb = b.hl.interpolate( &b.co, t as f32 );
			va.interpolate( &vb, t as f32 )
		}
	}
}


//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Player and Act trait							//


pub struct Record<C>	{
	name		: ~str,
	duration	: float,
	curves		: ~[C],
}

pub type RecordPtr<C> = rc::Rc<Record<C>>;

impl<C> Record<C>	{
	pub fn to_ptr( self )-> RecordPtr<C>	{
		rc::Rc::new( self )
	}
}

pub trait Player<C>	{
	//fn record_iter( self )-> std::vec::VecIterator< @Record<C> >;
	fn find_record( &self, &str )-> Option< RecordPtr<C> >;
	fn set_record( &mut self, &Record<C>, float );
}

pub type PlayerPtr<C> = rc::Rc<cell::RefCell< ~Player<C> >>;	//FIXME

pub trait Act	{
	fn update( &mut self, float )-> bool;
}

pub struct Wait	{
	end	: float,
}

impl Wait	{
	pub fn new( end_time: float )-> Wait	{
		Wait{
			end : end_time	
		} 
	}
}

impl Act for Wait	{
	fn update( &mut self, time: float )-> bool	{
		time < self.end
	}
}


pub struct Action<C>	{
	player	: PlayerPtr<C>,
	record	: RecordPtr<C>,
	start	: float,
}

impl<C> Action<C>	{
	pub fn new( player: PlayerPtr<C>, name: ~str, time: float )-> Option<Action<C>>	{
		let rec_opt = player.borrow().with( |p| p.find_record(name) );
		match rec_opt	{
			Some(r)	=> Some(Action{
				player	: player,
				record	: r,
				start	: time,
			}),
			None	=> None,
		}
	}
}

impl<C> Act for Action<C>	{
	fn update( &mut self, time: float )-> bool	{
		let t = time - self.start;
		if t>=0.0 && t<=self.record.borrow().duration	{
			self.player.borrow().with_mut(|p|	{
				p.set_record( self.record.borrow(), t )
			});
			true
		}else	{false}
	}
}
