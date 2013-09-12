use common::*;

pub fn load()-> Screen	{Screen	{
	alpha	: 0.5,
	children: ~[
		Child	{
			name	: ~"id",
			align	: ([-1,-1],RelParent,[-1,-1]),
			element	: ElFrame(Frame	{
				margin	: [[10,10],[10,10]],
				ground	: Some(Ground	{
					path	: ~"frame1-new.png",
					center	: [0.5,0.5],
				}),
				children: ~[
					Child	{
						name	: ~"caption",
						align	: ([-1,-1],RelParent,[-1,-1]),
						element	: ElFrame(Frame	{
							margin	: [[0,0],[0,0]],
							ground	: None,
							children: ~[
								Child	{
									name	: ~"text",
									align	: ([0,-1],RelParent,[0,-1]),
									element	: ElText(Text	{
										value	: ~"IDENTIFICATION",
										font	: Font	{
											path	: ~"Vera.ttf",
											size	: [10,10],
											kern	: [0,-10],
										},
										color	: 0x000000FF,
										bound	: [200,50],
										edit	: false,
									}),
								},	//text
								Child	{
									name	: ~"left",
									align	: ([1,-1],RelHead,[-1,-1]),
									element	: ElImage(~"tilde-left.png"),
								},	//left
								Child	{
									name	: ~"right",
									align	: ([-1,-1],RelHead,[1,-1]),
									element	: ElImage(~"tilde-right.png"),
								},	//right
							],
						}),
					},	//caption
				],
			}),
		},	//id
	],
}}
