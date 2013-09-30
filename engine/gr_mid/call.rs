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

	pub fn new_simple( name : ~str, col : frame::Target )-> PlaneMap	{
		let mut pm = PlaneMap::new_empty();
		pm.colors.insert( name, col );
		pm
	}

	pub fn new_main( gc : &context::Context, name : ~str )-> PlaneMap	{
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

	pub fn check( &self, rast : &rast::State )	{
		assert!( !rast.stencil.test	|| self.stencil	!= frame::TarEmpty );
		assert!( !rast.depth.test	|| self.depth	!= frame::TarEmpty );
		assert!( !rast.blend.on		|| !self.colors.is_empty() );
	}

	pub fn log( &self, lg : &journal::Log )	{
		if self.stencil != frame::TarEmpty	{
			lg.add(fmt!( "\t\tstencil\t= %s", self.stencil.to_str() ));
		}
		if self.depth != frame::TarEmpty	{
			lg.add(fmt!( "\t\tdepth\t= %s", self.depth.to_str() ));	
		}
		for (name,val) in self.colors.iter()	{
			lg.add(fmt!( "\t\t%s\t= %s", *name, val.to_str() ));
		}
	}
}


#[deriving(Clone)]
pub struct ClearData	{
	color	: Option<rast::Color>,
	depth	: Option<float>,
	stencil	: Option<uint>,
}

#[deriving(Clone)]
pub struct Input	{
	va		: @mut buf::VertexArray,
	mesh	: @mesh::Mesh,
	range	: mesh::Range,
}

impl Input	{
	pub fn new( va : @mut buf::VertexArray, m : @mesh::Mesh )-> Input	{
		Input	{
			va		: va,
			mesh	: m,
			range	: m.get_range(),
		}
	}
	pub fn log( &self, lg : &journal::Log )	{
		lg.add(fmt!( "\tMesh '%s' at VAO=%i with range [%u:%u]",
			self.mesh.name, *self.va.handle as int,
			self.range.start, self.range.start+self.range.num ));
	}
}


#[deriving(Clone)]
pub struct Output	{
	fb		: @mut frame::Buffer,
	pmap	: PlaneMap,
	area	: frame::Rect,
}

impl Output	{
	pub fn new( fb : @mut frame::Buffer, pmap : PlaneMap )-> Output	{
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
	pub fn log( &self, kind : &str, lg : &journal::Log )	{
		lg.add(fmt!( "\t%s FBO=%i with area %s", kind, *self.fb.handle as int, self.area.to_str() ));
		self.pmap.log( lg );
	}
}

#[deriving(Clone)]
pub enum Call	{
	// naming convention: What-Where-How
	CallEmpty,
	CallClear( ClearData, Output, rast::Mask ),
	CallBlit( Output, Output ),
	CallDraw( Input, Output, rast::State, @shade::Program, shade::DataMap ),
	CallTransfrom(),	//TODO
}

impl Call	{
	pub fn log( &self, lg : &journal::Log )	{
		match self	{
			&CallEmpty	=> lg.add(~"Call empty"),
			&CallClear(ref cd, ref out, ref _mask)	=>	{
				lg.add(~"Call clear");
				lg.add(fmt!( "\tValue %s %s %s",
					match cd.color	{
						Some(v)	=> fmt!("color(%f,%f,%f,%f)",
							v.r as float, v.g as float, v.b as float, v.a as float),
						None	=> ~"",
					},
					match cd.depth	{
						Some(v)	=> fmt!("depth(%f)", v),
						None	=> ~"",
					},
					match cd.stencil	{
						Some(v)	=> fmt!("stencil(%u)", v),
						None	=> ~"",
					}) );
				out.log( "Output", lg );
			}
			&CallBlit(ref src, ref dst)	=>	{
				lg.add(~"Call blit");
				src.log( "Src", lg );
				dst.log( "Dst", lg );
			},
			&CallDraw(ref inp, ref out, ref _rast, prog, ref data )	=>	{
				lg.add(~"Call draw");
				inp.log( lg );
				out.log( "Output", lg );
				lg.add(fmt!( "\tProgram=%i", *prog.handle as int ));
				data.log( lg );
			},
			&CallTransfrom()	=>	{
				lg.add(~"Call transform");
			},
		}
	}

	pub fn execute( &self, gc : &mut context::Context )	{
		match self	{
			&CallEmpty => {},
			&CallClear(ref cdata, ref out, ref mask)	=> {
				let mut colors : ~[frame::Target] = ~[];
				for (_,&target) in out.pmap.colors.iter()	{
					colors.push( target );
				}
				let has_color = colors.len()!=0 && (*out.fb.handle==0 || colors[0]!=frame::TarEmpty);
				gc.bind_frame_buffer( out.fb, true, out.pmap.stencil, out.pmap.depth, colors );
				gc.rast.scissor.activate( &out.gen_scissor(), 0 );
				gc.rast.mask.activate( mask, 0 );
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
						assert!( *out.fb.handle==0 || out.pmap.depth!=frame::TarEmpty );
						flags |= gl::DEPTH_BUFFER_BIT;
						gc.set_clear_depth( d );
					},
					None	=> 	()
				}
				match cdata.stencil	{
					Some(s)	=>	{
						assert!( *out.fb.handle==0 || out.pmap.stencil!=frame::TarEmpty );
						flags |= gl::STENCIL_BUFFER_BIT;
						gc.set_clear_stencil( s );
					},
					None	=>	()
				}
				gl::Clear( flags );
			},
			&CallBlit(ref src, ref dst)	=>	{
				assert!( *src.fb.handle != *dst.fb.handle );
				// bind frame buffers
				gc.bind_frame_buffer( src.fb, false, src.pmap.stencil, src.pmap.depth,
					src.pmap.colors.iter().map(|(_,&v)| v).collect() );
				gc.bind_frame_buffer( dst.fb, true, dst.pmap.stencil, dst.pmap.depth,
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
				let sizeA = src.fb.check_size();
				let sizeB = dst.fb.check_size();
				assert!( sizeA[3] == sizeB[3] || (sizeA[3]*sizeB[3]==0 && only_color) );
				let filter = if (only_color && sizeA[3]==0) {gl::LINEAR} else {gl::NEAREST};
				// call blit
				gl::BlitFramebuffer(
					0, 0, sizeA[0] as gl::types::GLint, sizeA[1] as gl::types::GLint,
					0, 0, sizeB[0] as gl::types::GLint, sizeB[1] as gl::types::GLint,
					flags, filter );
			},
			&CallDraw(ref inp, ref out, ref rast, prog, ref data)	=> {
				// bind FBO
				let mut attaches = std::vec::from_elem( out.pmap.colors.len(), frame::TarEmpty );
				for (name,target) in out.pmap.colors.iter()	{
					let loc = prog.find_output( name );
					assert!( loc < attaches.len() && attaches[loc] == frame::TarEmpty );
					attaches[loc] = *target;
				}
				gc.bind_frame_buffer( out.fb, true, out.pmap.stencil, out.pmap.depth, attaches );
				// check & activate raster
				let mut r2 = *rast;
				r2.scissor = out.gen_scissor();
				gc.rast.activate( &r2, inp.mesh.get_poly_size() );
				//assert_eq!( out.area, *gc.rast.view );
				// draw
				gc.draw_mesh( *inp, prog, data );
			},
			_	=> fail!(~"Unsupported call!")
		}
	}
}


impl context::Context	{
	pub fn flush( &mut self, queue	: &[Call], lg : &journal::Log )	{
		self.call_count += queue.len();
		for call in queue.iter()	{
			if lg.enable	{
				call.log( lg );
			}
			call.execute( self );
		}
	}
}
