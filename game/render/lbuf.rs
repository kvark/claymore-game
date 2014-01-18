extern mod engine;
extern mod cgmath;

use std::cell::RefCell;
use cgmath::matrix::ToMat4;
use cgmath::vector::Vec4;

use engine::{gr_low,gr_mid};
use scene = scene::common;


static use_array : bool	= false;

pub struct LightVolume	{
	mesh_point	: gr_mid::mesh::MeshPtr,
	mat_point	: gr_mid::draw::MaterialPtr,
}

impl LightVolume	{
	pub fn create( gc: &mut gr_low::context::Context, lg: &engine::journal::Log )->LightVolume	{
		LightVolume{
			mesh_point	: engine::load::load_mesh( "data/mesh/cube.k3mesh", gc, lg ).to_ptr(),
			mat_point	: gr_mid::draw::load_material( "data/code/mat/light/point" ).to_ptr(),
		}
	}
	pub fn query( &self, _kind: scene::LightKind )-> (gr_mid::mesh::MeshPtr,gr_mid::draw::MaterialPtr)	{
		( self.mesh_point.clone(), self.mat_point.clone() )
	}
}


pub struct Context	{
	tech_bake	: gr_mid::draw::Technique,
	tech_apply	: gr_mid::draw::Technique,
	fbo			: gr_low::frame::BufferPtr,
	vao			: gr_low::buf::VertexArrayPtr,
	ta_direction: gr_low::texture::TexturePtr,
	ta_color	: gr_low::texture::TexturePtr,
	t_depth		: gr_low::texture::TexturePtr,
	fbo_alt		: gr_low::frame::BufferPtr,
	cache		: RefCell<gr_mid::draw::Cache>,
}

impl Context	{
	pub fn create( gc: &mut gr_low::context::Context, layers: uint, div: uint )-> Context	{
		let (wid,het) = gc.get_screen_size();
		let (s_type,s_format,dim_depth) = if use_array {
			(~"2DArray",~"rgba16f",layers)
		} else {
			(~"2D",~"rgba8",0u)
		};
		let ta_dir = gc.create_texture( s_type, wid/div, het/div, dim_depth, 0u );
		gc.texture.init( &ta_dir, 1u, gr_low::texture::map_int_format(s_format), true );
		let ta_col = gc.create_texture( s_type, wid/div, het/div, dim_depth, 0u );
		gc.texture.init( &ta_col, 1u, gr_low::texture::map_int_format(s_format), true );
		let depth = gc.create_texture( "2D", wid/div, het/div, 0u, 0u );
		gc.texture.init_depth( &depth, false );
		let t_bake	= gr_mid::draw::load_technique( "data/code/tech/lbuf/bake" );
		let t_apply	= gr_mid::draw::load_technique( "data/code/tech/lbuf/apply" );
		Context{
			tech_bake	: t_bake,
			tech_apply	: t_apply,
			fbo			: gc.create_frame_buffer(),
			vao			: gc.create_vertex_array(),
			ta_direction: ta_dir,
			ta_color	: ta_col,
			t_depth		: depth,
			fbo_alt		: gc.create_frame_buffer(),
			cache		: RefCell::new( gr_mid::draw::make_cache() ),
		}
	}

	pub fn update_depth( &self, depth: &gr_low::texture::TexturePtr )-> gr_mid::call::Call	{
		let mut pm1 = gr_mid::call::PlaneMap::new_empty();
		let mut pm2 = gr_mid::call::PlaneMap::new_empty();
		pm1.depth = gr_low::frame::TarTexture( depth.clone(), 0 );
		pm2.depth = gr_low::frame::TarTexture( self.t_depth.clone(), 0 );
		let src = gr_mid::call::Output::new( &self.fbo_alt, pm1 );
		let dst = gr_mid::call::Output::new( &self.fbo, pm2 );
		gr_mid::call::CallBlit( src, dst )
	}

	pub fn bake_layer( &self, layer: uint, lights: &[scene::LightPtr], vol: &LightVolume, cam: &scene::Camera,
			gc: &gr_low::context::Context, lg: &engine::journal::Log )-> ~[gr_mid::call::Call]	{
		let mut pmap = gr_mid::call::PlaneMap::new_empty();
		pmap.depth = gr_low::frame::TarTexture( self.t_depth.clone(), 0 );
		fn to_target( t: &gr_low::texture::TexturePtr, l: uint )-> gr_low::frame::Target	{
			if use_array	{
				gr_low::frame::TarTextureLayer(t.clone(),l,0)
			}else	{ assert!( l == 0 );
				gr_low::frame::TarTexture(t.clone(),0)
			}
		}
		pmap.colors.insert( ~"o_Dir", to_target( &self.ta_direction, layer ));
		pmap.colors.insert( ~"o_Col", to_target( &self.ta_color, layer ));
		let (wid,het) = self.ta_color.borrow().get_level_size(0);
		let mut rast = gc.default_rast;
		rast.view = gr_low::rast::Viewport( gr_low::frame::Rect::new(wid,het) );
		rast.prime.cull = true;
		//rast.prime.front_cw = true;
		rast.set_blend( "s+d", "1", "1" );
		rast.set_depth( "<=", false );
		let output = gr_mid::call::Output::new( &self.fbo, pmap );
		let mut data = gr_low::shade::DataMap::new();
		{	// fill data
			let aspect = (wid as f32) / (het as f32);
			let sampler = Some( gr_low::texture::Sampler::new(2u,0) );
			data.set( ~"t_Depth", gr_low::shade::UniTexture(0, self.t_depth.clone(), sampler) );
			let target_size = Vec4::new( wid as f32, het as f32,
				1f32/(wid as f32), 1f32/(het as f32) );
			data.set( ~"u_TargetSize",		gr_low::shade::UniFloatVec(target_size) );
			let vpi = cam.get_inverse_matrix( aspect );
			cam.fill_data( &mut data, aspect );
			data.set( ~"u_ViewProjInverse",	gr_low::shade::UniMatrix(false,vpi) );
		}
		let cdata = gr_mid::call::ClearData	{
			color	: Some( gr_low::rast::Color::new(0x00000001u) ),
			depth	: None,
			stencil	: None,
		};
		let clear = gr_mid::call::CallClear( cdata, output.clone(), rast.mask );
		let mut queue = lights.iter().map( |lit|	{
			let (mesh,mat) = vol.query( lit.borrow().kind );
			lit.borrow().fill_data( &mut data, 1f32, 30f32 );
			let mw = lit.borrow().node.borrow().with( |n| n.world_space().to_mat4() );
			data.set( ~"u_World",	gr_low::shade::UniMatrix(false,mw) );
			let e = engine::object::Entity	{
				node	: lit.borrow().node.clone(),
				input	: gr_mid::call::Input::new( &self.vao, &mesh ),
				data	: data.clone(),
				modifier: ~() as gr_mid::draw::ModPtr,
				material: mat.clone(),
			};
			self.cache.with_mut(|cache|	{
				self.tech_bake.process( &e, output.clone(), rast, cache, gc, lg )
			})
		}).to_owned_vec();
		//todo: functional style in Rust-0.6
		queue.insert( 0u, clear );
		queue
	}

	pub fn fill_data( &self, data: &mut gr_low::shade::DataMap )	{
		let sampler = Some( gr_low::texture::Sampler::new( 2u, 0 ) );
		data.set( ~"t_LbufDir", gr_low::shade::UniTexture( 0, self.ta_direction.clone(),	sampler ));
		data.set( ~"t_LbufCol", gr_low::shade::UniTexture( 0, self.ta_color.clone(),		sampler ));
	}
}
