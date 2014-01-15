extern mod openal;

use std;
use std::{rc,ptr};

use openal::{al,alc};

use journal;
use load;

//- - - - - -
// TYPES	//
pub type float = f32;
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


pub type BufferPtr = rc::Rc<Buffer>;

pub struct Buffer	{
	handle		: BufferHandle,
	duration	: float,
}

impl Drop for BufferHandle	{
	fn drop( &mut self )	{
		let &BufferHandle(ref h) = self;
		unsafe{
			al::ffi::alDeleteBuffers( 1, ptr::to_unsafe_ptr(h) );
		}
	}
}


pub struct Source	{
	handle		: SourceHandle,
	priv buffer	: Option<BufferPtr>,
}

impl Drop for SourceHandle	{
	fn drop( &mut self )	{
		let &SourceHandle(ref h) = self;
		unsafe{
			al::ffi::alDeleteSources( 1, ptr::to_unsafe_ptr(h) );
		}
	}
}

impl Source	{
	pub fn bind( &mut self, buf: &BufferPtr )	{
		self.buffer = Some(buf.clone());
		let SourceHandle(sh) = self.handle;
		let BufferHandle(bh) = buf.borrow().handle;
		unsafe{
			al::ffi::alSourcei( sh, al::ffi::BUFFER, bh as al::types::ALint )
		}
	}

	pub fn unbind( &mut self )	{
		self.buffer = None;
		let SourceHandle(sh) = self.handle;
		unsafe{
			al::ffi::alSourcei( sh, al::ffi::BUFFER, 0 )
		}
	}

	pub fn play( &self )	{
		assert!( self.buffer.is_some() );
		let SourceHandle(sh) = self.handle;
		unsafe{
			al::ffi::alSourcePlay( sh );
		}
	}
}


pub struct Listener	{
	volume	: f32,
}


//- - - - - - - - - -
// IMPLEMENTATIONS	//

pub fn find_format( channels: uint, bits: uint )-> al::types::ALenum	{
	match (channels,bits)	{
		(1,8)	=> al::ffi::FORMAT_MONO8,
		(1,16)	=> al::ffi::FORMAT_MONO16,
		(2,8)	=> al::ffi::FORMAT_STEREO8,
		(2,16)	=> al::ffi::FORMAT_STEREO16,
		_	=> fail!( "Unknown format: {:u} channels, {:u} bits", channels, bits )
	}
}

impl Context	{
	pub fn create( dev_name: &str )-> Context	{
		let dev = alc::Device::open( dev_name ).
			expect(format!( "Audio device {:s} is not found", dev_name ));
		let ctx = dev.create_context( &[] ).
			expect("Unable to create audio context");
		ctx.make_current();
		Context	{
			device	: dev,
			context	: ctx,
		}
	}
	
	pub fn check( &self )	{
		let err = unsafe{ al::ffi::alGetError() };
		if err != al::ffi::NO_ERROR	{
			fail!( "AL error {:i}", err as int )
		}
	}
	
	pub fn check_extension( &self, name: &str )-> bool	{
		let mut yes = false;
		name.with_c_str( |text|	{
			yes = unsafe{al::ffi::alIsExtensionPresent(text)} != 0
		});
		yes
	}

	pub fn create_buffer<T>( &self, channels: uint, bits: uint, byte_rate: uint, 
		sample_rate : uint, data : ~[T] )-> BufferPtr	{
		let mut hid : al::types::ALuint = 0;
		let size = data.len() * std::mem::size_of::<T>();
		unsafe{
			al::ffi::alGenBuffers( 1, ptr::to_mut_unsafe_ptr(&mut hid) );
			al::ffi::alBufferData( hid, find_format(channels,bits),
				data.as_ptr() as *al::types::ALvoid,
				size as al::types::ALsizei, sample_rate as al::types::ALsizei );
		}		
		rc::Rc::new(Buffer{
			handle	: BufferHandle(hid),
			duration: (size as float) / (byte_rate as float),
		})
	}

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


pub fn read_wave_chunk( rd: &mut load::Reader )-> load::Chunk	{
	let name = std::str::from_utf8_owned( rd.get_bytes(4) );
	let size = rd.get_uint(4);
	//lg.add( ~"\tEntering " + name );
	load::Chunk{
		name	: name,
		size	: size,
		finish	: rd.position() + size,
	}
}

pub fn load_wav( at: &Context, path: &str, lg: &journal::Log )-> BufferPtr	{
	lg.add( "Loading " + path );
	let mut rd = load::Reader::create_ext( path, read_wave_chunk );
	assert!( rd.enter() == ~"RIFF" );
	let s_format = std::str::from_utf8_owned( rd.get_bytes(4) );
	assert!( s_format == ~"WAVE" );
	assert!( rd.enter() == ~"fmt " );
	let audio_format	= rd.get_uint(2);
	let channels		= rd.get_uint(2);
	let sample_rate		= rd.get_uint(4);
	let byte_rate		= rd.get_uint(4);
	let _byte_align		= rd.get_uint(2);
	let bits_per_sample	= rd.get_uint(2);
	lg.add(format!( "\tformat:{:u}, channels:{:u}, sample_rate:{:u}, byte_rate:{:u}",
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

