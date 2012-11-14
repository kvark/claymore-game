extern mod glcore;

pub enum Handle	= glcore::GLuint;


pub struct RenBinding	{
	target		: glcore::GLenum,
	mut active	: Handle,
}

pub struct Binding	{
	target		: glcore::GLenum,
	mut active	: Handle,
}

impl RenBinding : context::State	{
	fn sync_back()->bool	{
		let mut hid = 0 as glcore::GLint;
		unsafe	{
			glcore::glGetIntegerv( glcore::GL_RENDERBUFFER_BINDING, ptr::addr_of(&hid) );
		}
		let hu = hid as glcore::GLuint;
		if *self.active != hu	{
			self.active = Handle( hu );
			false
		}else {true}
	}
}

impl Binding : context::State	{
	fn sync_back()->bool	{
		let mut hid = 0 as glcore::GLint;
		unsafe	{
			glcore::glGetIntegerv( glcore::GL_FRAMEBUFFER_BINDING, ptr::addr_of(&hid) );
		}
		let hu = hid as glcore::GLuint;
		if *self.active != hu	{
			self.active = Handle( hu );
			false
		}else {true}	
	}
}


pub struct Surface	{
	handle	: Handle,
	target	: glcore::GLenum,
	width	: uint,
	height	: uint,
	samples	: uint,

	drop	{
		let hid = *self.handle as glcore::GLuint;
		unsafe	{
			glcore::glDeleteRenderbuffers( 1, ptr::addr_of(&hid) );	
		}
	}
}

impl Surface : context::State	{
	fn sync_back()->bool	{
		//FIXME
		true
	}
}


pub enum Target	{
	TarEmpty,
	TarSurface(@Surface),
	TarTexture(@texture::Texture,uint),
}

impl Target : Copy	{}

//FIXME: remove once auto-generated
impl Target : cmp::Eq	{
	pure fn eq( other : &Target )-> bool	{
		match (&self,other)	{
			(&TarEmpty,&TarEmpty)					=> true,
			(&TarSurface(s1),&TarSurface(s2))		=> *s1.handle == *s2.handle,
			(&TarTexture(t1,l1),&TarTexture(t2,l2))	=> *t1.handle == *t2.handle && l1==l2,
			(_,_) => false
		}
	}
	pure fn ne( other : &Target)-> bool	{
		!self.eq( other )
	}
}

impl Target	{
	priv fn attach( root : glcore::GLenum, slot : glcore::GLenum )-> bool	{
		match self	{
			TarEmpty			=> {},
			TarSurface(s)		=> {
				glcore::glFramebufferRenderbuffer( root, slot, s.target, *s.handle );
			},
			TarTexture(tex,lev)	=> {
				glcore::glFramebufferTexture( root, slot, *tex.handle, lev as glcore::GLint );
			}
		}
		true
	}
}


pub struct Rect	{
	x : uint,
	y : uint,
	w : uint,
	h : uint,
}

impl Rect : cmp::Eq	{
	pure fn eq( other : &Rect )-> bool	{
		self.x==other.x && self.y==other.y && self.w==other.w && self.h==other.h
	}
	pure fn ne( other : &Rect )-> bool	{
		!self.eq( other )
	}
}


pub struct Buffer	{
	handle			: Handle,
	priv mut viewport	: Rect,
	priv mut draw_mask	: uint,
	priv mut read_id	: Option<uint>,
	depth_stencil	: Target,
	colors			: ~[Target],

	drop	{
		let hid = *self.handle as glcore::GLuint;
		unsafe	{
			glcore::glDeleteFramebuffers( 1, ptr::addr_of(&hid) );
		}
	}
}


impl Buffer	{
	pure fn check_size()->(uint,uint,uint)	{
		let mut wid = 0u, het = 0u, sam = 0u;
		for (~[self.depth_stencil] + self.colors).each |target|	{
			match *target	{
				TarEmpty => {},
				TarSurface(sf) => 	{
					if wid==0u	{ wid=sf.width; het=sf.height; sam=sf.samples; }
					else	{ assert wid==sf.width && het==sf.height && sam==sf.samples };
				},
				TarTexture(tex,lev) =>	{
					let (w,h) = tex.get_level_size(lev);
					if wid==0u	{ wid=w; het=h; sam=tex.samples; }
					else	{ assert wid==w && het==h && sam==tex.samples; }
				}
			}
		}
		(wid,het,sam)
	}
}

impl Buffer : context::State	{
	fn sync_back()->bool	{
		//FIXME
		true
	}
}



impl context::Context	{
	fn create_render_buffer( wid:uint, het:uint, sam:uint )-> @Surface	{
		let mut hid = 0 as glcore::GLuint;
		unsafe	{
			glcore::glGenRenderbuffers( 1, ptr::addr_of(&hid) );
		}
		@Surface{ handle:Handle(hid), target:self.renderbuffer.target,
			width:wid, height:het, samples:sam }
	}

	fn _bind_render_buffer( h : Handle )	{
		let binding = &self.renderbuffer;
		if *binding.active != *h	{
			binding.active = h;
			glcore::glBindRenderbuffer( binding.target, *h );
		}
	}
	fn bind_render_buffer( rb : @Surface )	{
		self._bind_render_buffer( rb.handle );
	}
	fn unbind_render_buffers()	{
		self._bind_render_buffer( Handle(0) );
	}

	fn create_frame_buffer()-> Buffer	{
		let mut hid = 0 as glcore::GLuint;
		unsafe	{
			glcore::glGenFramebuffers( 1, ptr::addr_of(&hid) );
		}
		Buffer{ handle:Handle(hid), viewport:Rect{x:0u,y:0u,w:0u,h:0u},
			draw_mask:0u, read_id:None,
			depth_stencil	: TarEmpty,
			colors			: vec::from_elem( self.caps.max_color_attachments, TarEmpty ),
		}
	}

	fn _bind_frame_buffer( binding : &Binding, h : Handle )	{
		if *binding.active != *h	{
			binding.active = h;
			glcore::glBindFramebuffer( binding.target, *h );
		}
	}

	fn bind_frame_buffer( fb : &Buffer, draw : bool )-> glcore::GLenum	{
		let binding = if draw {&self.framebuffer_draw} else {&self.framebuffer_read};
		self._bind_frame_buffer( binding, fb.handle );
		binding.target
	}

	fn set_draw_buffers( fb : &Buffer, mask : uint )	{
		assert *self.framebuffer_draw.active == *fb.handle;
		if fb.draw_mask != mask	{
			fb.draw_mask = mask;
			let mut list :~[glcore::GLenum] = ~[];
			let mut i = 0u;
			while mask>>i != 0u	{
				if mask&(1<<i) != 0u	{
					list.push( glcore::GL_COLOR_ATTACHMENT0 + (i as glcore::GLenum) );
				}
				i += 1;
			}
			if list.len() == 0u	{
				glcore::glDrawBuffer( glcore::GL_NONE );
			}else
			if list.len() == 1u	{
				glcore::glDrawBuffer( list[0] );
			}else	{
				unsafe	{
					glcore::glDrawBuffers( list.len() as glcore::GLsizei, vec::raw::to_ptr(list) )
				}
			}
		}
		// update the viewport
		let r = {
			let (wid,het,_sam) = fb.check_size();
			Rect{ x:0u, y:0u, w:wid, h:het }
		};
		if fb.viewport != r	{
			fb.viewport = r;
			glcore::glViewport( r.x as glcore::GLint, r.y as glcore::GLint,
				r.w as glcore::GLsizei, r.h as glcore::GLsizei );
		}
	}

	fn set_read_buffer( fb : &Buffer, index : Option<uint> )	{
		assert *self.framebuffer_read.active == *fb.handle;
		if fb.read_id == index	{return;}
		fb.read_id = index;
		match index	{
			Some(id)	=> glcore::glReadBuffer( glcore::GL_COLOR_ATTACHMENT0 + (id as glcore::GLenum) ),
			None		=> glcore::glReadBuffer( glcore::GL_NONE ),
		}
	}

	fn unbind_frame_buffers()	{
		self._bind_frame_buffer( &self.framebuffer_draw, Handle(0) );
		self._bind_frame_buffer( &self.framebuffer_read, Handle(0) );
		glcore::glDrawBuffer( glcore::GL_BACK );
		glcore::glReadBuffer( glcore::GL_NONE );
	}
}
