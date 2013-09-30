pub enum MenuItem	{
	MenuAction( ~fn() ),
	MenuSubmenu( ~[MenuItem] ),
}

pub struct Menu	{
	root		: MenuItem,
	selected	: ~[u8],
}

impl Menu	{
	pub fn is_active( &self )-> bool	{
		!self.selected.is_empty()
	}
}