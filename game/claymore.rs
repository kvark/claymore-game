#[link(name = "claymore", 
       vers = "0.1", 
       author = "Dzmitry Malyshau",
       url = "http://code.google.com/p/claymore-game/")];

#[comment = "Claymore game"];
#[license = "MIT"];
#[crate_type = "bin"];

extern mod cgmath;
extern mod engine;
extern mod lmath;
extern mod numeric;
extern mod std;

//pub mod battle;
pub mod chared;
//pub mod grid;
pub mod hud;
pub mod main;
pub mod scene;

pub mod render	{
	pub mod depth;
	pub mod lbuf;
	pub mod shadow;
}
