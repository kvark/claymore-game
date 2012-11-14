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

	fn flush()	{
		for self.queue.each()	|call|	{
			match *call	{
				ClearCall(_fb,_pm,_data,_scissor,_mask)	=> {},
				DrawCall(_fb,_pm,_va,_mesh,_range,_prog,_data,_rast)	=> {},
				_	=> fail(~"Unsupported call!")
			}
		}
		self.queue = ~[];
	}
}