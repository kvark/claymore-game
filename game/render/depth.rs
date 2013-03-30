extern mod engine;

pub struct Data	{
	texture		: @engine::texture::Texture,
	tech_solid	: engine::draw::Technique,
	output		: engine::call::DrawOutput,
	call_clear	: engine::call::Call,
}

pub impl Data	{
	static fn create( gc : &engine::context::Context )-> Data	{
		let (wid,het) = gc.screen_size;
		let texture = @gc.create_texture( ~"2D", wid, het, 0u, 0u );
		gc.texture.init_shadow( texture, false );
		let mut pmap = engine::call::make_pmap_empty();
		pmap.depth = engine::frame::TarTexture(texture,0);
		let mut rast = copy gc.default_rast;
		rast.prime.cull = true;
		rast.set_depth( ~"<=", true );
		let out = ( @gc.create_frame_buffer(), pmap, rast );
		let clear = engine::call::ClearData{
				color:None, depth:Some(1f), stencil:None
			}.gen_call( copy out );
		Data{
			texture		: texture,
			tech_solid	: engine::draw::load_technique( ~"data/code/tech/pure/solid" ),
			output		: out,
			call_clear	: clear,
		}
	}
}
