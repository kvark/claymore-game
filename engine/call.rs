pub type PlaneMap = send_map::linear::LinearMap<~str,frame::Target>;

pub struct ClearData	{
	color	: Option<rast::Color>,
	depth	: Option<f32>,
	stencil	: Option<uint>,
}

pub struct Range	{
	start	: uint,
	num		: uint,
}


enum Call	{
	ClearCall( @frame::Buffer, PlaneMap, ClearData, rast::Scissor, rast::Mask ),
	BlitCall(),			//FIXME
	DrawCall( @frame::Buffer, PlaneMap, @buf::VertexArray, @mesh::Mesh, Range, @shade::Program, shade::DataMap, rast::State ),
	TransfromCall(),	//FIXME
}


pub struct Process	{
	mut queue	: ~[Call],
}

impl Process	{
	fn clear()	{}
	fn blit()	{}
	fn draw()	{}
	fn transform()	{}

	fn flush( ct : &context::Context )	{
		for (copy self.queue).each()	|call|	{
			match *call	{
				ClearCall(_fb,_pmap,_data,_scissor,_mask)	=> {},
				DrawCall(fb,pmap,va,mesh,_range,prog,data,rast)	=> {
					let mut attaches = vec::from_elem( pmap.len(), frame::TarEmpty );
					for pmap.each() |name,target|	{
						let loc = prog.find_output( name );
						attaches[loc] = *target;
					}
					let depth_stencil = match pmap.find(&~"")	{
						Some(t)	=> t,
						None	=> frame::TarEmpty,
					};
					ct.bind_frame_buffer( fb, true, depth_stencil, attaches );
					rast.activate( &mut ct.rast, mesh.get_poly_size() );
					ct.draw_mesh( mesh, va, prog, &data );
				},
				_	=> fail(~"Unsupported call!")
			}
		}
		self.queue = ~[];
	}
}
