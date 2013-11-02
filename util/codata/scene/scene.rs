#[link(
	name = "gen_scene", 
	vers = "0.1", 
	author = "Dzmitry Malyshau"
)];

#[comment = "Generated Claymore scenes"];
#[license = "MIT"];
#[crate_type = "lib"];

#[feature(globs)];


pub mod common;
pub mod battle	{
	pub mod main;
}
pub mod chared	{
	pub mod main;
	pub mod custom;
}
