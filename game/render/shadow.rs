extern mod engine;
extern mod cgmath;
extern mod lmath;

use engine::{gr_low,gr_mid};

use scene = scene::common;


pub struct Data	{
	light		: @scene::Light,
	call_clear	: gr_mid::call::Call,
	tech_solid	: gr_mid::draw::Technique,
	tech_alpha	: gr_mid::draw::Technique,
	output		: gr_mid::call::Output,
	rast		: gr_low::rast::State,
	par_shadow	: gr_low::shade::Uniform,
}

pub fn create_data( ct : &mut gr_low::context::Context, light : @scene::Light, size : uint )-> Data	{
	let shadow = ct.create_texture( ~"2D", size, size, 0u, 0u );
	ct.texture.init_depth( shadow, false );
	let fbo = ct.create_frame_buffer();
	let mut pmap = gr_mid::call::PlaneMap::new_empty();
	pmap.depth = gr_low::frame::TarTexture(shadow,0u);
	let mut rast = copy ct.default_rast;
	rast.view = gr_low::rast::Viewport( gr_low::frame::Rect::new(size,size) );
	rast.set_depth( ~"<", true );
	rast.prime.cull = true;
	rast.set_offset(2f);
	let cdata = gr_mid::call::ClearData{
		color	: None,
		depth	: Some(1f),
		stencil	: None,
	};
	let t_solid = gr_mid::draw::load_technique( ~"data/code/tech/shadow/spot" );
	let t_alpha = gr_mid::draw::load_technique( ~"data/code/tech/shadow/spot-alpha" );
	let mut samp = gr_low::texture::Sampler::new( 2u, 0 );
	samp.compare = Some( gr_low::rast::map_comparison(~"<") );
	let par = gr_low::shade::UniTexture( 0u, shadow, Some(samp) );
	let out = gr_mid::call::Output::new( fbo, pmap );
	let c0 = gr_mid::call::CallClear( copy out, cdata, rast.mask );
	Data{
		light		: light,
		call_clear	: c0,
		tech_solid	: t_solid,
		tech_alpha	: t_alpha,
		output		: out,
		rast		: rast,
		par_shadow	: par,
	}
}