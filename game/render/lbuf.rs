extern mod engine;
extern mod lmath;


pub struct LightVolume	{
	mesh_point	: @engine::mesh::Mesh,
	mat_point	: @engine::draw::Material,
}

pub impl LightVolume	{
	static fn create( gc : &engine::context::Context, lg : &engine::context::Log )->LightVolume	{
		LightVolume{
			mesh_point	: @engine::load::load_mesh( ~"data/mesh/cube.k3mesh", gc, lg ),
			mat_point	: @engine::draw::load_material( ~"data/code/mat/light/point" ),
		}
	}
	pure fn query(_kind : scene::LightKind)-> (@engine::mesh::Mesh,@engine::draw::Material)	{
		( self.mesh_point, self.mat_point )
	}
}


pub struct Context	{
	tech_bake	: engine::draw::Technique,
	tech_apply	: engine::draw::Technique,
	fbo			: @engine::frame::Buffer,
	vao			: @engine::buf::VertexArray,
	ta_direction: @engine::texture::Texture,
	ta_color	: @engine::texture::Texture,
}

pub impl Context	{
	static fn create( gc : &engine::context::Context, layers : uint, div : uint )-> Context	{
		let (wid,het) = gc.screen_size;
		let ta_dir = @gc.create_texture( ~"2DArray", wid/div, het/div, layers, 0u );
		let ta_col = @gc.create_texture( ~"2DArray", wid/div, het/div, layers, 0u );
		gc.texture.init( ta_dir, 1u, engine::texture::map_int_format(~"rgba8"), true );
		gc.texture.init( ta_col, 1u, engine::texture::map_int_format(~"rgba8"), true );
		let t_bake	= engine::draw::load_technique( ~"data/code/tech/lbuf/bake" );
		let t_apply	= engine::draw::load_technique( ~"data/code/tech/lbuf/apply" );
		Context{
			tech_bake	: t_bake,
			tech_apply	: t_apply,
			fbo			: @gc.create_frame_buffer(),
			vao			: @gc.create_vertex_array(),
			ta_direction: ta_dir,
			ta_color	: ta_col,
		}
	}

	fn bake_layer( layer : uint, lights : &[@scene::Light], vol : &LightVolume,
			depth : @engine::texture::Texture, cam : &scene::Camera,
			gc : &engine::context::Context, lg : &engine::context::Log )-> ~[engine::call::Call]	{
		let mut pmap = engine::call::make_pmap_empty();
		pmap.colors.insert( ~"o_Dir", engine::frame::TarTextureLayer(self.ta_direction,	layer, 0) );
		pmap.colors.insert( ~"o_Col", engine::frame::TarTextureLayer(self.ta_color,		layer, 0) );
		let (wid,het) = self.ta_color.get_level_size(0);
		let mut rast = copy gc.default_rast;
		rast.view = engine::rast::Viewport( engine::frame::make_rect(wid,het) );
		rast.prime.cull = true;
		rast.set_blend( ~"s+d", ~"1", ~"1" );
		let output = ( self.fbo, pmap, rast );
		let mut data = engine::shade::make_data();
		{	// fill data
			let sampler = Some( engine::texture::make_sampler(2u,0) );
			data.insert( ~"t_Depth", 			engine::shade::UniTexture(0,depth,sampler) );
			let target_size = lmath::gltypes::vec4::new( wid as f32, het as f32,
				1f32/(wid as f32), 1f32/(het as f32) );
			data.insert( ~"u_TargetSize",		engine::shade::UniFloatVec(target_size) );
			let vpi = cam.get_inverse_matrix();
			cam.fill_data( &mut data );
			data.insert( ~"u_ViewProjInverse",	engine::shade::UniMatrix(false,vpi) );
		}
		let clear = engine::call::ClearData	{
				color	: Some( engine::rast::make_color(0x00000001u) ),
				depth	: None,
				stencil	: None,
			}.gen_call( copy output );
		let mut queue = do vec::map(lights) |lit|	{
			let (mesh,mat) = vol.query( lit.kind );
			lit.fill_data( &mut data, 1f32, 30f32 );
			let mw = lit.node.world_space().to_matrix();
			data.insert( ~"u_World",	engine::shade::UniMatrix(false,mw) );
			let e = engine::draw::Entity	{
				node	: lit.node,
				input	: ( self.vao, mesh, mesh.get_range() ),
				data	: copy data,
				modifier: @() as @engine::draw::Mod,
				material: mat,
			};
			self.tech_bake.process( &e, copy output, gc, lg )
		};
		//todo: functional style in Rust-0.6
		queue.insert( 0u, clear );
		queue
	}

	fn fill_data( data : &mut engine::shade::DataMap )	{
		let sampler = Some( engine::texture::make_sampler( 2u, 0 ) );
		data.insert( ~"t_LbufDir", engine::shade::UniTexture( 0, self.ta_direction,	sampler ));
		data.insert( ~"t_LbufCol", engine::shade::UniTexture( 0, self.ta_color,		sampler ));
	}
}
