extern mod engine;

use engine::gr_low;
use engine::gr_mid;

pub struct Data	{
	texture		: @gr_low::texture::Texture,
	tech_solid	: gr_mid::draw::Technique,
	output		: gr_mid::call::Output,
	call_clear	: gr_mid::call::Call,
}

pub impl Data	{
	fn create( gc : &mut gr_low::context::Context )-> Data	{
		let (wid,het) = gc.screen_size;
		let texture = gc.create_texture( ~"2D", wid, het, 0u, 0u );
		gc.texture.init_depth( texture, false );
		let mut pmap = gr_mid::call::PlaneMap::new_empty();
		pmap.depth = gr_low::frame::TarTexture(texture,0);
		let mut rast = copy gc.default_rast;
		rast.prime.cull = true;
		rast.set_depth( ~"<=", true );
		let out = gr_mid::call::Output{
			fb	: gc.create_frame_buffer(),
			pmap: pmap,
			rast: rast,
		};
		let clear = gr_mid::call::ClearData{
				color:None, depth:Some(1f), stencil:None
			}.gen_call( &out );
		Data{
			texture		: texture,
			tech_solid	: gr_mid::draw::load_technique( ~"data/code/tech/pure/solid" ),
			output		: out,
			call_clear	: clear,
		}
	}
}
