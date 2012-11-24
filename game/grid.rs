extern mod lmath;
extern mod engine;


pub struct Grid	{
	mesh	: @engine::mesh::Mesh,
	program	: @engine::shade::Program,
	mut data: engine::shade::DataMap,
	rast	: engine::rast::State,
	nseg	: uint,
	texture	: @engine::texture::Texture,
	cells	: ~[engine::rast::Color],
}

impl Grid	{
	pure fn call( fbo : @engine::frame::Buffer, pmap : engine::call::PlaneMap,
			vao : @engine::buf::VertexArray )-> engine::call::Call	{
		engine::call::CallDraw( fbo, pmap, vao, self.mesh, self.mesh.get_range(),
			self.program, copy self.data, copy self.rast )
	}
	fn update_cells( tb : &engine::texture::Binding )	{
		tb.bind( self.texture );
		let fm_int = engine::texture::map_int_format( ~"rgba8" );
		let fm_pix = engine::texture::map_pix_format( ~"rgba" );
		let component = (self.cells[0].r as @engine::context::GLType).to_gl_type();
		tb.load_2D(	self.texture, 0u, fm_int, fm_pix, component, self.cells );
		tb.wrap(	self.texture, 0 );
		tb.filter(	self.texture, 1u );
	}
}


fn make_quad( ct : &engine::context::Context )-> engine::mesh::Mesh	{
	let vdata = ~[0i8,0i8,1i8,0i8,0i8,1i8,1i8,1i8];
	let count = 2u;
	let mut mesh = ct.create_mesh( ~"grid", ~"3s", vdata.len()/count, 0u );
	let vat = engine::mesh::make_attribute( ct, vdata, count, false );
	mesh.attribs.insert( ~"a_Vertex", vat );
	mesh
}

pub fn make_grid( ct : &engine::context::Context, segments : uint )-> Grid	{
	let mut data = engine::shade::create_data();
	let mut rast = engine::rast::create_rast(0,0);
	rast.prime.cull = true;
	rast.set_depth( ~"<=" );
	rast.set_blend( ~"s+d", ~"Sa", ~"1-Sa" );
	let cells = do vec::from_fn(segments*segments) |i|	{
		let col = if i%2u==0u
			{0x2040E040} else
			{0x20E02000};
		engine::rast::make_color(col)
	};
	let tex = @ct.create_texture( ~"2D", segments, segments, 0u, 0u );
	data.insert( ~"t_Grid", engine::shade::UniTexture(0,tex) );
	Grid{
		mesh	: @make_quad( ct ),
		program	: @engine::load::load_program( ct, ~"data/code-game/grid" ),
		data	: data,
		rast	: rast,
		nseg	: segments,
		texture	: tex,
		cells	: cells
	}
}
