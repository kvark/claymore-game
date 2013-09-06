use common::*;
pub fn load()-> Scene	{Scene{
		nodes	: ~[
			ChildNode(Node{
				name	: ~"Armature",
				children	: ~[
					ChildArmature(Armature{
						name	: ~"Armature",
						actions	: ~[
							~"ArmatureAction@Armature"
						],
						dual_quat	: false,
						bones	: ~[
							Bone{
								name	: ~"Bone",
								children	: ~[],
								space	: QuatSpace{
									scale	: 1.00,
									pos	: [-0.00, 0.00, -0.07],
									rot	: [0.71, 0.00, 0.00, 0.71],
								},
							},
							Bone{
								name	: ~"Bone.001",
								children	: ~[
									Bone{
										name	: ~"Bone.002",
										children	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [-0.00, 0.67, 0.00],
											rot	: [-0.05, 0.19, 0.03, 0.98],
										},
									}
								],
								space	: QuatSpace{
									scale	: 1.00,
									pos	: [-0.00, 0.00, -0.07],
									rot	: [0.02, -0.38, -0.01, 0.92],
								},
							},
							Bone{
								name	: ~"Bone.003",
								children	: ~[
									Bone{
										name	: ~"Bone.004",
										children	: ~[
											Bone{
												name	: ~"Bone.005",
												children	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [-0.00, 0.53, 0.00],
													rot	: [-0.26, 0.70, -0.65, 0.15],
												},
											}
										],
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [-0.00, 0.58, 0.00],
											rot	: [-0.00, 1.00, -0.01, 0.09],
										},
									}
								],
								space	: QuatSpace{
									scale	: 1.00,
									pos	: [-0.00, 0.00, -0.07],
									rot	: [-0.00, -0.00, 1.00, 0.00],
								},
							},
							Bone{
								name	: ~"Bone.006",
								children	: ~[
									Bone{
										name	: ~"Bone.007",
										children	: ~[
											Bone{
												name	: ~"Bone.008",
												children	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [-0.00, 0.51, 0.00],
													rot	: [-0.20, 0.03, 0.01, 0.98],
												},
											}
										],
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [-0.00, 0.64, 0.00],
											rot	: [0.19, 0.13, -0.04, 0.97],
										},
									},
									Bone{
										name	: ~"Bone.009",
										children	: ~[
											Bone{
												name	: ~"Bone.010",
												children	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [-0.00, 0.52, -0.00],
													rot	: [0.00, 0.98, -0.19, -0.00],
												},
											}
										],
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [-0.00, 0.64, 0.00],
											rot	: [0.03, -0.96, 0.21, 0.16],
										},
									}
								],
								space	: QuatSpace{
									scale	: 1.00,
									pos	: [-0.00, 0.00, -0.07],
									rot	: [-0.11, 0.70, -0.70, 0.11],
								},
							}
						],
					}),
					ChildNode(Node{
						name	: ~"Cube",
						children	: ~[],
						space	: QuatSpace{
							scale	: 1.00,
							pos	: [0.00, 0.00, -0.42],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						actions	: ~[],
					})
				],
				space	: QuatSpace{
					scale	: 1.00,
					pos	: [0.00, 0.00, 3.42],
					rot	: [0.00, 0.00, 0.00, 1.00],
				},
				actions	: ~[],
			}),
			ChildNode(Node{
				name	: ~"Plane",
				children	: ~[
					ChildEntity(Entity{
						armature	: ~"",
						range	: [0, 1536],
						material	: ~"Material",
						mesh	: ~"Plane@all",
					})
				],
				space	: QuatSpace{
					scale	: 1.00,
					pos	: [0.00, 0.00, 0.00],
					rot	: [0.00, 0.00, 0.00, 1.00],
				},
				actions	: ~[],
			}),
			ChildNode(Node{
				name	: ~"Lamp",
				children	: ~[
					ChildLight(Light{
						name	: ~"Lamp",
						attenuation	: [0.00, 1.00],
						kind	: KindOmni(Omni),
						distance	: 30.00,
						color	: [1.00, 1.00, 1.00],
						spherical	: false,
						energy	: 1.00,
					})
				],
				space	: QuatSpace{
					scale	: 1.00,
					pos	: [4.08, 1.01, 5.90],
					rot	: [0.17, 0.27, 0.76, 0.57],
				},
				actions	: ~[],
			}),
			ChildNode(Node{
				name	: ~"Camera",
				children	: ~[
					ChildCamera(Camera{
						name	: ~"Camera",
						range	: [0.10, 100.00],
						fov_y	: 0.86,
					})
				],
				space	: QuatSpace{
					scale	: 1.00,
					pos	: [7.48, -6.51, 5.34],
					rot	: [0.48, 0.21, 0.33, 0.78],
				},
				actions	: ~[],
			})
		],
		global	: Global{
			gravity	: [0.00, 0.00, -9.81],
		},
		materials	: ~[
			Material{
				name	: ~"Material",
				shader	: ~"phong",
				textures	: ~[
					Texture{
						name	: ~"Texture",
						scale	: [10.00, 10.00, 1.00],
						filter	: 3,
						wrap	: 0,
						offset	: [0.00, 0.00, 0.00],
						path	: ~"//SoilCracked0103_2_S.jpg",
					}
				],
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			}
		],
	}}
