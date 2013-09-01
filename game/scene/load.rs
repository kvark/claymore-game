use core::hashmap::linear::LinearMap;

use numeric::Float;
use lmath::{vec,quat};
use cgmath::projection;
use engine;
use engine::{gr_low,gr_mid,space};

use scene::common;
use gen = gen_scene::common;


priv fn parse_shader_data( imat : &gen::Material, tex_cache : &LinearMap<~str,@gr_low::texture::Texture>,
		lg : &engine::journal::Log )-> gr_low::shade::DataMap	{
	let mut out = gr_low::shade::make_data();
	fn color2vec(v : [f32, ..3])-> vec::vec4	{
		let kf = 1f32 / 255f32;
		vec::vec4::new( v[0] as f32 * kf, v[1] as f32 * kf, v[2] as f32 * kf, 1f32 )
	}
	fn color2vecU(c : uint)-> vec::vec4	{
		let kf = 1f32 / 255f32;
		vec::vec4::new( (c>>24) as f32 * kf, ((c>>16)&0xFF) as f32 * kf, ((c>>8)&0xFF) as f32 * kf, (c&0xFF) as f32 * kf )
	}
	for imat.data.each() |&(name,di)|	{
		let uni = match copy di	{
			gen::DataInt(v)		=> ( gr_low::shade::UniInt(v) ),
			gen::DataScalar(v)	=> ( gr_low::shade::UniFloat(v as float) ),
			gen::DataVector(v)	=> ( gr_low::shade::UniFloatVec(vec::vec4::from_array(v)) ),
			gen::DataColor(v)	=> ( gr_low::shade::UniFloatVec(color2vec(v)) ),
		};
		out.insert( ~"u_" + name, uni );
	}
	let phong_texture = ~"Main";
	for imat.textures.eachi() |i,ti|	{
		//print(fmt!( "\tLooking for texture %s\n", ti.path ));
		let tex = *tex_cache.get( &ti.path );
		let s_opt = Some(gr_low::texture::Sampler::new( ti.filter, ti.wrap ));
		let mut name = copy ti.name;
		if name != phong_texture && i==0 && imat.shader == ~"phong"	{
			name = copy phong_texture; //that's what the shader expects
			lg.add(fmt!( "(w) forcing texture '%s' name to %s", ti.name, phong_texture ));
		}
		out.insert( ~"t_" + name, gr_low::shade::UniTexture(0,tex,s_opt) );
		let u_transform = vec::vec4::new( ti.scale[0], ti.scale[1], ti.offset[0], ti.offset[1] );
		out.insert( fmt!("u_Tex%uTransform",i), gr_low::shade::UniFloatVec(u_transform) );
	}
	out
}


priv fn parse_materials( materials : &[gen::Material], prefix : ~str, ctx : &mut common::SceneContext,
		gc : &mut gr_low::context::Context, lg : &engine::journal::Log)	{
	let flat_mat = ~"flat";
	let mut future_textures : LinearMap<~str,engine::load::TextureFuture> = LinearMap::new();
	let async = true;
	for materials.each() |imat|	{
		let mut source = copy imat.shader;
		if ctx.materials.contains_key( &imat.name )	{
			lg.add(fmt!( "\tMaterial skipped: %s (%s)", imat.name, source ));
			loop;
		}
		lg.add(fmt!( "\tMaterial added: %s (%s)", imat.name, source ));
		if source == ~"phong" && imat.textures.is_empty()	{
			source = copy flat_mat;
			lg.add(fmt!( "(w) forcing material '%s' shader to '%s' due to no textures", imat.name, flat_mat ));
		}
		let mat = @gr_mid::draw::load_material( prefix + source );
		ctx.materials.insert( copy imat.name, mat );
		for imat.textures.each() |itex|	{
			if !ctx.textures.contains_key( &itex.path )	{
				let path = ~"data/texture/" + itex.path;
				let tex_add = match ctx.textures.find(&path)	{
					Some(t) => Some(*t),
					None if !async	=>	{
						let t = engine::load::load_texture_2D( gc, &path, true );
						Some(t)
					},
					None	=>	{
						if !future_textures.contains_key( &itex.path )	{
							let ft = engine::load::future_texture_2D( &path, true );
							future_textures.insert( copy itex.path, ft );
						}
						None
					}			
				};
				match tex_add	{
					Some(t)	=> ctx.textures.insert( copy itex.path, t ),
					None	=> true
				};
			}
		}
		
	}
	if async	{
		for future_textures.each() |&(name,ft)|	{
			let tex = ft.get( gc );
			//print(fmt!( "\tDeferred texture: %s\n", *name ));
			ctx.textures.insert( copy *name, tex );
		}
	}
	for materials.each() |imat|	{
		if !ctx.mat_data.contains_key( &imat.name )	{
			let data = parse_shader_data( imat, &ctx.textures, lg );
			ctx.mat_data.insert( copy imat.name, data );
		}
	}
}


priv fn parse_child( child : &gen::NodeChild, parent : Option<@mut space::Node>, scene : &mut common::Scene,
		get_input : &fn(~str)->gr_mid::call::Input )	{
	match child	{
		&gen::ChildNode(ref inode)	=>	{
			let qs = space::QuatSpace	{
				position	: vec::vec3::from_array( inode.space.pos ),
				orientation	: quat::quat::from_array( inode.space.rot ),
				scale		: inode.space.scale,
			};
			let n = @mut space::Node	{
				name	: copy inode.name,
				space	: qs,
				parent	: parent,
				actions	: ~[],
			};
			scene.context.nodes.insert( copy n.name, n );
			for inode.children.each() |child|	{
				parse_child( child, Some(n), scene, get_input );
			}
		},
		&gen::ChildEntity(ref ient)	=>	{
			let mut input = get_input( copy ient.mesh );
			input.range.start = ient.range[0];
			input.range.num = ient.range[1] - ient.range[0];
			let skel = if ient.armature.is_empty()	{
				@()	as @gr_mid::draw::Mod
			}else	{
				*scene.context.armatures.find( &ient.armature ).
					expect( ~"Armature not found: " + ient.armature )
					as @gr_mid::draw::Mod
			};
			scene.entities.push( engine::object::Entity{
				node	: parent.expect("Entity parent has to exist"),
				//body	: @node::Body,
				input	: input,
				data	: copy *scene.context.mat_data.find( &ient.material ).
					expect( ~"Material data not found: " + ient.material ),
				modifier: skel,
				material: *scene.context.materials.find( &ient.material ).
					expect( ~"Material not found: " + ient.material ),
			});
		},
		&gen::ChildCamera(ref icam)	=>	{
			scene.cameras.insert( copy icam.name, @common::Camera{
				node	: parent.expect("Camera parent has to exist"),
				proj	: projection::PerspectiveSym	{
					vfov	: icam.fov_y.degrees(),
					aspect	: 1f32,
					near	: icam.range[0],
					far		: icam.range[1],
				},
				ear		: engine::audio::Listener{ volume:0f },
			});
		},
		&gen::ChildLight(ref ilit)	=>	{
			scene.lights.insert( copy ilit.name, @common::Light{
				node	: parent.expect("Light parent has to exist"),
				color	: gr_low::rast::Color::from_array3( ilit.color ),
				attenu	: [1f32 / ilit.energy, ilit.attenuation[0], ilit.attenuation[1]],
				distance: ilit.distance,
				bounded	: ilit.spherical,
				kind	: match ilit.kind	{
					gen::KindOmni(_)	=> common::LiPoint,
					gen::KindSpot(spot)	=> common::LiSpot( spot.size.degrees(), spot.blend as float ),
				},
			});
		},
	}
}


pub fn parse( path : ~str, iscene : &gen::Scene, custom : &[gen::Material], gc : &mut gr_low::context::Context,
		opt_vao : Option<@mut gr_low::buf::VertexArray>, lg : &engine::journal::Log )-> common::Scene	{
	lg.add( ~"Loading scene: " + path );
	let c0 = engine::anim::get_time();
	let mut scene = common::Scene	{
		context		: common::SceneContext::new( copy path ),
		entities	: common::EntityGroup(~[]),
		cameras		: LinearMap::new(),
		lights		: LinearMap::new()
	};
	// materials
	parse_materials( custom, ~"data/code-game/",			&mut scene.context, gc, lg );
	parse_materials( iscene.materials, ~"data/code/mat/",	&mut scene.context, gc, lg );
	let c1 = engine::anim::get_time();
	lg.add(fmt!( "\t[p] Materials: %f sec", c1-c0 ));
	// armatures-0
	scene.context.read_armatures( &path, lg );
	// nodes and stuff
	let get_input = |mesh_name : ~str|	{
		let vao = match opt_vao	{
			Some(va)	=> va,
			None		=> gc.create_vertex_array(),
		};
		let mesh = scene.context.query_mesh( &mesh_name, gc, lg );
		gr_mid::call::Input::new( vao, mesh )
	};
	for iscene.nodes.each() |child|	{
		parse_child( child, None, &mut scene, get_input );
	}
	let c2 = engine::anim::get_time();
	lg.add(fmt!( "\t[p] Objects: %f sec", c2-c1 ));
	// armatures-1
	for scene.context.armatures.each_value() |arm|	{
		let name = copy arm.root.name;
		let root = *scene.context.nodes.find( &name ).
			expect( ~"Unable to find armature root " + name );
		arm.change_root( root );
	}
	let c3 = engine::anim::get_time();
	lg.add(fmt!( "\t[p] Total: %f sec", c3-c0 ));
	// done
	scene
}
