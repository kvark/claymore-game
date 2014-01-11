#[crate_id = "claymore-game.googlecode.com#engine:0.1"];
#[crate_type = "lib"];
#[comment = "Graphics engine for Claymore game"];
#[license = "MIT"];

#[feature(managed_boxes)];


extern mod extra;
extern mod freetype;
extern mod stb_image;
extern mod cgmath;
extern mod openal;


pub mod gr_low	{
	pub mod buf;
	pub mod context;
	pub mod frame;
	pub mod rast;
	pub mod shade;
	pub mod texture;
}

pub mod gr_mid	{
	pub mod call;
	pub mod draw;
	pub mod font;
	pub mod mesh;
}

pub mod anim;
pub mod audio;
pub mod journal;
pub mod load;
pub mod object;
pub mod space;

