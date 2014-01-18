use cgmath::matrix::ToMat4;

use gr_low;
use gr_mid;
use journal;
use space;


pub struct Entity	{
	node	: space::NodePtr,
	//body	: @node::Body,
	input	: gr_mid::call::Input,
	data	: gr_low::shade::DataMap,
	modifier: ~gr_mid::draw::Mod,
	material: gr_mid::draw::MaterialPtr,
}

impl Entity	{
	pub fn update_world( &mut self )	{
		let world = self.node.borrow().with( |n| n.world_space().to_mat4() );
		self.data.set( ~"u_World", gr_low::shade::UniMatrix(false,world) );
	}
}

impl gr_mid::draw::Technique	{
	pub fn process( &self, e: &Entity, output: gr_mid::call::Output, rast: gr_low::rast::Rast,
			cache: &mut gr_mid::draw::Cache, ct: &gr_low::context::Context,
			lg: &journal::Log )-> gr_mid::call::Call	{
		let op = self.get_program( &e.material, e.modifier, cache, ct, lg );
		match op	{
			Some(p)	=> gr_mid::call::CallDraw( e.input.clone(), output, rast, p, e.data.clone() ),
			None => gr_mid::call::CallEmpty
		}
	}
}
