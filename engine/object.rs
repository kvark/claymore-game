use gr_low;
use gr_mid;
use journal;
use space;
use space::Space;


pub struct Entity	{
	node	: @mut space::Node,
	//body	: @node::Body,
	input	: gr_mid::call::DrawInput,
	data	: gr_low::shade::DataMap,
	modifier: @gr_mid::draw::Mod,
	material: @gr_mid::draw::Material,
}

pub impl Entity	{
	fn update_world( &mut self )	{
		let world = self.node.world_space().to_matrix();
		self.data.insert( ~"u_World", gr_low::shade::UniMatrix(false,world) );
	}
}

pub impl gr_mid::draw::Technique	{
	fn process( &self, e : &Entity, output : gr_mid::call::DrawOutput, cache : Option<&mut gr_mid::draw::Cache>, ct : &gr_low::context::Context, lg : &journal::Log )-> gr_mid::call::Call	{
		let op = match cache	{
			Some(c)	=> self.get_program( e.material, e.modifier, c, ct, lg ),
			None	=> self.link( e.material, e.modifier, ct, lg ),
		};
		match op	{
			Some(p)	=> gr_mid::call::CallDraw( copy e.input, output, p, copy e.data ),
			None => gr_mid::call::CallEmpty
		}
	}
}