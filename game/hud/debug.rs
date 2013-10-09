use engine;
use gen = gen_hud::common;
use hud = hud::main;


pub enum MenuAction	{
	ActionFun( ~fn() ),
	ActionList( ~[MenuItem] ),
}

pub struct MenuItem	{
	name	: ~str,
	action	: MenuAction,
}


pub struct MenuListIter<'self>	{
	priv item		: &'self MenuItem,
	priv selection	: &'self [u8],
}

type ListIterUnit<'self> = (&'self str, &'self [MenuItem]);

impl<'self> Iterator<ListIterUnit<'self>> for MenuListIter<'self>	{
	fn next( &mut self )-> Option<ListIterUnit<'self>>	{
		match self.selection	{
			[head,..tail]	=>	{
				match self.item.action	{
					ActionList(ref list)	=>	{
						assert!( (head as uint) < list.len() );
						let name = self.item.name.as_slice();
						self.selection = tail;
						self.item = &'self list[head];
						Some(( name, list.as_slice() ))
					},
					_	=> fail!("Unexpected end of debug menu on item %s", self.item.name),
				}
			},
			[]	=> None,
		}
	}
}


pub struct MenuAllIter<'self>	{
	priv stack	: ~[&'self [MenuItem]],
	priv item	: &'self MenuItem,
}

impl<'self> Iterator<&'self MenuItem> for MenuAllIter<'self>	{
	fn next( &mut self )-> Option<&'self MenuItem>	{
		let list = match self.item.action	{
			ActionList(ref list) if !list.is_empty()=> list.as_slice(),
			_	if !self.stack.is_empty()			=> self.stack.pop(),
			_	=> return None,
		};
		self.item = &'self list[0];
		if list.len() > 1	{
			self.stack.push( list.slice_from(1) );
		}
		Some( self.item )
	}
}



static item_bound : [uint,..2] = [200,50];

pub struct Menu	{
	root		: MenuItem,
	selection	: ~[u8],
	font		: gen::Font,
}

impl Menu	{
	pub fn is_active( &self )-> bool	{
		!self.selection.is_empty()
	}

	pub fn selection_list_iter<'a>( &'a self )-> MenuListIter<'a>	{
		MenuListIter	{
			item		: &'a self.root,
			selection	: self.selection,
		}
	}

	pub fn get_selected_item<'a>( &'a self )-> &'a MenuItem	{
		let (_,ref list) = self.selection_list_iter().last().
			expect("Debug menu is not active");
		&'a list[ *self.selection.last() ]
	}

	pub fn all_iter<'a>( &'a self )-> MenuAllIter<'a>	{
		MenuAllIter	{
			stack	: ~[],
			item	: &'a self.root,
		}
	}

	pub fn preload( &self, hcon : &mut hud::Context, fcon : &engine::gr_mid::font::Context, 
			gcon : &mut engine::gr_low::context::Context, lg : &engine::journal::Log )	{
		let fc = hcon.preload_font( &self.font, fcon, lg );
		for item in self.all_iter()	{
			fc.cache.find_or_insert_with( item.name.clone(), |s|	{
				let bound = ( item_bound[0], item_bound[1] );
				fc.font.bake( gcon, *s, bound, lg )
			});
		}
	}

	fn build_vertical( items : &[MenuItem], font : &gen::Font, selected : u8 )-> ~[gen::Child]	{
		items.iter().enumerate().map( |(i,item)|	{
			let ground = if (i as u8)==selected	{
				gen::GroundSolid(0x80808080u)
			} else {
				gen::GroundFrame(0x80808080u)
			};
			gen::Child( item.name.clone(), gen::ElBox(
				gen::SizeRel( 1.0 ), gen::SizeAbs( 100 ),
				gen::Box	{
					align	: gen::AlignHor,
					ground	: ground,
					children: ~[
						gen::Child( ~"", gen::ElText(gen::Text	{
							value	: item.name.clone(),
							font	: font.clone(),
							color	: 0xFFFFFFFF,
							bound	: item_bound,
							edit	: false,
						})),
					],
				}
			))
		}).to_owned_vec()
	}

	fn build_horisontal( &self )-> ~[gen::Child]	{
		let mut item = &self.root;
		self.selection.iter().map( |&sel_id|	{
			let list = match item.action	{
				ActionList(ref l) if l.len()>(sel_id as uint)	=> l.as_slice(),
				_	=> fail!("Unexpected tail of debug menu: %s", item.name),
			};
			item = &list[sel_id];
			gen::Child( ~"group", gen::ElBox(
				gen::SizeRel( 0.3 ), gen::SizeRel( 1.0 ),
				gen::Box	{
					align	: gen::AlignVer,
					ground	: gen::GroundNone,
					children: Menu::build_vertical( list, &self.font, sel_id ),
				}
			))
		}).to_owned_vec()
	}
	
	pub fn build( &self, alpha : float )-> gen::Screen	{
		gen::Screen	{
			alpha	: alpha,
			root	: gen::Box{
				align	: gen::AlignHor,
				ground	: gen::GroundNone,
				children: ~[
					gen::Child( ~"tab",		gen::ElSpace( [100,100] ) ),
					gen::Child( ~"menu",	gen::ElBox(
						gen::SizeRel( 0.8 ), gen::SizeRel( 1.0 ),
						gen::Box{
							align	: gen::AlignHor,
							ground	: gen::GroundNone,
							children: self.build_horisontal(),
						}
					)),
				],
			},
		}
	}
}
