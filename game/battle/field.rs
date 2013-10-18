extern mod engine;

use std;


pub struct Position([int,..2]);
pub type Orientation = int;
pub type PartId		= u8;
pub type Health		= uint;

impl Eq for Position	{
	fn eq( &self, other : &Position )-> bool	{
		self[0]==other[0] && self[1]==other[1]
	}
}

pub enum DamageResult	{
	DamageNone,
	DamageDone,
	DamagePartCut,
	DamageKill,
}

pub trait Member	{
	fn get_name<'a>( &'a self )-> &'a str;
	fn iter_parts<'a>( &self )-> std::vec::VecIterator<Position>;
	fn receive_damage( &mut self, damage : Health, part : PartId )-> DamageResult;
	fn get_health( &self )-> Health;
}


struct Field	{
	priv members	: ~[@mut Member],
}

impl Field	{
	pub fn new()-> Field	{
		Field	{
			members		: ~[],
		}
	}

	pub fn with<T>( &self, name : &str, fun : &fn(&Member)->T )->Option<T>	{
		self.members.iter().find(|m| { std::str::eq_slice(name,m.get_name()) }).
			map_move(|m| { fun(*m) })
	}

	pub fn query( &self, p : Position )-> Option<(~str,PartId)>	{
		for m in self.members.iter()	{
			for (i,&pos) in m.iter_parts().enumerate()	{
				if pos == p	{
					return Some(( m.get_name().to_owned(), i as PartId ))
				}
			}
		}
		None
	}

	pub fn clear( &mut self )	{
		self.members.clear();
	}

	pub fn add_member( &mut self, m : @mut Member, _p : Position, _o : Orientation )	{
		self.members.push( m );
	}

	pub fn deal_damage( &mut self, p : Position, damage : Health )-> DamageResult	{
		for m in self.members.iter()	{
			for (i,&pos) in m.iter_parts().enumerate()	{
				if pos == p	{
					return m.receive_damage( damage, i as PartId )
					// FIXME: change self if the part/member is destroyed
				}
			}
		}
		DamageNone
	}
}
