#[link(name = "engine", 
       vers = "0.1", 
       author = "Dzmitry Malyshau")];

#[comment = "Graphics engine for Claymore game"];
#[license = "MIT"];
#[crate_type = "lib"];

extern mod freetype;
extern mod stb_image;
extern mod lmath;
extern mod openal;

pub mod anim;
pub mod audio;
pub mod buf;
pub mod call;
pub mod context;
pub mod draw;
pub mod font;
pub mod frame;
pub mod load;
pub mod mesh;
pub mod rast;
pub mod shade;
pub mod space;
pub mod texture;
