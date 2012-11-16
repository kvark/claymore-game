extern mod glcore;

pub type PlaneMap = send_map::linear::LinearMap<~str,frame::Target>;

pub struct ClearData	{
	color	: Option<rast::Color>,
	depth	: Option<f32>,
	stencil	: Option<uint>,
}


pub enum Call	{
	CallClear( @frame::Buffer, PlaneMap, ClearData, rast::Scissor, rast::Mask ),
	CallBlit(),			//FIXME
	CallDraw( @frame::Buffer, PlaneMap, @buf::VertexArray, @mesh::Mesh, mesh::Range, @shade::Program, shade::DataMap, rast::State ),
	CallTransfrom(),	//FIXME
}


impl context::Context	{
	fn flush( queue	: ~[Call] )	{
		for queue.each()	|call|	{
			match call	{
				&CallClear(fb,pmap,data,scissor,_mask)	=> {
					let mut depth_stencil = frame::TarEmpty;
					let mut colors : ~[frame::Target] = ~[];
					for pmap.each() |name,target|	{
						if *name == ~""	{
							assert depth_stencil == frame::TarEmpty;
							depth_stencil = *target;
						}else	{
							colors.push( *target );
						}
					}
					self.bind_frame_buffer( fb, true, depth_stencil, colors );
					self.rast.scissor.activate( &scissor, 0 );
					let mut flags = 0 as glcore::GLenum;
					//FIXME: cache this
					match data.color	{
						Some(c) =>	{
							flags |= glcore::GL_COLOR_BUFFER_BIT;
							glcore::glClearColor(
								c.r as glcore::GLfloat, c.g as glcore::GLfloat,
								c.b as glcore::GLfloat, c.a as glcore::GLfloat );
						},None	=>	{}
					}
					match data.depth	{
						Some(d) => 	{
							flags |= glcore::GL_DEPTH_BUFFER_BIT;
							glcore::glClearDepth( d as glcore::GLdouble );
						},None	=> 	{}
					}
					match data.stencil	{
						Some(s)	=>	{
							flags |= glcore::GL_STENCIL_BUFFER_BIT;
							glcore::glClearStencil( s as glcore::GLint );
						},None	=>	{}
					}
					glcore::glClear( flags );
				},
				&CallDraw(fb,pmap,va,mesh,range,prog,data,rast)	=> {
					let mut attaches = vec::from_elem( pmap.len(), frame::TarEmpty );
					let mut depth_stencil = frame::TarEmpty;
					for pmap.each() |name,target|	{
						if *name == ~""	{
							assert depth_stencil == frame::TarEmpty;
							depth_stencil = *target;
						}else	{
							let loc = prog.find_output( name );
							attaches[loc] = *target;
						}
					}
					self.bind_frame_buffer( fb, true, depth_stencil, attaches );
					self.rast.activate( &rast, mesh.get_poly_size() );
					self.draw_mesh( mesh, &range, va, prog, &data );
				},
				_	=> fail(~"Unsupported call!")
			}
		}
	}
}
