#[link(name = "claymore", 
       vers = "0.1", 
       author = "Dzmitry Malyshau",
       url = "http://code.google.com/p/claymore-game/")];

#[comment = "Claymore game"];
#[license = "MIT"];
#[crate_type = "bin"];

extern mod gen_scene;
extern mod cgmath;
extern mod engine;
extern mod lmath;
extern mod numeric;
extern mod std;

pub mod hud;
pub mod input;
pub mod main;

pub mod render	{
	pub mod depth;
	pub mod lbuf;
	pub mod shadow;
}

pub mod scene	{
	pub mod battle;
	pub mod chared;
	pub mod common;
	pub mod grid;
	pub mod intro;
	pub mod load;
}