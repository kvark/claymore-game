extern mod engine;
extern mod cgmath;
extern mod lmath;


pub struct Data	{
	light		: scene::Light,
	call_clear	: engine::call::Call,
	tech_bake	: engine::draw::Technique,
	par_shadow	: engine::shade::Uniform,
}

pub fn create_data( ct : &engine::context::Context, cache : @mut engine::draw::Cache,
		light : scene::Light, size : uint )-> Data	{
	let shadow = @ct.create_texture( ~"2D", size, size, 0u, 0u );
	ct.texture.init_2D_shadow( shadow, false );
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
	let call = engine::call::CallClear( fbo, copy pmap, cdata, rast.scissor, rast.mask );
	let tech = engine::draw::load_technique(~"shadow-spot",
		~"data/code/tech/shadow/spot", (fbo,pmap,rast), cache );
	let mut samp = engine::texture::make_sampler( 2u, 0 );
	samp.compare = Some( engine::rast::map_comparison(~"<") );
	let par = engine::shade::UniTexture( 0u, shadow, Some(samp) );
	Data{
		light		: light,
		call_clear	: call,
		tech_bake	: tech,
		par_shadow	: par,
	}
}