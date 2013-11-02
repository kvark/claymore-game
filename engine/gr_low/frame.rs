extern mod gl;

use std;
use std::{managed,ptr};
use std::to_str::ToStr;

use gr_low::{context,texture};


pub struct SurfaceHandle( gl::types::GLuint );

pub struct Surface	{
	handle	: SurfaceHandle,
	target	: gl::types::GLenum,
	width	: uint,
	height	: uint,
	samples	: uint,
}

impl Drop for SurfaceHandle	{
	fn drop( &mut self )	{
		unsafe{ gl::DeleteRenderbuffers( 1, ptr::to_unsafe_ptr(&**self) ); }
	}
}

impl context::ProxyState for Surface	{
	fn sync_back( &mut self )-> bool	{
		//FIXME
		true
	}
}

impl ToStr for Surface	{
	fn to_str( &self )-> ~str	{
		format!( "Surface(h={:i}, {:u}x{:u}, samples={:u})", *self.handle as int,
			self.width, self.height, self.samples )
	}
}

#[deriving(Clone)]
pub enum Target	{
	TarEmpty,
	TarSurface(@Surface),
	TarTexture(@texture::Texture,uint),
	TarTextureLayer(@texture::Texture,uint,uint),
}

impl std::cmp::Eq for Target	{
	fn eq( &self, other : &Target )-> bool	{
		match (self,other)	{
			(&TarEmpty,&TarEmpty)					=> true,
			(&TarSurface(s1),&TarSurface(s2))		=> *s1.handle == *s2.handle,
			(&TarTexture(t1,l1),&TarTexture(t2,l2))	=> *t1.handle == *t2.handle && l1==l2,
			(&TarTextureLayer(t1,r1,l1),&TarTextureLayer(t2,r2,l2))	=>
				*t1.handle == *t2.handle && r1==r2 && l1==l2,
			(_,_) => false
		}
	}
}

impl ToStr for Target	{
	fn to_str( &self )-> ~str	{
		match self	{
			&TarEmpty	=> ~"Empty",
			&TarSurface(s)	=> s.to_str(),
			&TarTexture(t,l)	=> format!( "{:s}.lod[{:u}]", t.to_str(), l ),
			&TarTextureLayer(t,r,l)	=> format!("{:s}.layer[{:u}].lod[{:u}]", t.to_str(), r, l ),
		}
	}
}


impl Target	{
	pub fn get_size( &self )-> [uint,..4]	{
		match self	{
			&TarEmpty	=> fail!(~"Empty target has no size"),
			&TarSurface(sf) => [sf.width, sf.height, 1, sf.samples],
			&TarTexture(tex,lev) =>	{
				let (w,h) = tex.get_level_size(lev);
				[w, h, tex.depth, tex.samples]
			},
			&TarTextureLayer(tex,_,lev) =>	{
				let (w,h) = tex.get_level_size(lev);
				[w, h, 1, tex.samples]
			}
		}
	}
	
	fn attach( &self, root : gl::types::GLenum, slot : gl::types::GLenum )-> bool	{
		match self	{
			&TarEmpty			=> {},
			&TarSurface(s)		=> {
				gl::FramebufferRenderbuffer( root, slot, s.target, *s.handle );
			},
			&TarTexture(tex,lev)	=> {
				assert!( tex.get_num_levels() > lev );
				//gl::FramebufferTexture( root, slot, *tex.handle, lev as gl::types::GLint );
				// workaround for Linux:
				assert!( tex.depth == 0 );
				gl::FramebufferTexture2D( root, slot, *tex.target, *tex.handle, lev as gl::types::GLint );
			},
			&TarTextureLayer(tex,layer,lev) => {
				assert!( tex.depth > layer && tex.get_num_levels() > lev );
				gl::FramebufferTextureLayer( root, slot, *tex.handle, layer as gl::types::GLint, lev as gl::types::GLint );
			},
		}
		true
	}
}


pub struct RenBinding	{
	target		: gl::types::GLenum,
	default		: @Surface,
	priv active	: @Surface,
}

impl RenBinding	{
	pub fn new( wid : uint, het : uint, ns : uint )-> RenBinding	{
		let t = gl::RENDERBUFFER;
		let s = @Surface{ handle:SurfaceHandle(0),
			target:t, width:wid, height:het, samples:ns
		};
		RenBinding{
			target: t, default: s, active: s,
		}
	}
}

impl context::ProxyState for RenBinding	{
	fn sync_back( &mut self )-> bool	{
		let mut hid = 0 as gl::types::GLint;
		unsafe{ gl::GetIntegerv( gl::RENDERBUFFER_BINDING, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		let hu = hid as gl::types::GLuint;
		if *self.active.handle != hu	{
			assert!( false, "Active render buffer mismatch" );
			false
		}else {true}
	}
}


#[deriving(Clone,Eq)]
pub struct Rect	{
	x : uint,
	y : uint,
	w : uint,
	h : uint,
}

impl Rect	{
	pub fn new( wid : uint, het : uint )-> Rect	{
		Rect{ x:0u, y:0u, w:wid, h:het }
	}
	pub fn contains( &self, x : uint, y : uint )-> bool	{
		x>=self.x && x<self.x+self.w && y>=self.y && y<self.y+self.h
	}
	pub fn contains_rect( &self, r : &Rect )-> bool	{
		r.x>=self.x && r.x+r.w<=self.x+self.w &&
		r.y>=self.y && r.y+r.h<=self.y+self.w
	}
	pub fn aspect( &self )-> f32	{
		if self.h==0 {0f32}
		else {(self.w as f32) / (self.h as f32)}
	}
	pub fn is_empty( &self )-> bool	{
		self.w==0 && self.h==0
	}
}

impl ToStr for Rect	{
	fn to_str( &self )-> ~str	{
		format!( "[{:u}.{:u} : {:u}.{:u}]", self.x, self.y, self.w, self.h )
	}
}


pub struct BufferHandle( gl::types::GLuint );

pub struct Buffer	{
	handle			: BufferHandle,
	priv draw_mask	: uint,
	priv read_id	: Option<uint>,
	stencil			: Target,
	depth			: Target,
	colors			: ~[Target],
}

impl Drop for BufferHandle	{
	fn drop( &mut self )	{
		unsafe{ gl::DeleteFramebuffers( 1, ptr::to_unsafe_ptr(&**self) ); }
	}
}

impl Buffer	{
	pub fn each_target( &self, fun : &fn(&Target) )	{
		let ds = &[self.stencil,self.depth];
		for &target in ds.iter().chain( self.colors.iter() )	{
			if target != TarEmpty	{
				fun(&target);
			}
		}
	}

	pub fn new_default( rb : @Surface )-> @mut Buffer	{
		let ts = TarSurface(rb);
		@mut Buffer{
			handle		:BufferHandle(0),
			draw_mask	:0x10u,	// invalid one
			read_id		:None,	// actually, GL_BACK
			stencil		:ts,
			depth		:ts,
			colors		:~[ts],
		}
	}
	
	pub fn check_size( &self )-> [uint,..4]	{
		let mut actual = [0u,0u,0u,0u];
		
		do self.each_target() |&target|	{
			let current = target.get_size();
			if current[0]==0u	{
				actual = current;
			}else	{
				assert_eq!( current, actual );
			}
		}
		actual
	}
}

impl context::ProxyState for Buffer	{
	fn sync_back( &mut self )-> bool	{
		//FIXME
		true
	}
}



pub struct Binding	{
	target		: gl::types::GLenum,
	priv active	: @mut Buffer,
}

impl context::ProxyState for Binding	{
	fn sync_back( &mut self )-> bool	{
		let mut hid = 0 as gl::types::GLint;
		unsafe{ gl::GetIntegerv( gl::FRAMEBUFFER_BINDING, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		let hu = hid as gl::types::GLuint;
		if *self.active.handle != hu	{
			assert!( false, "Active frame buffer mismatch" );
			false
		}else {true}	
	}
}

impl Binding	{
	pub fn new( target:gl::types::GLenum, active:@mut Buffer )-> Binding	{
		Binding{ target:target, active:active }
	}

	fn bind( &mut self, b : @mut Buffer )	{
		if !managed::mut_ptr_eq(self.active,b)	{
			self.active = b;
			gl::BindFramebuffer( self.target, *b.handle );
		}
	}

	fn attach_target( &self, new : Target, old : &mut Target, slot : gl::types::GLenum )-> bool	{
		if *old != new	{
			*old = new;
			new.attach( self.target, slot )
		}else	{true}
	}

	fn check( &self )	{
		let code = gl::CheckFramebufferStatus( self.target );
		if code == gl::FRAMEBUFFER_COMPLETE	{return};
		let message =
			if code == gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT			{~"format"}		else
			//if code == gl::FRAMEBUFFER_INCOMPLETE_DIMENSIONS		{~"dimensions"}	else
			if code == gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT	{~"missing"}	else
			if code == gl::FRAMEBUFFER_UNSUPPORTED					{~"hardware"}	else
			{~"unknown"};
		fail!("FBO {:i} is incomplete: {:s}", *self.active.handle as int, message)
	}
}


impl context::Context	{
	pub fn create_render_buffer( &self, wid:uint, het:uint, sam:uint )-> @Surface	{
		let mut hid = 0 as gl::types::GLuint;
		unsafe{ gl::GenRenderbuffers( 1, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		@Surface{ handle:SurfaceHandle(hid),
			target:self.render_buffer.target,
			width:wid, height:het, samples:sam,
		}
	}

	fn bind_render_buffer( &mut self, s : @Surface )	{
		let binding = &mut self.render_buffer;
		assert!( s.target == binding.target );
		if !managed::ptr_eq(binding.active,s)	{
			binding.active = s;
			gl::BindRenderbuffer( binding.target, *s.handle );
		}
	}
	pub fn unbind_render_buffers( &mut self )	{
		self.bind_render_buffer( self.render_buffer.default );
	}

	pub fn create_frame_buffer( &self )-> @mut Buffer	{
		let mut hid = 0 as gl::types::GLuint;
		unsafe{ gl::GenFramebuffers( 1, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		@mut Buffer{ handle:BufferHandle(hid),
			draw_mask:1u, read_id:Some(0u),
			stencil:TarEmpty, depth:TarEmpty,
			colors	: std::vec::from_elem( self.caps.max_color_attachments, TarEmpty ),
		}
	}

	pub fn bind_frame_buffer( &mut self, fb : @mut Buffer, draw : bool,
			stencil : Target, depth : Target, colors : ~[Target] )	{
		let binding = if draw {&mut self.frame_buffer_draw} else {&mut self.frame_buffer_read};
		binding.bind( fb );
		// work around main framebuffer
		if managed::mut_ptr_eq(fb,self.default_frame_buffer)	{
			let use_color = colors.len()!=0u;
			let value = if use_color {gl::BACK} else {gl::NONE} ;
			//FIXME: cache this
			if draw	{
				let mask = if use_color{1} else {0};
				if fb.draw_mask != mask	{
					fb.draw_mask = mask;
					gl::DrawBuffer( value );
				}
			}else	{
				let id = if use_color{Some(0u)} else {None};
				if fb.read_id != id	{
					fb.read_id = id;
					gl::ReadBuffer( value );
				}
			}
			return;
		}
		fn get_color( index : uint )->gl::types::GLenum	{
			gl::COLOR_ATTACHMENT0 + (index as gl::types::GLenum)
		};
		// attach planes
		gl::GetError();	//debug
		binding.attach_target( stencil,	&mut fb.stencil,	gl::STENCIL_ATTACHMENT );
		binding.attach_target( depth,	&mut fb.depth,		gl::DEPTH_ATTACHMENT );
		for (i,target) in colors.iter().enumerate()	{
			let mut val = fb.colors[i];	//FIXME
			binding.attach_target( *target, &mut val, get_color(i) );
			fb.colors[i] = val;
		}
		gl::GetError();	//debug
		let mask = (1u<<colors.len()) - 1u;
		if draw && fb.draw_mask != mask	{
			// set the draw mask
			fb.draw_mask = mask;
			let mut list :~[gl::types::GLenum] = ~[];
			let mut i = 0u;
			while mask>>i != 0u	{
				if mask&(1<<i) != 0u	{
					list.push( get_color(i) );
				}
				i += 1;
			}
			match list.len()	{
				0u	=> gl::DrawBuffer( gl::NONE ),
				1u	=> gl::DrawBuffer( list[0] ),
				_	=> unsafe{ gl::DrawBuffers(
					list.len() as gl::types::GLsizei, std::vec::raw::to_ptr(list)
					)},
			}
		}else if !draw	{
			// set the read mask
			assert!( mask&(mask-1u) == 0 );
			let new_id = if mask != 0u	{
					let mut i=0u;
					while mask>>i!=1u	{i+=1u;}
					Some(i)
				}else	{ None };
			if fb.read_id != new_id	{
				fb.read_id = new_id;
				match new_id	{
					Some(id)	=> gl::ReadBuffer( get_color(id) ),
					None		=> gl::ReadBuffer( gl::NONE ),
				}
			}
		}
		// check completeness
		gl::GetError();
		binding.check();	//FIXME: debug only
	}

	pub fn unbind_frame_buffers( &mut self )	{
		self.frame_buffer_draw.bind( self.default_frame_buffer );
		self.frame_buffer_read.bind( self.default_frame_buffer );
		gl::DrawBuffer( gl::BACK );
		gl::ReadBuffer( gl::NONE );
	}
}
