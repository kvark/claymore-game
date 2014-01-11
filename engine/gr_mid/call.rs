extern mod gl;

use std;
use std::hashmap::HashMap;

use journal;
use gr_low::{buf,context,frame,rast,shade};
use gr_low::rast::Stage;
use gr_mid::mesh;


#[deriving(Clone)]
pub struct PlaneMap	{
	stencil	: frame::Target,
	depth	: frame::Target,
	colors	: HashMap<~str,frame::Target>,
}

impl PlaneMap	{
	pub fn new_empty()-> PlaneMap	{
		PlaneMap	{
			stencil	: frame::TarEmpty,
			depth	: frame::TarEmpty,
			colors	: HashMap::new(),
		}
	}

	pub fn new_simple( name: ~str, col: frame::Target )-> PlaneMap	{
		let mut pm = PlaneMap::new_empty();
		pm.colors.insert( name, col );
		pm
	}

	pub fn new_main( gc: &context::Context, name: ~str )-> PlaneMap	{
		let tg = frame::TarSurface( gc.render_buffer.default );
		let mut pm = PlaneMap::new_empty();
		pm.stencil = tg;
		pm.depth = tg;
		pm.colors.insert( name, tg );
		pm
	}

	fn get_any_target( &self )-> Option<frame::Target>	{
		if self.stencil != frame::TarEmpty	{
			Some(self.stencil)
		}else
		if self.depth != frame::TarEmpty	{
			Some(self.depth)
		}else	{
			for (_,&val) in self.colors.iter()	{
				if val != frame::TarEmpty	{
					return Some(val);
				}
			}
			None
		}
	}

	pub fn get_area( &self )-> frame::Rect	{
		let size = match self.get_any_target()	{
			Some(tg)	=>	tg.get_size(),
			None		=> [0,0,0,0],
		};
		frame::Rect::new( size[0], size[1] )
	}

	pub fn check( &self, rast: &rast::State )	{
		assert!( !rast.stencil.test	|| self.stencil	!= frame::TarEmpty );
		assert!( !rast.depth.test	|| self.depth	!= frame::TarEmpty );
		assert!( !rast.blend.on		|| !self.colors.is_empty() );
	}

	pub fn log( &self, lg: &journal::Log )	{
		if self.stencil != frame::TarEmpty	{
			lg.add(format!( "\t\tstencil\t= {:s}", self.stencil.to_str() ));
		}
		if self.depth != frame::TarEmpty	{
			lg.add(format!( "\t\tdepth\t= {:s}", self.depth.to_str() ));	
		}
		for (name,val) in self.colors.iter()	{
			lg.add(format!( "\t\t{:s}\t= {:s}", *name, val.to_str() ));
		}
	}
}


#[deriving(Clone)]
pub struct ClearData	{
	color	: Option<rast::Color>,
	depth	: Option<f32>,
	stencil	: Option<u32>,
}

#[deriving(Clone)]
pub struct Input	{
	va		: buf::VertexArrayPtr,
	mesh	: @mesh::Mesh,
	range	: mesh::Range,
}

impl Input	{
	pub fn new( va: buf::VertexArrayPtr, m: @mesh::Mesh )-> Input	{
		Input	{
			va		: va,
			mesh	: m,
			range	: m.get_range(),
		}
	}
	pub fn log( &self, lg: &journal::Log )	{
		let buf::ArrayHandle(han) = self.va.borrow().borrow().get().handle;
		lg.add(format!( "\tMesh '{:s}' at VAO={} with range [{:u}:{:u}]",
			self.mesh.name, han, self.range.start, self.range.start+self.range.num ));
	}
}


#[deriving(Clone)]
pub struct Output	{
	fb		: frame::BufferPtr,
	pmap	: PlaneMap,
	area	: frame::Rect,
}

impl Output	{
	pub fn new( fb: frame::BufferPtr, pmap: PlaneMap )-> Output	{
		let area = pmap.get_area();
		Output	{
			fb	: fb,
			pmap: pmap,
			area: area,
		}
	}
	pub fn gen_scissor( &self )-> rast::Scissor	{
		rast::Scissor	{
			test: self.area != self.pmap.get_area(),
			area: self.area,
		}
	}
	pub fn log( &self, kind: &str, lg: &journal::Log )	{
		let frame::BufferHandle(han) = self.fb.borrow().borrow().get().handle;
		lg.add(format!( "\t{:s} FBO={} with area {:s}", kind, han, self.area.to_str() ));
		self.pmap.log( lg );
	}
}

#[deriving(Clone)]
pub enum Call	{
	// naming convention: What-Where-How
	CallEmpty,
	CallClear( ClearData, Output, rast::Mask ),
	CallBlit( Output, Output ),
	CallDraw( Input, Output, rast::State, shade::ProgramPtr, shade::DataMap ),
	CallTransfrom(),	//TODO
}

impl Call	{
	pub fn log( &self, lg: &journal::Log )	{
		match self	{
			&CallEmpty	=> lg.add("Call empty"),
			&CallClear(ref cd, ref out, ref _mask)	=>	{
				lg.add("Call clear");
				lg.add(format!( "\tValue {:s} {:s} {:s}",
					match cd.color	{
						Some(v)	=> format!("color({:f},{:f},{:f},{:f})",
							v.r, v.g, v.b, v.a),
						None	=> ~"",
					},
					match cd.depth	{
						Some(v)	=> format!("depth({:f})", v),
						None	=> ~"",
					},
					match cd.stencil	{
						Some(v)	=> format!("stencil({:u})", v),
						None	=> ~"",
					}) );
				out.log( "Output", lg );
			}
			&CallBlit(ref src, ref dst)	=>	{
				lg.add("Call blit");
				src.log( "Src", lg );
				dst.log( "Dst", lg );
			},
			&CallDraw(ref inp, ref out, ref _rast, ref prog, ref data )	=>	{
				lg.add("Call draw");
				inp.log( lg );
				out.log( "Output", lg );
				let han = prog.borrow().with(|p|	{
					let shade::ProgramHandle(h) = p.handle; h
				});
				lg.add(format!( "\tProgram={}", han ));
				data.log( lg );
			},
			&CallTransfrom()	=>	{
				lg.add("Call transform");
			},
		}
	}

	pub fn execute( self, gc: &mut context::Context )	{
		match self	{
			CallEmpty => {},
			CallClear(cdata, out, mask)	=> {
				let mut colors : ~[frame::Target] = ~[];
				for (_,&target) in out.pmap.colors.iter()	{
					colors.push( target );
				}
				let is_main_fb =	{
					let fb = out.fb.borrow().borrow();
					fb.get().handle == frame::BufferHandle(0)
				};
				let has_color = colors.len()!=0 && (is_main_fb || colors[0]!=frame::TarEmpty);
				gc.bind_frame_buffer( out.fb.clone(), true, out.pmap.stencil, out.pmap.depth, colors );
				gc.rast.scissor.activate( &out.gen_scissor(), 0 );
				gc.rast.mask.activate( &mask, 0 );
				let mut flags = 0 as gl::types::GLenum;
				//FIXME: cache this
				match cdata.color	{
					Some(c) =>	{
						assert!( has_color );
						flags |= gl::COLOR_BUFFER_BIT;
						gc.set_clear_color( &c );
					},
					None	=>	()
				}
				match cdata.depth	{
					Some(d) => 	{
						assert!( is_main_fb || out.pmap.depth!=frame::TarEmpty );
						flags |= gl::DEPTH_BUFFER_BIT;
						gc.set_clear_depth( d );
					},
					None	=> 	()
				}
				match cdata.stencil	{
					Some(s)	=>	{
						assert!( is_main_fb || out.pmap.stencil!=frame::TarEmpty );
						flags |= gl::STENCIL_BUFFER_BIT;
						gc.set_clear_stencil( s );
					},
					None	=>	()
				}
				gl::Clear( flags );
			},
			CallBlit(src, dst)	=>	{
				assert!( !src.fb.ptr_eq( &dst.fb ) );
				// bind frame buffers
				gc.bind_frame_buffer( src.fb.clone(), false, src.pmap.stencil, src.pmap.depth,
					src.pmap.colors.iter().map(|(_,&v)| v).collect() );
				gc.bind_frame_buffer( dst.fb.clone(), true, dst.pmap.stencil, dst.pmap.depth,
					dst.pmap.colors.iter().map(|(_,&v)| v).collect() );
				// set state
				gc.rast.scissor.activate( &dst.gen_scissor(), 0 );
				let mut flags = 0 as gl::types::GLenum;
				let mut only_color = true;
				if !src.pmap.colors.is_empty() || !dst.pmap.colors.is_empty()	{
					flags |= gl::COLOR_BUFFER_BIT;
				}
				if src.pmap.depth != frame::TarEmpty || dst.pmap.depth != frame::TarEmpty	{
					flags |= gl::DEPTH_BUFFER_BIT;
					only_color = false;
				}
				if src.pmap.stencil != frame::TarEmpty || dst.pmap.stencil != frame::TarEmpty	{
					flags |= gl::STENCIL_BUFFER_BIT;
					only_color = false;
				}
				// prepare
				let sizeA = { let sfb = src.fb.borrow().borrow(); sfb.get().check_size() };
				let sizeB = { let dfb = dst.fb.borrow().borrow(); dfb.get().check_size() };
				assert!( sizeA[3] == sizeB[3] || (sizeA[3]*sizeB[3]==0 && only_color) );
				let filter = if (only_color && sizeA[3]==0) {gl::LINEAR} else {gl::NEAREST};
				// call blit
				gl::BlitFramebuffer(
					0, 0, sizeA[0] as gl::types::GLint, sizeA[1] as gl::types::GLint,
					0, 0, sizeB[0] as gl::types::GLint, sizeB[1] as gl::types::GLint,
					flags, filter );
			},
			CallDraw(inp, out, mut rast, prog, data)	=> {
				// bind FBO
				let mut attaches = std::vec::from_elem( out.pmap.colors.len(), frame::TarEmpty );
				for (name,target) in out.pmap.colors.iter()	{
					let loc = prog.borrow().with(|p| p.find_output( name ));
					assert!( loc < attaches.len() && attaches[loc] == frame::TarEmpty );
					attaches[loc] = *target;
				}
				gc.bind_frame_buffer( out.fb.clone(), true, out.pmap.stencil, out.pmap.depth, attaches );
				// check & activate raster
				rast.scissor = out.gen_scissor();
				gc.rast.activate( &rast, inp.mesh.get_poly_size() );
				//assert_eq!( out.area, *gc.rast.view );
				// draw
				gc.draw_mesh( &inp, prog, &data );
			},
			_	=> fail!(~"Unsupported call!")
		}
	}
}


impl context::Context	{
	pub fn flush( &mut self, queue: ~[Call], lg: &journal::Log )	{
		self.call_count += queue.len();
		for call in queue.move_iter()	{
			if lg.enable	{
				call.log( lg );
			}
			call.execute( self );
		}
	}
}
