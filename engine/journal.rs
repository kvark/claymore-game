pub struct Log	{
	depth		: uint,
	priv wr		: @io::Writer,
}

pub impl Log	{
	fn create( path : ~str, depth : uint )->Log	{
		match io::file_writer( &path::Path(path), &[io::Create,io::Truncate] )	{
			Ok(wr)	=> Log{ depth:depth, wr:wr },
			Err(e)	=> fail!( e.to_str() ),
		}
	}
	fn add( &self, message : ~str )	{
		let d = str::find(message,char::is_alphanumeric).expect(~"Bad log record");
		if d<self.depth	{
			self.wr.write_line(message)
		}
	}
}
