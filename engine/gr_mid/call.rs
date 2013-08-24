extern mod glcore;

use core::hashmap::linear::LinearMap;

use gr_low::{buf,context,frame,rast,shade};
use gr_low::rast::Stage;
use gr_mid::mesh;


pub struct PlaneMap	{
	stencil	: frame::Target,
	depth	: frame::Target,
	colors	: LinearMap<~str,frame::Target>,
}

impl Copy for PlaneMap	{}

pub impl PlaneMap	{
	fn new_empty()-> PlaneMap	{
		PlaneMap	{
			stencil	: frame::TarEmpty,
			depth	: frame::TarEmpty,
			colors	: LinearMap::new(),
		}
	}
	fn new_simple( name : ~str, col : frame::Target )-> PlaneMap	{
		let mut pm = PlaneMap::new_empty();
		pm.colors.insert( name, col );
		pm
	}
	fn check( &self, rast : &rast::State )	{
		assert!( !rast.stencil.test	|| self.stencil	!= frame::TarEmpty );
		assert!( !rast.depth.test	|| self.depth	!= frame::TarEmpty );
		assert!( !rast.blend.on		|| !self.colors.is_empty() );
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

pub impl Output	{
	fn new( fb : @mut frame::Buffer, pmap : PlaneMap )-> Output	{
		Output	{
			fb	: fb,
			pmap: pmap,
			area: frame::Rect::new(0x10000,0x10000),
		}
	}
	fn gen_scissor( &self )-> rast::Scissor	{
		rast::Scissor	{
			test: self.area.w>0u || self.area.h>0u,
			area: copy self.area,
		}
	}
}


pub enum Call	{
	CallEmpty,
	CallClear( Output, ClearData, rast::Mask ),
	CallBlit( Output, Output ),
	CallDraw( Input, Output, rast::State, @shade::Program, shade::DataMap ),
	CallTransfrom(),	//TODO
}


pub impl context::Context	{
	fn flush( &mut self, queue	: ~[Call] )	{
		for vec::each_const(queue)	|&call|	{
			match call	{
				CallEmpty => {},
				CallClear(out,data,mask)	=> {
					let mut colors : ~[frame::Target] = ~[];
					for out.pmap.colors.each_value() |target|	{
						colors.push( *target );
					}
					let has_color = colors.len()!=0 && (*out.fb.handle==0 || colors[0]!=frame::TarEmpty);
					self.bind_frame_buffer( out.fb, true, out.pmap.stencil, out.pmap.depth, colors );
					self.rast.scissor.activate( &out.gen_scissor(), 0 );
					self.rast.mask.activate( &mask, 0 );
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
				CallBlit(src,dst)	=>	{
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
					let (wid1,het1,_dep1,sam1) = src.fb.check_size();
					let (wid2,het2,_dep2,sam2) = dst.fb.check_size();
					assert!( sam1 == sam2 || (sam1*sam2==0 && only_color) );
					let filter = if (only_color && sam1==0) {glcore::GL_LINEAR} else {glcore::GL_NEAREST};
					// call blit
					glcore::glBlitFramebuffer(
						0, 0, wid1 as glcore::GLint, het1 as glcore::GLint,
						0, 0, wid2 as glcore::GLint, het2 as glcore::GLint,
						flags, filter );
				},
				CallDraw(in,out,rast,prog,data)	=> {
					// bind FBO
					let mut attaches = vec::from_elem( out.pmap.colors.len(), frame::TarEmpty );
					for out.pmap.colors.each() |&(name,target)|	{
						let loc = prog.find_output( name );
						assert!( loc < attaches.len() && attaches[loc] == frame::TarEmpty );
						attaches[loc] = *target;
					}
					self.bind_frame_buffer( out.fb, true, out.pmap.stencil, out.pmap.depth, attaches );
					// check & activate raster
					let rect = if *out.fb.handle != 0	{
						out.pmap.check( &rast );
						let (wid,het,_dep,_sam) = out.fb.check_size();
						frame::Rect::new(wid,het)
					}else	{
						*self.default_rast.view
					};
					let mut r2 = rast;
					r2.scissor = out.gen_scissor();
					self.rast.activate( &r2, in.mesh.get_poly_size() );
					assert!( *self.rast.view == rect );
					// draw
					self.draw_mesh( in, prog, &data );
				},
				_	=> fail!(~"Unsupported call!")
			}
		}
	}
}
