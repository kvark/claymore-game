extern mod gen_hud;

use gen_hud::common::*;



priv fn get_among<T>( children : &[Child], path : &str, fun : &fn(&Child)->T )-> T	{
	let slash = str::find_char(path,'/');
	let name = match slash	{
		Some(p)	=> path.substr(0,p),
		None	=> path,
	};
	for children.each() |child|	{
		if str::eq_slice( child.name, name )	{
			return match slash	{
				Some(p)	=>	{
					let rest = path.substr( p+1, path.len()-p-1 );
					match &child.element	{
						&ElFrame(ref fr)	=> get_among( fr.children, rest, fun ),
						_	=> fail!(fmt!("Hud child is not a frame: %s", name))
					}
				},
				None	=> fun(child),
			}
		}
	}
	fail!(fmt!("Hud child not found: %s",name))
}

priv fn mod_among( children : &mut ~[Child], path : &str, fun : &fn(&mut Child) )	{
	let slash = str::find_char(path,'/');
	let name = match slash	{
		Some(p)	=> path.substr(0,p),
		None	=> path,
	};
	for children.each_mut() |child|	{
		if str::eq_slice( child.name, name )	{
			return match slash	{
				Some(p)	=>	{
					let rest = path.substr( p+1, path.len()-p-1 );
					child.element = match &child.element	{
						&ElFrame(ref fr)	=>	{
							let mut f2 = copy *fr;
							mod_among( &mut f2.children, rest, fun );
							ElFrame(f2)
						},
						_	=> fail!(fmt!("Hud child is not a frame: %s", name))
					}
				},
				None	=> fun(child),
			}
		}
	}
	fail!(fmt!("Hud child not found: %s",name))
}

pub fn get<T>( screen : &Screen, path : &str, fun : &fn(&Child)->T )-> T	{
	get_among( screen.children, path, fun )
}
