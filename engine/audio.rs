extern mod openal;

use std;
use std::ptr;

use openal::*;

use journal;
use load;

//- - - - - -
// TYPES	//

pub struct BufferHandle( al::types::ALuint );
pub struct SourceHandle( al::types::ALuint );

pub struct Context	{
	device	: alc::Device,
	context	: alc::Context,
}

impl Drop for Context	{
	fn drop( &mut self )	{
		self.device.close();
	}
}


pub struct Buffer	{
	handle		: BufferHandle,
	duration	: float,
}

impl Drop for BufferHandle	{
	#[fixed_stack_segment]
	fn drop( &mut self )	{
		unsafe{
			al::ffi::alDeleteBuffers( 1, ptr::to_unsafe_ptr(&**self) );
		}
	}
}


pub struct Source	{
	handle		: SourceHandle,
	priv buffer	: Option<@Buffer>,
}

impl Drop for SourceHandle	{
	#[fixed_stack_segment]
	fn drop( &mut self )	{
		unsafe{
			al::ffi::alDeleteSources( 1, ptr::to_unsafe_ptr(&**self) );
		}
	}
}

impl Source	{
	#[fixed_stack_segment]
	pub fn bind( &mut self, buf : @Buffer )	{
		self.buffer = Some(buf);
		unsafe{
			al::ffi::alSourcei( *self.handle, al::ffi::BUFFER, *buf.handle as al::types::ALint )
		}
	}

	#[fixed_stack_segment]
	pub fn unbind( &mut self )	{
		self.buffer = None;
		unsafe{
			al::ffi::alSourcei( *self.handle, al::ffi::BUFFER, 0 )
		}
	}

	#[fixed_stack_segment]
	pub fn play( &self )	{
		assert!( self.buffer.is_some() );
		unsafe{
			al::ffi::alSourcePlay( *self.handle );
		}
	}
}


pub struct Listener	{
	volume	: float,
}


//- - - - - - - - - -
// IMPLEMENTATIONS	//

pub fn find_format( channels : uint, bits : uint )-> al::types::ALenum	{
	match (channels,bits)	{
		(1,8)	=> al::ffi::FORMAT_MONO8,
		(1,16)	=> al::ffi::FORMAT_MONO16,
		(2,8)	=> al::ffi::FORMAT_STEREO8,
		(2,16)	=> al::ffi::FORMAT_STEREO16,
		_	=> fail!( "Unknown format: %u channels, %u bits", channels, bits )
	}
}

impl Context	{
	pub fn create( dev_name : &str )-> Context	{
		let dev = alc::Device::open( dev_name );
		let ctx = dev.create_context( &[] );
		ctx.make_current();
		Context	{
			device	: dev,
			context	: ctx,
		}
	}
	
	#[fixed_stack_segment]
	pub fn check( &self )	{
		let err = unsafe{ al::ffi::alGetError() };
		if err != al::ffi::NO_ERROR	{
			fail!( "AL error %d", err as int )
		}
	}
	
	#[fixed_stack_segment]
	pub fn check_extension( &self, name : &str )-> bool	{
		let mut yes = false;
		name.with_c_str( |text|	{
			yes = unsafe{al::ffi::alIsExtensionPresent(text)} != 0
		});
		yes
	}

	#[fixed_stack_segment]
	pub fn create_buffer<T>( &self, channels : uint, bits : uint, byte_rate : uint, 
		sample_rate : uint, data : ~[T] )-> Buffer	{
		let mut hid : al::types::ALuint = 0;
		let size = data.len() * std::sys::size_of::<T>();
		unsafe{
			al::ffi::alGenBuffers( 1, ptr::to_mut_unsafe_ptr(&mut hid) );
			al::ffi::alBufferData( hid, find_format(channels,bits),
				std::vec::raw::to_ptr(data) as *al::types::ALvoid,
				size as al::types::ALsizei, sample_rate as al::types::ALsizei );
		}		
		Buffer{
			handle	: BufferHandle(hid),
			duration: (size as float) / (byte_rate as float),
		}
	}

	#[fixed_stack_segment]
	pub fn create_source( &self )-> Source	{
		let mut hid : al::types::ALuint = 0;
		unsafe{
			al::ffi::alGenSources( 1, ptr::to_mut_unsafe_ptr(&mut hid) );
		}
		Source{
			handle	: SourceHandle(hid),
			buffer	: None,
		}
	}
}


pub fn read_wave_chunk( rd : &load::Reader )-> load::Chunk	{
	let name = std::str::from_utf8( rd.get_bytes(4) );
	let size = rd.get_uint(4);
	//lg.add( ~"\tEntering " + name );
	load::Chunk{
		name	: name,
		size	: size,
		finish	: rd.position()+size,
	}
}

pub fn load_wav( at : &Context, path : &str, lg : &journal::Log )-> Buffer	{
	struct Chunk	{
		id		: ~str,
		start	: uint,
		size	: uint,
	};
	lg.add( "Loading " + path );
	let mut rd = load::Reader::create_ext( path, read_wave_chunk );
	assert!( rd.enter() == ~"RIFF" );
	let s_format = std::str::from_utf8( rd.get_bytes(4) );
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
