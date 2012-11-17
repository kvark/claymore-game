pub trait Mod	{
	pure fn get_name()-> ~str;
	pure fn get_code()-> ~str;
}


pub struct Material	{
	name			: ~str,
	metas			: ~[~str],
	code_vertex		: ~str,
	code_framgment	: ~str,
}


pub struct Entity	{
	node	: @space::Node,
	//body	: @node::Body,
	vao		: @buf::VertexArray,
	mesh	: @mesh::Mesh,
	range	: mesh::Range,
	mods	: ~[@Mod],
	material: @Material,
}


pub struct Technique	{
	fbo		: @frame::Buffer,
	pmap	: call::PlaneMap,
	rast	: rast::State,
	priv mut effect_map	: send_map::linear::LinearMap<uint,shade::Handle>,
}

impl Technique	{
	fn process( e : &Entity, ct : &context::Context )-> call::Call	{
		let mut data = shade::create_data();
		let prog = @ct.create_program( ~[] );
		//self.effect_map.insert( 0u, prog.handle );
		call::CallDraw( self.fbo, copy self.pmap,
			e.vao, e.mesh, e.range, prog, data, self.rast )
	}
}