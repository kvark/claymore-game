extern mod glcore;

pub enum Handle	= glcore::GLuint;


pub struct Surface	{
	handle	: Handle,
	target	: glcore::GLenum,
	width	: uint,
	height	: uint,
	samples	: uint,
	priv pool	: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
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
	TarTextureLayer(@texture::Texture,uint,uint),
}

impl Target : Copy	{}

impl Target : cmp::Eq	{
	pure fn eq( &self, other : &Target )-> bool	{
		match (self,other)	{
			(&TarEmpty,&TarEmpty)					=> true,
			(&TarSurface(s1),&TarSurface(s2))		=> *s1.handle == *s2.handle,
			(&TarTexture(t1,l1),&TarTexture(t2,l2))	=> *t1.handle == *t2.handle && l1==l2,
			(&TarTextureLayer(t1,r1,l1),&TarTextureLayer(t2,r2,l2))	=>
				*t1.handle == *t2.handle && r1==r2 && l1==l2,
			(_,_) => false
		}
	}
	pure fn ne( &self, other : &Target )-> bool	{
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
				assert tex.get_levels() > lev;
				glcore::glFramebufferTexture( root, slot, *tex.handle, lev as glcore::GLint );
			},
			TarTextureLayer(tex,layer,lev) => {
				assert tex.depth > layer && tex.get_levels() > lev;
				glcore::glFramebufferTextureLayer( root, slot, *tex.handle, layer as glcore::GLint, lev as glcore::GLint );
			},
		}
		true
	}
}


pub struct RenBinding	{
	target			: glcore::GLenum,
	priv mut active	: Handle,
	priv pool		: @mut ~[Handle],
}

pub struct Binding	{
	target			: glcore::GLenum,
	priv mut active	: Handle,
	priv pool		: @mut ~[Handle],
}

impl RenBinding : context::State	{
	fn sync_back()->bool	{
		let mut hid = 0 as glcore::GLint;
		glcore::glGetIntegerv( glcore::GL_RENDERBUFFER_BINDING, ptr::addr_of(&hid) );
		let hu = hid as glcore::GLuint;
		if *self.active != hu	{
			self.active = Handle( hu );
			false
		}else {true}
	}
}

pub pure fn make_ren_binding()-> RenBinding	{
	RenBinding{
		target:glcore::GL_RENDERBUFFER,
		active:Handle(0), pool:@mut ~[],
	}
}


impl Binding : context::State	{
	fn sync_back()->bool	{
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
	priv fn attach_target( new : Target, old : &mut Target, slot : glcore::GLenum )-> bool	{
		if *old != new	{
			*old = new;
			new.attach( self.target, slot )
		}else	{true}
	}
	priv fn check()	{
		let code = glcore::glCheckFramebufferStatus( self.target );
		if code == glcore::GL_FRAMEBUFFER_COMPLETE	{return};
		let message =
			if code == glcore::GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT			{~"format"}		else
			//if code == glcore::GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS		{~"dimensions"}	else
			if code == glcore::GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT	{~"missing"}	else
			if code == glcore::GL_FRAMEBUFFER_UNSUPPORTED					{~"hardware"}	else
			{~"unknown"};
		fail fmt!("FBO %d is incomplete: %s", *self.active as int, message)
	}
}

pub pure fn make_binding( value : glcore::GLenum )-> Binding	{
	Binding{
		target:value, active:Handle(0), pool:@mut ~[],
	}
}


#[deriving_eq]
pub struct Rect	{
	x : uint,
	y : uint,
	w : uint,
	h : uint,
}

pub pure fn make_rect( wid : uint, het : uint )-> Rect	{
	Rect{ x:0u, y:0u, w:wid, h:het }
}

impl Rect	{
	pure fn contains( x : uint, y : uint )-> bool	{
		x>=self.x && x<self.x+self.w && y>=self.y && y<self.y+self.h
	}
	pure fn contains_rect( r : &Rect )-> bool	{
		r.x>=self.x && r.x+r.w<=self.x+self.w &&
		r.y>=self.y && r.y+r.h<=self.y+self.w
	}
}


pub struct Buffer	{
	handle				: Handle,
	priv mut draw_mask	: uint,
	priv mut read_id	: Option<uint>,
	mut stencil			: Target,
	mut depth			: Target,
	mut colors			: ~[Target],
	priv mut pool		: @mut ~[Handle],

	drop	{
		self.pool.push( self.handle );
	}
}

impl Buffer	{
	pure fn check_size()->(uint,uint,uint,uint)	{
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
				assert w==w2 && h==h2 && d==d2 && s==s2;
			}
		}
		actual
	}
}

impl Buffer : context::State	{
	fn sync_back()->bool	{
		//FIXME
		true
	}
}

pub fn default_frame_buffer()-> Buffer	{
	Buffer{
		handle:Handle(0),
		draw_mask		:0u,
		read_id			:None,
		stencil			:TarEmpty,
		depth			:TarEmpty,
		colors			:~[TarEmpty],
		pool			:@mut ~[],
	}
}


impl context::Context	{
	fn create_render_buffer( wid:uint, het:uint, sam:uint )-> Surface	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenRenderbuffers( 1, ptr::addr_of(&hid) );
		Surface{ handle:Handle(hid),
			target:self.render_buffer.target,
			width:wid, height:het, samples:sam,
			pool:self.render_buffer.pool }
	}

	fn _bind_render_buffer( h : Handle )	{
		let binding = &self.render_buffer;
		if *binding.active != *h	{
			binding.active = h;
			glcore::glBindRenderbuffer( binding.target, *h );
		}
	}
	fn bind_render_buffer( rb : &Surface )	{
		self._bind_render_buffer( rb.handle );
	}
	fn unbind_render_buffers()	{
		self._bind_render_buffer( Handle(0) );
	}

	fn create_frame_buffer()-> Buffer	{
		let mut hid = 0 as glcore::GLuint;
		glcore::glGenFramebuffers( 1, ptr::addr_of(&hid) );
		Buffer{ handle:Handle(hid), draw_mask:1u, read_id:None,
			stencil:TarEmpty, depth:TarEmpty,
			colors	: vec::from_elem( self.caps.max_color_attachments, TarEmpty ),
			pool	: self.frame_buffer_draw.pool,
		}
	}

	fn _bind_frame_buffer( binding : &Binding, h : Handle )	{
		if *binding.active != *h	{
			binding.active = h;
			glcore::glBindFramebuffer( binding.target, *h );
		}
	}

	fn bind_frame_buffer( fb : &Buffer, draw : bool,
			stencil : Target, depth : Target, colors : ~[Target] )	{
		let binding = if draw {&self.frame_buffer_draw} else {&self.frame_buffer_read};
		self._bind_frame_buffer( binding, fb.handle );
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
		pure fn get_color( index : uint )->glcore::GLenum	{
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
				glcore::glDrawBuffers( list.len() as glcore::GLsizei,
					unsafe{vec::raw::to_ptr(list)} );
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

	fn cleanup_frames()	{
		while self.render_buffer.pool.len()!=0	{
			let h = self.render_buffer.pool.pop();
			assert *h != 0;
			if *h == *self.render_buffer.active	{
				self.unbind_render_buffers();
			}
			glcore::glDeleteRenderbuffers( 1, ptr::addr_of(&*h) );	
		}
		while self.frame_buffer_draw.pool.len()!=0	{
			let h = self.frame_buffer_draw.pool.pop();
			assert *h != 0;
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
