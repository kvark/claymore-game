extern mod glcore;
extern mod lmath;
use io::ReaderUtil;

const NAME_SIZE	:uint = 8;


struct Reader	{
	path	: ~str,
	bin		: @io::Reader,
	priv mut chunks	: ~[uint],
}


impl Reader	{
	fn get_uint( size:uint )-> uint	{
		let bytes = self.bin.read_bytes(size);
		do vec::foldr(bytes,0u) |t,u|	{(u<<8u) + (*t as uint)}
	}

	fn get_bool()-> bool	{
		self.get_uint(1u) != 0u
	}

	fn get_string()-> ~str	{
		let size = self.bin.read_byte() as uint;
		str::from_bytes( self.bin.read_bytes(size) )
	}

	fn enter()-> ~str	{
		let name_bin = self.bin.read_bytes( NAME_SIZE );
		let size = self.get_uint(4u);
		self.chunks.push( self.bin.tell() + size );
		let name_clean = do name_bin.filter()	|b| {*b != 0u8};
		str::from_bytes(name_clean)
	}

	fn leave()	{
		let end = self.chunks.pop();
		assert self.bin.tell() == end;
	}
}


pub fn create_reader( path : ~str )->Reader	{
	let p = path::Path( path );
	match io::file_reader(&p)	{
		Ok(bin)		=> Reader{ path:path, bin:bin, chunks:~[] },
		Err(msg)	=> fail(fmt!( "Unable to read %s: %s", path, msg ))
	}
}


pub fn read_mesh( br : &Reader, context : &context::Context )-> mesh::Mesh	{
	let signature = br.enter();
	if signature != ~"k3mesh"	{
		fail(fmt!( "Invalid mesh signature '%s': %s", signature, br.path ));
	}
	let n_vert	= br.get_uint(4u);
	let n_ind	= br.get_uint(4u);
	io::println(fmt!( "Loading mesh of %u vertices and %u indices: %s", n_vert, n_ind, br.path ));
	let mut mesh = context.create_mesh( br.get_string(), ~"3", n_vert, n_ind );
	let mut num_buffers = br.get_uint(1u);
	while num_buffers>0u	{
		let buffer = @context.create_buffer();
		let stride = br.get_uint(1u);
		let mut offset = 0u;
		let format = br.get_string();
		io::println(fmt!( "\tbuf: stride:%u, format:%s", stride, format ));
		let mut i = 0;
		while i < format.len()	{
			let name = ~"a_" + br.get_string();
			let mut fm = str::substr( format, i, 2 );
			if br.get_bool()	{ fm += ~"."; }
			if !br.get_bool()	{ fm += ~"!"; }
			io::println(fmt!( "\t\tname:%s, type:%s", name, fm ));
			let (at,size) = mesh.create_attrib( fm, buffer, stride, offset );
			if stride == 0u	{
				assert at.count == 1u;
				mesh.index = Some( at );
			}else	{
				mesh.attribs.insert( name, at );
			}
			offset += size;
			i += 2;
		}
		assert stride==0u || offset == stride;
		let size = if stride==0u {offset * n_ind} else {stride * n_vert};
		let data = br.bin.read_bytes( size );
		context.load_buffer( buffer, data, false );
		num_buffers -= 1u;
	}
	br.leave();
	mesh
}
