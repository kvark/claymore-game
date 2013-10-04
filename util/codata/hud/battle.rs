use common::*;

pub fn load()-> Screen	{Screen	{
	alpha	: 0.5,
	root	: Box	{
		align	: AlignHor,
		ground	: GroundNone,
		children: ~[
			Child( ~"id", ElBox(Box	{
				align	: AlignVer,
				ground	: GroundImage( ~"frame1-new.png", [0.5,0.5] ),
				children: ~[
					Child( ~"caption", ElBox(Box	{
						align	: AlignHor,
						ground	: GroundNone,
						children: ~[
							Child( ~"left", ElImage(~"tilde-left.png") ),
							Child( ~"text", ElText(Text	{
								value	: ~"IDENTIFICATION",
								font	: Font	{
									path	: ~"Vera.ttf",
									size	: [10,10],
									kern	: [0,-10],
								},
								color	: 0x000000FF,
								bound	: [200,50],
								edit	: false,
							})),	//text
							Child( ~"right", ElImage(~"tilde-right.png") ),
						],
					})),	//caption
				],
			})),	//id
		],
	},
}}
