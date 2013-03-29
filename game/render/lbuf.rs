extern mod engine;

pub struct Context	{
	tech_apply	: engine::draw::Technique,
	ta_direction: @engine::texture::Texture,
	ta_color	: @engine::texture::Texture,
}

pub impl Context	{
	static fn create( gc : &engine::context::Context, layers : uint, div : uint )-> Context	{
		let (wid,het) = gc.screen_size;
		let ta_dir = @gc.create_texture( ~"2DArray", wid/div, het/div, layers, 0u );
		let ta_col = @gc.create_texture( ~"2DArray", wid/div, het/div, layers, 0u );
		/*let fbo = @gc.create_frame_buffer();
		let mut pmap = engine::call:make_pmap_empty();
		pmap.color.insert( ~"o_Dir", engine::frame:: );
		pmap.color.insert( ~"o_Col", engine::frame:: );
		*/
		let technique = engine::draw::load_technique( ~"data/code/tech/lbuf/apply" );
		Context{
			tech_apply	: technique,
			ta_direction: ta_dir,
			ta_color	: ta_col,
		}
	}

	fn fill_data( data : &mut engine::shade::DataMap )	{
		let sampler = Some( engine::texture::make_sampler( 2u, 0 ) );
		data.insert( ~"t_LbufDir", engine::shade::UniTexture( 0, self.ta_direction,	sampler ));
		data.insert( ~"t_LbufCol", engine::shade::UniTexture( 0, self.ta_color,		sampler ));
	}
}

pub struct Layer	{
}