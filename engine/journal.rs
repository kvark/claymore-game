use std::path::Path;
use std::io;


pub struct Log	{
	name		: ~str,
	enable		: bool,
	priv wr		: @mut io::Writer,	//FIXME: no managed
}

pub trait LoggedUnused	{
	fn log( &self, lg : &Log );
}


impl Log	{
	pub fn create( path : &str )-> Log	{
		match io::File::create( &Path::new(path) )	{
			Some(wr)	=> Log{ name:~"", enable:true, wr:@mut wr as @mut io::Writer },
			None		=> fail!("Unable to create log: {:s}", path),
		}
	}

	pub fn fork( &self, name : ~str )-> Log	{
		Log{ name:name, enable:self.enable, wr:self.wr }
	}

	pub fn add( &self, message : &str )	{
		if self.enable	{
			let msg = format!( "{:s}\n", message );
			self.wr.write( msg.as_bytes() );
		}
	}

	pub fn describe( &self, obj : &LoggedUnused )	{
		if self.enable	{
			obj.log( self );
		}
	}
}
