use gen = gen_hud::common;


pub enum MenuAction	{
	ActionFun( ~fn() ),
	ActionList( ~[MenuItem] ),
}

pub struct MenuItem	{
	name	: ~str,
	action	: MenuAction,
}

pub struct Menu	{
	children	: ~[MenuItem],
	selected	: ~[u8],
	font		: gen::Font,
}

impl Menu	{
	pub fn is_active( &self )-> bool	{
		!self.selected.is_empty()
	}

	fn build_vertical( items : &[MenuItem], font : &gen::Font, selected : u8 )-> ~[gen::Child]	{
		items.iter().enumerate().map( |(i,item)|	{
			let ground = if (i as u8)==selected	{
				gen::GroundSolid(0x80808080u)
			} else {
				gen::GroundFrame(0x80808080u)
			};
			let align = if i==0 {
				([-1,-1], gen::RelParent, [-1,-1])
			}else	{
				([-1,-1], gen::RelTail, [-1,1])
			};
			gen::Child	{
				name	: item.name.clone(),
				align	: align,
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

	fn build_horisontal( items : &[MenuItem], font : &gen::Font, selected : &[u8] )-> ~[gen::Child]	{
		~[]
	}
	
	pub fn build( &self, alpha : float )-> gen::Screen	{
		gen::Screen	{
			alpha	: alpha,
			children: Menu::build_vertical( self.children, &self.font, self.selected[0] ),
		}
	}
}
