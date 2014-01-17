use std::path::Path;
use std::{cell,io,rc};


pub struct Log	{
	name		: ~str,
	enable		: bool,
	priv wr		: rc::Rc<cell::RefCell<~io::Writer>>,	//TODO: better sharing
}

pub trait LoggedUnused	{
	fn log( &self, lg : &Log );
}


impl Log	{
	pub fn create( path: &str )-> Log	{
		match io::File::create( &Path::new(path) )	{
			Some(wr)	=> Log{
				name: ~"", enable: true,
				wr: rc::Rc::new(cell::RefCell::new( ~wr as ~io::Writer ))
			},
			None		=> fail!("Unable to create log: {:s}", path),
		}
	}

	pub fn fork( &self, name: ~str )-> Log	{
		Log{ name: name, enable: self.enable, wr: self.wr.clone() }
	}

	pub fn add( &self, message: &str )	{
		if self.enable	{
			let msg = format!( "{:s}\n", message );
			let mut w = self.wr.borrow().borrow_mut();
			w.get().write( msg.as_bytes() );
		}
	}

	pub fn describe( &self, obj: &LoggedUnused )	{
		if self.enable	{
			obj.log( self );
		}
	}
}
