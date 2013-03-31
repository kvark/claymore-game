extern mod engine;
extern mod cgmath;
extern mod lmath;


pub struct Data	{
	light		: @scene::Light,
	call_clear	: engine::call::Call,
	tech_solid	: engine::draw::Technique,
	tech_alpha	: engine::draw::Technique,
	output		: engine::call::DrawOutput,
	par_shadow	: engine::shade::Uniform,
}

pub fn create_data( ct : &engine::context::Context, light : @scene::Light, size : uint )-> Data	{
	let shadow = @ct.create_texture( ~"2D", size, size, 0u, 0u );
	ct.texture.init_depth( shadow, false );
	let fbo = @ct.create_frame_buffer();
	let mut pmap = engine::call::make_pmap_empty();
	pmap.depth = engine::frame::TarTexture(shadow,0u);
	let mut rast = copy ct.default_rast;
	rast.view = engine::rast::Viewport( engine::frame::make_rect(size,size) );
	rast.set_depth( ~"<", true );
	rast.prime.cull = true;
	rast.set_offset(2f);
	let cdata = engine::call::ClearData{
		color	: None,
		depth	: Some(1f),
		stencil	: None,
	};
	let t_solid = engine::draw::load_technique( ~"data/code/tech/shadow/spot" );
	let t_alpha = engine::draw::load_technique( ~"data/code/tech/shadow/spot-alpha" );
	let mut samp = engine::texture::make_sampler( 2u, 0 );
	samp.compare = Some( engine::rast::map_comparison(~"<") );
	let par = engine::shade::UniTexture( 0u, shadow, Some(samp) );
	let out = (fbo,pmap,rast);
	Data{
		light		: light,
		call_clear	: cdata.gen_call( copy out ),
		tech_solid	: t_solid,
		tech_alpha	: t_alpha,
		output		: out,
		par_shadow	: par,
	}
}