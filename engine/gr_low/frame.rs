extern mod glcore;

use core::managed;
use core::to_str::ToStr;

use gr_low::{context,texture};


pub struct SurfaceHandle( glcore::GLuint );

pub struct Surface	{
	handle	: SurfaceHandle,
	target	: glcore::GLenum,
	width	: uint,
	height	: uint,
	samples	: uint,
}

impl Drop for SurfaceHandle	{
	fn finalize( &self )	{
		glcore::glDeleteRenderbuffers( 1, ptr::addr_of(&**self) );
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
		fmt!( "Surface(h=%d, %ux%u, samples=%u)", *self.handle as int,
			self.width, self.height, self.samples )
	}
}


pub enum Target	{
	TarEmpty,
	TarSurface(@Surface),
	TarTexture(@texture::Texture,uint),
	TarTextureLayer(@texture::Texture,uint,uint),
}

impl Copy for Target	{}

impl cmp::Eq for Target	{
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
	fn ne( &self, other : &Target )-> bool	{
		!self.eq( other )
	}
}

impl ToStr for Target	{
	fn to_str( &self )-> ~str	{
		match self	{
			&TarEmpty	=> ~"Empty",
			&TarSurface(s)	=> s.to_str(),
			&TarTexture(t,l)	=> fmt!( "%s.lod[%u]", t.to_str(), l ),
			&TarTextureLayer(t,r,l)	=> fmt!("%s.layer[%u].lod[%u]", t.to_str(), r, l ),
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
	priv fn attach( &self, root : glcore::GLenum, slot : glcore::GLenum )-> bool	{
		match self	{
			&TarEmpty			=> {},
			&TarSurface(s)		=> {
				glcore::glFramebufferRenderbuffer( root, slot, s.target, *s.handle );
			},
			&TarTexture(tex,lev)	=> {
				assert!( tex.get_num_levels() > lev );
				//glcore::glFramebufferTexture( root, slot, *tex.handle, lev as glcore::GLint );
				// workaround for Linux:
				assert!( tex.depth == 0 );
				glcore::glFramebufferTexture2D( root, slot, *tex.target, *tex.handle, lev as glcore::GLint );
			},
			&TarTextureLayer(tex,layer,lev) => {
				assert!( tex.depth > layer && tex.get_num_levels() > lev );
				glcore::glFramebufferTextureLayer( root, slot, *tex.handle, layer as glcore::GLint, lev as glcore::GLint );
			},
		}
		true
	}
}


pub struct RenBinding	{
	target		: glcore::GLenum,
	default		: @Surface,
	priv active	: @Surface,
}

pub impl RenBinding	{
	fn new( wid : uint, het : uint, ns : uint )-> RenBinding	{
		let t = glcore::GL_RENDERBUFFER;
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
		let mut hid = 0 as glcore::GLint;
		glcore::glGetIntegerv( glcore::GL_RENDERBUFFER_BINDING, ptr::addr_of(&hid) );
		let hu = hid as glcore::GLuint;
		if *self.active.handle != hu	{
			assert!( false, "Active render buffer mismatch" );
			false
		}else {true}
	}
}


#[deriving(Eq)]
pub struct Rect	{
	x : uint,
	y : uint,
	w : uint,
	h : uint,
}

pub impl Rect	{
	fn new( wid : uint, het : uint )-> Rect	{
		Rect{ x:0u, y:0u, w:wid, h:het }
	}
	fn contains( &self, x : uint, y : uint )-> bool	{
		x>=self.x && x<self.x+self.w && y>=self.y && y<self.y+self.h
	}
	fn contains_rect( &self, r : &Rect )-> bool	{
		r.x>=self.x && r.x+r.w<=self.x+self.w &&
		r.y>=self.y && r.y+r.h<=self.y+self.w
	}
	fn aspect( &self )-> f32	{
		if self.h==0 {0f32}
		else {(self.w as f32) / (self.h as f32)}
	}
	fn is_empty( &self )-> bool	{
		self.w==0 && self.h==0
	}
}

pub struct BufferHandle( glcore::GLuint );

pub struct Buffer	{
	handle			: BufferHandle,
	priv draw_mask	: uint,
	priv read_id	: Option<uint>,
	stencil			: Target,
	depth			: Target,
	colors			: ~[Target],
}

impl Drop for BufferHandle	{
	fn finalize( &self )	{
		glcore::glDeleteFramebuffers( 1, ptr::addr_of(&**self) );
	}
}

pub impl Buffer	{
	fn new_default( rb : @Surface )-> @mut Buffer	{
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
	
	fn check_size( &self )-> [uint,..4]	{
		let mut actual = [0u,0u,0u,0u];
		for (~[self.stencil,self.depth] + self.colors).each |&target|	{
			if target == TarEmpty	{
				loop;
			}
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
	target		: glcore::GLenum,
	priv active	: @mut Buffer,
}

impl context::ProxyState for Binding	{
	fn sync_back( &mut self )-> bool	{
		let mut hid = 0 as glcore::GLint;
		glcore::glGetIntegerv( glcore::GL_FRAMEBUFFER_BINDING, ptr::addr_of(&hid) );
		let hu = hid as glcore::GLuint;
		if *self.active.handle != hu	{
			assert!( false, "Active frame buffer mismatch" );
			false
		}else {true}	
	}
}

impl Binding	{
	pub fn new( target:glcore::GLenum, active:@mut Buffer )-> Binding	{
		Binding{ target:target, active:active }
	}

	priv fn bind( &mut self, b : @mut Buffer )	{
		if !managed::mut_ptr_eq(self.active,b)	{
			self.active = b;
			glcore::glBindFramebuffer( self.target, *b.handle );
		}
	}

	priv fn attach_target( &self, new : Target, old : &mut Target, slot : glcore::GLenum )-> bool	{
		if *old != new	{
			*old = new;
			new.attach( self.target, slot )
		}else	{true}
	}

	priv fn check( &self )	{
		let code = glcore::glCheckFramebufferStatus( self.target );
		if code == glcore::GL_FRAMEBUFFER_COMPLETE	{return};
		let message =
			if code == glcore::GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT			{~"format"}		else
			//if code == glcore::GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS		{~"dimensions"}	else
			if code == glcore::GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT	{~"missing"}	else
			if code == glcore::GL_FRAMEBUFFER_UNSUPPORTED					{~"hardware"}	else
			{~"unknown"};
		fail!(fmt!( "FBO %d is incomplete: %s", *self.active.handle as int, message ))
	}
}


pub impl context::Context	{
	fn create_render_buffer( &self, wid:uint, het:uint, sam:uint )-> @Surface	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenRenderbuffers( 1, ptr::addr_of(&hid) );
		@Surface{ handle:SurfaceHandle(hid),
			target:self.render_buffer.target,
			width:wid, height:het, samples:sam,
		}
	}

	priv fn bind_render_buffer( &mut self, s : @Surface )	{
		let binding = &mut self.render_buffer;
		assert!( s.target == binding.target );
		if !managed::ptr_eq(binding.active,s)	{
			binding.active = s;
			glcore::glBindRenderbuffer( binding.target, *s.handle );
		}
	}
	fn unbind_render_buffers( &mut self )	{
		self.bind_render_buffer( self.render_buffer.default );
	}

	fn create_frame_buffer( &self )-> @mut Buffer	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenFramebuffers( 1, ptr::addr_of(&hid) );
		@mut Buffer{ handle:BufferHandle(hid),
			draw_mask:1u, read_id:Some(0u),
			stencil:TarEmpty, depth:TarEmpty,
			colors	: vec::from_elem( self.caps.max_color_attachments, TarEmpty ),
		}
	}

	fn bind_frame_buffer( &mut self, fb : @mut Buffer, draw : bool,
			stencil : Target, depth : Target, colors : ~[Target] )	{
		let binding = if draw {&mut self.frame_buffer_draw} else {&mut self.frame_buffer_read};
		binding.bind( fb );
		// work around main framebuffer
		if managed::mut_ptr_eq(fb,self.default_frame_buffer)	{
			let use_color = colors.len()!=0u;
			let value = if use_color {glcore::GL_BACK} else {glcore::GL_NONE} ;
			//FIXME: cache this
			if draw	{
				let mask = if use_color{1} else {0};
				if fb.draw_mask != mask	{
					fb.draw_mask = mask;
					glcore::glDrawBuffer( value );
				}
			}else	{
				let id = if use_color{Some(0u)} else {None};
				if fb.read_id != id	{
					fb.read_id = id;
					glcore::glReadBuffer( value );
				}
			}
			return;
		}
		fn get_color( index : uint )->glcore::GLenum	{
			glcore::GL_COLOR_ATTACHMENT0 + (index as glcore::GLenum)
		};
		// attach planes
		glcore::glGetError();	//debug
		binding.attach_target( stencil,	&mut fb.stencil,	glcore::GL_STENCIL_ATTACHMENT );
		binding.attach_target( depth,	&mut fb.depth,		glcore::GL_DEPTH_ATTACHMENT );
		for colors.eachi() |i,target|	{
			let mut val = fb.colors[i];	//FIXME
			binding.attach_target( *target, &mut val, get_color(i) );
			fb.colors[i] = val;
		}
		glcore::glGetError();	//debug
		let mask = (1u<<colors.len()) - 1u;
		if draw && fb.draw_mask != mask	{
			// set the draw mask
			fb.draw_mask = mask;
			let mut list :~[glcore::GLenum] = ~[];
			let mut i = 0u;
			while mask>>i != 0u	{
				if mask&(1<<i) != 0u	{
					list.push( get_color(i) );
				}
				i += 1;
			}
			match list.len()	{
				0u	=> glcore::glDrawBuffer( glcore::GL_NONE ),
				1u	=> glcore::glDrawBuffer( list[0] ),
				_	=> glcore::glDrawBuffers( list.len() as glcore::GLsizei,
					unsafe{vec::raw::to_ptr(list)} ),
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
					Some(id)	=> glcore::glReadBuffer( get_color(id) ),
					None		=> glcore::glReadBuffer( glcore::GL_NONE ),
				}
			}
		}
		// check completeness
		glcore::glGetError();
		binding.check();	//FIXME: debug only
	}

	fn unbind_frame_buffers( &mut self )	{
		self.frame_buffer_draw.bind( self.default_frame_buffer );
		self.frame_buffer_read.bind( self.default_frame_buffer );
		glcore::glDrawBuffer( glcore::GL_BACK );
		glcore::glReadBuffer( glcore::GL_NONE );
	}
}
