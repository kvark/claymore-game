use engine;
use gen = gen_hud::common;
use hud = hud::main;


pub enum MenuAction<T>	{
	ActionFun( proc(&mut T) ),
	ActionList( ~[MenuItem<T>] ),
}

pub struct MenuItem<T>	{
	name	: ~str,
	action	: MenuAction<T>,
}


pub trait AccessMut<T>{
	fn access_mut<'a>(&'a mut self)-> &'a mut T;
}

impl<T> MenuItem<T>	{
	pub fn convert<U : AccessMut<T>>( self )-> MenuItem<U>	{
		let MenuItem{ name, action } = self;
		MenuItem	{
			name	: name,
			action	: match action	{
				ActionFun(f)		=> do ActionFun	|u:&mut U| {f(u.access_mut())},
				ActionList(list)	=> ActionList(
					list.move_iter().map(
						|item| {item.convert()}
					).to_owned_vec()
				),
			},
		}
	}
}


pub struct MenuListIter<'a,T>	{
	priv item		: &'a MenuItem<T>,
	priv selection	: &'a [u8],
}

type ListIterUnit<'a,T> = (&'a str, &'a [MenuItem<T>]);

impl<'a,T> Iterator<ListIterUnit<'a,T>> for MenuListIter<'a,T>	{
	fn next( &mut self )-> Option<ListIterUnit<'a,T>>	{
		match self.selection	{
			[head,..tail]	=>	{
				match self.item.action	{
					ActionList(ref list)	=>	{
						assert!( (head as uint) < list.len() );
						let name = self.item.name.as_slice();
						self.selection = tail;
						self.item = &'a list[head];
						Some(( name, list.as_slice() ))
					},
					_	=> fail!("Unexpected end of debug menu on item {:s}", self.item.name),
				}
			},
			[]	=> None,
		}
	}
}


pub struct MenuAllIter<'a,T>	{
	priv stack	: ~[&'a [MenuItem<T>]],
	priv item	: &'a MenuItem<T>,
}

impl<'a,T> Iterator<&'a MenuItem<T>> for MenuAllIter<'a,T>	{
	fn next( &mut self )-> Option<&'a MenuItem<T>>	{
		let list = match self.item.action	{
			ActionList(ref list) if !list.is_empty()=> list.as_slice(),
			_	if !self.stack.is_empty()			=> self.stack.pop().unwrap(),
			_	=> return None,
		};
		self.item = &'a list[0];
		if list.len() > 1	{
			self.stack.push( list.slice_from(1) );
		}
		Some( self.item )
	}
}



static item_bound : [uint,..2] = [200,50];

pub struct Menu<T>	{
	root		: MenuItem<T>,
	selection	: ~[u8],
	font		: gen::Font,
}

impl<T> Menu<T>	{
	pub fn is_active( &self )-> bool	{
		!self.selection.is_empty()
	}

	pub fn selection_list_iter<'a>( &'a self )-> MenuListIter<'a,T>	{
		MenuListIter	{
			item		: &'a self.root,
			selection	: self.selection,
		}
	}

	pub fn get_selected_item<'a>( &'a self )-> &'a MenuItem<T>	{
		let (_,ref list) = self.selection_list_iter().last().
			expect("Debug menu is not active");
		&'a list[ *self.selection.last().unwrap() ]
	}

	pub fn all_iter<'a>( &'a self )-> MenuAllIter<'a,T>	{
		MenuAllIter	{
			stack	: ~[],
			item	: &'a self.root,
		}
	}

	pub fn preload( &self, gcon : &mut engine::gr_low::context::Context,
			fcon : &engine::gr_mid::font::Context, hcon : &mut hud::Context,
			lg : &engine::journal::Log )	{
		let fc = hcon.preload_font( &self.font, fcon, lg );
		for item in self.all_iter()	{
			fc.cache.find_or_insert_with( item.name.clone(), |s|	{
				let bound = ( item_bound[0], item_bound[1] );
				fc.font.borrow().bake( gcon, *s, bound, lg )
			});
		}
	}

	fn build_vertical( items : &[MenuItem<T>], font : &gen::Font, selected : u8 )-> ~[gen::Child]	{
		items.iter().enumerate().map( |(i,item)|	{
			let ground = if (i as u8)==selected	{
				gen::GroundSolid(0x80808080u)
			} else {
				gen::GroundFrame(0x80808080u, 2.0)
			};
			gen::Child( item.name.clone(), gen::ElBox(
				gen::SizeRel( 1.0 ), gen::SizeAbs( 30 ),
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
				_	=> fail!("Unexpected tail of debug menu: {:s}", item.name),
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
	
	pub fn build( &self, alpha : f32 )-> gen::Screen	{
		let offset = [100u,100u];
		gen::Screen	{
			alpha	: alpha,
			root	: gen::Box{
				align	: gen::AlignHor,
				ground	: gen::GroundNone,
				children: ~[
					gen::Child( ~"tab_hor",	gen::ElSpace( [offset[0],0] )),
					gen::Child( ~"sub-tab",	gen::ElBox(
						gen::SizeRel( 0.8 ), gen::SizeRel( 1.0 ),
						gen::Box{
							align	: gen::AlignVer,
							ground	: gen::GroundNone,
							children: ~[
								gen::Child( ~"tab_ver",	gen::ElSpace( [0,offset[1]] )),
								gen::Child( ~"menu",	gen::ElBox(
									gen::SizeRel( 0.8 ), gen::SizeRel( 0.8 ),
									gen::Box{
										align	: gen::AlignHor,
										ground	: gen::GroundNone,
										children: self.build_horisontal(),
									}
								)),	//menu
							],
						}
					)),	//sub-tab
				],
			},
		}
	}
}
