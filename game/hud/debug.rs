use gen = gen_hud::common;


pub enum MenuAction	{
	ActionFun( ~fn() ),
	ActionList( ~[MenuItem] ),
}

pub struct MenuItem	{
	name	: ~str,
	action	: MenuAction,
}


pub struct MenuSelectionIter<'self>	{
	priv item		: &'self MenuItem,
	priv selection	: &'self [u8],
}

impl<'self> Iterator<&'self MenuItem> for MenuSelectionIter<'self>	{
	fn next( &mut self )-> Option<&'self MenuItem>	{
		match self.selection	{
			[head,..tail]	=>	{
				self.item = match self.item.action	{
					ActionList(ref list) if list.len()>(head as uint)	=>	{
						self.selection = tail;
						&'self list[head]
					},
					_	=> fail!("Unexpected end of debug menu on item %s", self.item.name),
				};
				Some(self.item)
			},
			[]	=> None,
		}
	}
}


pub struct Menu	{
	root		: MenuItem,
	selection	: ~[u8],
	font		: gen::Font,
}

impl Menu	{
	pub fn is_active( &self )-> bool	{
		!self.selection.is_empty()
	}

	
	/*
	pub fn selection_iter( &self )-> MenuSelectionIter<'self>	{
		MenuSelectionIter	{
			item		: &'self self.root,
			selection	: self.selection,
		}
	}*/

	fn build_vertical( items : &[MenuItem], font : &gen::Font, selected : u8 )-> ~[gen::Child]	{
		items.iter().enumerate().map( |(i,item)|	{
			let ground = if (i as u8)==selected	{
				gen::GroundSolid(0x80808080u)
			} else {
				gen::GroundFrame(0x80808080u)
			};
			gen::Child	{
				name	: item.name.clone(),
				align	: ([-1,-1], gen::RelTail, [-1,1]),
				element	: gen::ElFrame(gen::Frame	{
					margin	: [[0,0],[0,0]],
					ground	: ground,
					children: ~[gen::Child{
						name	: ~"",
						align	: ([0,0], gen::RelParent, [0,0]),
						element	: gen::ElText(gen::Text	{
							value	: item.name.clone(),
							font	: font.clone(),
							color	: 0xFFFFFFFF,
							bound	: [0,0],
							edit	: false,
						}),
					}],
				}),
			}
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
			gen::Child	{
				name	: ~"group",
				align	: ([-1,1], gen::RelTail, [1,1]),
				element	: gen::ElFrame(gen::Frame	{
					margin	: [[0,0],[0,0]],
					ground	: gen::GroundNone,
					children: Menu::build_vertical( list, &self.font, sel_id ),
				}),
			}
		}).to_owned_vec()
	}
	
	pub fn build( &self, alpha : float )-> gen::Screen	{
		gen::Screen	{
			alpha	: alpha,
			children: ~[
				gen::Child	{
					name	: ~"tab",
					align	: ([-1,1], gen::RelParent, [-1,1]),
					element	: gen::ElSpace( [100,100] ),
				},
				gen::Child	{
					name	: ~"menu",
					align	: ([-1,1], gen::RelHead,[1,-1]),
					element	: gen::ElFrame(gen::Frame{
						margin	: [[0,0],[0,0]],
						ground	: gen::GroundNone,
						children: self.build_horisontal(),
					}),
				}
			],
		}
	}
}
