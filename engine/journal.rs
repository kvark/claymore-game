use std::{io,path};

pub struct Log	{
	name		: ~str,
	enable		: bool,
	priv wr		: @io::Writer,
}

pub trait LoggedUnused	{
	fn log( &self, lg : &Log );
}


impl Log	{
	pub fn create( path : ~str )-> Log	{
		match io::file_writer( &path::Path(path), &[io::Create,io::Truncate] )	{
			Ok(wr)	=> Log{ name:~"", enable:true, wr:wr },
			Err(e)	=> fail!( e.to_str() ),
		}
	}

	pub fn fork( &self, name : ~str )-> Log	{
		Log{ name:name, enable:self.enable, wr:self.wr }
	}

	pub fn add( &self, message : ~str )	{
		if self.enable	{
			self.wr.write_line(message);
		}
	}

	pub fn describe( &self, obj : &LoggedUnused )	{
		if self.enable	{
			obj.log( self );
		}
	}
}
