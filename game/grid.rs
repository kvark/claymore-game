extern mod lmath;
extern mod engine;


pub struct Grid	{
	mesh	: @engine::mesh::Mesh,
	program	: @engine::shade::Program,
	mut data: engine::shade::DataMap,
	rast	: engine::rast::State,
}

impl Grid	{
	pure fn call( fbo : @engine::frame::Buffer, pmap : engine::call::PlaneMap,
			vao : @engine::buf::VertexArray )-> engine::call::Call	{
		engine::call::CallDraw( fbo, pmap, vao, self.mesh, self.mesh.get_range(),
			self.program, copy self.data, copy self.rast )
	}
}


fn make_quad( ct : &engine::context::Context )-> engine::mesh::Mesh	{
	let vdata = ~[-1i8,-1i8,1i8,-1i8,-1i8,1i8,1i8,1i8];
	let count = 2u;
	let mut mesh = ct.create_mesh( ~"grid", ~"3s", vdata.len()/count, 0u );
	let vat = engine::mesh::make_attribute( ct, vdata, count, false );
	mesh.attribs.insert( ~"a_Vertex", vat );
	mesh
}

pub fn make_grid( ct : &engine::context::Context )-> Grid	{
	let mut data = engine::shade::create_data();
	let mut rast = engine::rast::create_rast(0,0);
	rast.prime.cull = true;
	rast.set_depth( ~"<=" );
	rast.set_blend( ~"s+d", ~"Sa", ~"1-Sa" );
	Grid{
		mesh	: @make_quad( ct ),
		program	: @engine::load::load_program( ct, ~"data/code-game/grid" ),
		data	: data,
		rast	: rast
	}
}
