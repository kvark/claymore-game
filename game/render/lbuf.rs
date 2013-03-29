extern mod engine;

pub struct Context	{
	tech_apply	: engine::draw::Technique,
	fbo			: @engine::frame::Buffer,
	rast		: engine::rast::State,
	ta_direction: @engine::texture::Texture,
	ta_color	: @engine::texture::Texture,
}

pub impl Context	{
	static fn create( gc : &engine::context::Context, layers : uint, div : uint )-> Context	{
		let (wid,het) = gc.screen_size;
		let ta_dir = @gc.create_texture( ~"2DArray", wid/div, het/div, layers, 0u );
		let ta_col = @gc.create_texture( ~"2DArray", wid/div, het/div, layers, 0u );
		let technique = engine::draw::load_technique( ~"data/code/tech/lbuf/apply" );
		Context{
			tech_apply	: technique,
			fbo			: @gc.create_frame_buffer(),
			rast		: copy gc.default_rast,
			ta_direction: ta_dir,
			ta_color	: ta_col,
		}
	}

	fn render_layer( layer : uint, _lights : ~[@scene::Light] )->engine::call::DrawOutput	{
		let mut pmap = engine::call::make_pmap_empty();
		pmap.colors.insert( ~"o_Dir", engine::frame::TarTextureLayer(self.ta_direction,	layer, 0) );
		pmap.colors.insert( ~"o_Col", engine::frame::TarTextureLayer(self.ta_color,		layer, 0) );
		let out = ( self.fbo, pmap, copy self.rast );
		out
	}

	fn fill_data( data : &mut engine::shade::DataMap )	{
		let sampler = Some( engine::texture::make_sampler( 2u, 0 ) );
		data.insert( ~"t_LbufDir", engine::shade::UniTexture( 0, self.ta_direction,	sampler ));
		data.insert( ~"t_LbufCol", engine::shade::UniTexture( 0, self.ta_color,		sampler ));
	}
}

pub struct Layer	{
}