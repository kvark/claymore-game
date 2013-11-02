use common::*;
pub fn load()-> Scene	{Scene{
		global	: Global{
			gravity	: [0.00, 0.00, -9.81],
		},
		materials	: ~[
			Material{
				name	: ~"anisotropic1",
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.09, 0.09, 0.08])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				name	: ~"armor",
				textures	: ~[
					Texture{
						scale	: [1.00, 1.00, 1.00],
						wrap	: 0,
						name	: ~"Main",
						filter	: 3,
						offset	: [0.00, 0.00, 0.00],
						path	: ~"//Metal_R-Spec_G-Bump_B-Reflect.jpg",
					}
				],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				name	: ~"cloak",
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				name	: ~"cornea",
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.50, 0.50, 0.50])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
			},
			Material{
				name	: ~"EyeLashes",
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				name	: ~"Eyes",
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.44, 0.44, 0.54])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.49, 0.49, 0.49])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
			},
			Material{
				name	: ~"Material",
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				name	: ~"Pupil_SS",
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				name	: ~"skin",
				textures	: ~[
					Texture{
						scale	: [1.00, 1.00, 1.00],
						wrap	: 0,
						name	: ~"Main.001",
						filter	: 3,
						offset	: [0.00, 0.00, 0.00],
						path	: ~"//Skin_Diffuse.jpg",
					}
				],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
			},
			Material{
				name	: ~"Teeth",
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.75, 0.75, 0.75])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				name	: ~"Tongue",
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.40, 0.08, 0.08])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.50, 0.50, 0.50])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
			}
		],
		nodes	: ~[
			ChildNode(Node{
				name	: ~"Plane",
				children	: ~[
					ChildEntity(Entity{
						mesh	: ~"Plane@all",
						material	: ~"Material",
						armature	: ~"",
						range	: [0, 6],
					})
				],
				space	: Space{
					scale	: 100.00,
					pos	: [0.00, 0.00, -1.00],
					rot	: [0.00, 0.00, 0.00, 1.00],
				},
				actions	: ~[],
			}),
			ChildNode(Node{
				name	: ~"Camera",
				children	: ~[
					ChildCamera(Camera{
						fov_y	: 0.87,
						name	: ~"Camera",
						range	: [10.00, 300.00],
					})
				],
				space	: Space{
					scale	: 1.00,
					pos	: [140.00, 0.00, 90.00],
					rot	: [0.41, 0.41, 0.58, 0.58],
				},
				actions	: ~[],
			}),
			ChildNode(Node{
				name	: ~"Clare",
				children	: ~[
					ChildNode(Node{
						name	: ~"R_ik_foot_grp",
						children	: ~[
							ChildNode(Node{
								name	: ~"ikHandle8",
								children	: ~[],
								space	: Space{
									scale	: 1.00,
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								actions	: ~[],
							})
						],
						space	: Space{
							scale	: 1.00,
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						actions	: ~[],
					}),
					ChildNode(Node{
						name	: ~"L_leg_ikHandle_zero.001",
						children	: ~[],
						space	: Space{
							scale	: 1.00,
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						actions	: ~[],
					}),
					ChildNode(Node{
						name	: ~"L_ik_foot_grp",
						children	: ~[
							ChildNode(Node{
								name	: ~"ikHandle7",
								children	: ~[],
								space	: Space{
									scale	: 1.00,
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								actions	: ~[],
							})
						],
						space	: Space{
							scale	: 1.00,
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						actions	: ~[],
					}),
					ChildNode(Node{
						name	: ~"Transform",
						children	: ~[
							ChildNode(Node{
								name	: ~"Controls",
								children	: ~[
									ChildNode(Node{
										name	: ~"c_worldTransform_ctrl",
										children	: ~[
											ChildNode(Node{
												name	: ~"Armature.002",
												children	: ~[
													ChildArmature(Armature{
														dual_quat	: false,
														name	: ~"Armature.002",
														bones	: ~[
															Bone{
																name	: ~"cog",
																children	: ~[
																	Bone{
																		name	: ~"c_spine_001_joint",
																		children	: ~[
																			Bone{
																				name	: ~"c_spine_002_joint",
																				children	: ~[
																					Bone{
																						name	: ~"c_spine_003_joint",
																						children	: ~[
																							Bone{
																								name	: ~"c_spine_004_joint",
																								children	: ~[
																									Bone{
																										name	: ~"c_spine_005_joint",
																										children	: ~[
																											Bone{
																												name	: ~"c_spine_006_joint",
																												children	: ~[
																													Bone{
																														name	: ~"c_spine_007_joint",
																														children	: ~[
																															Bone{
																																name	: ~"c_neck_01_joint",
																																children	: ~[
																																	Bone{
																																		name	: ~"c_neck_02_joint",
																																		children	: ~[
																																			Bone{
																																				name	: ~"head_joint",
																																				children	: ~[
																																					Bone{
																																						name	: ~"head_end",
																																						children	: ~[
																																							Bone{
																																								name	: ~"L_eye_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"L_eye_end_joint",
																																										children	: ~[],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [0.00, 0.69, 0.00],
																																											rot	: [0.00, 0.71, -0.00, 0.70],
																																										},
																																									}
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [1.26, -4.62, 3.06],
																																									rot	: [0.71, -0.01, 0.01, 0.71],
																																								},
																																							},
																																							Bone{
																																								name	: ~"R_eye_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"R_eye_end_joint",
																																										children	: ~[],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [0.00, 0.69, -0.00],
																																											rot	: [-0.00, 0.70, -0.00, 0.71],
																																										},
																																									}
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [-1.26, -4.62, 3.06],
																																									rot	: [0.71, 0.01, -0.01, 0.71],
																																								},
																																							},
																																							Bone{
																																								name	: ~"R_eye_blink_base_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"R_eye_blink_01_joint",
																																										children	: ~[],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [-0.00, 0.50, -0.00],
																																											rot	: [0.00, 0.71, -0.00, 0.71],
																																										},
																																									}
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [-1.26, -4.62, 3.06],
																																									rot	: [0.77, -0.00, 0.00, 0.64],
																																								},
																																							},
																																							Bone{
																																								name	: ~"L_eye_blink_base_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"L_eye_blink_01_joint",
																																										children	: ~[],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [0.00, 0.50, -0.00],
																																											rot	: [0.00, 0.71, -0.00, 0.71],
																																										},
																																									}
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [1.26, -4.62, 3.06],
																																									rot	: [0.77, -0.00, 0.00, 0.64],
																																								},
																																							},
																																							Bone{
																																								name	: ~"R_eye_blink_02_base_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"R_eye_blink_02_joint",
																																										children	: ~[],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [-0.00, 0.67, -0.00],
																																											rot	: [-0.00, 0.71, -0.00, 0.71],
																																										},
																																									}
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [-1.26, -4.62, 3.06],
																																									rot	: [0.60, -0.00, 0.00, 0.80],
																																								},
																																							},
																																							Bone{
																																								name	: ~"L_eye_blink_02_base_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"L_eye_blink_02_joint",
																																										children	: ~[],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [-0.00, 0.67, -0.00],
																																											rot	: [0.00, 0.71, 0.00, 0.71],
																																										},
																																									}
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [1.26, -4.62, 3.06],
																																									rot	: [0.60, -0.00, 0.00, 0.80],
																																								},
																																							}
																																						],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [-0.00, 6.34, 0.04],
																																							rot	: [-0.04, 0.00, 0.00, 1.00],
																																						},
																																					},
																																					Bone{
																																						name	: ~"jaw_joint",
																																						children	: ~[
																																							Bone{
																																								name	: ~"jaw_end_joint",
																																								children	: ~[],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [0.00, 3.68, -0.00],
																																									rot	: [-0.00, 0.71, 0.00, 0.71],
																																								},
																																							}
																																						],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [-0.00, -0.41, 0.55],
																																							rot	: [0.82, 0.00, 0.00, 0.57],
																																						},
																																					}
																																				],
																																				space	: Space{
																																					scale	: 1.00,
																																					pos	: [0.00, 2.37, -0.00],
																																					rot	: [-0.00, 0.00, 0.00, 1.00],
																																				},
																																			}
																																		],
																																		space	: Space{
																																			scale	: 1.00,
																																			pos	: [-0.00, 2.10, -1.00],
																																			rot	: [-0.00, 0.98, -0.21, -0.00],
																																		},
																																	}
																																],
																																space	: Space{
																																	scale	: 1.00,
																																	pos	: [0.00, 0.44, -0.29],
																																	rot	: [0.00, -0.00, 0.00, 1.00],
																																},
																															},
																															Bone{
																																name	: ~"c_shealth_01_joint",
																																children	: ~[
																																	Bone{
																																		name	: ~"c_shealth_end_joint",
																																		children	: ~[],
																																		space	: Space{
																																			scale	: 1.00,
																																			pos	: [0.00, 1.81, 0.00],
																																			rot	: [0.00, 1.00, -0.00, -0.00],
																																		},
																																	}
																																],
																																space	: Space{
																																	scale	: 1.00,
																																	pos	: [-0.00, 0.04, 2.55],
																																	rot	: [-0.00, 0.16, 0.99, 0.00],
																																},
																															},
																															Bone{
																																name	: ~"L_clav_joint",
																																children	: ~[
																																	Bone{
																																		name	: ~"L_clav_end_joint",
																																		children	: ~[
																																			Bone{
																																				name	: ~"L_shoulder_joint",
																																				children	: ~[
																																					Bone{
																																						name	: ~"L_arm_01_joint",
																																						children	: ~[
																																							Bone{
																																								name	: ~"L_arm_02_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"L_elbow_joint",
																																										children	: ~[
																																											Bone{
																																												name	: ~"L_forearm_01_joint",
																																												children	: ~[
																																													Bone{
																																														name	: ~"L_forearm_02_joint",
																																														children	: ~[
																																															Bone{
																																																name	: ~"L_wrist_joint",
																																																children	: ~[
																																																	Bone{
																																																		name	: ~"L_wrist_end_joint",
																																																		children	: ~[
																																																			Bone{
																																																				name	: ~"L_pinkyFinger_01_joint",
																																																				children	: ~[
																																																					Bone{
																																																						name	: ~"L_pinkyFinger_02_joint",
																																																						children	: ~[
																																																							Bone{
																																																								name	: ~"L_pinkyFinger_03_joint",
																																																								children	: ~[
																																																									Bone{
																																																										name	: ~"L_pinkyFinger_04_joint",
																																																										children	: ~[],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 0.96, 0.00],
																																																											rot	: [0.00, 0.94, 0.00, 0.35],
																																																										},
																																																									}
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 0.75, 0.00],
																																																									rot	: [0.12, 0.34, 0.07, 0.93],
																																																								},
																																																							}
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [-0.00, 1.18, 0.00],
																																																							rot	: [0.25, 0.56, -0.16, 0.77],
																																																						},
																																																					}
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [-0.54, -0.80, -1.23],
																																																					rot	: [-0.37, 0.10, -0.71, 0.59],
																																																				},
																																																			},
																																																			Bone{
																																																				name	: ~"L_ringFinger_01_joint",
																																																				children	: ~[
																																																					Bone{
																																																						name	: ~"L_ringFinger_02_joint",
																																																						children	: ~[
																																																							Bone{
																																																								name	: ~"L_ringFinger_03_joint",
																																																								children	: ~[
																																																									Bone{
																																																										name	: ~"L_ringFinger_04_joint",
																																																										children	: ~[],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 1.00, 0.00],
																																																											rot	: [0.00, 0.96, -0.00, 0.29],
																																																										},
																																																									}
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [0.00, 0.95, -0.00],
																																																									rot	: [0.12, 0.37, 0.10, 0.92],
																																																								},
																																																							}
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.63, 0.00],
																																																							rot	: [0.27, 0.81, -0.24, 0.46],
																																																						},
																																																					}
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [0.26, -0.29, -0.98],
																																																					rot	: [-0.54, -0.07, -0.59, 0.60],
																																																				},
																																																			},
																																																			Bone{
																																																				name	: ~"L_middleFinger_01_joint",
																																																				children	: ~[
																																																					Bone{
																																																						name	: ~"L_middleFinger_02_joint",
																																																						children	: ~[
																																																							Bone{
																																																								name	: ~"L_middleFinger_03_joint",
																																																								children	: ~[
																																																									Bone{
																																																										name	: ~"L_middleFinger_04_joint",
																																																										children	: ~[],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [0.00, 1.24, 0.00],
																																																											rot	: [-0.00, 0.24, 0.00, 0.97],
																																																										},
																																																									}
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [0.00, 1.01, -0.00],
																																																									rot	: [0.07, 0.19, 0.02, 0.98],
																																																								},
																																																							}
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.64, 0.00],
																																																							rot	: [0.16, 0.83, -0.31, 0.44],
																																																						},
																																																					}
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [0.92, 0.08, -0.55],
																																																					rot	: [-0.58, -0.15, -0.55, 0.58],
																																																				},
																																																			},
																																																			Bone{
																																																				name	: ~"L_indexFinger_01_joint",
																																																				children	: ~[
																																																					Bone{
																																																						name	: ~"L_indexFinger_02_joint",
																																																						children	: ~[
																																																							Bone{
																																																								name	: ~"L_indexFinger_03_joint",
																																																								children	: ~[
																																																									Bone{
																																																										name	: ~"L_indexFinger_04_joint",
																																																										children	: ~[],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 1.09, -0.00],
																																																											rot	: [-0.00, 0.99, -0.00, 0.17],
																																																										},
																																																									}
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 1.03, -0.00],
																																																									rot	: [0.14, 0.36, -0.05, 0.92],
																																																								},
																																																							}
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.48, -0.00],
																																																							rot	: [-0.05, 0.73, -0.36, 0.58],
																																																						},
																																																					}
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [1.60, 0.39, 0.17],
																																																					rot	: [-0.63, -0.19, -0.46, 0.59],
																																																				},
																																																			}
																																																		],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [-0.06, 2.48, -0.37],
																																																			rot	: [0.11, -0.29, 0.75, 0.59],
																																																		},
																																																	},
																																																	Bone{
																																																		name	: ~"L_thumb_01_joint",
																																																		children	: ~[
																																																			Bone{
																																																				name	: ~"L_thumb_02_joint",
																																																				children	: ~[
																																																					Bone{
																																																						name	: ~"L_thumb_03_joint",
																																																						children	: ~[
																																																							Bone{
																																																								name	: ~"L_thumb_04_joint",
																																																								children	: ~[],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [0.00, 1.17, 0.00],
																																																									rot	: [0.00, -0.82, 0.00, 0.57],
																																																								},
																																																							}
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [-0.00, 1.10, -0.00],
																																																							rot	: [0.01, 0.12, 0.02, 0.99],
																																																						},
																																																					}
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [0.00, 1.91, 0.00],
																																																					rot	: [-0.06, -0.32, -0.13, 0.94],
																																																				},
																																																			}
																																																		],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [-0.43, 1.21, 0.79],
																																																			rot	: [0.38, 0.51, 0.19, 0.75],
																																																		},
																																																	}
																																																],
																																																space	: Space{
																																																	scale	: 1.00,
																																																	pos	: [0.00, 5.42, -0.00],
																																																	rot	: [-0.00, 0.02, 0.00, 1.00],
																																																},
																																															}
																																														],
																																														space	: Space{
																																															scale	: 1.00,
																																															pos	: [-0.00, 2.64, -0.00],
																																															rot	: [-0.00, -0.02, 0.02, 1.00],
																																														},
																																													}
																																												],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [0.00, 2.77, 0.00],
																																													rot	: [0.00, 0.01, 0.00, 1.00],
																																												},
																																											}
																																										],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [-0.00, 3.95, -0.00],
																																											rot	: [0.24, 0.27, 0.02, 0.93],
																																										},
																																									}
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [0.00, 3.58, 0.00],
																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																								},
																																							}
																																						],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [0.00, 3.42, 0.00],
																																							rot	: [0.00, 0.91, 0.00, 0.40],
																																						},
																																					}
																																				],
																																				space	: Space{
																																					scale	: 1.00,
																																					pos	: [-0.52, 0.95, -0.03],
																																					rot	: [-0.15, 0.73, 0.20, 0.63],
																																				},
																																			},
																																			Bone{
																																				name	: ~"L_mainSpaulder_joint",
																																				children	: ~[
																																					Bone{
																																						name	: ~"L_mainSpaulder_end_joint",
																																						children	: ~[
																																							Bone{
																																								name	: ~"L_subSpaulder_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"L_subSpaulder_end_joint",
																																										children	: ~[],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [0.00, 2.16, -0.00],
																																											rot	: [0.00, 0.87, 0.00, 0.50],
																																										},
																																									}
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [0.00, 1.03, -0.00],
																																									rot	: [0.24, 0.35, -0.81, 0.40],
																																								},
																																							}
																																						],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [0.00, 3.22, 0.00],
																																							rot	: [0.33, -0.01, 0.91, 0.25],
																																						},
																																					}
																																				],
																																				space	: Space{
																																					scale	: 1.00,
																																					pos	: [-0.31, -0.70, 0.27],
																																					rot	: [-0.37, 0.26, -0.12, 0.89],
																																				},
																																			},
																																			Bone{
																																				name	: ~"L_armIK_01_joint",
																																				children	: ~[
																																					Bone{
																																						name	: ~"L_armIK_02_joint",
																																						children	: ~[
																																							Bone{
																																								name	: ~"L_armIK_03_joint",
																																								children	: ~[],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [-0.00, 10.84, -0.00],
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																								},
																																							}
																																						],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [0.00, 10.95, 0.00],
																																							rot	: [0.13, 0.96, -0.21, 0.13],
																																						},
																																					}
																																				],
																																				space	: Space{
																																					scale	: 1.00,
																																					pos	: [-0.52, 0.95, -0.03],
																																					rot	: [-0.15, 0.73, 0.20, 0.63],
																																				},
																																			}
																																		],
																																		space	: Space{
																																			scale	: 1.00,
																																			pos	: [-0.00, 5.59, -0.00],
																																			rot	: [-0.00, -0.93, -0.00, 0.36],
																																		},
																																	}
																																],
																																space	: Space{
																																	scale	: 1.00,
																																	pos	: [-1.00, -2.13, -2.41],
																																	rot	: [-0.31, 0.71, 0.50, 0.39],
																																},
																															},
																															Bone{
																																name	: ~"R_clav_joint",
																																children	: ~[
																																	Bone{
																																		name	: ~"R_clav_end_joint",
																																		children	: ~[
																																			Bone{
																																				name	: ~"R_shoulder_joint",
																																				children	: ~[
																																					Bone{
																																						name	: ~"R_arm_01_joint",
																																						children	: ~[
																																							Bone{
																																								name	: ~"R_arm_02_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"R_elbow_joint",
																																										children	: ~[
																																											Bone{
																																												name	: ~"R_forearm_01_joint",
																																												children	: ~[
																																													Bone{
																																														name	: ~"R_forearm_02_joint",
																																														children	: ~[
																																															Bone{
																																																name	: ~"R_wrist_joint",
																																																children	: ~[
																																																	Bone{
																																																		name	: ~"R_thumb_01_joint",
																																																		children	: ~[
																																																			Bone{
																																																				name	: ~"R_thumb_02_joint",
																																																				children	: ~[
																																																					Bone{
																																																						name	: ~"R_thumb_03_joint",
																																																						children	: ~[
																																																							Bone{
																																																								name	: ~"R_thumb_04_joint",
																																																								children	: ~[],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 1.17, 0.00],
																																																									rot	: [-0.00, 0.82, -0.00, 0.57],
																																																								},
																																																							}
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.10, -0.00],
																																																							rot	: [0.01, -0.12, -0.02, 0.99],
																																																						},
																																																					}
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [0.00, 1.91, -0.00],
																																																					rot	: [-0.06, 0.32, 0.13, 0.94],
																																																				},
																																																			}
																																																		],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [0.43, 1.21, 0.79],
																																																			rot	: [0.38, -0.51, -0.19, 0.75],
																																																		},
																																																	},
																																																	Bone{
																																																		name	: ~"R_wrist_end_joint",
																																																		children	: ~[
																																																			Bone{
																																																				name	: ~"R_indexFinger_01_joint",
																																																				children	: ~[
																																																					Bone{
																																																						name	: ~"R_indexFinger_02_joint",
																																																						children	: ~[
																																																							Bone{
																																																								name	: ~"R_indexFinger_03_joint",
																																																								children	: ~[
																																																									Bone{
																																																										name	: ~"R_indexFinger_04_joint",
																																																										children	: ~[],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 1.09, -0.00],
																																																											rot	: [-0.00, -0.99, 0.00, 0.17],
																																																										},
																																																									}
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [0.00, 1.03, -0.00],
																																																									rot	: [0.15, -0.31, 0.02, 0.94],
																																																								},
																																																							}
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [-0.00, 1.48, -0.00],
																																																							rot	: [0.07, -0.67, 0.35, 0.65],
																																																						},
																																																					}
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [-1.60, 0.39, 0.17],
																																																					rot	: [-0.45, 0.13, 0.60, 0.65],
																																																				},
																																																			},
																																																			Bone{
																																																				name	: ~"R_middleFinger_01_joint",
																																																				children	: ~[
																																																					Bone{
																																																						name	: ~"R_middleFinger_02_joint",
																																																						children	: ~[
																																																							Bone{
																																																								name	: ~"R_middleFinger_03_joint",
																																																								children	: ~[
																																																									Bone{
																																																										name	: ~"R_middleFinger_04_joint",
																																																										children	: ~[],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 1.24, -0.00],
																																																											rot	: [0.00, -0.24, -0.00, 0.97],
																																																										},
																																																									}
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 1.01, -0.00],
																																																									rot	: [0.07, -0.19, -0.02, 0.98],
																																																								},
																																																							}
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.64, 0.00],
																																																							rot	: [0.16, -0.83, 0.31, 0.44],
																																																						},
																																																					}
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [-0.92, 0.08, -0.55],
																																																					rot	: [-0.58, 0.15, 0.55, 0.58],
																																																				},
																																																			},
																																																			Bone{
																																																				name	: ~"R_ringFinger_01_joint",
																																																				children	: ~[
																																																					Bone{
																																																						name	: ~"R_ringFinger_02_joint",
																																																						children	: ~[
																																																							Bone{
																																																								name	: ~"R_ringFinger_03_joint",
																																																								children	: ~[
																																																									Bone{
																																																										name	: ~"R_ringFinger_04_joint",
																																																										children	: ~[],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [0.00, 1.00, 0.00],
																																																											rot	: [-0.00, -0.96, -0.00, 0.29],
																																																										},
																																																									}
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 0.95, -0.00],
																																																									rot	: [0.12, -0.41, -0.09, 0.90],
																																																								},
																																																							}
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [-0.00, 1.63, -0.00],
																																																							rot	: [0.23, -0.84, 0.28, 0.40],
																																																						},
																																																					}
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [-0.26, -0.29, -0.98],
																																																					rot	: [-0.62, 0.09, 0.52, 0.57],
																																																				},
																																																			},
																																																			Bone{
																																																				name	: ~"R_pinkyFinger_01_joint",
																																																				children	: ~[
																																																					Bone{
																																																						name	: ~"R_pinkyFinger_02_joint",
																																																						children	: ~[
																																																							Bone{
																																																								name	: ~"R_pinkyFinger_03_joint",
																																																								children	: ~[
																																																									Bone{
																																																										name	: ~"R_pinkyFinger_04_joint",
																																																										children	: ~[],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [0.00, 0.96, 0.00],
																																																											rot	: [0.00, -0.94, 0.00, 0.35],
																																																										},
																																																									}
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 0.75, 0.00],
																																																									rot	: [0.12, -0.42, -0.07, 0.90],
																																																								},
																																																							}
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.18, 0.00],
																																																							rot	: [0.21, -0.68, 0.22, 0.67],
																																																						},
																																																					}
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [0.54, -0.80, -1.23],
																																																					rot	: [-0.57, -0.04, 0.62, 0.53],
																																																				},
																																																			}
																																																		],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [0.06, 2.48, -0.37],
																																																			rot	: [0.11, 0.29, -0.75, 0.59],
																																																		},
																																																	}
																																																],
																																																space	: Space{
																																																	scale	: 1.00,
																																																	pos	: [0.00, 5.42, -0.00],
																																																	rot	: [0.00, -0.02, 0.00, 1.00],
																																																},
																																															}
																																														],
																																														space	: Space{
																																															scale	: 1.00,
																																															pos	: [0.00, 2.64, -0.00],
																																															rot	: [-0.00, 0.02, -0.02, 1.00],
																																														},
																																													}
																																												],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [0.00, 2.77, 0.00],
																																													rot	: [-0.00, -0.01, -0.00, 1.00],
																																												},
																																											}
																																										],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [0.00, 3.95, -0.00],
																																											rot	: [0.24, -0.27, -0.02, 0.93],
																																										},
																																									}
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [0.00, 3.58, -0.00],
																																									rot	: [0.00, -0.00, -0.00, 1.00],
																																								},
																																							}
																																						],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [0.00, 3.42, -0.00],
																																							rot	: [0.00, -0.21, 0.00, 0.98],
																																						},
																																					}
																																				],
																																				space	: Space{
																																					scale	: 1.00,
																																					pos	: [-0.31, 0.95, 0.42],
																																					rot	: [0.11, 0.20, 0.23, 0.95],
																																				},
																																			},
																																			Bone{
																																				name	: ~"R_mainSpaulder_joint",
																																				children	: ~[
																																					Bone{
																																						name	: ~"R_mainSpaulder_end_joint",
																																						children	: ~[
																																							Bone{
																																								name	: ~"R_subSpaulder_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"R_subSpaulder_end_joint",
																																										children	: ~[],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [-0.00, 2.16, 0.00],
																																											rot	: [-0.00, 0.38, 0.00, 0.93],
																																										},
																																									}
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [-0.00, 1.03, -0.00],
																																									rot	: [0.24, -0.35, 0.81, 0.40],
																																								},
																																							}
																																						],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [0.00, 3.22, 0.00],
																																							rot	: [-0.54, 0.21, -0.80, 0.14],
																																						},
																																					}
																																				],
																																				space	: Space{
																																					scale	: 1.00,
																																					pos	: [-0.40, -0.70, 0.06],
																																					rot	: [0.38, 0.65, -0.05, 0.65],
																																				},
																																			},
																																			Bone{
																																				name	: ~"R_armIK_01_joint",
																																				children	: ~[
																																					Bone{
																																						name	: ~"R_armIK_02_joint",
																																						children	: ~[
																																							Bone{
																																								name	: ~"R_armIK_03_joint",
																																								children	: ~[],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [0.00, 10.84, 0.00],
																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																								},
																																							}
																																						],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [0.00, 10.95, -0.00],
																																							rot	: [0.24, -0.46, 0.02, 0.85],
																																						},
																																					}
																																				],
																																				space	: Space{
																																					scale	: 1.00,
																																					pos	: [-0.31, 0.95, 0.42],
																																					rot	: [0.11, 0.20, 0.23, 0.95],
																																				},
																																			}
																																		],
																																		space	: Space{
																																			scale	: 1.00,
																																			pos	: [0.00, 5.59, -0.00],
																																			rot	: [0.00, -0.72, -0.00, 0.70],
																																		},
																																	}
																																],
																																space	: Space{
																																	scale	: 1.00,
																																	pos	: [1.00, -2.13, -2.41],
																																	rot	: [-0.32, -0.71, -0.50, 0.38],
																																},
																															}
																														],
																														space	: Space{
																															scale	: 1.00,
																															pos	: [0.00, 6.39, -0.06],
																															rot	: [-0.00, 1.00, -0.00, -0.00],
																														},
																													},
																													Bone{
																														name	: ~"L_breast_base_joint",
																														children	: ~[
																															Bone{
																																name	: ~"L_breast_joint",
																																children	: ~[
																																	Bone{
																																		name	: ~"L_breast_end_joint",
																																		children	: ~[],
																																		space	: Space{
																																			scale	: 1.00,
																																			pos	: [0.00, 0.00, 0.00],
																																			rot	: [0.00, -0.71, 0.00, 0.71],
																																		},
																																	}
																																],
																																space	: Space{
																																	scale	: 1.00,
																																	pos	: [-0.00, 3.94, -0.00],
																																	rot	: [-0.00, 0.58, -0.00, 0.81],
																																},
																															}
																														],
																														space	: Space{
																															scale	: 1.00,
																															pos	: [3.20, -0.56, 2.70],
																															rot	: [0.84, 0.12, -0.06, 0.52],
																														},
																													},
																													Bone{
																														name	: ~"R_breast_base_joint1",
																														children	: ~[
																															Bone{
																																name	: ~"R_breast_joint1",
																																children	: ~[
																																	Bone{
																																		name	: ~"R_breast_end_joint1",
																																		children	: ~[],
																																		space	: Space{
																																			scale	: 1.00,
																																			pos	: [0.00, -0.00, -0.00],
																																			rot	: [0.00, 0.71, -0.00, 0.71],
																																		},
																																	}
																																],
																																space	: Space{
																																	scale	: 1.00,
																																	pos	: [-0.00, 3.94, -0.00],
																																	rot	: [-0.00, -0.58, 0.00, 0.81],
																																},
																															}
																														],
																														space	: Space{
																															scale	: 1.00,
																															pos	: [-3.20, -0.56, 2.70],
																															rot	: [0.84, -0.12, 0.06, 0.52],
																														},
																													}
																												],
																												space	: Space{
																													scale	: 1.00,
																													pos	: [0.00, 6.18, -0.00],
																													rot	: [-0.00, 0.00, 0.00, 1.00],
																												},
																											}
																										],
																										space	: Space{
																											scale	: 1.00,
																											pos	: [-0.00, 1.50, 0.00],
																											rot	: [-0.11, 0.00, -0.00, 0.99],
																										},
																									}
																								],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 1.45, 0.00],
																									rot	: [-0.02, 0.00, -0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [-0.00, 1.54, 0.00],
																							rot	: [-0.10, 0.00, -0.00, 0.99],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [0.00, 1.49, 0.00],
																					rot	: [-0.00, 0.71, 0.00, 0.71],
																				},
																			}
																		],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [-0.04, 0.00, -0.00],
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																		},
																	},
																	Bone{
																		name	: ~"C_hip_joint",
																		children	: ~[
																			Bone{
																				name	: ~"R_hip_base_joint",
																				children	: ~[
																					Bone{
																						name	: ~"R_hip_joint",
																						children	: ~[],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [-0.00, 4.60, -0.00],
																							rot	: [-0.00, 0.71, 0.00, 0.71],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [4.99, 0.48, 2.82],
																					rot	: [-0.69, -0.14, -0.69, 0.14],
																				},
																			},
																			Bone{
																				name	: ~"R_leg_joint",
																				children	: ~[
																					Bone{
																						name	: ~"R_kneePivot_03_joint",
																						children	: ~[
																							Bone{
																								name	: ~"R_kneePivot_02_joint",
																								children	: ~[
																									Bone{
																										name	: ~"R_kneePivot_01_joint",
																										children	: ~[],
																										space	: Space{
																											scale	: 1.00,
																											pos	: [0.00, 4.95, 0.00],
																											rot	: [0.00, -0.00, 0.00, 1.00],
																										},
																									}
																								],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [0.00, 5.50, -0.00],
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [-0.19, -6.70, -0.11],
																							rot	: [0.85, -0.02, 0.53, -0.00],
																						},
																					},
																					Bone{
																						name	: ~"R_knee_01_joint",
																						children	: ~[
																							Bone{
																								name	: ~"R_ankle_joint",
																								children	: ~[
																									Bone{
																										name	: ~"R_ball_joint",
																										children	: ~[
																											Bone{
																												name	: ~"R_toe_joint",
																												children	: ~[],
																												space	: Space{
																													scale	: 1.00,
																													pos	: [0.00, 2.80, -0.00],
																													rot	: [-0.00, -0.72, 0.00, 0.70],
																												},
																											}
																										],
																										space	: Space{
																											scale	: 1.00,
																											pos	: [-0.00, 8.00, -0.00],
																											rot	: [-0.23, 0.02, 0.01, 0.97],
																										},
																									}
																								],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 19.52, 0.00],
																									rot	: [-0.56, -0.19, -0.14, 0.80],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [-0.53, -18.67, -0.32],
																							rot	: [0.22, -0.06, -0.97, 0.00],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [4.92, 0.46, 4.10],
																					rot	: [0.51, -0.51, 0.49, 0.49],
																				},
																			},
																			Bone{
																				name	: ~"L_hip_base_joint",
																				children	: ~[
																					Bone{
																						name	: ~"L_hip_joint",
																						children	: ~[],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [-0.00, 4.60, 0.00],
																							rot	: [0.00, 0.71, 0.00, 0.71],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [4.99, 0.48, -2.82],
																					rot	: [-0.69, -0.14, -0.69, 0.14],
																				},
																			},
																			Bone{
																				name	: ~"L_leg_joint",
																				children	: ~[
																					Bone{
																						name	: ~"L_kneePivot_03_joint",
																						children	: ~[
																							Bone{
																								name	: ~"L_kneePivot_02_joint",
																								children	: ~[
																									Bone{
																										name	: ~"L_kneePivot_01_joint",
																										children	: ~[],
																										space	: Space{
																											scale	: 1.00,
																											pos	: [0.00, 4.95, -0.00],
																											rot	: [-0.00, 0.00, 0.00, 1.00],
																										},
																									}
																								],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 5.50, 0.00],
																									rot	: [0.00, -0.00, 0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.19, -6.70, -0.06],
																							rot	: [0.95, 0.01, -0.30, -0.00],
																						},
																					},
																					Bone{
																						name	: ~"L_knee_01_joint",
																						children	: ~[
																							Bone{
																								name	: ~"L_ankle_joint",
																								children	: ~[
																									Bone{
																										name	: ~"L_ball_joint",
																										children	: ~[
																											Bone{
																												name	: ~"L_toe_joint",
																												children	: ~[],
																												space	: Space{
																													scale	: 1.00,
																													pos	: [0.00, 2.80, -0.00],
																													rot	: [0.00, -0.69, 0.00, 0.72],
																												},
																											}
																										],
																										space	: Space{
																											scale	: 1.00,
																											pos	: [-0.00, 8.00, 0.00],
																											rot	: [-0.23, -0.06, -0.02, 0.97],
																										},
																									}
																								],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [0.00, 19.52, 0.00],
																									rot	: [-0.55, 0.25, 0.14, 0.79],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.53, -18.67, -0.16],
																							rot	: [0.25, 0.05, 0.97, 0.00],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [4.92, 0.46, -4.10],
																					rot	: [0.49, -0.49, 0.51, 0.51],
																				},
																			},
																			Bone{
																				name	: ~"R_frontMid_skirtplate_01_joint",
																				children	: ~[
																					Bone{
																						name	: ~"R_frontMid_skirtplate_02_joint",
																						children	: ~[
																							Bone{
																								name	: ~"R_frontMid_skirtplate_03_joint",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 5.95, 0.00],
																									rot	: [0.00, -0.00, -0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, -0.00],
																							rot	: [-0.07, 0.99, 0.04, 0.07],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [1.22, 4.37, 3.97],
																					rot	: [0.68, 0.70, -0.05, 0.19],
																				},
																			},
																			Bone{
																				name	: ~"R_frontInner_skirtplate_01_joint",
																				children	: ~[
																					Bone{
																						name	: ~"R_frontInner_skirtplate_02_joint",
																						children	: ~[
																							Bone{
																								name	: ~"R_frontInner_skirtplate_03_joint",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [0.00, 5.95, -0.00],
																									rot	: [0.00, -0.00, -0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, -0.00],
																							rot	: [0.07, -0.93, 0.04, 0.35],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [2.94, 4.48, 2.04],
																					rot	: [-0.68, -0.73, -0.00, 0.00],
																				},
																			},
																			Bone{
																				name	: ~"L_frontInner_skirtplate_01_joint",
																				children	: ~[
																					Bone{
																						name	: ~"L_frontInner_skirtplate_02_joint",
																						children	: ~[
																							Bone{
																								name	: ~"L_frontInner_skirtplate_03_joint",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [0.00, 5.95, 0.00],
																									rot	: [0.00, -0.00, 0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, 0.00],
																							rot	: [0.07, 0.93, -0.04, 0.35],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [2.94, 4.48, -2.04],
																					rot	: [-0.00, -0.00, -0.68, 0.73],
																				},
																			},
																			Bone{
																				name	: ~"L_side_skirtplate_01_joint",
																				children	: ~[
																					Bone{
																						name	: ~"L_side_skirtplate_02_joint",
																						children	: ~[
																							Bone{
																								name	: ~"L_side_skirtplate_03_joint",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 5.95, -0.00],
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, -0.00],
																							rot	: [-0.13, 0.98, 0.14, 0.03],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [-1.03, 1.62, -5.94],
																					rot	: [0.08, 0.39, -0.73, 0.56],
																				},
																			},
																			Bone{
																				name	: ~"L_rearSide_skirtplate_01_joint",
																				children	: ~[
																					Bone{
																						name	: ~"L_rearSide_skirtplate_02_joint",
																						children	: ~[
																							Bone{
																								name	: ~"L_rearSide_skirtplate_03_joint",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 5.95, 0.00],
																									rot	: [0.00, 0.00, 0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [-0.00, 4.80, 0.00],
																							rot	: [0.13, -0.96, -0.19, 0.17],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [-0.29, -2.24, -3.34],
																					rot	: [-0.34, -0.02, -0.78, 0.53],
																				},
																			},
																			Bone{
																				name	: ~"C_rear_skirtplate_01_joint",
																				children	: ~[
																					Bone{
																						name	: ~"C_rear_skirtplate_02_joint",
																						children	: ~[
																							Bone{
																								name	: ~"C_rear_skirtplate_03_joint",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 3.88, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [-0.00, 4.80, -0.00],
																							rot	: [0.01, 0.97, 0.25, 0.04],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [1.63, -3.54, -0.00],
																					rot	: [-0.61, -0.37, -0.60, 0.36],
																				},
																			},
																			Bone{
																				name	: ~"R_rearSide_skirtplate_01_joint",
																				children	: ~[
																					Bone{
																						name	: ~"R_rearSide_skirtplate_02_joint",
																						children	: ~[
																							Bone{
																								name	: ~"R_rearSide_skirtplate_03_joint",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 5.95, -0.00],
																									rot	: [0.00, -0.00, -0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, -0.00],
																							rot	: [0.13, 0.96, 0.19, 0.17],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [-0.29, -2.24, 3.34],
																					rot	: [-0.78, -0.53, -0.34, 0.02],
																				},
																			},
																			Bone{
																				name	: ~"R_side_skirtplate_01_joint",
																				children	: ~[
																					Bone{
																						name	: ~"R_side_skirtplate_02_joint",
																						children	: ~[
																							Bone{
																								name	: ~"R_side_skirtplate_03_joint",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [0.00, 5.95, 0.00],
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, 0.00],
																							rot	: [-0.13, -0.98, -0.14, 0.03],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [-1.03, 1.62, 5.94],
																					rot	: [0.73, 0.56, -0.08, 0.39],
																				},
																			},
																			Bone{
																				name	: ~"L_frontMid_skirtplate_01_joint",
																				children	: ~[
																					Bone{
																						name	: ~"L_frontMid_skirtplate_02_joint",
																						children	: ~[
																							Bone{
																								name	: ~"L_frontMid_skirtplate_03_joint",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [0.00, 5.95, 0.00],
																									rot	: [0.00, 0.00, 0.00, 1.00],
																								},
																							}
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, -0.00],
																							rot	: [-0.07, -0.99, -0.04, 0.07],
																						},
																					}
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [1.22, 4.37, -3.97],
																					rot	: [0.05, 0.19, -0.68, 0.70],
																				},
																			}
																		],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [0.00, -0.10, 0.04],
																			rot	: [0.00, -0.71, 0.00, 0.71],
																		},
																	}
																],
																space	: Space{
																	scale	: 1.00,
																	pos	: [0.00, 47.22, 0.24],
																	rot	: [0.71, 0.00, 0.00, 0.71],
																},
															}
														],
														actions	: ~[
															~"Armature.002Action@Armature.002"
														],
													})
												],
												space	: Space{
													scale	: 1.00,
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												actions	: ~[],
											}),
											ChildNode(Node{
												name	: ~"SKELETON",
												children	: ~[],
												space	: Space{
													scale	: 1.00,
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												actions	: ~[],
											}),
											ChildNode(Node{
												name	: ~"c_eye_ctrl_grp",
												children	: ~[
													ChildNode(Node{
														name	: ~"mainEye_ctrl_zero",
														children	: ~[
															ChildNode(Node{
																name	: ~"mainEye_ctrl",
																children	: ~[
																	ChildNode(Node{
																		name	: ~"R_eye_ctrl_grp",
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"R_eye_ctrl_zero",
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"R_eye_ctrl",
																						children	: ~[],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, 0.00, -0.00],
																							rot	: [0.00, -0.00, -0.00, 1.00],
																						},
																						actions	: ~[],
																					})
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [-0.00, -0.00, 0.00],
																					rot	: [0.00, 0.00, -0.00, 1.00],
																				},
																				actions	: ~[],
																			})
																		],
																		space	: Space{
																			scale	: 0.72,
																			pos	: [-0.95, -0.00, 0.00],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																		},
																		actions	: ~[],
																	}),
																	ChildNode(Node{
																		name	: ~"L_eye_ctrl_grp",
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"L_eye_ctrl_zero",
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"L_eye_ctrl",
																						children	: ~[],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, 0.00, -0.00],
																							rot	: [0.00, 0.00, -0.00, 1.00],
																						},
																						actions	: ~[],
																					})
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [0.00, 0.00, -0.00],
																					rot	: [0.00, 0.00, -0.00, 1.00],
																				},
																				actions	: ~[],
																			})
																		],
																		space	: Space{
																			scale	: 0.72,
																			pos	: [0.95, 0.00, -0.00],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																		},
																		actions	: ~[],
																	})
																],
																space	: Space{
																	scale	: 1.00,
																	pos	: [0.04, -71.88, -17.98],
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																},
																actions	: ~[
																	~"mainEye_ctrlAction@nodes"
																],
															})
														],
														space	: Space{
															scale	: 1.00,
															pos	: [-0.04, 71.88, 17.98],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														actions	: ~[],
													})
												],
												space	: Space{
													scale	: 1.00,
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												actions	: ~[],
											}),
											ChildNode(Node{
												name	: ~"Locators",
												children	: ~[
													ChildNode(Node{
														name	: ~"L_eye_centerLocator",
														children	: ~[],
														space	: Space{
															scale	: 1.00,
															pos	: [1.26, 71.88, 0.51],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														actions	: ~[],
													}),
													ChildNode(Node{
														name	: ~"R_eye_centerLocator",
														children	: ~[],
														space	: Space{
															scale	: 1.00,
															pos	: [-1.26, 71.88, 0.51],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														actions	: ~[],
													})
												],
												space	: Space{
													scale	: 1.00,
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												actions	: ~[],
											}),
											ChildNode(Node{
												name	: ~"LegControls",
												children	: ~[
													ChildNode(Node{
														name	: ~"L_leg_ikHandle_grp",
														children	: ~[
															ChildNode(Node{
																name	: ~"L_leg_ikHandle_zero",
																children	: ~[
																	ChildNode(Node{
																		name	: ~"L_foot_ik_ctrl",
																		children	: ~[],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [-0.00, 0.00, 0.00],
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																		},
																		actions	: ~[],
																	})
																],
																space	: Space{
																	scale	: 1.00,
																	pos	: [0.00, -0.00, 0.00],
																	rot	: [-0.00, -0.00, -0.00, 1.00],
																},
																actions	: ~[],
															})
														],
														space	: Space{
															scale	: 1.00,
															pos	: [3.00, -0.31, -3.16],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														actions	: ~[],
													}),
													ChildNode(Node{
														name	: ~"R_leg_ikHandle_grp",
														children	: ~[
															ChildNode(Node{
																name	: ~"R_leg_ikHandle_zero",
																children	: ~[
																	ChildNode(Node{
																		name	: ~"R_foot_ik_ctrl",
																		children	: ~[],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [0.00, 0.00, 0.00],
																			rot	: [-0.01, -0.00, 0.00, 1.00],
																		},
																		actions	: ~[],
																	})
																],
																space	: Space{
																	scale	: 1.00,
																	pos	: [-0.00, -0.00, 0.00],
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																},
																actions	: ~[],
															})
														],
														space	: Space{
															scale	: 1.00,
															pos	: [-3.00, -0.31, -3.16],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														actions	: ~[],
													}),
													ChildNode(Node{
														name	: ~"R_legPole_ctrl",
														children	: ~[
															ChildNode(Node{
																name	: ~"R_legPole_ctrl_zero",
																children	: ~[
																	ChildNode(Node{
																		name	: ~"R_legPole_ctrl.001",
																		children	: ~[],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [0.00, 0.00, 0.00],
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																		},
																		actions	: ~[],
																	})
																],
																space	: Space{
																	scale	: 1.00,
																	pos	: [0.57, -18.67, 10.59],
																	rot	: [0.00, -0.00, -0.00, 1.00],
																},
																actions	: ~[],
															})
														],
														space	: Space{
															scale	: 1.00,
															pos	: [-4.14, 43.04, -0.16],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														actions	: ~[],
													}),
													ChildNode(Node{
														name	: ~"L_legPole_ctrl_cons",
														children	: ~[
															ChildNode(Node{
																name	: ~"L_legPole_ctrl_grp",
																children	: ~[
																	ChildNode(Node{
																		name	: ~"L_legPole_cntr_zero",
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"L_legPole_ctrl",
																				children	: ~[],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [0.00, -0.00, -0.00],
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																				},
																				actions	: ~[],
																			})
																		],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [-0.55, -17.93, 9.74],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																		},
																		actions	: ~[],
																	})
																],
																space	: Space{
																	scale	: 1.00,
																	pos	: [0.00, 0.00, -0.00],
																	rot	: [0.47, -0.53, 0.49, 0.51],
																},
																actions	: ~[],
															})
														],
														space	: Space{
															scale	: 1.00,
															pos	: [4.10, 42.26, 0.61],
															rot	: [-0.47, 0.53, -0.49, 0.51],
														},
														actions	: ~[],
													})
												],
												space	: Space{
													scale	: 1.00,
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												actions	: ~[],
											}),
											ChildNode(Node{
												name	: ~"L_arm_IK_ctrl_grp",
												children	: ~[
													ChildNode(Node{
														name	: ~"L_arm_IK_ctrl_zero",
														children	: ~[
															ChildNode(Node{
																name	: ~"L_arm_IK_ctrl",
																children	: ~[
																	ChildNode(Node{
																		name	: ~"L_armIK_handle",
																		children	: ~[],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [-0.00, 0.00, -0.00],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																		},
																		actions	: ~[],
																	}),
																	ChildNode(Node{
																		name	: ~"L_hand_grp",
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"L_hand_zero",
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"L_palm_ctrl",
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"L_thumb_ctrl",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								actions	: ~[
																									~"L_thumb_ctrlAction@nodes"
																								],
																							}),
																							ChildNode(Node{
																								name	: ~"L_indexF_ctrl",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								actions	: ~[
																									~"L_indexF_ctrlAction@nodes"
																								],
																							}),
																							ChildNode(Node{
																								name	: ~"L_middleF_ctrl",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								actions	: ~[
																									~"L_middleF_ctrlAction@nodes"
																								],
																							}),
																							ChildNode(Node{
																								name	: ~"L_ringF_ctrl",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								actions	: ~[
																									~"L_ringF_ctrlAction@nodes"
																								],
																							}),
																							ChildNode(Node{
																								name	: ~"L_pinkyF_ctrl",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								actions	: ~[
																									~"L_pinkyF_ctrlAction@nodes"
																								],
																							})
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, -0.00, 0.00],
																							rot	: [-0.00, 0.00, -0.00, 1.00],
																						},
																						actions	: ~[
																							~"L_palm_ctrlAction@nodes"
																						],
																					})
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [-0.00, 0.00, -0.00],
																					rot	: [0.00, -0.00, 0.00, 1.00],
																				},
																				actions	: ~[],
																			})
																		],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [-0.00, 0.00, -0.00],
																			rot	: [-0.26, -0.08, -0.07, 0.96],
																		},
																		actions	: ~[],
																	})
																],
																space	: Space{
																	scale	: 1.00,
																	pos	: [-27.41, -60.54, 2.62],
																	rot	: [0.00, -0.00, 0.00, 1.00],
																},
																actions	: ~[
																	~"L_arm_IK_ctrlAction@nodes"
																],
															})
														],
														space	: Space{
															scale	: 1.00,
															pos	: [0.00, -0.00, 0.00],
															rot	: [0.26, 0.08, 0.07, 0.96],
														},
														actions	: ~[],
													})
												],
												space	: Space{
													scale	: 1.00,
													pos	: [27.41, 60.54, -2.62],
													rot	: [-0.26, -0.08, -0.07, 0.96],
												},
												actions	: ~[],
											}),
											ChildNode(Node{
												name	: ~"c_hips_cntr_backup",
												children	: ~[],
												space	: Space{
													scale	: 1.00,
													pos	: [0.00, 46.47, 1.11],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												actions	: ~[],
											}),
											ChildNode(Node{
												name	: ~"R_arm_IK_ctrl_grp",
												children	: ~[
													ChildNode(Node{
														name	: ~"R_arm_IK_ctrl_zero",
														children	: ~[
															ChildNode(Node{
																name	: ~"R_arm_IK_ctrl1",
																children	: ~[
																	ChildNode(Node{
																		name	: ~"ikHandle4",
																		children	: ~[],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [-54.83, -0.00, 0.00],
																			rot	: [0.00, 0.00, 0.00, 1.00],
																		},
																		actions	: ~[],
																	}),
																	ChildNode(Node{
																		name	: ~"R_hand_grp",
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"R_hand_zero",
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"R_palm_ctrl",
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"R_thumb_ctrl",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																								},
																								actions	: ~[
																									~"R_thumb_ctrlAction@nodes"
																								],
																							}),
																							ChildNode(Node{
																								name	: ~"R_indexF_ctrl",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																								},
																								actions	: ~[
																									~"R_indexF_ctrlAction@nodes"
																								],
																							}),
																							ChildNode(Node{
																								name	: ~"R_middleF_ctrl",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																								},
																								actions	: ~[
																									~"R_middleF_ctrlAction@nodes"
																								],
																							}),
																							ChildNode(Node{
																								name	: ~"R_ringF_ctrl",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																								},
																								actions	: ~[
																									~"R_ringF_ctrlAction@nodes"
																								],
																							}),
																							ChildNode(Node{
																								name	: ~"R_pinkyF_ctrl",
																								children	: ~[],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																								},
																								actions	: ~[
																									~"R_pinkyF_ctrlAction@nodes"
																								],
																							})
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, 0.00, -0.00],
																							rot	: [-0.07, 0.96, -0.26, 0.08],
																						},
																						actions	: ~[
																							~"R_palm_ctrlAction@nodes"
																						],
																					})
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [0.00, -0.00, 0.00],
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																				},
																				actions	: ~[],
																			})
																		],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [-108.05, 6.80, 11.25],
																			rot	: [0.07, -0.96, 0.26, 0.08],
																		},
																		actions	: ~[],
																	})
																],
																space	: Space{
																	scale	: 1.00,
																	pos	: [-27.33, -60.54, 2.62],
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																},
																actions	: ~[
																	~"R_arm_IK_ctrl1Action@nodes"
																],
															})
														],
														space	: Space{
															scale	: 1.00,
															pos	: [0.00, 0.00, 0.00],
															rot	: [-0.00, -0.00, 0.00, 1.00],
														},
														actions	: ~[],
													})
												],
												space	: Space{
													scale	: 1.00,
													pos	: [27.33, 60.54, -2.62],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												actions	: ~[],
											}),
											ChildNode(Node{
												name	: ~"c_cog_ctrl_grp",
												children	: ~[
													ChildNode(Node{
														name	: ~"c_cog_ctrl_zero",
														children	: ~[
															ChildNode(Node{
																name	: ~"c_cog_ctrl",
																children	: ~[
																	ChildNode(Node{
																		name	: ~"c_hips_ctrl_grp",
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"c_hips_ctrl_zero",
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"c_hips_ctrl",
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"group13",
																								children	: ~[
																									ChildNode(Node{
																										name	: ~"cluster3Handle",
																										children	: ~[],
																										space	: Space{
																											scale	: 1.00,
																											pos	: [-0.00, -0.00, 0.00],
																											rot	: [0.00, 0.00, -0.00, 1.00],
																										},
																										actions	: ~[],
																									}),
																									ChildNode(Node{
																										name	: ~"cluster2Handle",
																										children	: ~[],
																										space	: Space{
																											scale	: 1.00,
																											pos	: [-0.00, -0.00, 0.00],
																											rot	: [0.00, 0.00, -0.00, 1.00],
																										},
																										actions	: ~[],
																									}),
																									ChildNode(Node{
																										name	: ~"cluster1Handle",
																										children	: ~[],
																										space	: Space{
																											scale	: 1.00,
																											pos	: [-0.00, -0.00, 0.00],
																											rot	: [0.00, 0.00, -0.00, 1.00],
																										},
																										actions	: ~[],
																									})
																								],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [-46.65, -7.08, 0.00],
																									rot	: [0.46, 0.54, -0.46, 0.54],
																								},
																								actions	: ~[],
																							})
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [0.00, -0.00, 0.00],
																							rot	: [0.46, 0.54, -0.46, 0.54],
																						},
																						actions	: ~[
																							~"c_hips_ctrlAction@nodes"
																						],
																					})
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [0.00, -0.00, -0.00],
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																				},
																				actions	: ~[],
																			})
																		],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [-0.00, -0.04, -0.10],
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																		},
																		actions	: ~[],
																	}),
																	ChildNode(Node{
																		name	: ~"c_spine_01_ctrl_grp",
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"c_spine_01_ctrl_zero",
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"c_spine_01_ctrl",
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"c_spine_03_ctrl_grp",
																								children	: ~[
																									ChildNode(Node{
																										name	: ~"c_spine_03_ctrl_zero",
																										children	: ~[
																											ChildNode(Node{
																												name	: ~"c_spine_03_ctrl",
																												children	: ~[
																													ChildNode(Node{
																														name	: ~"group14",
																														children	: ~[
																															ChildNode(Node{
																																name	: ~"cluster4Handle",
																																children	: ~[],
																																space	: Space{
																																	scale	: 1.00,
																																	pos	: [0.00, 0.00, 0.00],
																																	rot	: [0.00, 0.00, -0.00, 1.00],
																																},
																																actions	: ~[],
																															})
																														],
																														space	: Space{
																															scale	: 1.00,
																															pos	: [-50.08, 3.75, -0.04],
																															rot	: [0.52, 0.48, -0.52, 0.48],
																														},
																														actions	: ~[],
																													}),
																													ChildNode(Node{
																														name	: ~"c_spine_05_ctrl_grp",
																														children	: ~[
																															ChildNode(Node{
																																name	: ~"c_spine_05_ctrl_zero",
																																children	: ~[
																																	ChildNode(Node{
																																		name	: ~"c_spine_05_ctrl",
																																		children	: ~[
																																			ChildNode(Node{
																																				name	: ~"group12",
																																				children	: ~[
																																					ChildNode(Node{
																																						name	: ~"cluster6Handle",
																																						children	: ~[],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [-0.00, -0.00, 0.00],
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																						},
																																						actions	: ~[],
																																					}),
																																					ChildNode(Node{
																																						name	: ~"cluster5Handle",
																																						children	: ~[],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [-0.00, -0.00, 0.00],
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																						},
																																						actions	: ~[],
																																					})
																																				],
																																				space	: Space{
																																					scale	: 1.00,
																																					pos	: [-50.19, 17.53, -0.04],
																																					rot	: [0.57, 0.41, -0.57, 0.41],
																																				},
																																				actions	: ~[],
																																			}),
																																			ChildNode(Node{
																																				name	: ~"c_chest_ctrl_grp",
																																				children	: ~[
																																					ChildNode(Node{
																																						name	: ~"c_chest_ctrl_zero",
																																						children	: ~[
																																							ChildNode(Node{
																																								name	: ~"c_chest_ctrl",
																																								children	: ~[
																																									ChildNode(Node{
																																										name	: ~"group11",
																																										children	: ~[
																																											ChildNode(Node{
																																												name	: ~"cluster9Handle",
																																												children	: ~[],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [0.00, 0.00, -0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																												},
																																												actions	: ~[],
																																											}),
																																											ChildNode(Node{
																																												name	: ~"cluster8Handle",
																																												children	: ~[],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [0.00, 0.00, -0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																												},
																																												actions	: ~[],
																																											}),
																																											ChildNode(Node{
																																												name	: ~"cluster7Handle",
																																												children	: ~[],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [0.00, 0.00, -0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																												},
																																												actions	: ~[],
																																											})
																																										],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [-56.22, 18.02, -0.04],
																																											rot	: [0.58, 0.41, -0.58, 0.41],
																																										},
																																										actions	: ~[],
																																									}),
																																									ChildNode(Node{
																																										name	: ~"c_breastControls_grp",
																																										children	: ~[
																																											ChildNode(Node{
																																												name	: ~"null1",
																																												children	: ~[],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [0.00, -0.00, -0.00],
																																													rot	: [-0.00, 0.00, -0.00, 1.00],
																																												},
																																												actions	: ~[],
																																											}),
																																											ChildNode(Node{
																																												name	: ~"R_breast_IK_control_grp",
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"R_breast_IK_cntr_zero",
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"R_breast_IK_ctrl",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"ikHandle2",
																																																		children	: ~[],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [0.00, 0.00, -0.00],
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																		},
																																																		actions	: ~[],
																																																	}),
																																																	ChildNode(Node{
																																																		name	: ~"R_breastTweak_cntr_grp",
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"R_breastTweak_zero",
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"R_breastTweak_ctrl",
																																																						children	: ~[],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [-0.00, 0.00, -0.00],
																																																							rot	: [-0.00, 0.00, 0.00, 1.00],
																																																						},
																																																						actions	: ~[
																																																							~"R_breastTweak_ctrlAction@nodes"
																																																						],
																																																					})
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [-0.00, 0.00, -0.00],
																																																					rot	: [0.00, 0.00, -0.00, 1.00],
																																																				},
																																																				actions	: ~[],
																																																			})
																																																		],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [0.00, 0.00, -0.00],
																																																			rot	: [0.03, -0.70, 0.06, 0.71],
																																																		},
																																																		actions	: ~[],
																																																	})
																																																],
																																																space	: Space{
																																																	scale	: 1.00,
																																																	pos	: [0.00, 0.00, -0.00],
																																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																																},
																																																actions	: ~[
																																																	~"R_breast_IK_ctrlAction@nodes"
																																																],
																																															})
																																														],
																																														space	: Space{
																																															scale	: 1.00,
																																															pos	: [0.00, 0.00, 0.00],
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																														},
																																														actions	: ~[],
																																													})
																																												],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [-2.31, -6.09, 4.23],
																																													rot	: [-0.10, -0.08, -0.84, 0.52],
																																												},
																																												actions	: ~[],
																																											}),
																																											ChildNode(Node{
																																												name	: ~"L_breast_IK_control_grp",
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"L_breast_IK_cntr_zero",
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"L_breastTweak_cntr_grp",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"L_breastTweak_zero",
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"L_breastTweak_ctrl",
																																																				children	: ~[],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [0.00, -0.00, -0.00],
																																																					rot	: [-0.00, -0.00, -0.00, 1.00],
																																																				},
																																																				actions	: ~[
																																																					~"L_breastTweak_ctrlAction@nodes"
																																																				],
																																																			})
																																																		],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [0.00, 0.00, 0.00],
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																		},
																																																		actions	: ~[],
																																																	})
																																																],
																																																space	: Space{
																																																	scale	: 1.00,
																																																	pos	: [-0.00, 0.00, -0.00],
																																																	rot	: [-0.03, 0.70, 0.06, 0.71],
																																																},
																																																actions	: ~[],
																																															}),
																																															ChildNode(Node{
																																																name	: ~"L_breast_IK_ctrl",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"ikHandle3",
																																																		children	: ~[],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [0.00, -0.00, 0.00],
																																																			rot	: [-0.00, 0.00, -0.00, 1.00],
																																																		},
																																																		actions	: ~[],
																																																	})
																																																],
																																																space	: Space{
																																																	scale	: 1.00,
																																																	pos	: [-0.00, 0.00, -0.00],
																																																	rot	: [-0.00, 0.00, 0.00, 1.00],
																																																},
																																																actions	: ~[
																																																	~"L_breast_IK_ctrlAction@nodes"
																																																],
																																															})
																																														],
																																														space	: Space{
																																															scale	: 1.00,
																																															pos	: [0.00, 0.00, 0.00],
																																															rot	: [0.00, 0.00, 0.00, 1.00],
																																														},
																																														actions	: ~[],
																																													})
																																												],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [-2.31, -6.09, -4.23],
																																													rot	: [0.10, 0.08, -0.84, 0.52],
																																												},
																																												actions	: ~[],
																																											})
																																										],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [0.00, 0.00, 0.00],
																																											rot	: [-0.00, 0.00, 0.00, 1.00],
																																										},
																																										actions	: ~[],
																																									}),
																																									ChildNode(Node{
																																										name	: ~"c_neck_01_ctrl_grp",
																																										children	: ~[
																																											ChildNode(Node{
																																												name	: ~"R_clav_ctrl_grp",
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"R_clav_ctrl_zero",
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"R_clav_ctrl",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"R_mainSpaulder_ctrl_grp",
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"R_mainSpaulder_ctrl_zero",
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"R_mainSpaulder_ctrl",
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								name	: ~"R_subSpaulder_ctrl_grp",
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										name	: ~"R_subSpaulder_ctrl_zero",
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												name	: ~"R_subSpaulder_ctrl",
																																																												children	: ~[],
																																																												space	: Space{
																																																													scale	: 1.00,
																																																													pos	: [0.00, 0.00, -0.00],
																																																													rot	: [-0.00, 0.00, -0.04, 1.00],
																																																												},
																																																												actions	: ~[
																																																													~"R_subSpaulder_ctrlAction@nodes"
																																																												],
																																																											})
																																																										],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 0.00, -0.00],
																																																											rot	: [-0.00, -0.00, 0.00, 1.00],
																																																										},
																																																										actions	: ~[],
																																																									})
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [2.32, 0.02, 0.51],
																																																									rot	: [-0.00, -0.30, 0.00, 0.96],
																																																								},
																																																								actions	: ~[],
																																																							})
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 0.00, -0.00],
																																																							rot	: [0.00, 0.00, -0.03, 1.00],
																																																						},
																																																						actions	: ~[
																																																							~"R_mainSpaulder_ctrlAction@nodes"
																																																						],
																																																					})
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [-0.00, 0.00, 0.00],
																																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																																				},
																																																				actions	: ~[],
																																																			})
																																																		],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [4.89, -0.16, 0.38],
																																																			rot	: [0.13, 0.32, -0.21, 0.91],
																																																		},
																																																		actions	: ~[],
																																																	})
																																																],
																																																space	: Space{
																																																	scale	: 1.00,
																																																	pos	: [0.00, -0.00, 0.00],
																																																	rot	: [0.00, -0.12, 0.95, 0.28],
																																																},
																																																actions	: ~[
																																																	~"R_clav_ctrlAction@nodes"
																																																],
																																															})
																																														],
																																														space	: Space{
																																															scale	: 1.00,
																																															pos	: [0.00, -0.00, -0.00],
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																														},
																																														actions	: ~[],
																																													})
																																												],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [-1.39, -3.03, 1.00],
																																													rot	: [-0.19, -0.65, 0.22, 0.71],
																																												},
																																												actions	: ~[],
																																											}),
																																											ChildNode(Node{
																																												name	: ~"L_clav_ctrl_grp",
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"L_clav_ctrl_zero",
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"L_clav_ctrl",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"L_mainSpaulder_ctrl_grp",
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"L_mainSpaulder_ctrl_zero",
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"L_mainSpaulder_ctrl",
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								name	: ~"L_subSpaulder_ctrl_grp",
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										name	: ~"L_subSpaulder_ctrl_zero",
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												name	: ~"L_subSpaulder_ctrl",
																																																												children	: ~[],
																																																												space	: Space{
																																																													scale	: 1.00,
																																																													pos	: [0.00, 0.00, -0.00],
																																																													rot	: [-0.00, -0.00, -0.04, 1.00],
																																																												},
																																																												actions	: ~[
																																																													~"L_subSpaulder_ctrlAction@nodes"
																																																												],
																																																											})
																																																										],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 0.00, -0.00],
																																																											rot	: [-0.00, -0.00, -0.00, 1.00],
																																																										},
																																																										actions	: ~[],
																																																									})
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [2.32, 0.02, -0.51],
																																																									rot	: [0.00, 0.30, 0.00, 0.96],
																																																								},
																																																								actions	: ~[],
																																																							})
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [0.00, -0.00, -0.00],
																																																							rot	: [-0.00, -0.00, -0.03, 1.00],
																																																						},
																																																						actions	: ~[
																																																							~"L_mainSpaulder_ctrlAction@nodes"
																																																						],
																																																					})
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [0.00, -0.00, 0.00],
																																																					rot	: [0.00, 0.00, 0.00, 1.00],
																																																				},
																																																				actions	: ~[],
																																																			})
																																																		],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [4.89, -0.12, -0.39],
																																																			rot	: [-0.09, -0.32, -0.22, 0.92],
																																																		},
																																																		actions	: ~[],
																																																	})
																																																],
																																																space	: Space{
																																																	scale	: 1.00,
																																																	pos	: [0.00, 0.00, -0.00],
																																																	rot	: [0.00, -0.01, -0.28, 0.96],
																																																},
																																																actions	: ~[
																																																	~"L_clav_ctrlAction@nodes"
																																																],
																																															})
																																														],
																																														space	: Space{
																																															scale	: 1.00,
																																															pos	: [0.00, 0.00, 0.00],
																																															rot	: [-0.00, -0.00, -0.00, 1.00],
																																														},
																																														actions	: ~[],
																																													})
																																												],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [-1.39, -3.03, -1.00],
																																													rot	: [0.16, 0.64, 0.24, 0.71],
																																												},
																																												actions	: ~[],
																																											}),
																																											ChildNode(Node{
																																												name	: ~"c_neck_01_ctrl_zero",
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"c_neck_01_ctrl",
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"c_neck_02_ctrl_grp",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"c_neck_02_ctrl_zero",
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"c_neck_02_ctrl",
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"c_neck_03_ctrl_grp",
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								name	: ~"c_neck_03_ctrl_zero",
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										name	: ~"c_neck_03_ctrl",
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												name	: ~"c_jaw_ctrl_grp",
																																																												children	: ~[
																																																													ChildNode(Node{
																																																														name	: ~"c_jaw_ctrl_zero",
																																																														children	: ~[
																																																															ChildNode(Node{
																																																																name	: ~"c_jaw_ctrl",
																																																																children	: ~[],
																																																																space	: Space{
																																																																	scale	: 1.00,
																																																																	pos	: [0.00, -0.00, 0.00],
																																																																	rot	: [-0.00, 0.00, -0.00, 1.00],
																																																																},
																																																																actions	: ~[
																																																																	~"c_jaw_ctrlAction@nodes"
																																																																],
																																																															})
																																																														],
																																																														space	: Space{
																																																															scale	: 1.00,
																																																															pos	: [-0.00, 0.00, 0.00],
																																																															rot	: [-0.00, 0.00, -0.00, 1.00],
																																																														},
																																																														actions	: ~[],
																																																													})
																																																												],
																																																												space	: Space{
																																																													scale	: 1.00,
																																																													pos	: [-0.41, -0.55, 0.00],
																																																													rot	: [0.00, 0.00, -0.82, 0.57],
																																																												},
																																																												actions	: ~[],
																																																											})
																																																										],
																																																										space	: Space{
																																																											scale	: 1.00,
																																																											pos	: [0.00, 0.00, 0.00],
																																																											rot	: [-0.00, -0.00, 0.05, 1.00],
																																																										},
																																																										actions	: ~[
																																																											~"c_neck_03_ctrlAction@nodes"
																																																										],
																																																									})
																																																								],
																																																								space	: Space{
																																																									scale	: 1.00,
																																																									pos	: [0.00, 0.00, 0.00],
																																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																																								},
																																																								actions	: ~[],
																																																							})
																																																						],
																																																						space	: Space{
																																																							scale	: 1.00,
																																																							pos	: [2.37, 0.00, 0.00],
																																																							rot	: [-0.00, -0.00, -0.00, 1.00],
																																																						},
																																																						actions	: ~[],
																																																					})
																																																				],
																																																				space	: Space{
																																																					scale	: 1.00,
																																																					pos	: [0.00, 0.00, -0.00],
																																																					rot	: [-0.00, -0.00, 0.04, 1.00],
																																																				},
																																																				actions	: ~[
																																																					~"c_neck_02_ctrlAction@nodes"
																																																				],
																																																			})
																																																		],
																																																		space	: Space{
																																																			scale	: 1.00,
																																																			pos	: [0.00, 0.00, -0.00],
																																																			rot	: [-0.00, -0.00, -0.00, 1.00],
																																																		},
																																																		actions	: ~[],
																																																	})
																																																],
																																																space	: Space{
																																																	scale	: 1.00,
																																																	pos	: [2.33, 0.01, -0.00],
																																																	rot	: [0.00, 0.00, 0.02, 1.00],
																																																},
																																																actions	: ~[],
																																															})
																																														],
																																														space	: Space{
																																															scale	: 1.00,
																																															pos	: [0.00, 0.00, -0.00],
																																															rot	: [-0.00, -0.00, 0.06, 1.00],
																																														},
																																														actions	: ~[
																																															~"c_neck_01_ctrlAction@nodes"
																																														],
																																													})
																																												],
																																												space	: Space{
																																													scale	: 1.00,
																																													pos	: [0.00, 0.00, 0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																												},
																																												actions	: ~[],
																																											})
																																										],
																																										space	: Space{
																																											scale	: 1.00,
																																											pos	: [6.82, -0.29, 0.00],
																																											rot	: [0.00, 0.00, -0.23, 0.97],
																																										},
																																										actions	: ~[],
																																									})
																																								],
																																								space	: Space{
																																									scale	: 1.00,
																																									pos	: [-0.00, 0.00, 0.00],
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																								},
																																								actions	: ~[
																																									~"c_chest_ctrlAction@nodes"
																																								],
																																							})
																																						],
																																						space	: Space{
																																							scale	: 1.00,
																																							pos	: [0.00, 0.00, -0.00],
																																							rot	: [-0.00, -0.00, 0.00, 1.00],
																																						},
																																						actions	: ~[],
																																					})
																																				],
																																				space	: Space{
																																					scale	: 1.00,
																																					pos	: [6.18, -0.00, -0.00],
																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																				},
																																				actions	: ~[],
																																			})
																																		],
																																		space	: Space{
																																			scale	: 1.00,
																																			pos	: [0.00, -0.00, -0.00],
																																			rot	: [0.57, 0.41, -0.57, 0.41],
																																		},
																																		actions	: ~[
																																			~"c_spine_05_ctrlAction@nodes"
																																		],
																																	})
																																],
																																space	: Space{
																																	scale	: 1.00,
																																	pos	: [0.00, 0.00, 0.00],
																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																},
																																actions	: ~[],
																															})
																														],
																														space	: Space{
																															scale	: 1.00,
																															pos	: [2.95, 0.06, -0.00],
																															rot	: [0.00, 0.00, 0.13, 0.99],
																														},
																														actions	: ~[],
																													})
																												],
																												space	: Space{
																													scale	: 1.00,
																													pos	: [-50.12, 3.07, 0.00],
																													rot	: [-0.00, -0.00, -0.00, 1.00],
																												},
																												actions	: ~[
																													~"c_spine_03_ctrlAction@nodes"
																												],
																											})
																										],
																										space	: Space{
																											scale	: 1.00,
																											pos	: [0.00, 0.00, 0.00],
																											rot	: [-0.00, 0.00, -0.00, 1.00],
																										},
																										actions	: ~[],
																									})
																								],
																								space	: Space{
																									scale	: 1.00,
																									pos	: [3.03, 0.02, -0.00],
																									rot	: [-0.00, 0.00, 0.11, 0.99],
																								},
																								actions	: ~[],
																							})
																						],
																						space	: Space{
																							scale	: 1.00,
																							pos	: [-0.00, 0.00, 0.00],
																							rot	: [-0.00, -0.00, 0.08, 1.00],
																						},
																						actions	: ~[
																							~"c_spine_01_ctrlAction@nodes"
																						],
																					})
																				],
																				space	: Space{
																					scale	: 1.00,
																					pos	: [0.00, -0.00, 0.00],
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																				},
																				actions	: ~[],
																			})
																		],
																		space	: Space{
																			scale	: 1.00,
																			pos	: [-0.04, -0.00, 0.00],
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																		},
																		actions	: ~[],
																	})
																],
																space	: Space{
																	scale	: 1.00,
																	pos	: [-0.00, -47.22, -0.24],
																	rot	: [-0.00, 0.00, 0.00, 1.00],
																},
																actions	: ~[
																	~"c_cog_ctrlAction@nodes"
																],
															})
														],
														space	: Space{
															scale	: 1.00,
															pos	: [0.00, -0.00, 0.00],
															rot	: [-0.00, 0.00, -0.00, 1.00],
														},
														actions	: ~[],
													})
												],
												space	: Space{
													scale	: 1.00,
													pos	: [0.00, 47.22, 0.24],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												actions	: ~[],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[
											~"c_worldTransform_ctrlAction@nodes"
										],
									})
								],
								space	: Space{
									scale	: 1.00,
									pos	: [0.00, 1.11, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								actions	: ~[],
							})
						],
						space	: Space{
							scale	: 1.00,
							pos	: [-0.00, -1.11, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						actions	: ~[],
					}),
					ChildNode(Node{
						name	: ~"noTrasnform",
						children	: ~[
							ChildNode(Node{
								name	: ~"Body",
								children	: ~[
									ChildNode(Node{
										name	: ~"tongue_geo1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"tongue_geo1Shape@all",
												material	: ~"Tongue",
												armature	: ~"",
												range	: [0, 528],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"polySurface172",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"polySurfaceShape174@all",
												material	: ~"cloak",
												armature	: ~"",
												range	: [0, 15252],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"topJaw_geo2",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"topJaw_geo2Shape@all",
												material	: ~"Teeth",
												armature	: ~"",
												range	: [0, 4656],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"lowerJaw_geo2",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"lowerJaw_geo2Shape@all",
												material	: ~"Teeth",
												armature	: ~"",
												range	: [0, 4248],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"L_upper_lash1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"L_upper_lash1Shape@all",
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 13716],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"L_lower_lash1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"L_lower_lash1Shape@all",
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 8964],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"R_upper_lash1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_upper_lash1Shape@all",
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 13716],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"R_lower_lash1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_lower_lash1Shape@all",
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 8964],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"L_eye_geo1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"L_eye_geo1Shape@all",
												material	: ~"Eyes",
												armature	: ~"",
												range	: [0, 2784],
											}),
											ChildEntity(Entity{
												mesh	: ~"L_eye_geo1Shape@all",
												material	: ~"Pupil_SS",
												armature	: ~"",
												range	: [2784, 3264],
											}),
											ChildEntity(Entity{
												mesh	: ~"L_eye_geo1Shape@all",
												material	: ~"cornea",
												armature	: ~"",
												range	: [3264, 5568],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"R_eye_geo1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_eye_geo1Shape@all",
												material	: ~"Eyes",
												armature	: ~"",
												range	: [0, 2784],
											}),
											ChildEntity(Entity{
												mesh	: ~"R_eye_geo1Shape@all",
												material	: ~"Pupil_SS",
												armature	: ~"",
												range	: [2784, 3264],
											}),
											ChildEntity(Entity{
												mesh	: ~"R_eye_geo1Shape@all",
												material	: ~"cornea",
												armature	: ~"",
												range	: [3264, 5568],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"Hair_Geo2",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"Hair_Geo2Shape@all",
												material	: ~"anisotropic1",
												armature	: ~"",
												range	: [0, 6954],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"Body_geo8",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"Body_geo8Shape@all",
												material	: ~"skin",
												armature	: ~"",
												range	: [0, 50496],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									})
								],
								space	: Space{
									scale	: 1.00,
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								actions	: ~[],
							}),
							ChildNode(Node{
								name	: ~"Armor",
								children	: ~[
									ChildNode(Node{
										name	: ~"boots",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_boot1Shape@all",
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 9042],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, -0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"backShealth1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"backShealth1Shape@all",
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 5550],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"skirt",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_skirt_06Shape@all",
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 10236],
											}),
											ChildEntity(Entity{
												mesh	: ~"R_skirt_06Shape@all",
												material	: ~"skin",
												armature	: ~"",
												range	: [10236, 12102],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"bracket",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"bracket_05_geo1Shape@all",
												material	: ~"skin",
												armature	: ~"",
												range	: [0, 8448],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"bracers",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"L_bracer1Shape@all",
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 2304],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									}),
									ChildNode(Node{
										name	: ~"spaulders",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_subSpaulder1Shape@all",
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 6960],
											})
										],
										space	: Space{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										actions	: ~[],
									})
								],
								space	: Space{
									scale	: 1.00,
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								actions	: ~[],
							}),
							ChildNode(Node{
								name	: ~"Eyes_Geo",
								children	: ~[],
								space	: Space{
									scale	: 1.00,
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								actions	: ~[],
							})
						],
						space	: Space{
							scale	: 1.00,
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						actions	: ~[],
					})
				],
				space	: Space{
					scale	: 1.00,
					pos	: [0.00, 0.00, 0.00],
					rot	: [0.50, 0.50, 0.50, 0.50],
				},
				actions	: ~[],
			}),
			ChildNode(Node{
				name	: ~"Lamp",
				children	: ~[
					ChildLight(Light{
						kind	: KindSpot(Spot{
							blend	: 0.15,
							size	: 1.31,
						}),
						attenuation	: [0.00, 1.00],
						distance	: 100.00,
						color	: [1.00, 1.00, 1.00],
						name	: ~"Lamp",
						energy	: 3.00,
						spherical	: false,
					})
				],
				space	: Space{
					scale	: 1.00,
					pos	: [43.55, 25.15, 80.51],
					rot	: [0.27, 0.31, 0.78, 0.47],
				},
				actions	: ~[],
			})
		],
	}}
