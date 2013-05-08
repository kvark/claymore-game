extern mod engine;
extern mod lmath;

use lmath::vec::vec4;

use engine::space::Space;

use scene = scene::common;


static use_array : bool	= false;

pub struct LightVolume	{
	mesh_point	: @engine::mesh::Mesh,
	mat_point	: @engine::draw::Material,
}

pub impl LightVolume	{
	fn create( gc : &mut engine::context::Context, lg : &engine::context::Log )->LightVolume	{
		LightVolume{
			mesh_point	: @engine::load::load_mesh( ~"data/mesh/cube.k3mesh", gc, lg ),
			mat_point	: @engine::draw::load_material( ~"data/code/mat/light/point" ),
		}
	}
	fn query( &self, _kind : scene::LightKind )-> (@engine::mesh::Mesh,@engine::draw::Material)	{
		( self.mesh_point, self.mat_point )
	}
}


pub struct Context	{
	tech_bake	: @engine::draw::Technique,
	tech_apply	: @engine::draw::Technique,
	fbo			: @mut engine::frame::Buffer,
	vao			: @mut engine::buf::VertexArray,
	ta_direction: @engine::texture::Texture,
	ta_color	: @engine::texture::Texture,
	t_depth		: @engine::texture::Texture,
	fbo_alt		: @mut engine::frame::Buffer,
}

pub impl Context	{
	fn create( gc : &mut engine::context::Context, layers : uint, div : uint )-> Context	{
		let (wid,het) = gc.screen_size;
		let (s_type,s_format,dim_depth) = if use_array {
			(~"2DArray",~"rgba16f",layers)
		} else {
			(~"2D",~"rgba8",0u)
		};
		let ta_dir = gc.create_texture( copy s_type, wid/div, het/div, dim_depth, 0u );
		gc.texture.init( ta_dir, 1u, engine::texture::map_int_format(copy s_format), true );
		let ta_col = gc.create_texture( copy s_type, wid/div, het/div, dim_depth, 0u );
		gc.texture.init( ta_col, 1u, engine::texture::map_int_format(copy s_format), true );
		let depth = gc.create_texture( ~"2D", wid/div, het/div, 0u, 0u );
		gc.texture.init_depth( depth, false );
		let t_bake	= @engine::draw::load_technique( ~"data/code/tech/lbuf/bake" );
		let t_apply	= @engine::draw::load_technique( ~"data/code/tech/lbuf/apply" );
		Context{
			tech_bake	: t_bake,
			tech_apply	: t_apply,
			fbo			: @mut gc.create_frame_buffer(),
			vao			: @mut gc.create_vertex_array(),
			ta_direction: ta_dir,
			ta_color	: ta_col,
			t_depth		: depth,
			fbo_alt		: @mut gc.create_frame_buffer(),
		}
	}

	fn update_depth( &self, depth : @engine::texture::Texture )-> engine::call::Call	{
		let mut pm1 = engine::call::PlaneMap::new_empty();
		let mut pm2 = engine::call::PlaneMap::new_empty();
		pm1.depth = engine::frame::TarTexture( depth, 0 );
		pm2.depth = engine::frame::TarTexture( self.t_depth, 0 );
		let scissor = engine::rast::Scissor{
			test:false, area:engine::frame::Rect::new(0,0)
		};
		engine::call::CallBlit( self.fbo_alt, pm1, self.fbo, pm2, scissor )
	}

	fn bake_layer( &self, layer : uint, lights : &[@scene::Light], vol : &LightVolume, cam : &scene::Camera,
			gc : &engine::context::Context, lg : &engine::context::Log )-> ~[engine::call::Call]	{
		let mut pmap = engine::call::PlaneMap::new_empty();
		pmap.depth = engine::frame::TarTexture( self.t_depth, 0 );
		fn to_target( t : @engine::texture::Texture, l : uint )-> engine::frame::Target	{
			if use_array	{
				engine::frame::TarTextureLayer(t,l,0)
			}else	{ assert!( l == 0 );
				engine::frame::TarTexture(t,0)
			}
		}
		pmap.colors.insert( ~"o_Dir", to_target( self.ta_direction, layer ));
		pmap.colors.insert( ~"o_Col", to_target( self.ta_color, layer ));
		let (wid,het) = self.ta_color.get_level_size(0);
		let mut rast = copy gc.default_rast;
		rast.view = engine::rast::Viewport( engine::frame::Rect::new(wid,het) );
		rast.prime.cull = true;
		//rast.prime.front_cw = true;
		rast.set_blend( ~"s+d", ~"1", ~"1" );
		rast.set_depth( ~"<=", false );
		let output = ( self.fbo, pmap, rast );
		let mut data = engine::shade::make_data();
		{	// fill data
			let sampler = Some( engine::texture::Sampler::new(2u,0) );
			data.insert( ~"t_Depth", engine::shade::UniTexture(0,self.t_depth,sampler) );
			let target_size = vec4::new( wid as f32, het as f32,
				1f32/(wid as f32), 1f32/(het as f32) );
			data.insert( ~"u_TargetSize",		engine::shade::UniFloatVec(target_size) );
			let vpi = cam.get_inverse_matrix();
			cam.fill_data( &mut data );
			data.insert( ~"u_ViewProjInverse",	engine::shade::UniMatrix(false,vpi) );
		}
		let clear = engine::call::ClearData	{
				color	: Some( engine::rast::Color::new(0x00000001u) ),
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

	fn fill_data( &self, data : &mut engine::shade::DataMap )	{
		let sampler = Some( engine::texture::Sampler::new( 2u, 0 ) );
		data.insert( ~"t_LbufDir", engine::shade::UniTexture( 0, self.ta_direction,	sampler ));
		data.insert( ~"t_LbufCol", engine::shade::UniTexture( 0, self.ta_color,		sampler ));
	}
}
