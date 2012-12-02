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
	priv pool	: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
	}
}

pub struct Source	{
	handle		: Handle,
	priv pool	: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
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

	pub fn create_buffer()-> Buffer	{
		let mut hid : al::ALuint = 0;
		al::alGenBuffers( 1, ptr::addr_of(&hid) );
		Buffer{
			handle	: Handle(hid),
			pool	: self.pool_buffers,
		}
	}
}
