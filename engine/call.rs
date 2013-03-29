extern mod glcore;


pub struct PlaneMap	{
	stencil	: frame::Target,
	depth	: frame::Target,
	colors	: send_map::linear::LinearMap<~str,frame::Target>,
}

impl PlaneMap : Copy	{}
pub fn make_pmap_empty()-> PlaneMap	{
	PlaneMap	{
		stencil	: frame::TarEmpty,
		depth	: frame::TarEmpty,
		colors	: send_map::linear::LinearMap::<~str,frame::Target>(),
	}
}
pub fn make_pmap_simple( name : ~str, col : frame::Target )-> PlaneMap	{
	let mut pm = make_pmap_empty();
	pm.colors.insert( name, col );
	pm
}


pub struct ClearData	{
	color	: Option<rast::Color>,
	depth	: Option<float>,
	stencil	: Option<uint>,
}

pub type DrawInput = (@buf::VertexArray, @mesh::Mesh, mesh::Range);
pub type DrawOutput = (@frame::Buffer, PlaneMap, rast::State);

pub enum Call	{
	CallEmpty,
	CallClear( @frame::Buffer, PlaneMap, ClearData, rast::Scissor, rast::Mask ),
	CallBlit(),			//FIXME
	CallDraw( DrawInput, DrawOutput, @shade::Program, shade::DataMap ),
	CallTransfrom(),	//FIXME
}

impl ClearData	{
	pub fn genCall( output : DrawOutput )-> Call	{
		let (fbo,pmap,rast) = output;
		CallClear( fbo, pmap, self, rast.scissor, rast.mask )
	}
}


impl context::Context	{
	fn flush( queue	: ~[Call] )	{
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
							assert has_color;
							flags |= glcore::GL_COLOR_BUFFER_BIT;
							self.set_clear_color( &c );
						},None	=>	{}
					}
					match data.depth	{
						Some(d) => 	{
							assert *fb.handle==0 || pmap.depth!=frame::TarEmpty;
							flags |= glcore::GL_DEPTH_BUFFER_BIT;
							self.set_clear_depth( d );
						},None	=> 	{}
					}
					match data.stencil	{
						Some(s)	=>	{
							assert *fb.handle==0 || pmap.stencil!=frame::TarEmpty;
							flags |= glcore::GL_STENCIL_BUFFER_BIT;
							self.set_clear_stencil( s );
						},None	=>	{}
					}
					glcore::glClear( flags );
				},
				CallDraw(input,output,prog,data)	=> {
					let &(fb,pmap,rast) = &output;
					let mut attaches = vec::from_elem( pmap.colors.len(), frame::TarEmpty );
					for pmap.colors.each() |name,target|	{
						let loc = prog.find_output( name );
						attaches[loc] = *target;
					}
					// check & activate raster
					let rect = if *fb.handle != 0	{
						assert !rast.stencil.test	|| pmap.stencil	!=frame::TarEmpty;
						assert !rast.depth.test		|| pmap.depth	!=frame::TarEmpty;
						assert !rast.blend.on		|| attaches[0]	!= frame::TarEmpty;
						let (wid,het,_sam) = fb.check_size();
						frame::make_rect(wid,het)
					}else	{
						*self.default_rast.view
					};
					let (_,mesh,_) = input;
					self.rast.activate( &rast, mesh.get_poly_size() );
					assert *self.rast.view == rect;
					// draw
					self.bind_frame_buffer( fb, true, pmap.stencil, pmap.depth, attaches );
					self.draw_mesh( input, prog, &data );
				},
				_	=> fail ~"Unsupported call!"
			}
		}
	}
}
