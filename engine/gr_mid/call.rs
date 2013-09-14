extern mod glcore;

use core::hashmap::linear::LinearMap;

use journal;
use gr_low::{buf,context,frame,rast,shade};
use gr_low::rast::Stage;
use gr_mid::mesh;


pub struct PlaneMap	{
	stencil	: frame::Target,
	depth	: frame::Target,
	colors	: LinearMap<~str,frame::Target>,
}

impl Copy for PlaneMap	{}

impl PlaneMap	{
	pub fn new_empty()-> PlaneMap	{
		PlaneMap	{
			stencil	: frame::TarEmpty,
			depth	: frame::TarEmpty,
			colors	: LinearMap::new(),
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

	priv fn get_any_target( &self )-> Option<frame::Target>	{
		if self.stencil != frame::TarEmpty	{
			Some(self.stencil)
		}else
		if self.depth != frame::TarEmpty	{
			Some(self.depth)
		}else	{
			for self.colors.each_value |&val|	{
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
			lg.add(fmt!( "\t\t_stencil\t= %s", self.stencil.to_str() ));
		}
		if self.depth != frame::TarEmpty	{
			lg.add(fmt!( "\t\t_depth\t= %s", self.depth.to_str() ));	
		}
		for self.colors.each	|&(name,val)|	{
			lg.add(fmt!( "\t\t%s\t= %s", *name, val.to_str() ));
		}
	}
}


pub struct ClearData	{
	color	: Option<rast::Color>,
	depth	: Option<float>,
	stencil	: Option<uint>,
}

pub struct Input	{
	va		: @mut buf::VertexArray,
	mesh	: @mesh::Mesh,
	range	: mesh::Range,
}

pub impl Input	{
	fn new( va : @mut buf::VertexArray, m : @mesh::Mesh )-> Input	{
		Input	{
			va		: va,
			mesh	: m,
			range	: m.get_range(),
		}
	}
}


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
			area: copy self.area,
		}
	}
	pub fn log( &self, lg : &journal::Log )	{
		lg.add(fmt!( "Output to fb %d with area %s", *self.fb.handle as int, self.area.to_str() ));
	}
}


pub enum Call	{
	CallEmpty,
	CallClear( Output, ClearData, rast::Mask ),
	CallBlit( Output, Output ),
	CallDraw( Input, Output, rast::State, @shade::Program, shade::DataMap ),
	CallTransfrom(),	//TODO
}

impl Call	{
	pub fn log( &self, lg : &journal::Log )	{
		match self	{
			&CallEmpty	=> lg.add(~"Call empty"),
			&CallClear(ref out, ref data, ref mask)	=>	{
				lg.add(~"Call clear");
				out.log( lg );
			}
			_	=> (),
		}
	}
}


impl context::Context	{
	pub fn flush( &mut self, queue	: &[Call] )	{
		self.call_count += queue.len();
		for queue.each()	|call|	{
			match call	{
				&CallEmpty => {},
				&CallClear(ref out, ref data, ref mask)	=> {
					let mut colors : ~[frame::Target] = ~[];
					for out.pmap.colors.each_value() |target|	{
						colors.push( *target );
					}
					let has_color = colors.len()!=0 && (*out.fb.handle==0 || colors[0]!=frame::TarEmpty);
					self.bind_frame_buffer( out.fb, true, out.pmap.stencil, out.pmap.depth, colors );
					self.rast.scissor.activate( &out.gen_scissor(), 0 );
					self.rast.mask.activate( mask, 0 );
					let mut flags = 0 as glcore::GLenum;
					//FIXME: cache this
					match data.color	{
						Some(c) =>	{
							assert!( has_color );
							flags |= glcore::GL_COLOR_BUFFER_BIT;
							self.set_clear_color( &c );
						},None	=>	{}
					}
					match data.depth	{
						Some(d) => 	{
							assert!( *out.fb.handle==0 || out.pmap.depth!=frame::TarEmpty );
							flags |= glcore::GL_DEPTH_BUFFER_BIT;
							self.set_clear_depth( d );
						},None	=> 	{}
					}
					match data.stencil	{
						Some(s)	=>	{
							assert!( *out.fb.handle==0 || out.pmap.stencil!=frame::TarEmpty );
							flags |= glcore::GL_STENCIL_BUFFER_BIT;
							self.set_clear_stencil( s );
						},None	=>	{}
					}
					glcore::glClear( flags );
				},
				&CallBlit(ref src, ref dst)	=>	{
					assert!( *src.fb.handle != *dst.fb.handle );
					// bind frame buffers
					let mut colors : ~[frame::Target] = ~[];
					for src.pmap.colors.each_value() |target|	{
						colors.push( *target );
					}
					self.bind_frame_buffer( src.fb, false, src.pmap.stencil, src.pmap.depth, colors );
					colors = ~[];
					for dst.pmap.colors.each_value() |target|	{
						colors.push( *target );
					}
					self.bind_frame_buffer( dst.fb, true, dst.pmap.stencil, dst.pmap.depth, colors );
					// set state
					self.rast.scissor.activate( &dst.gen_scissor(), 0 );
					let mut flags = 0 as glcore::GLenum;
					let mut only_color = true;
					if !src.pmap.colors.is_empty() || !dst.pmap.colors.is_empty()	{
						flags |= glcore::GL_COLOR_BUFFER_BIT;
					}
					if src.pmap.depth != frame::TarEmpty || dst.pmap.depth != frame::TarEmpty	{
						flags |= glcore::GL_DEPTH_BUFFER_BIT;
						only_color = false;
					}
					if src.pmap.stencil != frame::TarEmpty || dst.pmap.stencil != frame::TarEmpty	{
						flags |= glcore::GL_STENCIL_BUFFER_BIT;
						only_color = false;
					}
					// prepare
					let sizeA = src.fb.check_size();
					let sizeB = dst.fb.check_size();
					assert!( sizeA[3] == sizeB[3] || (sizeA[3]*sizeB[3]==0 && only_color) );
					let filter = if (only_color && sizeA[3]==0) {glcore::GL_LINEAR} else {glcore::GL_NEAREST};
					// call blit
					glcore::glBlitFramebuffer(
						0, 0, sizeA[0] as glcore::GLint, sizeA[1] as glcore::GLint,
						0, 0, sizeB[0] as glcore::GLint, sizeB[1] as glcore::GLint,
						flags, filter );
				},
				&CallDraw(ref in, ref out, ref rast, ref prog, ref data)	=> {
					// bind FBO
					let mut attaches = vec::from_elem( out.pmap.colors.len(), frame::TarEmpty );
					for out.pmap.colors.each() |&(name,target)|	{
						let loc = prog.find_output( name );
						assert!( loc < attaches.len() && attaches[loc] == frame::TarEmpty );
						attaches[loc] = *target;
					}
					self.bind_frame_buffer( out.fb, true, out.pmap.stencil, out.pmap.depth, attaches );
					// check & activate raster
					let mut r2 = *rast;
					r2.scissor = out.gen_scissor();
					self.rast.activate( &r2, in.mesh.get_poly_size() );
					//assert_eq!( out.area, *self.rast.view );
					// draw
					self.draw_mesh( *in, *prog, data );
				},
				_	=> fail!(~"Unsupported call!")
			}
		}
	}
}
