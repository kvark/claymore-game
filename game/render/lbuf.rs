extern mod engine;
extern mod lmath;

use lmath::vec::vec4;

use engine::{gr_low,gr_mid};
use engine::space::Space;

use scene = scene::common;


static use_array : bool	= false;

pub struct LightVolume	{
	mesh_point	: @gr_mid::mesh::Mesh,
	mat_point	: @gr_mid::draw::Material,
}

pub impl LightVolume	{
	fn create( gc : &mut gr_low::context::Context, lg : &engine::journal::Log )->LightVolume	{
		LightVolume{
			mesh_point	: @engine::load::load_mesh( ~"data/mesh/cube.k3mesh", gc, lg ),
			mat_point	: @gr_mid::draw::load_material( ~"data/code/mat/light/point" ),
		}
	}
	fn query( &self, _kind : scene::LightKind )-> (@gr_mid::mesh::Mesh,@gr_mid::draw::Material)	{
		( self.mesh_point, self.mat_point )
	}
}


pub struct Context	{
	tech_bake	: @gr_mid::draw::Technique,
	tech_apply	: @gr_mid::draw::Technique,
	fbo			: @mut gr_low::frame::Buffer,
	vao			: @mut gr_low::buf::VertexArray,
	ta_direction: @gr_low::texture::Texture,
	ta_color	: @gr_low::texture::Texture,
	t_depth		: @gr_low::texture::Texture,
	fbo_alt		: @mut gr_low::frame::Buffer,
}

pub impl Context	{
	fn create( gc : &mut gr_low::context::Context, layers : uint, div : uint )-> Context	{
		let (wid,het) = gc.screen_size;
		let (s_type,s_format,dim_depth) = if use_array {
			(~"2DArray",~"rgba16f",layers)
		} else {
			(~"2D",~"rgba8",0u)
		};
		let ta_dir = gc.create_texture( copy s_type, wid/div, het/div, dim_depth, 0u );
		gc.texture.init( ta_dir, 1u, gr_low::texture::map_int_format(copy s_format), true );
		let ta_col = gc.create_texture( copy s_type, wid/div, het/div, dim_depth, 0u );
		gc.texture.init( ta_col, 1u, gr_low::texture::map_int_format(copy s_format), true );
		let depth = gc.create_texture( ~"2D", wid/div, het/div, 0u, 0u );
		gc.texture.init_depth( depth, false );
		let t_bake	= @gr_mid::draw::load_technique( ~"data/code/tech/lbuf/bake" );
		let t_apply	= @gr_mid::draw::load_technique( ~"data/code/tech/lbuf/apply" );
		Context{
			tech_bake	: t_bake,
			tech_apply	: t_apply,
			fbo			: gc.create_frame_buffer(),
			vao			: gc.create_vertex_array(),
			ta_direction: ta_dir,
			ta_color	: ta_col,
			t_depth		: depth,
			fbo_alt		: gc.create_frame_buffer(),
		}
	}

	fn update_depth( &self, depth : @gr_low::texture::Texture )-> gr_mid::call::Call	{
		let mut pm1 = gr_mid::call::PlaneMap::new_empty();
		let mut pm2 = gr_mid::call::PlaneMap::new_empty();
		pm1.depth = gr_low::frame::TarTexture( depth, 0 );
		pm2.depth = gr_low::frame::TarTexture( self.t_depth, 0 );
		let src = gr_mid::call::Output::new( self.fbo_alt, pm1 );
		let dst = gr_mid::call::Output::new( self.fbo, pm2 );
		gr_mid::call::CallBlit( src, dst )
	}

	fn bake_layer( &self, layer : uint, lights : &[@scene::Light], vol : &LightVolume, cam : &scene::Camera,
			gc : &gr_low::context::Context, lg : &engine::journal::Log )-> ~[gr_mid::call::Call]	{
		let mut pmap = gr_mid::call::PlaneMap::new_empty();
		pmap.depth = gr_low::frame::TarTexture( self.t_depth, 0 );
		fn to_target( t : @gr_low::texture::Texture, l : uint )-> gr_low::frame::Target	{
			if use_array	{
				gr_low::frame::TarTextureLayer(t,l,0)
			}else	{ assert!( l == 0 );
				gr_low::frame::TarTexture(t,0)
			}
		}
		pmap.colors.insert( ~"o_Dir", to_target( self.ta_direction, layer ));
		pmap.colors.insert( ~"o_Col", to_target( self.ta_color, layer ));
		let (wid,het) = self.ta_color.get_level_size(0);
		let mut rast = copy gc.default_rast;
		rast.view = gr_low::rast::Viewport( gr_low::frame::Rect::new(wid,het) );
		rast.prime.cull = true;
		//rast.prime.front_cw = true;
		rast.set_blend( ~"s+d", ~"1", ~"1" );
		rast.set_depth( ~"<=", false );
		let output = gr_mid::call::Output::new( self.fbo, pmap );
		let mut data = gr_low::shade::make_data();
		{	// fill data
			let sampler = Some( gr_low::texture::Sampler::new(2u,0) );
			data.insert( ~"t_Depth", gr_low::shade::UniTexture(0,self.t_depth,sampler) );
			let target_size = vec4::new( wid as f32, het as f32,
				1f32/(wid as f32), 1f32/(het as f32) );
			data.insert( ~"u_TargetSize",		gr_low::shade::UniFloatVec(target_size) );
			let vpi = cam.get_inverse_matrix();
			cam.fill_data( &mut data );
			data.insert( ~"u_ViewProjInverse",	gr_low::shade::UniMatrix(false,vpi) );
		}
		let cdata = gr_mid::call::ClearData	{
			color	: Some( gr_low::rast::Color::new(0x00000001u) ),
			depth	: None,
			stencil	: None,
		};
		let clear = gr_mid::call::CallClear( copy output, cdata, rast.mask );
		let mut queue = do vec::map(lights) |lit|	{
			let (mesh,mat) = vol.query( lit.kind );
			lit.fill_data( &mut data, 1f32, 30f32 );
			let mw = lit.node.world_space().to_matrix();
			data.insert( ~"u_World",	gr_low::shade::UniMatrix(false,mw) );
			let e = engine::object::Entity	{
				node	: lit.node,
				input	: gr_mid::call::Input::new( self.vao, mesh ),
				data	: copy data,
				modifier: @() as @gr_mid::draw::Mod,
				material: mat,
			};
			self.tech_bake.process( &e, copy output, copy rast, None, gc, lg )
		};
		//todo: functional style in Rust-0.6
		queue.insert( 0u, clear );
		queue
	}

	fn fill_data( &self, data : &mut gr_low::shade::DataMap )	{
		let sampler = Some( gr_low::texture::Sampler::new( 2u, 0 ) );
		data.insert( ~"t_LbufDir", gr_low::shade::UniTexture( 0, self.ta_direction,	sampler ));
		data.insert( ~"t_LbufCol", gr_low::shade::UniTexture( 0, self.ta_color,		sampler ));
	}
}
