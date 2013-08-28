use core::hashmap::linear::LinearMap;

use numeric::Float;
use lmath::{vec,quat};
use cgmath::projection;
use engine;
use engine::{gr_low,gr_mid,space};

use scene::common;
use gen = gen_scene::common;


priv fn parse_shader_data( mat : &gen::Material, tex_cache : &LinearMap<~str,@gr_low::texture::Texture> )-> gr_low::shade::DataMap	{
	let mut out = gr_low::shade::make_data();
	fn color2vec(v : [f32, ..3])-> vec::vec4	{
		let kf = 1f32 / 255f32;
		vec::vec4::new( v[0] as f32 * kf, v[1] as f32 * kf, v[2] as f32 * kf, 1f32 )
	}
	fn color2vecU(c : uint)-> vec::vec4	{
		let kf = 1f32 / 255f32;
		vec::vec4::new( (c>>24) as f32 * kf, ((c>>16)&0xFF) as f32 * kf, ((c>>8)&0xFF) as f32 * kf, (c&0xFF) as f32 * kf )
	}
	for mat.data.each() |&(name,di)|	{
		let uni = match copy di	{
			gen::DataInt(v)		=> ( gr_low::shade::UniInt(v) ),
			gen::DataScalar(v)	=> ( gr_low::shade::UniFloat(v as float) ),
			gen::DataVector(v)	=> ( gr_low::shade::UniFloatVec(vec::vec4::from_array(v)) ),
			gen::DataColor(v)	=> ( gr_low::shade::UniFloatVec(color2vec(v)) ),
		};
		out.insert( ~"u_" + name, uni );
	}
	for mat.textures.eachi() |i,ti|	{
		let tex = *tex_cache.get( &ti.path );
		let s_opt = Some(gr_low::texture::Sampler::new( ti.filter, ti.wrap ));
		out.insert( ~"t_"+ti.name, gr_low::shade::UniTexture(0,tex,s_opt) );
		let u_transform = vec::vec4::new( ti.scale[0], ti.scale[1], ti.offset[0], ti.offset[1] );
		out.insert( fmt!("u_Tex%uTransform",i), gr_low::shade::UniFloatVec(u_transform) );
	}
	out
}


priv fn parse_child( child : &gen::NodeChild, parent : Option<@mut space::Node>, ctx : &mut common::SceneContext )	{
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
				parse_child( child, Some(n),ctx );
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


pub fn parse( scene : &gen::Scene, gc : &mut gr_low::context::Context,
		_opt_vao : Option<@mut gr_low::buf::VertexArray>, lg : &engine::journal::Log )-> common::Scene	{
	let mut context = common::SceneContext::new( ~"" );
	for scene.materials.each() |imat|	{
		let mat_source = match imat.kind	{
			gen::KindFlat			=> ~"flat",
			gen::KindPhong			=> ~"phong",
			gen::KindAnisotropic	=> ~"aniso"
		};
		let mat = @gr_mid::draw::load_material( ~"data/code/mat/" + mat_source );
		if !context.materials.contains_key( &imat.name )	{
			lg.add( ~"\tStandard material: " + imat.name );
			context.materials.insert( copy imat.name, mat );
		}
		for imat.textures.each() |itex|	{
			if !context.textures.contains_key( &itex.path )	{
				let path = ~"data/texture/" + itex.path;
				let tex = match context.textures.find(&path)	{
					Some(t) => *t,
					None	=> engine::load::load_texture_2D( gc, &path, true ),
				};
				context.textures.insert( copy itex.path, tex );
			}
		}
		let data = parse_shader_data( imat, &context.textures );
		context.mat_data.insert( copy imat.name, data );
	}
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
