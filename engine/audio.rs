extern mod openal;

use openal::*;

use context;
use load;

//- - - - - -
// TYPES	//

pub struct Handle( types::ALuint );

pub struct Context	{
	device	: *types::ALCdevice,
	context	: *types::ALCcontext,
	priv pool_buffers	: @mut ~[Handle],
	priv pool_sources	: @mut ~[Handle],
}

#[unsafe_destructor]
impl Drop for Context	{
	fn finalize( &self )	{
		ll::alcMakeContextCurrent( ptr::null() );
		ll::alcDestroyContext( self.context );
		ll::alcCloseDevice( self.device );
	}
}


pub struct Buffer	{
	handle		: Handle,
	duration	: float,
	priv pool	: @mut ~[Handle],
}

#[unsafe_destructor]
impl Drop for Buffer	{
	fn finalize( &self )	{
		self.pool.push( self.handle );
	}
}


pub struct Source	{
	handle		: Handle,
	priv buffer	: Option<@Buffer>,
	priv pool	: @mut ~[Handle],
}

#[unsafe_destructor]
impl Drop for Source	{
	fn finalize( &self )	{
		self.pool.push( self.handle );
	}
}

pub impl Source	{
	fn bind( &mut self, buf : @Buffer )	{
		self.buffer = Some(buf);
		ll::alSourcei( *self.handle, consts::al::BUFFER, *buf.handle as types::ALint )
	}
	fn unbind( &mut self )	{
		self.buffer = None;
		ll::alSourcei( *self.handle, consts::al::BUFFER, 0 )
	}
	fn play( &self )	{
		assert!( self.buffer.is_some() );
		ll::alSourcePlay( *self.handle );
	}
}


pub struct Listener	{
	volume	: float,
}


//- - - - - - - - - -
// IMPLEMENTATIONS	//

pub fn find_format( channels : uint, bits : uint )-> types::ALenum	{
	match (channels,bits)	{
		(1,8)	=> consts::al::FORMAT_MONO8,
		(1,16)	=> consts::al::FORMAT_MONO16,
		(2,8)	=> consts::al::FORMAT_STEREO8,
		(2,16)	=> consts::al::FORMAT_STEREO16,
		_	=> fail!(fmt!( "Unknown format: %u channels, %u bits", channels, bits ))
	}
}

pub impl Context	{
	fn create()-> Context	{
		let dev = ll::alcOpenDevice( ptr::null() );
		let ctx = ll::alcCreateContext( dev, ptr::null() );
		ll::alcMakeContextCurrent( ctx );
		Context{
			device:dev, context:ctx,
			pool_buffers	:@mut ~[],
			pool_sources	:@mut ~[],
		}
	}
	
	fn check( &self )	{
		let err = ll::alGetError();
		if err != consts::al::NO_ERROR	{
			fail!(fmt!("AL error %d", err as int))
		}
	}
	
	fn check_extension( &self, name : &str )-> bool	{
		let mut yes = false;
		do str::as_c_str(name) |text|	{
			yes = ll::alIsExtensionPresent(text) != 0
		}
		yes
	}

	fn cleanup( &self )	{
		//empty
	}

	fn create_buffer<T>( &self, channels : uint, bits : uint, byte_rate : uint, 
		sample_rate : uint, data : ~[T] )-> Buffer	{
		let mut hid : types::ALuint = 0;
		ll::alGenBuffers( 1, ptr::addr_of(&hid) );
		let size = data.len() * sys::size_of::<T>();
		ll::alBufferData( hid, find_format(channels,bits),
			unsafe{vec::raw::to_ptr(data) as *types::ALvoid},
			size as types::ALsizei, sample_rate as types::ALsizei );
		Buffer{
			handle	: Handle(hid),
			duration: (size as float) / (byte_rate as float),
			pool	: self.pool_buffers,
		}
	}

	fn create_source( &self )-> Source	{
		let mut hid : types::ALuint = 0;
		ll::alGenSources( 1, ptr::addr_of(&hid) );
		Source{
			handle	: Handle(hid),
			buffer	: None,
			pool	: self.pool_sources,
		}
	}
}


pub fn read_wave_chunk( rd : &load::Reader )-> load::Chunk	{
	let name = str::from_bytes( rd.get_bytes(4) );
	let size = rd.get_uint(4);
	//lg.add( ~"\tEntering " + name );
	load::Chunk{
		name	: name,
		size	: size,
		finish	: rd.position()+size,
	}
}

pub fn load_wav( at : &Context, path : ~str, lg : &context::Log )-> Buffer	{
	struct Chunk	{
		id		: ~str,
		start	: uint,
		size	: uint,
	};
	lg.add( ~"Loading " + path );
	let mut rd = load::Reader::create_ext( path, read_wave_chunk );
	assert!( rd.enter() == ~"RIFF" );
	let s_format = str::from_bytes( rd.get_bytes(4) );
	assert!( s_format == ~"WAVE" );
	assert!( rd.enter() == ~"fmt " );
	let audio_format	= rd.get_uint(2);
	let channels		= rd.get_uint(2);
	let sample_rate		= rd.get_uint(4);
	let byte_rate		= rd.get_uint(4);
	let _byte_align		= rd.get_uint(2);
	let bits_per_sample	= rd.get_uint(2);
	lg.add(fmt!( "\tformat:%u, channels:%u, sample_rate:%u, byte_rate:%u",
		audio_format, channels, sample_rate, byte_rate ));
	let is_PCM = audio_format == 1u;
	if !is_PCM	{
		let extra = rd.get_uint(2);
		rd.get_bytes(extra);
	}
	rd.leave();	//fmt
	while rd.enter() != ~"data"	{
		rd.skip();
		rd.leave();
	}
	let size = rd.has_more();
	let data = rd.get_bytes( size );
	rd.leave();	//data
	if size&1u != 0	{
		rd.get_bytes(1);
	}
	rd.leave();	//riff
	at.create_buffer( channels, bits_per_sample, byte_rate, sample_rate, data )
}