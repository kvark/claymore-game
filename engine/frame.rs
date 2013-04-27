extern mod glcore;

use context;
use texture;


pub struct Handle( glcore::GLuint );


pub struct Surface	{
	handle	: Handle,
	target	: glcore::GLenum,
	width	: uint,
	height	: uint,
	samples	: uint,
	priv pool	: @mut ~[Handle],
}

#[unsafe_destructor]
impl Drop for Surface	{
	fn finalize( &self )	{
		self.pool.push( self.handle );
	}
}

impl context::ProxyState for Surface	{
	fn sync_back( &mut self )-> bool	{
		//FIXME
		true
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


impl Target	{
	priv fn attach( &self, root : glcore::GLenum, slot : glcore::GLenum )-> bool	{
		match self	{
			&TarEmpty			=> {},
			&TarSurface(s)		=> {
				glcore::glFramebufferRenderbuffer( root, slot, s.target, *s.handle );
			},
			&TarTexture(tex,lev)	=> {
				assert!( tex.get_levels() > lev );
				glcore::glFramebufferTexture( root, slot, *tex.handle, lev as glcore::GLint );
			},
			&TarTextureLayer(tex,layer,lev) => {
				assert!( tex.depth > layer && tex.get_levels() > lev );
				glcore::glFramebufferTextureLayer( root, slot, *tex.handle, layer as glcore::GLint, lev as glcore::GLint );
			},
		}
		true
	}
}


pub struct RenBinding	{
	target		: glcore::GLenum,
	priv active	: Handle,
	priv pool	: @mut ~[Handle],
}

pub impl RenBinding	{
	fn new()-> RenBinding	{
		RenBinding{
			target : glcore::GL_RENDERBUFFER,
			active : Handle(0), pool : @mut ~[],
		}
	}
}

impl context::ProxyState for RenBinding	{
	fn sync_back( &mut self )-> bool	{
		let mut hid = 0 as glcore::GLint;
		glcore::glGetIntegerv( glcore::GL_RENDERBUFFER_BINDING, ptr::addr_of(&hid) );
		let hu = hid as glcore::GLuint;
		if *self.active != hu	{
			self.active = Handle( hu );
			false
		}else {true}
	}
}



pub struct Binding	{
	target		: glcore::GLenum,
	priv active	: Handle,
	priv pool	: @mut ~[Handle],
}

impl context::ProxyState for Binding	{
	fn sync_back( &mut self )-> bool	{
		let mut hid = 0 as glcore::GLint;
		glcore::glGetIntegerv( glcore::GL_FRAMEBUFFER_BINDING, ptr::addr_of(&hid) );
		let hu = hid as glcore::GLuint;
		if *self.active != hu	{
			self.active = Handle( hu );
			false
		}else {true}	
	}
}

impl Binding	{
	pub fn new( value : glcore::GLenum )-> Binding	{
		Binding{
			target:value, active:Handle(0), pool:@mut ~[],
		}
	}

	priv fn _bind( &mut self, h : Handle )	{
		if *self.active != *h	{
			self.active = h;
			glcore::glBindFramebuffer( self.target, *h );
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
		fail!(fmt!( "FBO %d is incomplete: %s", *self.active as int, message ))
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
}


pub struct Buffer	{
	handle			: Handle,
	priv draw_mask	: uint,
	priv read_id	: Option<uint>,
	stencil			: Target,
	depth			: Target,
	colors			: ~[Target],
	priv pool		: @mut ~[Handle],
}

#[unsafe_destructor]
impl Drop for Buffer	{
	fn finalize( &self )	{
		self.pool.push( self.handle );
	}
}

pub impl Buffer	{
	fn new_default()-> Buffer	{
		Buffer{
			handle:Handle(0),
			draw_mask		:0u,
			read_id			:None,	// actually, GL_BACK
			stencil			:TarEmpty,
			depth			:TarEmpty,
			colors			:~[TarEmpty],
			pool			:@mut ~[],
		}
	}
	
	fn check_size( &self )->(uint,uint,uint,uint)	{
		let mut actual = (0u,0u,0u,0u);
		for (~[self.stencil,self.depth] + self.colors).each |target|	{
			let current = match *target	{
				TarEmpty => actual,
				TarSurface(sf) => (sf.width,sf.height,1,sf.samples),
				TarTexture(tex,lev) =>	{
					let (w,h) = tex.get_level_size(lev);
					(w,h,tex.depth,tex.samples)
				},
				TarTextureLayer(tex,_,lev) =>	{
					let (w,h) = tex.get_level_size(lev);
					(w,h,1,tex.samples)
				}
			};
			let (w,h,d,s) = actual;
			if w==0u	{ actual = current; }
			else	{
				let (w2,h2,d2,s2) = current;
				assert!( w==w2 && h==h2 && d==d2 && s==s2 );
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


pub impl context::Context	{
	fn create_render_buffer( &self, wid:uint, het:uint, sam:uint )-> Surface	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenRenderbuffers( 1, ptr::addr_of(&hid) );
		Surface{ handle:Handle(hid),
			target:self.render_buffer.target,
			width:wid, height:het, samples:sam,
			pool:self.render_buffer.pool }
	}

	priv fn _bind_render_buffer( &mut self, h : Handle )	{
		let binding = &mut self.render_buffer;
		if *binding.active != *h	{
			binding.active = h;
			glcore::glBindRenderbuffer( binding.target, *h );
		}
	}
	fn bind_render_buffer( &mut self, rb : &Surface )	{
		self._bind_render_buffer( rb.handle );
	}
	fn unbind_render_buffers( &mut self )	{
		self._bind_render_buffer( Handle(0) );
	}

	fn create_frame_buffer( &self )-> Buffer	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenFramebuffers( 1, ptr::addr_of(&hid) );
		Buffer{ handle:Handle(hid),
			draw_mask:1u, read_id:Some(0u),
			stencil:TarEmpty, depth:TarEmpty,
			colors	: vec::from_elem( self.caps.max_color_attachments, TarEmpty ),
			pool	: self.frame_buffer_draw.pool,
		}
	}

	fn bind_frame_buffer( &mut self, fb : &mut Buffer, draw : bool,
			stencil : Target, depth : Target, colors : ~[Target] )	{
		let binding = if draw {&mut self.frame_buffer_draw} else {&mut self.frame_buffer_read};
		binding._bind( fb.handle );
		// work around main framebuffer
		if *fb.handle == 0	{
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
		binding.attach_target( stencil,	&mut fb.stencil,	glcore::GL_STENCIL_ATTACHMENT );
		binding.attach_target( depth,	&mut fb.depth,		glcore::GL_DEPTH_ATTACHMENT );
		for colors.eachi() |i,target|	{
			let mut val = fb.colors[i];	//FIXME
			binding.attach_target( *target, &mut val, get_color(i) );
			fb.colors[i] = val;
		}
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
		binding.check();	//FIXME: debug only
	}

	/*fn unbind_frame_buffers()	{
		self._bind_frame_buffer( &self.framebuffer_draw, Handle(0) );
		self._bind_frame_buffer( &self.framebuffer_read, Handle(0) );
		glcore::glDrawBuffer( glcore::GL_BACK );
		glcore::glReadBuffer( glcore::GL_NONE );
	}*/

	fn cleanup_frames( &mut self )	{
		let rb_pool : &mut ~[Handle] = self.render_buffer.pool;
		while !rb_pool.is_empty()	{
			let h = rb_pool.pop();
			assert!( *h != 0 );
			if *h == *self.render_buffer.active	{
				self.unbind_render_buffers();
			}
			glcore::glDeleteRenderbuffers( 1, ptr::addr_of(&*h) );	
		}
		let fb_pool : &mut ~[Handle] = self.frame_buffer_draw.pool;
		while !fb_pool.is_empty()	{
			let h = fb_pool.pop();
			assert!( *h != 0 );
			if *h == *self.frame_buffer_draw.active	{
				//self.unbind_draw_buffer();
			}
			if *h == *self.frame_buffer_read.active	{
				//self.unbind_read_buffer();
			}
			glcore::glDeleteFramebuffers( 1, ptr::addr_of(&*h) );
		}
	}
}
