extern mod engine;

use std;
use battle::{grid,unit};


pub type Team		= int;
pub static team_dead: Team = -1;

pub type MemberKey	= uint;

pub struct Link   {
	name	: ~str,
	team	: Team,
	location    : grid::Location,
	orientation : grid::Orientation,
}


pub enum Cell	{
	CellEmpty,
	CellRock,
	CellPart(MemberKey,~[unit::LimbKey]),
}


struct Member	{
	link	: Link,
	effects	: ~[unit::Effect],
}

pub struct Field	{
	priv cells		: ~[Cell],
	priv members	: ~[Member],
	priv revision	: uint,
}

impl Field	{
	pub fn new( size: uint )-> Field	{
		Field	{
			cells	: std::vec::from_fn( size, |_i| CellEmpty ),
			members	: ~[],
			revision: 0,
		}
	}

	pub fn get_revision( &self )-> uint	{
		self.revision
	}

	pub fn get_cell<'a>( &'a self, index: uint )-> &'a Cell	{
		&self.cells[index]
	}

	pub fn get_by_location<'a>( &'a self, loc: grid::Location, grid: &grid::TopologyGrid )-> &'a Cell	{
		match grid.get_index(loc)	{
			Some(index)	=> self.get_cell(index),
			None		=> &CellRock,
		}
	}

	pub fn get_member<'a>( &'a self, key: MemberKey ) -> &'a Link	{
		&self.members[key].link
	}
	
	pub fn each_member( &self, fun: |MemberKey,&Link| )	{
		for (i,mr) in self.members.iter().enumerate()	{
			fun( i, &mr.link );
		}
	}

	pub fn clear( &mut self )	{
		self.members.clear();
		for cell in self.cells.mut_iter()	{
			*cell = CellEmpty;
		}
		self.revision += 1;
	}
	
	fn add_member_cells( &mut self, mk: MemberKey, member: &Link, unit: &unit::Unit, grid: &grid::TopologyGrid )	{
		unit.each_limb( |lk,_limb,offset|	{
			let pos = grid.offset_position( member.location, member.orientation, offset );
			grid.get_index(pos).map(|index|	{
				match self.cells[index]	{
					CellEmpty	=> self.cells[index] = CellPart( mk,~[lk] ),
					CellPart(mkey,ref mut limbs) if mkey==mk	=> limbs.push(lk),
					_	=> fail!( "Unable to add member {} to cell {}", unit.get_name(), pos.to_str() )
				};
			});
		});
		self.revision += 1;
	}

	pub fn add_member<M: unit::Unit + 'static>( &mut self, member: Link, unit: &M, grid: &grid::TopologyGrid )-> MemberKey	{
		let mk = self.members.len();
		print!( "Field: added member '{:s}' with key {:?}\n", unit.get_name(), mk );
		self.add_member_cells( mk, &member, unit, grid );
		let rec = Member	{
			link	: member,
			effects	: ~[],
		};
		self.members.insert( mk, rec );
		mk
	}

	pub fn fill_grid( &self, texels: &mut [grid::Texel] )	{
		for (src,dst) in self.cells.iter().zip(texels.mut_iter())	{
			*dst = match *src	{
				CellEmpty	=> grid::CELL_EMPTY,
				_			=> grid::CELL_OCCUPIED,
			}
		}
	}
	
	fn remove_member_cells( &mut self, key: MemberKey )	{
		self.cells.retain(|cell|	{
			match cell	{
				&CellPart(mk,_) if mk==key	=> false,
				_	=> true,
			}
		});
		self.revision += 1;
	}

	pub fn remove_member( &mut self, key: MemberKey )	{
		self.remove_member_cells( key );
		self.members[key].link.team = team_dead;
	}
	
	pub fn update_member<M: unit::Unit + 'static>( &mut self, key: MemberKey, member: &Link, unit: &M, grid: &grid::TopologyGrid )-> bool	{
		self.remove_member_cells( key );
		self.add_member_cells( key, member, unit, grid );
		true
	}

	pub fn deal_damage( &mut self, index: uint, limb_key_opt: Option<unit::LimbKey>, damage: unit::Health )-> bool	{
		let (mk,lk) = match self.cells[index]	{
			CellPart(mk,ref limbs) 	=>	{
				let lk = limb_key_opt.unwrap_or(limbs[0]);
				(mk,lk)
			},
			_	=> return false
		};
		self.members[mk].effects.push( unit::EfDamage(lk,damage) );
		self.revision += 1;
		true
	}
}

