extern mod engine;

use std;
use battle::grid;

pub use battle::grid::{Location,Offset,Orientation};
pub type PartId		= u8;
pub type Health		= uint;


pub enum DamageResult	{
	DamageNone,
	DamageSome,
	DamagePartCut,
	DamageKill,
}

pub trait Member	{
	fn get_name<'a>( &'a self )-> &'a str;
	fn get_health( &self )-> Health;
	fn get_parts<'a>( &'a self )-> &'a [Offset];
	fn receive_damage( &mut self, damage : Health, part : Option<PartId> )-> DamageResult;
	fn is_busy( &self )-> bool;
}

fn is_same_member(a : @mut Member, b : @mut Member)-> bool	{
	//std::managed::mut_ptr_eq(a,b)
	std::str::eq_slice( a.get_name(), b.get_name() )
}

pub enum Cell<T>	{
	CellEmpty,
	CellObstacle,
	CellCore(T,Orientation),
	CellPart(T,PartId),
}

struct Field	{
	priv cells		: ~[Cell<@mut Member>],
}

impl Field	{
	pub fn new( size : uint )-> Field	{
		Field	{
			cells	: std::vec::from_fn( size, |_i| CellEmpty ),
		}
	}

	pub fn get( &self, index : uint )-> Cell<~str>	{
		match self.cells[index]	{
			CellEmpty		=> CellEmpty,
			CellObstacle	=> CellObstacle,
			CellCore(m,o)	=> CellCore( m.get_name().to_owned(), o ),
			CellPart(m,p)	=> CellPart( m.get_name().to_owned(), p ),
		}
	}

	pub fn get_by_location( &self, loc : Location, grid : &grid::TopologyGrid )-> (Option<uint>,Cell<~str>)	{
		match grid.get_index(loc)	{
			Some(index)	=> (Some(index), self.get(index)),
			None	=> (None, CellEmpty),
		}
	}

	pub fn with_member<T>( &self, name : &str, fun : &fn(&Member,Orientation)->T )->Option<T>	{
		for cell in self.cells.iter()	{
			match cell	{
				&CellCore(m,o) if std::str::eq_slice(name,m.get_name())	=> return Some(fun(m,o)),
				_	=> ()
			}
		}
		None
	}

	pub fn has_member( &self, m : &Member )-> bool	{
		self.with_member( m.get_name(), |_,_| () ).is_some()
	}

	pub fn clear( &mut self )	{
		for cell in self.cells.mut_iter()	{
			*cell = CellEmpty;
		}
	}

	pub fn add_member( &mut self, member : @mut Member, d : Location, o : Orientation, grid : &grid::TopologyGrid )	{
		assert!( !self.has_member(member) );
		let core_index = grid.get_index(d).expect("Member core position should exist");
		self.cells[ core_index ] = CellCore( member, o );
		for (i,&offset) in member.get_parts().iter().enumerate()	{
			let pos = grid.offset_position( d, o, offset );
			match grid.get_index(pos)	{
				Some(index)	=> self.cells[index] = CellPart(member,i as PartId),
				None		=> ()
			}
		}
	}

	pub fn fill_grid( &self, texels : &mut [grid::Texel] )	{
		for (src,dst) in self.cells.iter().zip(texels.mut_iter())	{
			*dst = match *src	{
				CellEmpty	=> grid::CELL_EMPTY,
				_			=> grid::CELL_OCCUPIED,
			}
		}
	}

	pub fn remove_member( &mut self, name : &str )	{
		for cell in self.cells.mut_iter()	{
			let clear = match cell	{
				&CellCore(m,_)	if std::str::eq_slice(name,m.get_name())	=> true,
				&CellPart(m,_)	if std::str::eq_slice(name,m.get_name())	=> true,
				_	=> false
			};
			if clear	{
				*cell = CellEmpty;
			}
		}
	}

	pub fn deal_damage( &mut self, index : uint, damage : Health )-> DamageResult	{
		let (dr,mo) = match self.cells[index]	{
			CellCore(m,_)	=> (m.receive_damage( damage, None ),	Some(m)),
			CellPart(m,p)	=> (m.receive_damage( damage, Some(p) ),Some(m)),
			_	=> (DamageNone,None),
		};
		match dr	{
			DamagePartCut	=> self.cells[index] = CellEmpty,
			DamageKill		=> self.remove_member( mo.unwrap().get_name() ),
			_	=> ()
		}
		dr
	}
}
