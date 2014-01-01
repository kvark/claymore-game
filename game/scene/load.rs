use std::hashmap::HashMap;

use cgmath::{quaternion,vector};
use cgmath::{angle,projection,transform};
use cgmath::transform::Transform;
use engine;
use engine::{gr_low,gr_mid,space};

use scene::common;
use gen = gen_scene::common;


fn parse_shader_data( imat : &gen::Material, tex_cache : &HashMap<~str,@gr_low::texture::Texture>,
		lg : &engine::journal::Log )-> gr_low::shade::DataMap	{
	let mut out = gr_low::shade::DataMap::new();
	fn color2vec(v : [f32, ..3])-> vector::Vec4<f32>	{
		let kf = 1f32 / 255f32;
		vector::Vec4::new( v[0] as f32 * kf, v[1] as f32 * kf, v[2] as f32 * kf, 1f32 )
	}
	fn color2vecU(c : uint)-> vector::Vec4<f32>	{
		let kf = 1f32 / 255f32;
		vector::Vec4::new( (c>>24) as f32 * kf, ((c>>16)&0xFF) as f32 * kf, ((c>>8)&0xFF) as f32 * kf, (c&0xFF) as f32 * kf )
	}
	for &(ref name, ref di) in imat.data.iter()	{
		let uni = match di	{
			&gen::DataInt(v)	=> ( gr_low::shade::UniInt(v) ),
			&gen::DataScalar(v)	=> ( gr_low::shade::UniFloat(v) ),
			&gen::DataVector(v)	=> ( gr_low::shade::UniFloatVec(vector::Vec4::new(v[0],v[1],v[2],v[3])) ),
			&gen::DataColor(v)	=> ( gr_low::shade::UniFloatVec(color2vec(v)) ),
		};
		out.insert( "u_" + *name, uni );
	}
	let phong_texture = ~"Main";
	for (i,ti) in imat.textures.iter().enumerate()	{
		//print(format!( "\tLooking for texture {:s}\n", ti.path ));
		let tex = *tex_cache.get( &ti.path );
		let s_opt = Some(gr_low::texture::Sampler::new( ti.filter, ti.wrap ));
		let mut name = ti.name.clone();
		if name != phong_texture && i==0 && imat.shader == ~"phong"	{
			name = phong_texture.clone(); //that's what the shader expects
			lg.add(format!( "\t\t(w) forcing texture '{:s}' name to {:s}", ti.name, phong_texture ));
		}
		out.insert( ~"t_" + name, gr_low::shade::UniTexture(0,tex,s_opt) );
		let u_transform = vector::Vec4::new( ti.scale[0], ti.scale[1], ti.offset[0], ti.offset[1] );
		out.insert( format!("u_Tex{:u}Transform",i), gr_low::shade::UniFloatVec(u_transform) );
	}
	out
}


fn parse_materials( materials : &[gen::Material], prefix : &str, ctx : &mut common::SceneContext,
		gc : &mut gr_low::context::Context, lg : &engine::journal::Log)	{
	let flat_mat = ~"flat";
	let mut future_textures : HashMap<~str,engine::load::TextureFuture> = HashMap::new();
	let async = true;
	for imat in materials.iter()	{
		let mut source = imat.shader.clone();
		if ctx.materials.contains_key( &imat.name )	{
			lg.add(format!( "\tMaterial skipped: {:s} ({:s})", imat.name, source ));
			continue;
		}
		lg.add(format!( "\tMaterial added: {:s} ({:s})", imat.name, source ));
		if source == ~"phong" && imat.textures.is_empty()	{
			source = flat_mat.clone();
			lg.add(format!( "\t\t(w) forcing shader to '{:s}' due to no textures", flat_mat ));
		}
		let mat = @gr_mid::draw::load_material( prefix + source );
		ctx.materials.insert( imat.name.clone(), mat );
		for itex in imat.textures.iter()	{
			if !ctx.textures.contains_key( &itex.path )	{
				let path = ~"data/texture/" + itex.path;
				let tex_add = match ctx.textures.find(&path)	{
					Some(t) => Some(*t),
					None if !async	=>	{
						let t = engine::load::load_texture_2D( gc, path, true );
						Some(t)
					},
					None	=>	{
						if !future_textures.contains_key( &itex.path )	{
							let ft = engine::load::TextureFuture::new_2D( path, true );
							future_textures.insert( itex.path.clone(), ft );
						}
						None
					}			
				};
				match tex_add	{
					Some(t)	=> ctx.textures.insert( itex.path.clone(), t ),
					None	=> true
				};
			}
		}
		
	}
	if async	{
		for (name,ft) in future_textures.mut_iter()	{
			let tex = ft.get( gc );
			//print(format!( "\tDeferred texture: {:s}\n", *name ));
			ctx.textures.insert( name.clone(), tex );
		}
	}
	for imat in materials.iter()	{
		if !ctx.mat_data.contains_key( &imat.name )	{
			let data = parse_shader_data( imat, &ctx.textures, lg );
			ctx.mat_data.insert( imat.name.clone(), data );
		}
	}
}

fn parse_space( s : &gen::Space )-> space::Space	{
	space::make( s.scale,
		quaternion::Quat::new( s.rot[3], s.rot[0], s.rot[1], s.rot[2] ),
		vector::Vec3::new( s.pos[0], s.pos[1], s.pos[2] ))
}

fn parse_bones( bin : &[gen::Bone], par_id : Option<uint>, par_node : @mut engine::space::Node,
		bot : &mut ~[engine::space::Bone] )	{
	for ibone in bin.iter()	{
		let space = parse_space( &ibone.space );
		let bind_inv = space.invert().expect("Failed to invert bone space");
		let bind_pose_inv = match par_id	{
			Some(pid)	=> bind_inv.concat( &bot[pid].bind_pose_inv ),
			None		=> bind_inv,
		};
		let node = @mut space::Node{
			name	: ibone.name.clone(),
			space	: space,
			parent	: Some(par_node),
			actions	:~[],
		};
		let cid = Some( bot.len() );
		bot.push(space::Bone{
			node			: node,
			bind_space		: space,
			bind_pose_inv	: bind_pose_inv,
			transform		: transform::Transform::identity(),
			parent_id		: par_id,
		});
		parse_bones( ibone.children, cid, node, bot );
	}
}

fn parse_child( child : &gen::NodeChild, parent : @mut space::Node, scene : &mut common::Scene,
		get_input : |~str|->gr_mid::call::Input, lg : &engine::journal::Log )	{
	match child	{
		&gen::ChildNode(ref inode)	=>	{
			let n = @mut space::Node	{
				name	: inode.name.clone(),
				space	: parse_space( &inode.space ),
				parent	: Some(parent),
				actions	: ~[],
			};
			scene.context.nodes.insert( n.name.clone(), n );
			for child in inode.children.iter()	{
				parse_child( child, n, scene, |s| get_input(s), lg );
			}
		},
		&gen::ChildArmature(ref iarm)	=>	{
			let (shader,max) = engine::load::get_armature_shader( iarm.dual_quat );
			let a = @mut space::Armature{
				root	: parent,
				bones	: ~[],
				code	: shader,
				actions	: ~[],
				max_bones	: max,
			};
			parse_bones( iarm.bones, None, parent, &mut a.bones );
			for iaction in iarm.actions.iter()	{
				let act = scene.context.query_action( iaction, &mut a.bones, lg );
				a.actions.push( act );
			}
			scene.context.armatures.insert( iarm.name.clone(), a );
		},
		&gen::ChildEntity(ref ient)	=>	{
			let mut input = get_input( ient.mesh.clone() );
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
				node	: parent,
				//body	: @node::Body,
				input	: input,
				data	: scene.context.mat_data.find( &ient.material ).
					expect( ~"Material data not found: " + ient.material ).
					clone(),
				modifier: skel,
				material: *scene.context.materials.find( &ient.material ).
					expect( ~"Material not found: " + ient.material ),
			});
		},
		&gen::ChildCamera(ref icam)	=>	{
			scene.cameras.insert( icam.name.clone(), @common::Camera{
				node	: parent,
				proj	: projection::PerspectiveFov	{
					fovy	: angle::rad(icam.fov_y),
					aspect	: 1f32,
					near	: icam.range[0],
					far		: icam.range[1],
				},
				ear		: engine::audio::Listener{ volume:0.0 },
			});
		},
		&gen::ChildLight(ref ilit)	=>	{
			scene.lights.insert( ilit.name.clone(), @common::Light{
				node	: parent,
				color	: gr_low::rast::Color::from_array3( ilit.color ),
				attenu	: [1f32 / ilit.energy, ilit.attenuation[0], ilit.attenuation[1]],
				distance: ilit.distance,
				bounded	: ilit.spherical,
				kind	: match ilit.kind	{
					gen::KindOmni(_)	=> common::LiPoint,
					gen::KindSpot(spot)	=> common::LiSpot(
						angle::rad(spot.size), spot.blend ),
				},
			});
		},
	}
}


pub fn parse( path : &str, iscene : &gen::Scene, custom : &[gen::Material], gc : &mut gr_low::context::Context,
		opt_vao : Option<@mut gr_low::buf::VertexArray>, lg : &engine::journal::Log )-> common::Scene	{
	lg.add( ~"Loading scene: " + path );
	let c0 = engine::load::get_time();
	let mut scene = common::Scene	{
		context		: common::SceneContext::new( path.to_owned() ),
		entities	: common::EntityGroup(~[]),
		cameras		: HashMap::new(),
		lights		: HashMap::new()
	};
	// materials
	parse_materials( custom, "data/code-game/",			&mut scene.context, gc, lg );
	parse_materials( iscene.materials, "data/code/mat/",&mut scene.context, gc, lg );
	let c1 = engine::load::get_time();
	lg.add(format!( "\t[p] Materials: {:f} sec", c1-c0 ));
	// nodes and stuff
	let get_input = |mesh_name : ~str|	{
		let vao = match opt_vao	{
			Some(va)	=> va,
			None		=> gc.create_vertex_array(),
		};
		let mesh = scene.context.query_mesh( &mesh_name, gc, lg );
		gr_mid::call::Input::new( vao, mesh )
	};
	let root = @mut engine::space::Node::new( ~"root" );
	for child in iscene.nodes.iter()	{
		parse_child( child, root, &mut scene, |s| get_input(s), lg );
	}
	let c2 = engine::load::get_time();
	lg.add(format!( "\t[p] Objects: {:f} sec", c2-c1 ));
	// armatures-1
	for (_,arm) in scene.context.armatures.iter()	{
		let name = &arm.root.name;
		let root = *scene.context.nodes.find( name ).
			expect( ~"Unable to find armature root " + *name );
		arm.change_root( root );
	}
	let c3 = engine::load::get_time();
	lg.add(format!( "\t[p] Total: {:f} sec", c3-c0 ));
	// done
	scene
}
