extern mod glcore;

pub enum Handle	= glcore::GLuint;

pub struct Binding	{
	target		: glcore::GLenum,
	mut active	: Handle,
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


pub enum Target	{
	TarEmpty,
	TarSurface(@Surface),
	TarTexture(@texture::Texture,uint),
}

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


struct Attachment	{
	slot			: glcore::GLenum,
	mut value		: Target,
	priv mut cache	: Target,
}

priv fn create_at( slot : glcore::GLenum )-> Attachment	{
	Attachment{ slot:slot, value:TarEmpty, cache:TarEmpty }
}

impl Attachment	{
	priv fn attach( root : glcore::GLenum )-> bool	{
		if self.value == self.cache	{
			return false;
		}
		self.value = self.cache;
		match copy self.value	{
			TarEmpty			=> {},
			TarSurface(s)		=> {
				glcore::glFramebufferRenderbuffer( root, self.slot, s.target, *s.handle );
			},
			TarTexture(tex,lev)	=> {
				glcore::glFramebufferTexture( root, self.slot, *tex.handle, lev as glcore::GLint );
			}
		}
		true
	}
}


pub struct Buffer	{
	handle			: Handle,
	//viewport,
	priv mut mask	: uint,
	depth_stencil	: Attachment,
	color0			: Attachment,
	color1			: Attachment,
	color2			: Attachment,
	color3			: Attachment,

	drop	{
		let hid = *self.handle as glcore::GLuint;
		unsafe	{
			glcore::glDeleteFramebuffers( 1, ptr::addr_of(&hid) );
		}
	}
}


impl Buffer	{
	pure fn check()	{
		let mut wid = 0u, het = 0u, sam = 0u;
		for [&self.depth_stencil,&self.color0,&self.color1,&self.color2,&self.color3].each |at|	{
			match copy at.value	{
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

	fn _bind_render_buffer( binding : &Binding, h : Handle )	{
		if *binding.active != *h	{
			binding.active = h;
			glcore::glBindRenderbuffer( binding.target, *h );
		}
	}
	fn bind_render_buffer( rb : @Surface )	{
		assert self.renderbuffer.target == rb.target;
		self._bind_render_buffer( &self.renderbuffer, rb.handle );
	}
	fn unbind_render_buffers()	{
		self._bind_render_buffer( &self.renderbuffer, Handle(0) );
	}

	fn create_frame_buffer()-> Buffer	{
		let mut hid = 0 as glcore::GLuint;
		unsafe	{
			glcore::glGenFramebuffers( 1, ptr::addr_of(&hid) );
		}
		Buffer{ handle:Handle(hid), mask:0u,
			depth_stencil	: create_at(glcore::GL_DEPTH_STENCIL_ATTACHMENT),
			color0			: create_at(glcore::GL_COLOR_ATTACHMENT0),
			color1			: create_at(glcore::GL_COLOR_ATTACHMENT1),
			color2			: create_at(glcore::GL_COLOR_ATTACHMENT2),
			color3			: create_at(glcore::GL_COLOR_ATTACHMENT3),
		}
	}

	fn _bind_frame_buffer( binding : &Binding, h : Handle )	{
		if *binding.active != *h	{
			binding.active = h;
			glcore::glBindFramebuffer( binding.target, *h );
		}
	}

	fn bind_frame_buffer( fb : &Buffer, draw : bool, mask : uint )	{
		fb.check();
		// bind FBO
		let binding = if draw {&self.framebuffer_draw} else {&self.framebuffer_read};
		self._bind_frame_buffer( binding, fb.handle );
		// attach missing attributes
		let array = [&fb.depth_stencil,&fb.color0,&fb.color1,&fb.color2,&fb.color3];
		for array.each |at|	{
			at.attach( binding.target );
		}
		// set new mask
		if fb.mask != mask	{
			fb.mask = mask;
			let mut list :~[glcore::GLenum] = ~[];
			let mut i = 0u;
			while mask>>i != 0u	{
				if mask&(1<<i) != 0u	{
					list.push( array[i+1].slot );
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
	}

	fn unbind_frame_buffers()	{
		self._bind_render_buffer( &self.framebuffer_draw, Handle(0) );
		self._bind_render_buffer( &self.framebuffer_read, Handle(0) );
		glcore::glDrawBuffer( glcore::GL_BACK );
		glcore::glReadBuffer( glcore::GL_NONE );
	}
}
