extern mod engine;

use std;
use std::hashmap::HashMap;


pub struct Position([int,..2]);
pub type Orientation = int;
pub type PartId		= u8;
pub type Health		= uint;

impl Eq for Position	{
	fn eq( &self, other : &Position )-> bool	{
		self[0]==other[0] && self[1]==other[1]
	}
}
impl IterBytes for Position	{
	fn iter_bytes( &self, lsb0 : bool, f : std::to_bytes::Cb )-> bool	{
		self[0].iter_bytes( lsb0, |x| f(x) ) &&
		self[1].iter_bytes( lsb0, |x| f(x) )
  	}
}

impl Position	{
	pub fn add( &self, p : &Position, _o : Orientation )-> Position	{
		Position([ self[0] + p[0], self[1] + p[1] ])
	}
}


pub enum DamageResult	{
	DamageNone,
	DamageSome,
	DamagePartCut,
	DamageKill,
}

pub trait Member	{
	fn get_name<'a>( &'a self )-> &'a str;
	fn get_health( &self )-> Health;
	fn get_parts<'a>( &'a self )-> &'a [Position];
	fn receive_damage( &mut self, damage : Health, part : PartId )-> DamageResult;
}

fn is_same_member(a : @mut Member, b : @mut Member)-> bool	{
	//std::managed::mut_ptr_eq(a,b)
	std::str::eq_slice( a.get_name(), b.get_name() )
}


struct Field	{
	priv members	: ~[(@mut Member,Orientation)],
	priv cells		: HashMap<Position,(@mut Member,PartId)>,
}

impl Field	{
	pub fn new()-> Field	{
		Field	{
			members	: ~[],
			cells	: HashMap::new(),
		}
	}

	pub fn with<T>( &self, name : &str, fun : &fn(&Member)->T )->Option<T>	{
		self.members.iter().find(|& &(m,_)| { std::str::eq_slice(name,m.get_name()) }).
			map_move(|&(m,_)| { fun(m) })
	}

	pub fn query( &self, p : Position )-> Option<(~str,PartId)>	{
		self.cells.find(&p).map_move(|&(m,part)| {( m.get_name().to_owned(), part )})
	}

	pub fn clear( &mut self )	{
		self.members.clear();
		self.cells.clear();
	}

	pub fn add_member( &mut self, m : @mut Member, p : Position, o : Orientation )	{
		self.members.push(( m, o ));
		for (i,&offset) in m.get_parts().iter().enumerate()	{
			let pos = p.add( &offset, o );
			self.cells.insert( pos, (m,i as PartId) );
		}
	}

	pub fn remove_member( &mut self, member : @mut Member )	{
		let positions = self.cells.iter().
			filter( |&(_,&(m,_))| is_same_member(m,member) )
			.map( |(&p,_)| p ).to_owned_vec();
		for p in positions.iter()	{
			self.cells.remove(p);
		}
		self.members.retain( |&(m,_)| is_same_member(m,member) );
	}

	pub fn deal_damage( &mut self, pos : Position, damage : Health )-> DamageResult	{
		let dr = match self.cells.find(&pos)	{
			Some(&(m,part))	=> m.receive_damage( damage, part ),	//FIXME
			None	=> DamageNone,
		};
		match dr	{
			DamagePartCut	=> {
				self.cells.remove(&pos);
			},
			DamageKill		=> {
				let (member,_) = self.cells.pop(&pos).unwrap();
				self.remove_member( member );
			},
			_	=> ()
		}
		dr
	}
}
