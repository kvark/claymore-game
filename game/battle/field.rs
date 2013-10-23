extern mod engine;

use std;
use std::hashmap::HashMap;
use battle::grid;

pub use Position	= battle::grid::Location;
pub use battle::grid::Orientation;
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

fn compute_position( base : &Position, _o : Orientation, off : &Position )-> Position	{
	Position::new( base[0] + off[0], base[1] + off[1] )
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
	fn is_busy( &self )-> bool;
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

	pub fn reset( &mut self )	{
		self.members.clear();
		self.cells.clear();
	}

	fn has_member( &mut self, member : @mut Member )-> bool	{
		self.cells.iter().
			find( |&(_,&(m,_))| is_same_member(m,member) ).
			is_some()
	}

	pub fn add_member( &mut self, member : @mut Member, p : Position, o : Orientation )	{
		assert!( !self.has_member(member) );
		self.members.push(( member, o ));
		for (i,&offset) in member.get_parts().iter().enumerate()	{
			let pos = compute_position( &p, o, &offset );
			self.cells.insert( pos, (member,i as PartId) );
		}
	}

	pub fn remove_member( &mut self, member : @mut Member, parts_removed : uint )	{
		let parts_left = member.get_parts().len() - parts_removed;
		if parts_left > 0	{
			let positions = self.cells.iter().
				filter( |&(_,&(m,_))| is_same_member(m,member) )
				.map( |(&p,_)| p ).to_owned_vec();
			assert_eq!( positions.len(), parts_left );
			for p in positions.iter()	{
				self.cells.remove(p);
			}
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
				self.remove_member( member, 1 );
			},
			_	=> ()
		}
		dr
	}

	pub fn fill_grid( &self, grid : &mut grid::MutableGrid )	{
		for (&pos,_) in self.cells.iter()	{
			grid.set_cell( pos, grid::CellOccupied );
		}
	}
}
