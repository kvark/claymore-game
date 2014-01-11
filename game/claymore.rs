#[crate_id = "claymore-game.googlecode.com#claymore:0.1"];
#[crate_type = "bin"];
#[comment = "Claymore game"];
#[license = "MIT"];
#[link_args = "-lglfw"];

#[feature(managed_boxes)];


extern mod gen_hud;
extern mod gen_scene;
extern mod glfw;
extern mod cgmath;
extern mod engine;
extern mod extra;


pub mod input;
pub mod logic;
pub mod main;

pub mod battle	{
	pub mod field;
	pub mod grid;
	pub mod main;
	pub mod think;
	pub mod unit;
}

pub mod hud	{
	pub mod debug;
	pub mod main;
	pub mod main_json;
}

pub mod render	{
	pub mod depth;
	pub mod lbuf;
	pub mod shadow;
}

pub mod scene	{
	pub mod chared;
	pub mod common;
	pub mod intro;
	pub mod load;
	pub mod load_json;
}
