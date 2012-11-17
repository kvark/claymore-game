extern mod glcore;


pub struct PlaneMap	{
	depth_stencil	: frame::Target,
	colors			: send_map::linear::LinearMap<~str,frame::Target>,
}

pub fn create_plane_map( name : ~str, col : frame::Target )-> PlaneMap	{
	let mut pmap = PlaneMap	{
		depth_stencil : frame::TarEmpty,
		colors : send_map::linear::LinearMap::<~str,frame::Target>(),
	};
	pmap.colors.insert( name, col );
	pmap
}


pub struct ClearData	{
	color	: Option<rast::Color>,
	depth	: Option<f32>,
	stencil	: Option<uint>,
}


pub enum Call	{
	CallClear( @frame::Buffer, &PlaneMap, ClearData, rast::Scissor, rast::Mask ),
	CallBlit(),			//FIXME
	CallDraw( @frame::Buffer, &PlaneMap, @buf::VertexArray, @mesh::Mesh, mesh::Range, @shade::Program, shade::DataMap, rast::State ),
	CallTransfrom(),	//FIXME
}


impl context::Context	{
	fn flush( queue	: ~[Call] )	{
		for queue.each()	|call|	{
			match call	{
				&CallClear(fb,pmap,data,scissor,_mask)	=> {
					let mut colors : ~[frame::Target] = ~[];
					for pmap.colors.each_value() |target|	{
						colors.push( *target );
					}
					let has_color = colors.len()!=0 && (*fb.handle==0 || colors[0]!=frame::TarEmpty);
					self.bind_frame_buffer( fb, true, pmap.depth_stencil, colors );
					self.rast.scissor.activate( &scissor, 0 );
					let mut flags = 0 as glcore::GLenum;
					//FIXME: cache this
					match data.color	{
						Some(c) =>	{
							assert has_color;
							flags |= glcore::GL_COLOR_BUFFER_BIT;
							glcore::glClearColor(
								c.r as glcore::GLfloat, c.g as glcore::GLfloat,
								c.b as glcore::GLfloat, c.a as glcore::GLfloat );
						},None	=>	{}
					}
					match data.depth	{
						Some(d) => 	{
							assert *fb.handle==0 || pmap.depth_stencil!=frame::TarEmpty;
							flags |= glcore::GL_DEPTH_BUFFER_BIT;
							glcore::glClearDepth( d as glcore::GLdouble );
						},None	=> 	{}
					}
					match data.stencil	{
						Some(s)	=>	{
							assert *fb.handle==0 || pmap.depth_stencil!=frame::TarEmpty;
							flags |= glcore::GL_STENCIL_BUFFER_BIT;
							glcore::glClearStencil( s as glcore::GLint );
						},None	=>	{}
					}
					glcore::glClear( flags );
				},
				&CallDraw(fb,pmap,va,mesh,range,prog,data,rast)	=> {
					let mut attaches = vec::from_elem( pmap.colors.len(), frame::TarEmpty );
					for pmap.colors.each() |name,target|	{
						let loc = prog.find_output( name );
						attaches[loc] = *target;
					}
					if rast.depth.test || rast.stencil.test	{
						assert *fb.handle==0 || pmap.depth_stencil!=frame::TarEmpty;
					}
					self.bind_frame_buffer( fb, true, pmap.depth_stencil, attaches );
					self.rast.activate( &rast, mesh.get_poly_size() );
					self.draw_mesh( mesh, &range, va, prog, &data );
				},
				_	=> fail(~"Unsupported call!")
			}
		}
	}
}
