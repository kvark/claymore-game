use common::*;
pub fn load()-> Scene	{Scene{
		nodes	: ~[
			ChildNode(Node{
				actions	: ~[],
				name	: ~"ArmatureBoss",
				children	: ~[
					ChildArmature(Armature{
						bones	: ~[
							Bone{
								name	: ~"Bone",
								children	: ~[],
								space	: Space{
									scale	: 1.00,
									rot	: [0.71, 0.00, 0.00, 0.71],
									pos	: [0.00, 0.00, 0.00],
								},
							}
						],
						name	: ~"ArmatureBoss",
						actions	: ~[
							~"ArmatureBossAction@ArmatureBoss"
						],
						dual_quat	: false,
					}),
					ChildNode(Node{
						actions	: ~[],
						name	: ~"Boss",
						children	: ~[
							ChildEntity(Entity{
								armature	: ~"ArmatureBoss",
								range	: [0, 714],
								material	: ~"MatBoss",
								mesh	: ~"Boss@all",
							})
						],
						space	: Space{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.97, -0.64, 0.12],
						},
					})
				],
				space	: Space{
					scale	: 1.00,
					rot	: [0.00, 0.00, 0.00, 1.00],
					pos	: [5.39, -1.01, 1.05],
				},
			}),
			ChildNode(Node{
				actions	: ~[],
				name	: ~"Armature",
				children	: ~[
					ChildArmature(Armature{
						bones	: ~[
							Bone{
								name	: ~"Bone",
								children	: ~[],
								space	: Space{
									scale	: 1.00,
									rot	: [0.71, 0.00, 0.00, 0.71],
									pos	: [-0.00, 0.00, -0.07],
								},
							},
							Bone{
								name	: ~"Bone.001",
								children	: ~[
									Bone{
										name	: ~"Bone.002",
										children	: ~[],
										space	: Space{
											scale	: 1.00,
											rot	: [-0.05, 0.19, 0.03, 0.98],
											pos	: [-0.00, 0.67, 0.00],
										},
									}
								],
								space	: Space{
									scale	: 1.00,
									rot	: [0.02, -0.38, -0.01, 0.92],
									pos	: [-0.00, 0.00, -0.07],
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
												space	: Space{
													scale	: 1.00,
													rot	: [-0.26, 0.70, -0.65, 0.15],
													pos	: [0.00, 0.53, 0.00],
												},
											}
										],
										space	: Space{
											scale	: 1.00,
											rot	: [-0.00, 1.00, -0.01, 0.09],
											pos	: [-0.00, 0.58, 0.00],
										},
									}
								],
								space	: Space{
									scale	: 1.00,
									rot	: [-0.00, -0.00, 1.00, 0.00],
									pos	: [-0.00, 0.00, -0.07],
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
												children	: ~[
													Bone{
														name	: ~"Bone.012",
														children	: ~[],
														space	: Space{
															scale	: 1.00,
															rot	: [0.50, -0.50, 0.50, 0.50],
															pos	: [-0.00, 0.57, -0.00],
														},
													}
												],
												space	: Space{
													scale	: 1.00,
													rot	: [-0.20, 0.03, 0.01, 0.98],
													pos	: [0.00, 0.51, 0.00],
												},
											}
										],
										space	: Space{
											scale	: 1.00,
											rot	: [0.19, 0.13, -0.04, 0.97],
											pos	: [-0.00, 0.64, -0.00],
										},
									},
									Bone{
										name	: ~"Bone.009",
										children	: ~[
											Bone{
												name	: ~"Bone.010",
												children	: ~[
													Bone{
														name	: ~"Bone.011",
														children	: ~[],
														space	: Space{
															scale	: 1.00,
															rot	: [0.51, -0.51, 0.49, 0.49],
															pos	: [-0.00, 0.58, 0.00],
														},
													}
												],
												space	: Space{
													scale	: 1.00,
													rot	: [0.00, 0.98, -0.19, -0.00],
													pos	: [0.00, 0.52, 0.00],
												},
											}
										],
										space	: Space{
											scale	: 1.00,
											rot	: [0.03, -0.96, 0.21, 0.16],
											pos	: [-0.00, 0.64, -0.00],
										},
									}
								],
								space	: Space{
									scale	: 1.00,
									rot	: [-0.11, 0.70, -0.70, 0.11],
									pos	: [-0.00, 0.00, -0.07],
								},
							}
						],
						name	: ~"Armature",
						actions	: ~[
							~"ArmatureBossAction@Armature"
						],
						dual_quat	: false,
					}),
					ChildNode(Node{
						actions	: ~[],
						name	: ~"Player",
						children	: ~[
							ChildEntity(Entity{
								armature	: ~"Armature",
								range	: [0, 2844],
								material	: ~"MatPlayer",
								mesh	: ~"Player@all",
							})
						],
						space	: Space{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, -0.42],
						},
					})
				],
				space	: Space{
					scale	: 1.00,
					rot	: [0.00, 0.00, 0.00, 1.00],
					pos	: [0.00, -2.00, 1.68],
				},
			}),
			ChildNode(Node{
				actions	: ~[],
				name	: ~"Plane",
				children	: ~[
					ChildEntity(Entity{
						armature	: ~"",
						range	: [0, 1536],
						material	: ~"Material",
						mesh	: ~"Plane@all",
					})
				],
				space	: Space{
					scale	: 1.00,
					rot	: [0.00, 0.00, 0.00, 1.00],
					pos	: [0.00, 0.00, 0.00],
				},
			}),
			ChildNode(Node{
				actions	: ~[],
				name	: ~"Lamp",
				children	: ~[
					ChildLight(Light{
						spherical	: false,
						kind	: KindOmni(Omni),
						attenuation	: [0.00, 1.00],
						name	: ~"Lamp",
						color	: [1.00, 1.00, 1.00],
						distance	: 30.00,
						energy	: 1.00,
					})
				],
				space	: Space{
					scale	: 1.00,
					rot	: [0.17, 0.27, 0.76, 0.57],
					pos	: [4.08, 1.01, 5.90],
				},
			}),
			ChildNode(Node{
				actions	: ~[],
				name	: ~"Camera",
				children	: ~[
					ChildCamera(Camera{
						fov_y	: 0.86,
						range	: [0.10, 100.00],
						name	: ~"Camera",
					})
				],
				space	: Space{
					scale	: 1.00,
					rot	: [0.48, 0.21, 0.33, 0.78],
					pos	: [7.48, -6.51, 5.34],
				},
			})
		],
		global	: Global{
			gravity	: [0.00, 0.00, -9.81],
		},
		materials	: ~[
			Material{
				textures	: ~[
					Texture{
						filter	: 3,
						path	: ~"//diffuse.jpg",
						wrap	: 0,
						name	: ~"Texture.001",
						offset	: [0.00, 0.00, 0.00],
						scale	: [1.00, 1.00, 1.00],
					}
				],
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				name	: ~"MatBoss",
				shader	: ~"phong",
			},
			Material{
				textures	: ~[
					Texture{
						filter	: 3,
						path	: ~"//../../data/texture/SoilCracked0103_2_S.jpg",
						wrap	: 0,
						name	: ~"Texture",
						offset	: [0.00, 0.00, 0.00],
						scale	: [10.00, 10.00, 1.00],
					}
				],
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				name	: ~"Material",
				shader	: ~"phong",
			},
			Material{
				textures	: ~[
					Texture{
						filter	: 3,
						path	: ~"//diffuse.jpg",
						wrap	: 0,
						name	: ~"Texture.002",
						offset	: [0.00, 0.00, 0.00],
						scale	: [1.00, 1.00, 1.00],
					}
				],
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				name	: ~"MatPlayer",
				shader	: ~"phong",
			}
		],
	}}
