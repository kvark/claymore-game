use core::hashmap::linear::LinearMap;

use numeric::Float;
use lmath::{vec,quat};
use cgmath::projection;
use engine;
use engine::{gr_low,space};

use scene::common;
use gen = gen_scene::common;



pub fn parse( scene : &gen::Scene, _gc : &mut gr_low::context::Context,
		_opt_vao : Option<@mut gr_low::buf::VertexArray>, _aspect : float,
		_lg : &engine::journal::Log )-> common::Scene	{
	fn parse_child( child : &gen::NodeChild, parent : Option<@mut space::Node>, ctx : &mut common::SceneContext )	{
		match child	{
			&gen::ChildNode(ref node)	=>	{
				let qs = space::QuatSpace	{
					position	: vec::vec3::from_array( node.space.pos ),
					orientation	: quat::quat::from_array( node.space.rot ),
					scale		: node.space.scale,
				};
				let n = @mut space::Node	{
					name	: copy node.name,
					space	: qs,
					parent	: parent,
					actions	: ~[],
				};
				ctx.nodes.insert( copy n.name, n );
				for node.children.each() |child|	{
					parse_child( child, Some(n), ctx );
				}
			},
			&gen::ChildEntity(ref _ent)	=> (),
			&gen::ChildCamera(ref cam)	=>	{
				let _c = @common::Camera	{
					node	: parent.expect("Camera parent has to exist"),
					proj	: projection::PerspectiveSym	{
						vfov	: cam.fov_y.degrees(),
						aspect	: 1f32,	//fixme
						near	: cam.range[0],
						far		: cam.range[1],
					},
					ear		: engine::audio::Listener{ volume:0f },
				};
			},
			&gen::ChildLight(ref _lit)	=> (),
		}
	}
	let mut context = common::SceneContext::new( ~"" );
	let mut entities : ~[engine::object::Entity] = ~[];
	for scene.nodes.each() |child|	{
		parse_child( child, None, &mut context );
	}
	common::Scene	{
		context		: context,
		entities	: common::EntityGroup(entities),
		cameras		: LinearMap::new(),
		lights		: LinearMap::new()
	}
}
