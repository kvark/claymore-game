extern mod gl;

use std;
use std::{borrow,cell,ptr,rc};
use std::to_str::ToStr;

use gr_low::{context,texture};

#[deriving(Eq)]
pub struct SurfaceHandle( gl::types::GLuint );
pub type SurfacePtr = rc::Rc<Surface>;

pub struct Surface	{
	handle	: SurfaceHandle,
	target	: gl::types::GLenum,
	width	: uint,
	height	: uint,
	samples	: uint,
}

impl Drop for SurfaceHandle	{
	fn drop( &mut self )	{
		let SurfaceHandle(ref h) = *self;
		unsafe{ gl::DeleteRenderbuffers( 1, ptr::to_unsafe_ptr(h) ); }
	}
}

impl context::ProxyState for Surface	{
	fn sync_back( &mut self )-> bool	{
		//TODO
		true
	}
}

impl ToStr for Surface	{
	fn to_str( &self )-> ~str	{
		let SurfaceHandle(h) = self.handle;
		format!( "Surface(h={}, {:u}x{:u}, samples={:u})", h,
			self.width, self.height, self.samples )
	}
}

#[deriving(Clone)]
pub enum Target	{
	TarEmpty,
	TarSurface(SurfacePtr),
	TarTexture(texture::TexturePtr,uint),
	TarTextureLayer(texture::TexturePtr,uint,uint),
}

impl std::cmp::Eq for Target	{
	fn eq( &self, other: &Target )-> bool	{
		match (self,other)	{
			(&TarEmpty,&TarEmpty)					=> true,
			(&TarSurface(ref s1),&TarSurface(ref s2))		=>
				borrow::ref_eq(s1.borrow(),s2.borrow()),
			(&TarTexture(ref t1,l1),&TarTexture(ref t2,l2))	=>
				borrow::ref_eq(t1.borrow(),t2.borrow()) && l1==l2,
			(&TarTextureLayer(ref t1,r1,l1),&TarTextureLayer(ref t2,r2,l2))	=>
				borrow::ref_eq(t1.borrow(),t2.borrow()) && r1==r2 && l1==l2,
			(_,_) => false
		}
	}
}

impl ToStr for Target	{
	fn to_str( &self )-> ~str	{
		match self	{
			&TarEmpty	=> ~"Empty",
			&TarSurface(ref s)			=> s.borrow().to_str(),
			&TarTexture(ref t,l)		=> format!( "{:s}.lod[{:u}]", t.borrow().to_str(), l ),
			&TarTextureLayer(ref t,r,l)	=> format!("{:s}.layer[{:u}].lod[{:u}]", t.borrow().to_str(), r, l ),
		}
	}
}


impl Target	{
	pub fn get_size( &self )-> [uint,..4]	{
		match self	{
			&TarEmpty	=> fail!(~"Empty target has no size"),
			&TarSurface(ref sf) =>	{
				let s = sf.borrow();
				[s.width, s.height, 1, s.samples]
			},
			&TarTexture(ref tex,lev) =>	{
				let t = tex.borrow();
				let (w,h) = t.get_level_size(lev);
				[w, h, t.depth, t.samples]
			},
			&TarTextureLayer(ref tex,_,lev) =>	{
				let t = tex.borrow();
				let (w,h) = t.get_level_size(lev);
				[w, h, 1, t.samples]
			}
		}
	}
	
	fn attach( &self, root: gl::types::GLenum, slot: gl::types::GLenum )-> bool	{
		match self	{
			&TarEmpty			=> {},
			&TarSurface(ref s)		=> {
				let SurfaceHandle(h) = s.borrow().handle;
				gl::FramebufferRenderbuffer( root, slot, s.borrow().target, h );
			},
			&TarTexture(ref tex,lev)	=> {
				let t = tex.borrow();
				let texture::Handle(han) = t.handle;
				let texture::Target(tar) = t.target;
				assert!( t.get_num_levels() > lev );
				//gl::FramebufferTexture( root, slot, *tex.handle, lev as gl::types::GLint );
				// workaround for Linux:
				assert!( t.depth == 0 );
				gl::FramebufferTexture2D( root, slot, tar, han, lev as gl::types::GLint );
			},
			&TarTextureLayer(ref tex,layer,lev) => {
				let t = tex.borrow();
				let texture::Handle(h) = t.handle;
				assert!( t.depth > layer && t.get_num_levels() > lev );
				gl::FramebufferTextureLayer( root, slot, h, layer as gl::types::GLint, lev as gl::types::GLint );
			},
		}
		true
	}
}


pub struct RenBinding	{
	target		: gl::types::GLenum,
	default		: SurfacePtr,
	priv active	: SurfacePtr,
}

impl RenBinding	{
	pub fn new( wid: uint, het: uint, ns: uint )-> RenBinding	{
		let t = gl::RENDERBUFFER;
		let s = rc::Rc::new(Surface{
			handle:SurfaceHandle(0),
			target:t, width:wid, height:het, samples:ns
		});
		RenBinding{
			target: t, default: s.clone(), active: s,
		}
	}
}

impl context::ProxyState for RenBinding	{
	fn sync_back( &mut self )-> bool	{
		let mut hid = 0 as gl::types::GLint;
		unsafe{ gl::GetIntegerv( gl::RENDERBUFFER_BINDING, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		let sf = SurfaceHandle(hid as gl::types::GLuint);
		if self.active.borrow().handle != sf	{
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
	pub fn new( wid: uint, het: uint )-> Rect	{
		Rect{ x:0u, y:0u, w:wid, h:het }
	}
	pub fn contains( &self, x: uint, y: uint )-> bool	{
		x>=self.x && x<self.x+self.w && y>=self.y && y<self.y+self.h
	}
	pub fn contains_rect( &self, r: &Rect )-> bool	{
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


#[deriving(Eq)]
pub struct BufferHandle( gl::types::GLuint );
pub type BufferPtr = rc::Rc<cell::RefCell<Buffer>>;

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
		let BufferHandle(ref h) = *self;
		unsafe{ gl::DeleteFramebuffers( 1, ptr::to_unsafe_ptr(h) ); }
	}
}

impl Buffer	{
	pub fn each_target( &self, fun: |&Target| )	{
		let mut iter = self.colors.iter().
			chain( Some(&self.stencil).move_iter() ).
			chain( Some(&self.depth).move_iter() );
		for target in iter	{
			if *target != TarEmpty	{
				fun(target);
			}
		}
	}

	pub fn new_default( rb: SurfacePtr )-> BufferPtr	{
		let ts = TarSurface(rb);
		rc::Rc::new(cell::RefCell::new( Buffer{
			handle		:BufferHandle(0),
			draw_mask	:0x10u,	// invalid one
			read_id		:None,	// actually, GL_BACK
			stencil		:ts.clone(),
			depth		:ts.clone(),
			colors		:~[ts],
		}))
	}
	
	pub fn check_size( &self )-> [uint,..4]	{
		let mut actual = [0u,0u,0u,0u];
		
		self.each_target(|target|	{
			let current = target.get_size();
			if current[0]==0u	{
				actual = current;
			}else	{
				assert_eq!( current, actual );
			}
		});
		actual
	}
}

impl context::ProxyState for Buffer	{
	fn sync_back( &mut self )-> bool	{
		//TODO
		true
	}
}



pub struct Binding	{
	target		: gl::types::GLenum,
	priv active	: BufferPtr,
}

impl context::ProxyState for Binding	{
	fn sync_back( &mut self )-> bool	{
		let mut hid = 0 as gl::types::GLint;
		unsafe{ gl::GetIntegerv( gl::FRAMEBUFFER_BINDING, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		let sf = BufferHandle(hid as gl::types::GLuint);
		let active = self.active.borrow().borrow();
		if active.get().handle != sf	{
			assert!( false, "Active frame buffer mismatch" );
			false
		}else {true}	
	}
}

impl Binding	{
	pub fn new( target: gl::types::GLenum, active: BufferPtr )-> Binding	{
		Binding{ target:target, active:active }
	}

	fn bind( &mut self, b: &BufferPtr )	{
		if !borrow::ref_eq(self.active.borrow(), b.borrow())	{
			let BufferHandle(h) = b.borrow().borrow().get().handle;
			gl::BindFramebuffer( self.target, h );
			self.active = b.clone();
		}
	}

	fn attach_target( &self, new: &Target, old: &mut Target, slot: gl::types::GLenum )-> bool	{
		if *old != *new	{
			*old = new.clone();
			old.attach( self.target, slot )
		}else	{true}
	}

	fn check( &self )	{
		let code = gl::CheckFramebufferStatus( self.target );
		if code == gl::FRAMEBUFFER_COMPLETE	{return};
		let message =
			if code == gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT			{~"format"}		else
			//if code == gl::FRAMEBUFFER_INCOMPLETE_DIMENSIONS			{~"dimensions"}	else
			if code == gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT	{~"missing"}	else
			if code == gl::FRAMEBUFFER_UNSUPPORTED						{~"hardware"}	else
			{~"unknown"};
		let BufferHandle(h) = self.active.borrow().borrow().get().handle;
		fail!("FBO {} is incomplete: {:s}", h, message)
	}
}


impl context::Context	{
	pub fn create_render_buffer( &self, wid:uint, het:uint, sam:uint )-> SurfacePtr	{
		let mut hid = 0 as gl::types::GLuint;
		unsafe{ gl::GenRenderbuffers( 1, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		rc::Rc::new(Surface{
			handle:SurfaceHandle(hid),
			target:self.render_buffer.target,
			width:wid, height:het, samples:sam,
		})
	}

	fn bind_render_buffer( &mut self, s: &SurfacePtr )	{
		let binding = &mut self.render_buffer;
		assert!( s.borrow().target == binding.target );
		if !borrow::ref_eq( binding.active.borrow(), s.borrow() )	{
			binding.active = s.clone();
			let SurfaceHandle(h) = s.borrow().handle;
			gl::BindRenderbuffer( binding.target, h );
		}
	}
	pub fn unbind_render_buffers( &mut self )	{
		let rb = self.render_buffer.default.clone();
		self.bind_render_buffer( &rb );
	}

	pub fn create_frame_buffer( &self )-> BufferPtr	{
		let mut hid = 0 as gl::types::GLuint;
		unsafe{ gl::GenFramebuffers( 1, ptr::to_mut_unsafe_ptr(&mut hid) ); }
		rc::Rc::new(cell::RefCell::new( Buffer{ handle:BufferHandle(hid),
			draw_mask:1u, read_id:Some(0u),
			stencil:TarEmpty, depth:TarEmpty,
			colors	: std::vec::from_elem( self.caps.max_color_attachments, TarEmpty ),
		}))
	}

	pub fn bind_frame_buffer( &mut self, fbp: &BufferPtr, draw: bool,
			stencil: &Target, depth: &Target, colors: ~[Target] )	{
		let binding = if draw {&mut self.frame_buffer_draw} else {&mut self.frame_buffer_read};
		binding.bind( fbp );
		let is_main_fb = borrow::ref_eq( self.default_frame_buffer.borrow(), fbp.borrow() );
		let mut fb = fbp.borrow().borrow_mut();
		// work around main framebuffer
		if 	is_main_fb{
			let use_color = colors.len()!=0u;
			let value = if use_color {gl::BACK} else {gl::NONE} ;
			//TODO: cache this
			if draw	{
				let mask = if use_color{1} else {0};
				if fb.get().draw_mask != mask	{
					fb.get().draw_mask = mask;
					gl::DrawBuffer( value );
				}
			}else	{
				let id = if use_color{Some(0u)} else {None};
				if fb.get().read_id != id	{
					fb.get().read_id = id;
					gl::ReadBuffer( value );
				}
			}
			return;
		}
		fn get_color( index: uint )->gl::types::GLenum	{
			gl::COLOR_ATTACHMENT0 + (index as gl::types::GLenum)
		};
		// attach planes
		gl::GetError();	//debug
		binding.attach_target( stencil,	&mut fb.get().stencil,	gl::STENCIL_ATTACHMENT );
		binding.attach_target( depth,	&mut fb.get().depth,	gl::DEPTH_ATTACHMENT );
		for (i,target) in colors.iter().enumerate()	{
			binding.attach_target( target, &mut fb.get().colors[i], get_color(i) );
		}
		gl::GetError();	//debug
		let mask = (1u<<colors.len()) - 1u;
		if draw && fb.get().draw_mask != mask	{
			// set the draw mask
			fb.get().draw_mask = mask;
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
					list.len() as gl::types::GLsizei, list.as_ptr()
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
			if fb.get().read_id != new_id	{
				fb.get().read_id = new_id;
				match new_id	{
					Some(id)	=> gl::ReadBuffer( get_color(id) ),
					None		=> gl::ReadBuffer( gl::NONE ),
				}
			}
		}
		// check completeness
		gl::GetError();
		binding.check();	//TODO: debug only
	}

	pub fn unbind_frame_buffers( &mut self )	{
		self.frame_buffer_draw.bind( &self.default_frame_buffer );
		self.frame_buffer_read.bind( &self.default_frame_buffer );
		gl::DrawBuffer( gl::BACK );
		gl::ReadBuffer( gl::NONE );
	}
}
