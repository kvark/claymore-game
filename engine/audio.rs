extern mod openal;
use openal::*;

//- - - - - -
// TYPES	//

pub enum Handle = al::ALuint;

pub struct Context	{
	device	: *alc::ALCdevice,
	context	: *alc::ALCcontext,
	priv pool_buffers	: @mut ~[Handle],
	priv pool_sources	: @mut ~[Handle],

	drop	{
		alc::alcMakeContextCurrent( ptr::null() );
		alc::alcDestroyContext( self.context );
		alc::alcCloseDevice( self.device );
	}
}

pub struct Buffer	{
	handle		: Handle,
	duration	: float,
	priv pool	: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
	}
}

pub struct Source	{
	handle			: Handle,
	priv mut buffer	: Option<@Buffer>,
	priv pool		: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
	}
}

impl Source	{
	pub fn bind( buf : @Buffer )	{
		self.buffer = Some(buf);
		al::alSourcei( *self.handle, al::AL_BUFFER, *buf.handle as al::ALint )
	}
	pub fn unbind()	{
		self.buffer = None;
		al::alSourcei( *self.handle, al::AL_BUFFER, 0 )
	}
	pub fn play()	{
		assert self.buffer.is_some();
		al::alSourcePlay( *self.handle );
	}
}

pub struct Listener	{
	volume	: float,
}


//- - - - - - - - - -
// IMPLEMENTATIONS	//

pub fn create_context()-> Context	{
	let dev = alc::alcOpenDevice( ptr::null() );
	let ctx = alc::alcCreateContext( dev, ptr::null() );
	alc::alcMakeContextCurrent( ctx );
	Context{
		device:dev, context:ctx,
		pool_buffers	:@mut ~[],
		pool_sources	:@mut ~[],
	}
}

pub pure fn find_format( channels : uint, bits : uint )-> al::ALenum	{
	match (channels,bits)	{
		(1,8)	=> al::AL_FORMAT_MONO8,
		(1,16)	=> al::AL_FORMAT_MONO16,
		(2,8)	=> al::AL_FORMAT_STEREO8,
		(2,16)	=> al::AL_FORMAT_STEREO16,
		_	=> fail fmt!( "Unknown format: %u channels, %u bits", channels, bits )
	}
}

impl Context	{
	pub fn check()	{
		let err = al::alGetError();
		if err != al::AL_NO_ERROR	{
			fail fmt!("AL error %d", err as int)
		}
	}
	pub fn check_extension( name : &str )-> bool	{
		let mut yes = false;
		do str::as_c_str(name) |text|	{
			yes = al::alIsExtensionPresent(text) != 0
		}
		yes
	}
	pub fn cleanup()	{
		//empty
	}

	pub fn create_buffer<T>( channels : uint, bits : uint, byte_rate : uint, 
		sample_rate : uint, data : ~[T] )-> Buffer	{
		let mut hid : al::ALuint = 0;
		al::alGenBuffers( 1, ptr::addr_of(&hid) );
		let size = data.len() * sys::size_of::<T>();
		al::alBufferData( hid, find_format(channels,bits),
			unsafe{vec::raw::to_ptr(data) as *al::ALvoid},
			size as al::ALsizei, sample_rate as al::ALsizei );
		Buffer{
			handle	: Handle(hid),
			duration: (size as float) / (byte_rate as float),
			pool	: self.pool_buffers,
		}
	}

	pub fn create_source()-> Source	{
		let mut hid : al::ALuint = 0;
		al::alGenSources( 1, ptr::addr_of(&hid) );
		Source{
			handle	: Handle(hid),
			buffer	: None,
			pool	: self.pool_sources,
		}
	}
}


pub fn load_wav( at : &Context, path : ~str )-> Buffer	{
	struct Chunk	{
		id		: ~str,
		start	: uint,
		size	: uint,
	};
	fn open_chunk( rd : &load::Reader )-> Chunk	{
		let pos = rd.bin.tell();
		let name = str::from_bytes( rd.get_bytes(4) );
		io::println( ~"\tEntering " + name );
		Chunk{
			id		: name,
			start	: pos,
			size	: rd.get_uint(4),
		}
	}
	fn close_chunk( c : &Chunk, rd : &load::Reader )	{
		io::println( ~"\tLeaving " + c.id );
		//io::println(fmt!( "\tChunk start:%u len:%u cur:%u", c.start, c.size, rd.bin.tell() ));
		assert rd.bin.tell() == c.start + 8u + c.size;
	}
	io::println( ~"Loading " + path );
	let rd = load::create_reader(path);
	let c_riff = open_chunk(&rd);
	assert c_riff.id == ~"RIFF";
	let s_format = str::from_bytes( rd.get_bytes(4) );
	assert s_format == ~"WAVE";
	let c_fmt = open_chunk(&rd);
	assert c_fmt.id == ~"fmt ";
	let audio_format	= rd.get_uint(2);
	let channels		= rd.get_uint(2);
	let sample_rate		= rd.get_uint(4);
	let byte_rate		= rd.get_uint(4);
	let _byte_align		= rd.get_uint(2);
	let bits_per_sample	= rd.get_uint(2);
	io::println(fmt!( "\tformat:%u, channels:%u, sample_rate:%u, byte_rate:%u",
		audio_format, channels, sample_rate, byte_rate ));
	let is_PCM = audio_format == 1u;
	if !is_PCM	{
		let extra = rd.get_uint(2);
		rd.get_bytes(extra);
	}
	close_chunk( &c_fmt, &rd );
	if !is_PCM	{
		let c_fact = open_chunk(&rd);
		assert c_fact.id == ~"fact";
		let _sample_len = rd.get_uint(4);
		close_chunk(&c_fact,&rd);
	}
	let mut c_data = open_chunk(&rd);
	while c_data.id != ~"data"	{
		assert rd.bin.tell() < c_riff.start + 8 + c_riff.size;
		rd.get_bytes( c_data.size );
		close_chunk( &c_data, &rd );
		c_data = open_chunk(&rd);
	}
	let data = rd.get_bytes( c_data.size );
	close_chunk( &c_data, &rd );
	if c_data.size&1u != 0	{
		rd.get_bytes(1);
	}
	close_chunk( &c_riff, &rd );
	at.create_buffer( channels, bits_per_sample, byte_rate, sample_rate, data )
}