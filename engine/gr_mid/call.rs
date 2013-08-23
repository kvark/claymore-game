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
}


pub struct ClearData	{
	color	: Option<rast::Color>,
	depth	: Option<float>,
	stencil	: Option<uint>,
}

pub type DrawInput = (@mut buf::VertexArray, @mesh::Mesh, mesh::Range);
pub type DrawOutput = (@mut frame::Buffer, PlaneMap, rast::State);

pub enum Call	{
	CallEmpty,
	CallClear( @mut frame::Buffer, PlaneMap, ClearData, rast::Scissor, rast::Mask ),
	CallBlit( @mut frame::Buffer, PlaneMap, @mut frame::Buffer, PlaneMap, rast::Scissor ),
	CallDraw( DrawInput, DrawOutput, @shade::Program, shade::DataMap ),
	CallTransfrom(),	//FIXME
}

pub impl ClearData	{
	fn gen_call( &self, output : DrawOutput )-> Call	{
		let (fbo,pmap,rast) = output;
		CallClear( fbo, pmap, copy *self, rast.scissor, rast.mask )
	}
}


pub impl context::Context	{
	fn flush( &mut self, queue	: ~[Call] )	{
		for vec::each_const(queue)	|&call|	{
			match call	{
				CallEmpty => {},
				CallClear(fb,pmap,data,scissor,mask)	=> {
					let mut colors : ~[frame::Target] = ~[];
					for pmap.colors.each_value() |target|	{
						colors.push( *target );
					}
					let has_color = colors.len()!=0 && (*fb.handle==0 || colors[0]!=frame::TarEmpty);
					self.bind_frame_buffer( fb, true, pmap.stencil, pmap.depth, colors );
					self.rast.scissor.activate( &scissor, 0 );
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
							assert!( *fb.handle==0 || pmap.depth!=frame::TarEmpty );
							flags |= glcore::GL_DEPTH_BUFFER_BIT;
							self.set_clear_depth( d );
						},None	=> 	{}
					}
					match data.stencil	{
						Some(s)	=>	{
							assert!( *fb.handle==0 || pmap.stencil!=frame::TarEmpty );
							flags |= glcore::GL_STENCIL_BUFFER_BIT;
							self.set_clear_stencil( s );
						},None	=>	{}
					}
					glcore::glClear( flags );
				},
				CallBlit(f1,pm1,f2,pm2,scissor)	=>	{
					assert!( *f1.handle != *f2.handle );
					// bind frame buffers
					let mut colors : ~[frame::Target] = ~[];
					for pm1.colors.each_value() |target|	{
						colors.push( *target );
					}
					self.bind_frame_buffer( f1, false, pm1.stencil, pm1.depth, colors );
					colors = ~[];
					for pm2.colors.each_value() |target|	{
						colors.push( *target );
					}
					self.bind_frame_buffer( f2, true, pm2.stencil, pm2.depth, colors );
					// set state
					self.rast.scissor.activate( &scissor, 0 );
					let mut flags = 0 as glcore::GLenum;
					let mut only_color = true;
					if !pm1.colors.is_empty() || !pm2.colors.is_empty()	{
						flags |= glcore::GL_COLOR_BUFFER_BIT;
					}
					if pm1.depth != frame::TarEmpty || pm2.depth != frame::TarEmpty	{
						flags |= glcore::GL_DEPTH_BUFFER_BIT;
						only_color = false;
					}
					if pm1.stencil != frame::TarEmpty || pm2.stencil != frame::TarEmpty	{
						flags |= glcore::GL_STENCIL_BUFFER_BIT;
						only_color = false;
					}
					// prepare
					let (wid1,het1,_dep1,sam1) = f1.check_size();
					let (wid2,het2,_dep2,sam2) = f2.check_size();
					assert!( sam1 == sam2 || (sam1*sam2==0 && only_color) );
					let filter = if (only_color && sam1==0) {glcore::GL_LINEAR} else {glcore::GL_NEAREST};
					// call blit
					glcore::glBlitFramebuffer(
						0, 0, wid1 as glcore::GLint, het1 as glcore::GLint,
						0, 0, wid2 as glcore::GLint, het2 as glcore::GLint,
						flags, filter );
				},
				CallDraw(input,output,prog,data)	=> {
					let &(fb,pmap,rast) = &output;
					// bind FBO
					let mut attaches = vec::from_elem( pmap.colors.len(), frame::TarEmpty );
					for pmap.colors.each() |&(name,target)|	{
						let loc = prog.find_output( name );
						assert!( loc < attaches.len() && attaches[loc] == frame::TarEmpty );
						attaches[loc] = *target;
					}
					self.bind_frame_buffer( fb, true, pmap.stencil, pmap.depth, attaches );
					// check & activate raster
					let rect = if *fb.handle != 0	{
						assert!( !rast.stencil.test	|| pmap.stencil	!= frame::TarEmpty );
						assert!( !rast.depth.test	|| pmap.depth	!= frame::TarEmpty );
						assert!( !rast.blend.on		|| !pmap.colors.is_empty() );
						let (wid,het,_dep,_sam) = fb.check_size();
						frame::Rect::new(wid,het)
					}else	{
						*self.default_rast.view
					};
					let (_,mesh,_) = input;
					self.rast.activate( &rast, mesh.get_poly_size() );
					assert!( *self.rast.view == rect );
					// draw
					self.draw_mesh( input, prog, &data );
				},
				_	=> fail!(~"Unsupported call!")
			}
		}
	}
}
