extern mod std;
extern mod lmath;


trait Interpolate	{
	pure fn interpolate( other : &self, t : float )-> self;
}

pub trait Channel<T>	{
	pure fn sample( time : float )-> T;
}

struct KeySimple<T>	{
	t	: float,
	co	: T,
}

struct KeyBezier<T>	{
	t	: float,
	co	: T,
	hl	: T,
	hr	: T
}


impl<T:Copy Interpolate> @[KeySimple<T>] : Channel<T>	{
	pure fn sample( time : float )-> T	{
		let mut i = self.len();
		while i>0u && self[i-1].t>time	{ i-=1;	}
		if i==self.len()	{
			self[i-1u].co
		}else if i==0u && self[0].t>time	{
			self[0].co
		}else	{
			let a = &self[i-1u], b = &self[i];
			assert a.t < b.t;
			let t = (time - a.t) / (b.t - a.t);
			a.co.interpolate( &b.co, t )	
		}
	}
}

impl<T:Copy Interpolate> @[KeyBezier<T>] : Channel<T>	{
	pure fn sample( time : float )-> T	{
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
			assert a.t < b.t;
			let t = (time - a.t) / (b.t - a.t);
			let va = a.co.interpolate( &a.hr, t );
			let vb = b.hl.interpolate( &b.co, t );
			va.interpolate( &vb, t )
		}
	}
}


pub struct Record<C>	{
	name		: ~str,
	duration	: float,
	channels	: ~[C],
}

pub trait Player<C>	{
	pure fn find_record( name : ~str )-> Option< @Record<C> >;
	fn set_record( rec : &Record<C>, time : float );
}


pub fn get_time()-> float	{
	std::time::precise_time_s()
}

pub trait Act	{
	fn update()-> bool;
}

pub struct Action<C>	{
	player	: @Player<C>,
	record	: @Record<C>,
	start	: float,
}

pub fn make_action<C>( p : @Player<C>, name : ~str )-> Option<Action<C>>	{
	match p.find_record(name)	{
		Some(r)	=> Some(Action	{
			player	: p,
			record	: r,
			start	: get_time(),
		}),
		None => None
	}
}

impl<C> Action<C> : Act	{
	fn update()-> bool	{
		let t = get_time() - self.start;
		if t<0f || t>self.record.duration	{
			false
		}else	{
			self.player.set_record( self.record, t );
			true
		}
	}
}