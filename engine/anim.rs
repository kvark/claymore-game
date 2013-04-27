extern mod std;

use space::Interpolate;

//- - - - - - - - - - - - - - - - - - - - - - - - - - - //
//		Curves											//

pub trait Curve<T>	{
	fn sample( &self, time : float )-> T;
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

impl<T:Copy+Interpolate> Curve<T> for ~[KeySimple<T>]	{
	fn sample( &self, time : float )-> T	{
		let mut i = self.len();
		while i>0u && self[i-1].t>time	{ i-=1;	}
		if i==self.len()	{
			self[i-1u].co
		}else if i==0u && self[0].t>time	{
			self[0].co
		}else	{
			let a = &self[i-1u], b = &self[i];
			assert!( a.t < b.t );
			let t = (time - a.t) / (b.t - a.t);
			a.co.interpolate( &b.co, t )	
		}
	}
}

impl<T:Copy+Interpolate> Curve<T> for ~[KeyBezier<T>]	{
	fn sample( &self, time : float )-> T	{
		let mut i = self.len();
		while i>0u && self[i-1].t>time	{ i-=1;	}
		if i==self.len()	{
			let a = &self[i-1u];
			a.co.interpolate( &a.hr, time-a.t )
		}else if i==0u && self[0].t>time	{
			let a = &self[0];
			a.co.interpolate( &a.hl, a.t-time )
		}else	{
			let a = &self[i-1u], b = &self[i];
			assert!( a.t < b.t );
			let t = (time - a.t) / (b.t - a.t);
			let va = a.co.interpolate( &a.hr, t );
			let vb = b.hl.interpolate( &b.co, t );
			va.interpolate( &vb, t )
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

pub trait Player<C>	{
	//FIXME: use &str when possible
	fn find_record( &self, name : ~str )-> Option< @Record<C> >;
	fn set_record( &mut self, rec : &Record<C>, time : float );
}


pub fn get_time()-> float	{
	//std::time::precise_time_s()
	let tm = std::time::get_time();
	(tm.sec as float) + 0.000000001f * (tm.nsec as float)
}

pub trait Act	{
	fn update( &mut self )-> bool;
}

pub struct Delay	{
	end	: float,
}

pub impl Delay	{
	fn new( time : float )-> Delay	{
		Delay{
			end : get_time() + time	
		} 
	}
}

impl Act for Delay	{
	fn update( &mut self )-> bool	{
		get_time() < self.end
	}
}


pub struct Action<C>	{
	player	: @Player<C>,
	record	: @Record<C>,
	start	: float,
}

pub impl<C> Action<C>	{
	fn new( p : @Player<C>, name : ~str )-> Option<Action<C>>	{
		match p.find_record(name)	{
			Some(r)	=> Some(Action	{
				player	: p,
				record	: r,
				start	: get_time(),
			}),
			None => None
		}
	}
}

impl<C> Act for Action<C>	{
	fn update( &mut self )-> bool	{
		let t = get_time() - self.start;
		if t>=0f && t<=self.record.duration	{
			self.player.set_record( self.record, t );
			true
		}else	{false}
	}
}