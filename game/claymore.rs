#[link(
	name = "claymore", 
	vers = "0.1", 
	author = "Dzmitry Malyshau",
	url = "http://code.google.com/p/claymore-game/"
)];

#[comment = "Claymore game"];
#[license = "MIT"];
#[crate_type = "bin"];


extern mod gen_hud;
extern mod gen_scene;
extern mod glfw;
extern mod cgmath;
extern mod engine;
extern mod extra;


pub mod hud;
pub mod hud_new;
pub mod input;
pub mod main;

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

pub mod battle	{
	pub mod grid;
	pub mod main;
	pub mod time;
}