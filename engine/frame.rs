extern mod glcore;

pub enum Handle	= glcore::GLuint;


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

impl Binding	{
	priv fn attach_target( new : Target, old : &mut Target, slot : glcore::GLenum )-> bool	{
		if *old != new	{
			*old = new;
			new.attach( self.target, slot )
		}else	{true}
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
	handle				: Handle,
	priv mut viewport	: Rect,
	priv mut draw_mask	: uint,
	priv mut read_id	: Option<uint>,
	mut depth_stencil	: Target,
	mut colors			: ~[Target],

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

	fn bind_frame_buffer( fb : &Buffer, draw : bool, depth_stencil : Target, colors : ~[Target] )	{
		pure fn get_color( index : uint )->glcore::GLenum	{
			glcore::GL_COLOR_ATTACHMENT0 + (index as glcore::GLenum)
		};
		let binding = if draw {&self.framebuffer_draw} else {&self.framebuffer_read};
		self._bind_frame_buffer( binding, fb.handle );
		// attach planes
		binding.attach_target( depth_stencil, &mut fb.depth_stencil, glcore::GL_DEPTH_STENCIL_ATTACHMENT );
		for colors.eachi() |i,target|	{
			let mut val = fb.colors[i];	//FIXME
			binding.attach_target( *target, &mut val, get_color(i) );
			fb.colors[i] = val;
		}
		let mask = (1u<<colors.len()) - 1u;
		// set the read mask
		if !draw	{
			assert mask&(mask-1u) == 0;
			let new_id =
			if mask != 0u	{
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
			return;
		}
		// set the draw mask
		if fb.draw_mask != mask	{
			fb.draw_mask = mask;
			let mut list :~[glcore::GLenum] = ~[];
			let mut i = 0u;
			while mask>>i != 0u	{
				if mask&(1<<i) != 0u	{
					list.push( get_color(i) );
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

	fn unbind_frame_buffers()	{
		self._bind_frame_buffer( &self.framebuffer_draw, Handle(0) );
		self._bind_frame_buffer( &self.framebuffer_read, Handle(0) );
		glcore::glDrawBuffer( glcore::GL_BACK );
		glcore::glReadBuffer( glcore::GL_NONE );
	}
}
