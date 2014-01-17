extern mod engine;

use engine::{gr_low,gr_mid};

pub struct Data	{
	texture		: gr_low::texture::TexturePtr,
	tech_solid	: gr_mid::draw::Technique,
	output		: gr_mid::call::Output,
	rast		: gr_low::rast::Rast,
	call_clear	: gr_mid::call::Call,
}

impl Data	{
	pub fn create( gc : &mut gr_low::context::Context )-> Data	{
		let (wid,het) = gc.get_screen_size();
		let texture = gc.create_texture( "2D", wid, het, 0u, 0u );
		gc.texture.init_depth( &texture, false );
		let mut pmap = gr_mid::call::PlaneMap::new_empty();
		pmap.depth = gr_low::frame::TarTexture(texture.clone(), 0);
		let mut rast = gc.default_rast;
		rast.prime.cull = true;
		rast.set_depth( "<=", true );
		let out = gr_mid::call::Output::new( &gc.create_frame_buffer(), pmap );
		let cdata = gr_mid::call::ClearData{
			color:None, depth:Some(1.0), stencil:None
		};
		let clear = gr_mid::call::CallClear( cdata, out.clone(), rast.mask );
		Data{
			texture		: texture,
			tech_solid	: gr_mid::draw::load_technique( "data/code/tech/pure/solid" ),
			output		: out,
			rast		: rast,
			call_clear	: clear,
		}
	}
}
