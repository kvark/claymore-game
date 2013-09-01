use common::*;

pub fn load()-> ~[Material]	{~[
	Material{
		name	: ~"armor",
		shader	: ~"char/armor",
		textures	: ~[
			Texture{
				name	: ~"DiffuseDirt",
				path	: ~"Metal_Diffuse_Dirt.jpg",
				filter	: 3,
				wrap	: 0,
				scale	: [1.0,1.0,1.0],
				offset	: [0.0,0.0,0.0],
			},
			Texture{
				name	: ~"SpecBumpReflect",
				path	: ~"Metal_R-Spec_G-Bump_B-Reflect.jpg",
				filter	: 3,
				wrap	: 0,
				scale	: [1.0,1.0,1.0],
				offset	: [0.0,0.0,0.0],
			},
			Texture{
				name	: ~"Reflection",
				path	: ~"Topanga_Forest_B_3k.hdr",
				filter	: 3,
				wrap	: 0,
				scale	: [1.0,1.0,1.0],
				offset	: [0.0,0.0,0.0],
			},
		],
		data	: ~[
			(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00])),
		],
	},
	Material{
		name	: ~"skin",
		shader	: ~"char/skin",
		textures	: ~[
			Texture{
				name	: ~"Diffuse",
				path	: ~"Skin_Diffuse.jpg",
				filter	: 3,
				wrap	: 0,
				scale	: [1.0,1.0,1.0],
				offset	: [0.0,0.0,0.0],
			},
		],
		data	: ~[
			(	~"PhongParams",	DataVector(	[1.0, 0.8, 0.1, 10.0])),
		],
	},
	Material{
		name	: ~"anisotropic1",
		shader	: ~"char/hair",
		textures	: ~[
			Texture{
				name	: ~"Normal",
				path	: ~"HairNormals1024.jpg",
				filter	: 3,
				wrap	: 0,
				scale	: [1.0,1.0,1.0],
				offset	: [0.0,0.0,0.0],
			},
			Texture{
				name	: ~"SpecAlphaRefl",
				path	: ~"Hair_R-spec_G-alpha_B-SSS-Refl-wt1024.jpg",
				filter	: 3,
				wrap	: 0,
				scale	: [1.0,1.0,1.0],
				offset	: [0.0,0.0,0.0],
			},
		],
		data	: ~[
			(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00])),
		],
	},
	Material{
		name	: ~"cloak",
		shader	: ~"char/cloak",
		textures	: ~[],
		data	: ~[
			(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00])),
		],
	},
]}