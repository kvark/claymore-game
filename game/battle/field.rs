extern mod engine;

use std;
use battle::grid;


pub type Health		= uint;
pub type Team		= int;

#[deriving(Clone,Eq)]
pub enum LimbType	{
	LimbBody,
	LimbHead,
	LimbArm,
	LimbLeg,
	LimbTail,
}

impl LimbType	{
	pub fn is_vital( &self )-> bool	{
		match *self	{
			LimbBody | LimbHead	=> true,
			_	=> false,
		}
	}
}

pub type LimbId		= u8;
pub type LimbKey	= (LimbType,LimbId);

#[deriving(Clone)]
pub struct Limb	{
	key		: LimbKey,
	health	: Health,
	node	: @mut engine::space::Node,
}

pub type MemberKey	= int;
pub type MemberLimb	= (grid::Location,Limb);

pub enum Cell	{
	CellEmpty,
	CellRock,
	CellPart(MemberKey,~[Limb]),
}

pub enum DamageResult	{
	DamageNone,
	DamageSome,
	DamageLimbCut,
	DamageKill,
}


pub trait Member	{
	fn get_name<'a>( &'a self )-> &'a str;
	fn get_limbs<'a>( &'a self )-> &'a [MemberLimb];
	fn get_team( &self )-> Team;
	fn move( &mut self, grid::Location, grid::Orientation );
	fn receive_damage( &mut self, Health, LimbKey )-> DamageResult;
	fn is_same(a : @mut Member, b : @mut Member)-> bool	{
		//std::managed::mut_ptr_eq(a,b)
		std::str::eq_slice( a.get_name(), b.get_name() )
	}
}

pub struct Field	{
	priv cells		: ~[Cell],
	priv members	: std::hashmap::HashMap<MemberKey,@mut Member>,
	priv revision	: uint,
	priv next_key	: MemberKey,
}

impl Field	{
	pub fn new( size : uint )-> Field	{
		Field	{
			cells	: std::vec::from_fn( size, |_i| CellEmpty ),
			members	: std::hashmap::HashMap::new(),
			revision: 0,
			next_key: 1,
		}
	}

	pub fn get_revision( &self )-> uint	{
		self.revision
	}

	pub fn get_cell<'a>( &'a self, index : uint )-> &'a Cell	{
		&self.cells[index]
	}

	pub fn get_by_location<'a>( &'a self, loc : grid::Location, grid : &grid::TopologyGrid )-> &'a Cell	{
		match grid.get_index(loc)	{
			Some(index)	=> self.get_cell(index),
			None		=> &CellRock,
		}
	}

	pub fn with_member<T>( &self, key : MemberKey, fun : |&Member|->T )->Option<T>	{
		self.members.find(&key).map(|m| fun(*m))
	}

	pub fn clear( &mut self )	{
		self.members.clear();
		for cell in self.cells.mut_iter()	{
			*cell = CellEmpty;
		}
	}
	
	fn add_member_cells( &mut self, m_key: MemberKey, m_limbs: &[MemberLimb], grid: &grid::TopologyGrid )	{
		for &(pos,limb) in m_limbs.iter()	{
			grid.get_index(pos).map(|index|	{
				match self.cells[index]	{
					CellEmpty	=> self.cells[index] = CellPart( m_key,~[limb] ),
					CellPart(mkey,ref mut limbs) if mkey==m_key	=> limbs.push(limb),
					_	=> assert!({let (ref kind,_) = limb.key; !kind.is_vital() }),
				};
			});
		}
	}

	pub fn add_member( &mut self, member : @mut Member, grid : &grid::TopologyGrid )-> MemberKey	{
		let mk = self.next_key;
		self.next_key += 1;
		self.members.insert( mk, member );
		self.add_member_cells( mk, member.get_limbs(), grid );
		self.revision += 1;
		mk
	}

	pub fn fill_grid( &self, texels : &mut [grid::Texel] )	{
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
	}

	pub fn remove_member( &mut self, key: MemberKey )	{
		self.remove_member_cells( key );
		self.members.remove( &key );
		self.revision += 1;
	}
	
	pub fn update_member( &mut self, mk: MemberKey, grid : &grid::TopologyGrid )-> bool	{
		let m = match self.members.find( &mk )	{
			Some(mem)	=> *mem,
			None		=> return false,
		};
		self.remove_member_cells( mk );
		self.add_member_cells( mk, m.get_limbs(), grid );
		true
	}

	pub fn deal_damage( &mut self, index : uint, limb_key : LimbKey, damage : Health )-> DamageResult	{
		let mk = match self.cells[index]	{
			CellPart(mk,ref limbs) 	=>	{
				assert!( limbs.iter().find(|l| l.key==limb_key).is_some() );
				mk
			},
			_	=> return DamageNone
		};
		self.revision += 1;
		self.members.get(&mk).receive_damage( damage, limb_key )
	}
}
