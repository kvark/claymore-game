use core::hashmap::linear::LinearMap;

use engine;
use engine::{gr_low,gr_mid,space};

use scene::common;
use gen = gen_scene::common;



pub fn parse( scene : &gen::Scene, gc : &mut gr_low::context::Context,
		opt_vao : Option<@mut gr_low::buf::VertexArray>, aspect : float,
		lg : &engine::journal::Log )-> common::Scene	{
	let mut ctx = common::SceneContext::new( ~"" );
	let mut entities : ~[engine::object::Entity] = ~[];
	fn parse_child( child : &gen::NodeChild )	{
		match child	{
			&gen::ChildNode(ref node)	=> (),
			&gen::ChildEntity(ref ent)	=> (),
			&gen::ChildCamera(ref cam)	=> (),
			&gen::ChildLight(ref lit)	=> (),
		}
	}
	for scene.nodes.each() |child|	{
		parse_child( child );
	}
	common::Scene	{
		context		: ctx,
		entities	: common::EntityGroup(entities),
		cameras		: LinearMap::new(),
		lights		: LinearMap::new()
	}
}
