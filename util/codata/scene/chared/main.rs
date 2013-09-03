use common::*;
pub fn load()-> Scene	{Scene{
		materials	: ~[
			Material{
				textures	: ~[],
				name	: ~"anisotropic1",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.09, 0.09, 0.08])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			},
			Material{
				textures	: ~[
					Texture{
						path	: ~"//Metal_R-Spec_G-Bump_B-Reflect.jpg",
						scale	: [1.00, 1.00, 1.00],
						offset	: [0.00, 0.00, 0.00],
						wrap	: 0,
						name	: ~"Main",
						filter	: 3,
					}
				],
				name	: ~"armor",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			},
			Material{
				textures	: ~[],
				name	: ~"cloak",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			},
			Material{
				textures	: ~[],
				name	: ~"cornea",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.50, 0.50, 0.50])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			},
			Material{
				textures	: ~[],
				name	: ~"EyeLashes",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			},
			Material{
				textures	: ~[],
				name	: ~"Eyes",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.44, 0.44, 0.54])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.49, 0.49, 0.49])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			},
			Material{
				textures	: ~[],
				name	: ~"Material",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			},
			Material{
				textures	: ~[],
				name	: ~"Pupil_SS",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			},
			Material{
				textures	: ~[
					Texture{
						path	: ~"//Skin_Diffuse.jpg",
						scale	: [1.00, 1.00, 1.00],
						offset	: [0.00, 0.00, 0.00],
						wrap	: 0,
						name	: ~"Main.001",
						filter	: 3,
					}
				],
				name	: ~"skin",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			},
			Material{
				textures	: ~[],
				name	: ~"Teeth",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.75, 0.75, 0.75])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			},
			Material{
				textures	: ~[],
				name	: ~"Tongue",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.40, 0.08, 0.08])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.50, 0.50, 0.50])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				shader	: ~"phong",
			}
		],
		nodes	: ~[
			ChildNode(Node{
				name	: ~"Plane",
				actions	: ~[],
				children	: ~[
					ChildEntity(Entity{
						armature	: ~"",
						material	: ~"Material",
						mesh	: ~"Plane@all",
						range	: [0, 6],
					})
				],
				space	: QuatSpace{
					scale	: 100.00,
					rot	: [0.00, 0.00, 0.00, 1.00],
					pos	: [0.00, 0.00, -1.00],
				},
			}),
			ChildNode(Node{
				name	: ~"Camera",
				actions	: ~[],
				children	: ~[
					ChildCamera(Camera{
						name	: ~"Camera",
						fov_y	: 0.87,
						range	: [10.00, 300.00],
					})
				],
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.41, 0.41, 0.58, 0.58],
					pos	: [140.00, 0.00, 90.00],
				},
			}),
			ChildNode(Node{
				name	: ~"Clare",
				actions	: ~[],
				children	: ~[
					ChildNode(Node{
						name	: ~"R_ik_foot_grp",
						actions	: ~[],
						children	: ~[
							ChildNode(Node{
								name	: ~"ikHandle8",
								actions	: ~[],
								children	: ~[],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
							})
						],
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
					}),
					ChildNode(Node{
						name	: ~"L_leg_ikHandle_zero.001",
						actions	: ~[],
						children	: ~[],
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
					}),
					ChildNode(Node{
						name	: ~"L_ik_foot_grp",
						actions	: ~[],
						children	: ~[
							ChildNode(Node{
								name	: ~"ikHandle7",
								actions	: ~[],
								children	: ~[],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
							})
						],
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
					}),
					ChildNode(Node{
						name	: ~"Transform",
						actions	: ~[],
						children	: ~[
							ChildNode(Node{
								name	: ~"Controls",
								actions	: ~[],
								children	: ~[
									ChildNode(Node{
										name	: ~"c_worldTransform_ctrl",
										actions	: ~[],
										children	: ~[
											ChildNode(Node{
												name	: ~"Armature.002",
												actions	: ~[],
												children	: ~[
													ChildArmature(Armature{
														name	: ~"Armature.002",
														actions	: ~[],
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
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, -0.00, 0.70],
																																											pos	: [0.00, 0.69, 0.00],
																																										},
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.71, -0.01, 0.01, 0.71],
																																									pos	: [1.26, -4.62, 3.06],
																																								},
																																							},
																																							Bone{
																																								name	: ~"R_eye_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"R_eye_end_joint",
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.70, -0.00, 0.71],
																																											pos	: [0.00, 0.69, -0.00],
																																										},
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.71, 0.01, -0.01, 0.71],
																																									pos	: [-1.26, -4.62, 3.06],
																																								},
																																							},
																																							Bone{
																																								name	: ~"R_eye_blink_base_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"R_eye_blink_01_joint",
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, -0.00, 0.71],
																																											pos	: [-0.00, 0.50, -0.00],
																																										},
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.77, -0.00, 0.00, 0.64],
																																									pos	: [-1.26, -4.62, 3.06],
																																								},
																																							},
																																							Bone{
																																								name	: ~"L_eye_blink_base_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"L_eye_blink_01_joint",
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, -0.00, 0.71],
																																											pos	: [0.00, 0.50, -0.00],
																																										},
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.77, -0.00, 0.00, 0.64],
																																									pos	: [1.26, -4.62, 3.06],
																																								},
																																							},
																																							Bone{
																																								name	: ~"R_eye_blink_02_base_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"R_eye_blink_02_joint",
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.71, -0.00, 0.71],
																																											pos	: [-0.00, 0.67, -0.00],
																																										},
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.60, -0.00, 0.00, 0.80],
																																									pos	: [-1.26, -4.62, 3.06],
																																								},
																																							},
																																							Bone{
																																								name	: ~"L_eye_blink_02_base_joint",
																																								children	: ~[
																																									Bone{
																																										name	: ~"L_eye_blink_02_joint",
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, 0.00, 0.71],
																																											pos	: [-0.00, 0.67, -0.00],
																																										},
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.60, -0.00, 0.00, 0.80],
																																									pos	: [1.26, -4.62, 3.06],
																																								},
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [-0.04, 0.00, 0.00, 1.00],
																																							pos	: [-0.00, 6.34, 0.04],
																																						},
																																					},
																																					Bone{
																																						name	: ~"jaw_joint",
																																						children	: ~[
																																							Bone{
																																								name	: ~"jaw_end_joint",
																																								children	: ~[],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [-0.00, 0.71, 0.00, 0.71],
																																									pos	: [0.00, 3.68, -0.00],
																																								},
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.82, 0.00, 0.00, 0.57],
																																							pos	: [-0.00, -0.41, 0.55],
																																						},
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.00, 0.00, 0.00, 1.00],
																																					pos	: [0.00, 2.37, -0.00],
																																				},
																																			}
																																		],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [-0.00, 0.98, -0.21, -0.00],
																																			pos	: [-0.00, 2.10, -1.00],
																																		},
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [0.00, -0.00, 0.00, 1.00],
																																	pos	: [0.00, 0.44, -0.29],
																																},
																															},
																															Bone{
																																name	: ~"c_shealth_01_joint",
																																children	: ~[
																																	Bone{
																																		name	: ~"c_shealth_end_joint",
																																		children	: ~[],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, 1.00, -0.00, -0.00],
																																			pos	: [0.00, 1.81, 0.00],
																																		},
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, 0.16, 0.99, 0.00],
																																	pos	: [-0.00, 0.04, 2.55],
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
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, 0.94, 0.00, 0.35],
																																																											pos	: [-0.00, 0.96, 0.00],
																																																										},
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, 0.34, 0.07, 0.93],
																																																									pos	: [-0.00, 0.75, 0.00],
																																																								},
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.25, 0.56, -0.16, 0.77],
																																																							pos	: [-0.00, 1.18, 0.00],
																																																						},
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.37, 0.10, -0.71, 0.59],
																																																					pos	: [-0.54, -0.80, -1.23],
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
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, 0.96, -0.00, 0.29],
																																																											pos	: [-0.00, 1.00, 0.00],
																																																										},
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, 0.37, 0.10, 0.92],
																																																									pos	: [0.00, 0.95, -0.00],
																																																								},
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.27, 0.81, -0.24, 0.46],
																																																							pos	: [0.00, 1.63, 0.00],
																																																						},
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.54, -0.07, -0.59, 0.60],
																																																					pos	: [0.26, -0.29, -0.98],
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
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, 0.24, 0.00, 0.97],
																																																											pos	: [0.00, 1.24, 0.00],
																																																										},
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.07, 0.19, 0.02, 0.98],
																																																									pos	: [0.00, 1.01, -0.00],
																																																								},
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.16, 0.83, -0.31, 0.44],
																																																							pos	: [0.00, 1.64, 0.00],
																																																						},
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.58, -0.15, -0.55, 0.58],
																																																					pos	: [0.92, 0.08, -0.55],
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
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, 0.99, -0.00, 0.17],
																																																											pos	: [-0.00, 1.09, -0.00],
																																																										},
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.14, 0.36, -0.05, 0.92],
																																																									pos	: [-0.00, 1.03, -0.00],
																																																								},
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.05, 0.73, -0.36, 0.58],
																																																							pos	: [0.00, 1.48, -0.00],
																																																						},
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.63, -0.19, -0.46, 0.59],
																																																					pos	: [1.60, 0.39, 0.17],
																																																				},
																																																			}
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.11, -0.29, 0.75, 0.59],
																																																			pos	: [-0.06, 2.48, -0.37],
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
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.00, -0.82, 0.00, 0.57],
																																																									pos	: [0.00, 1.17, 0.00],
																																																								},
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.01, 0.12, 0.02, 0.99],
																																																							pos	: [-0.00, 1.10, -0.00],
																																																						},
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.06, -0.32, -0.13, 0.94],
																																																					pos	: [0.00, 1.91, 0.00],
																																																				},
																																																			}
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.38, 0.51, 0.19, 0.75],
																																																			pos	: [-0.43, 1.21, 0.79],
																																																		},
																																																	}
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.00, 0.02, 0.00, 1.00],
																																																	pos	: [0.00, 5.42, -0.00],
																																																},
																																															}
																																														],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, -0.02, 0.02, 1.00],
																																															pos	: [-0.00, 2.64, -0.00],
																																														},
																																													}
																																												],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.00, 0.01, 0.00, 1.00],
																																													pos	: [0.00, 2.77, 0.00],
																																												},
																																											}
																																										],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.24, 0.27, 0.02, 0.93],
																																											pos	: [-0.00, 3.95, -0.00],
																																										},
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																									pos	: [0.00, 3.58, 0.00],
																																								},
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, 0.91, 0.00, 0.40],
																																							pos	: [0.00, 3.42, 0.00],
																																						},
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.15, 0.73, 0.20, 0.63],
																																					pos	: [-0.52, 0.95, -0.03],
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
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.87, 0.00, 0.50],
																																											pos	: [0.00, 2.16, -0.00],
																																										},
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.24, 0.35, -0.81, 0.40],
																																									pos	: [0.00, 1.03, -0.00],
																																								},
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.33, -0.01, 0.91, 0.25],
																																							pos	: [0.00, 3.22, 0.00],
																																						},
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.37, 0.26, -0.12, 0.89],
																																					pos	: [-0.31, -0.70, 0.27],
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
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																									pos	: [-0.00, 10.84, -0.00],
																																								},
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.13, 0.96, -0.21, 0.13],
																																							pos	: [0.00, 10.95, 0.00],
																																						},
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.15, 0.73, 0.20, 0.63],
																																					pos	: [-0.52, 0.95, -0.03],
																																				},
																																			}
																																		],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [-0.00, -0.93, -0.00, 0.36],
																																			pos	: [-0.00, 5.59, -0.00],
																																		},
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.31, 0.71, 0.50, 0.39],
																																	pos	: [-1.00, -2.13, -2.41],
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
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [-0.00, 0.82, -0.00, 0.57],
																																																									pos	: [-0.00, 1.17, 0.00],
																																																								},
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.01, -0.12, -0.02, 0.99],
																																																							pos	: [0.00, 1.10, -0.00],
																																																						},
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.06, 0.32, 0.13, 0.94],
																																																					pos	: [0.00, 1.91, -0.00],
																																																				},
																																																			}
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.38, -0.51, -0.19, 0.75],
																																																			pos	: [0.43, 1.21, 0.79],
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
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.99, 0.00, 0.17],
																																																											pos	: [-0.00, 1.09, -0.00],
																																																										},
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.15, -0.31, 0.02, 0.94],
																																																									pos	: [0.00, 1.03, -0.00],
																																																								},
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.07, -0.67, 0.35, 0.65],
																																																							pos	: [-0.00, 1.48, -0.00],
																																																						},
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.45, 0.13, 0.60, 0.65],
																																																					pos	: [-1.60, 0.39, 0.17],
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
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, -0.24, -0.00, 0.97],
																																																											pos	: [-0.00, 1.24, -0.00],
																																																										},
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.07, -0.19, -0.02, 0.98],
																																																									pos	: [-0.00, 1.01, -0.00],
																																																								},
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.16, -0.83, 0.31, 0.44],
																																																							pos	: [0.00, 1.64, 0.00],
																																																						},
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.58, 0.15, 0.55, 0.58],
																																																					pos	: [-0.92, 0.08, -0.55],
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
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.96, -0.00, 0.29],
																																																											pos	: [0.00, 1.00, 0.00],
																																																										},
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, -0.41, -0.09, 0.90],
																																																									pos	: [-0.00, 0.95, -0.00],
																																																								},
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.23, -0.84, 0.28, 0.40],
																																																							pos	: [-0.00, 1.63, -0.00],
																																																						},
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.62, 0.09, 0.52, 0.57],
																																																					pos	: [-0.26, -0.29, -0.98],
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
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, -0.94, 0.00, 0.35],
																																																											pos	: [0.00, 0.96, 0.00],
																																																										},
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, -0.42, -0.07, 0.90],
																																																									pos	: [-0.00, 0.75, 0.00],
																																																								},
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.21, -0.68, 0.22, 0.67],
																																																							pos	: [0.00, 1.18, 0.00],
																																																						},
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.57, -0.04, 0.62, 0.53],
																																																					pos	: [0.54, -0.80, -1.23],
																																																				},
																																																			}
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.11, 0.29, -0.75, 0.59],
																																																			pos	: [0.06, 2.48, -0.37],
																																																		},
																																																	}
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, -0.02, 0.00, 1.00],
																																																	pos	: [0.00, 5.42, -0.00],
																																																},
																																															}
																																														],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, 0.02, -0.02, 1.00],
																																															pos	: [0.00, 2.64, -0.00],
																																														},
																																													}
																																												],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.01, -0.00, 1.00],
																																													pos	: [0.00, 2.77, 0.00],
																																												},
																																											}
																																										],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.24, -0.27, -0.02, 0.93],
																																											pos	: [0.00, 3.95, -0.00],
																																										},
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.00, -0.00, -0.00, 1.00],
																																									pos	: [0.00, 3.58, -0.00],
																																								},
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, -0.21, 0.00, 0.98],
																																							pos	: [0.00, 3.42, -0.00],
																																						},
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.11, 0.20, 0.23, 0.95],
																																					pos	: [-0.31, 0.95, 0.42],
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
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.38, 0.00, 0.93],
																																											pos	: [-0.00, 2.16, 0.00],
																																										},
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.24, -0.35, 0.81, 0.40],
																																									pos	: [-0.00, 1.03, -0.00],
																																								},
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [-0.54, 0.21, -0.80, 0.14],
																																							pos	: [0.00, 3.22, 0.00],
																																						},
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.38, 0.65, -0.05, 0.65],
																																					pos	: [-0.40, -0.70, 0.06],
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
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																									pos	: [0.00, 10.84, 0.00],
																																								},
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.24, -0.46, 0.02, 0.85],
																																							pos	: [0.00, 10.95, -0.00],
																																						},
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.11, 0.20, 0.23, 0.95],
																																					pos	: [-0.31, 0.95, 0.42],
																																				},
																																			}
																																		],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, -0.72, -0.00, 0.70],
																																			pos	: [0.00, 5.59, -0.00],
																																		},
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.32, -0.71, -0.50, 0.38],
																																	pos	: [1.00, -2.13, -2.41],
																																},
																															}
																														],
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [-0.00, 1.00, -0.00, -0.00],
																															pos	: [0.00, 6.39, -0.06],
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
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, -0.71, 0.00, 0.71],
																																			pos	: [0.00, 0.00, 0.00],
																																		},
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, 0.58, -0.00, 0.81],
																																	pos	: [-0.00, 3.94, -0.00],
																																},
																															}
																														],
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.84, 0.12, -0.06, 0.52],
																															pos	: [3.20, -0.56, 2.70],
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
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, 0.71, -0.00, 0.71],
																																			pos	: [0.00, -0.00, -0.00],
																																		},
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, -0.58, 0.00, 0.81],
																																	pos	: [-0.00, 3.94, -0.00],
																																},
																															}
																														],
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.84, -0.12, 0.06, 0.52],
																															pos	: [-3.20, -0.56, 2.70],
																														},
																													}
																												],
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [-0.00, 0.00, 0.00, 1.00],
																													pos	: [0.00, 6.18, -0.00],
																												},
																											}
																										],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.11, 0.00, -0.00, 0.99],
																											pos	: [-0.00, 1.50, 0.00],
																										},
																									}
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.02, 0.00, -0.00, 1.00],
																									pos	: [-0.00, 1.45, 0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.10, 0.00, -0.00, 0.99],
																							pos	: [-0.00, 1.54, 0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, 0.71, 0.00, 0.71],
																					pos	: [0.00, 1.49, 0.00],
																				},
																			}
																		],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			pos	: [-0.04, 0.00, -0.00],
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
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.00, 0.71, 0.00, 0.71],
																							pos	: [-0.00, 4.60, -0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.69, -0.14, -0.69, 0.14],
																					pos	: [4.99, 0.48, 2.82],
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
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, -0.00, 0.00, 1.00],
																											pos	: [0.00, 4.95, 0.00],
																										},
																									}
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																									pos	: [0.00, 5.50, -0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.85, -0.02, 0.53, -0.00],
																							pos	: [-0.19, -6.70, -0.11],
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
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [-0.00, -0.72, 0.00, 0.70],
																													pos	: [0.00, 2.80, -0.00],
																												},
																											}
																										],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.23, 0.02, 0.01, 0.97],
																											pos	: [-0.00, 8.00, -0.00],
																										},
																									}
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.56, -0.19, -0.14, 0.80],
																									pos	: [-0.00, 19.52, 0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.22, -0.06, -0.97, 0.00],
																							pos	: [-0.53, -18.67, -0.32],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.51, -0.51, 0.49, 0.49],
																					pos	: [4.92, 0.46, 4.10],
																				},
																			},
																			Bone{
																				name	: ~"L_hip_base_joint",
																				children	: ~[
																					Bone{
																						name	: ~"L_hip_joint",
																						children	: ~[],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.00, 0.71, 0.00, 0.71],
																							pos	: [-0.00, 4.60, 0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.69, -0.14, -0.69, 0.14],
																					pos	: [4.99, 0.48, -2.82],
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
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.00, 0.00, 0.00, 1.00],
																											pos	: [0.00, 4.95, -0.00],
																										},
																									}
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 5.50, 0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.95, 0.01, -0.30, -0.00],
																							pos	: [0.19, -6.70, -0.06],
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
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [0.00, -0.69, 0.00, 0.72],
																													pos	: [0.00, 2.80, -0.00],
																												},
																											}
																										],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.23, -0.06, -0.02, 0.97],
																											pos	: [-0.00, 8.00, 0.00],
																										},
																									}
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.55, 0.25, 0.14, 0.79],
																									pos	: [0.00, 19.52, 0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.25, 0.05, 0.97, 0.00],
																							pos	: [0.53, -18.67, -0.16],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.49, -0.49, 0.51, 0.51],
																					pos	: [4.92, 0.46, -4.10],
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
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, -0.00, 1.00],
																									pos	: [-0.00, 5.95, 0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.07, 0.99, 0.04, 0.07],
																							pos	: [0.00, 4.80, -0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.68, 0.70, -0.05, 0.19],
																					pos	: [1.22, 4.37, 3.97],
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
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, -0.00, 1.00],
																									pos	: [0.00, 5.95, -0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.07, -0.93, 0.04, 0.35],
																							pos	: [0.00, 4.80, -0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.68, -0.73, -0.00, 0.00],
																					pos	: [2.94, 4.48, 2.04],
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
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, 0.00, 1.00],
																									pos	: [0.00, 5.95, 0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.07, 0.93, -0.04, 0.35],
																							pos	: [0.00, 4.80, 0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, -0.00, -0.68, 0.73],
																					pos	: [2.94, 4.48, -2.04],
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
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																									pos	: [-0.00, 5.95, -0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.13, 0.98, 0.14, 0.03],
																							pos	: [0.00, 4.80, -0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.08, 0.39, -0.73, 0.56],
																					pos	: [-1.03, 1.62, -5.94],
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
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, 0.00, 1.00],
																									pos	: [-0.00, 5.95, 0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.13, -0.96, -0.19, 0.17],
																							pos	: [-0.00, 4.80, 0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.34, -0.02, -0.78, 0.53],
																					pos	: [-0.29, -2.24, -3.34],
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
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, 3.88, -0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.01, 0.97, 0.25, 0.04],
																							pos	: [-0.00, 4.80, -0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.61, -0.37, -0.60, 0.36],
																					pos	: [1.63, -3.54, -0.00],
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
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, -0.00, 1.00],
																									pos	: [-0.00, 5.95, -0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.13, 0.96, 0.19, 0.17],
																							pos	: [0.00, 4.80, -0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.78, -0.53, -0.34, 0.02],
																					pos	: [-0.29, -2.24, 3.34],
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
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																									pos	: [0.00, 5.95, 0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.13, -0.98, -0.14, 0.03],
																							pos	: [0.00, 4.80, 0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.73, 0.56, -0.08, 0.39],
																					pos	: [-1.03, 1.62, 5.94],
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
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, 0.00, 1.00],
																									pos	: [0.00, 5.95, 0.00],
																								},
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.07, -0.99, -0.04, 0.07],
																							pos	: [0.00, 4.80, -0.00],
																						},
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.05, 0.19, -0.68, 0.70],
																					pos	: [1.22, 4.37, -3.97],
																				},
																			}
																		],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, -0.71, 0.00, 0.71],
																			pos	: [0.00, -0.10, 0.04],
																		},
																	}
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.71, 0.00, 0.00, 0.71],
																	pos	: [0.00, 47.22, 0.24],
																},
															}
														],
														dual_quat	: false,
													})
												],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
											}),
											ChildNode(Node{
												name	: ~"SKELETON",
												actions	: ~[],
												children	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
											}),
											ChildNode(Node{
												name	: ~"c_eye_ctrl_grp",
												actions	: ~[],
												children	: ~[
													ChildNode(Node{
														name	: ~"mainEye_ctrl_zero",
														actions	: ~[],
														children	: ~[
															ChildNode(Node{
																name	: ~"mainEye_ctrl",
																actions	: ~[],
																children	: ~[
																	ChildNode(Node{
																		name	: ~"R_eye_ctrl_grp",
																		actions	: ~[],
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"R_eye_ctrl_zero",
																				actions	: ~[],
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"R_eye_ctrl",
																						actions	: ~[],
																						children	: ~[],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.00, -0.00, -0.00, 1.00],
																							pos	: [0.00, 0.00, -0.00],
																						},
																					})
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.00, 0.00, -0.00, 1.00],
																					pos	: [-0.00, -0.00, 0.00],
																				},
																			})
																		],
																		space	: QuatSpace{
																			scale	: 0.72,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [-0.95, -0.00, 0.00],
																		},
																	}),
																	ChildNode(Node{
																		name	: ~"L_eye_ctrl_grp",
																		actions	: ~[],
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"L_eye_ctrl_zero",
																				actions	: ~[],
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"L_eye_ctrl",
																						actions	: ~[],
																						children	: ~[],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.00, 0.00, -0.00, 1.00],
																							pos	: [0.00, 0.00, -0.00],
																						},
																					})
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.00, 0.00, -0.00, 1.00],
																					pos	: [0.00, 0.00, -0.00],
																				},
																			})
																		],
																		space	: QuatSpace{
																			scale	: 0.72,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [0.95, 0.00, -0.00],
																		},
																	})
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	pos	: [0.04, -71.88, -17.98],
																},
															})
														],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-0.04, 71.88, 17.98],
														},
													})
												],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
											}),
											ChildNode(Node{
												name	: ~"Locators",
												actions	: ~[],
												children	: ~[
													ChildNode(Node{
														name	: ~"L_eye_centerLocator",
														actions	: ~[],
														children	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [1.26, 71.88, 0.51],
														},
													}),
													ChildNode(Node{
														name	: ~"R_eye_centerLocator",
														actions	: ~[],
														children	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-1.26, 71.88, 0.51],
														},
													})
												],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
											}),
											ChildNode(Node{
												name	: ~"LegControls",
												actions	: ~[],
												children	: ~[
													ChildNode(Node{
														name	: ~"L_leg_ikHandle_grp",
														actions	: ~[],
														children	: ~[
															ChildNode(Node{
																name	: ~"L_leg_ikHandle_zero",
																actions	: ~[],
																children	: ~[
																	ChildNode(Node{
																		name	: ~"L_foot_ik_ctrl",
																		actions	: ~[],
																		children	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																			pos	: [-0.00, 0.00, 0.00],
																		},
																	})
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, -0.00, 1.00],
																	pos	: [0.00, -0.00, 0.00],
																},
															})
														],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [3.00, -0.31, -3.16],
														},
													}),
													ChildNode(Node{
														name	: ~"R_leg_ikHandle_grp",
														actions	: ~[],
														children	: ~[
															ChildNode(Node{
																name	: ~"R_leg_ikHandle_zero",
																actions	: ~[],
																children	: ~[
																	ChildNode(Node{
																		name	: ~"R_foot_ik_ctrl",
																		actions	: ~[],
																		children	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.01, -0.00, 0.00, 1.00],
																			pos	: [0.00, 0.00, 0.00],
																		},
																	})
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	pos	: [-0.00, -0.00, 0.00],
																},
															})
														],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-3.00, -0.31, -3.16],
														},
													}),
													ChildNode(Node{
														name	: ~"R_legPole_ctrl",
														actions	: ~[],
														children	: ~[
															ChildNode(Node{
																name	: ~"R_legPole_ctrl_zero",
																actions	: ~[],
																children	: ~[
																	ChildNode(Node{
																		name	: ~"R_legPole_ctrl.001",
																		actions	: ~[],
																		children	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																			pos	: [0.00, 0.00, 0.00],
																		},
																	})
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.00, -0.00, -0.00, 1.00],
																	pos	: [0.57, -18.67, 10.59],
																},
															})
														],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-4.14, 43.04, -0.16],
														},
													}),
													ChildNode(Node{
														name	: ~"L_legPole_ctrl_cons",
														actions	: ~[],
														children	: ~[
															ChildNode(Node{
																name	: ~"L_legPole_ctrl_grp",
																actions	: ~[],
																children	: ~[
																	ChildNode(Node{
																		name	: ~"L_legPole_cntr_zero",
																		actions	: ~[],
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"L_legPole_ctrl",
																				actions	: ~[],
																				children	: ~[],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																					pos	: [0.00, -0.00, -0.00],
																				},
																			})
																		],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [-0.55, -17.93, 9.74],
																		},
																	})
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.47, -0.53, 0.49, 0.51],
																	pos	: [0.00, 0.00, -0.00],
																},
															})
														],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [-0.47, 0.53, -0.49, 0.51],
															pos	: [4.10, 42.26, 0.61],
														},
													})
												],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
											}),
											ChildNode(Node{
												name	: ~"L_arm_IK_ctrl_grp",
												actions	: ~[],
												children	: ~[
													ChildNode(Node{
														name	: ~"L_arm_IK_ctrl_zero",
														actions	: ~[],
														children	: ~[
															ChildNode(Node{
																name	: ~"L_arm_IK_ctrl",
																actions	: ~[],
																children	: ~[
																	ChildNode(Node{
																		name	: ~"L_armIK_handle",
																		actions	: ~[],
																		children	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [-0.00, 0.00, -0.00],
																		},
																	}),
																	ChildNode(Node{
																		name	: ~"L_hand_grp",
																		actions	: ~[],
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"L_hand_zero",
																				actions	: ~[],
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"L_palm_ctrl",
																						actions	: ~[],
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"L_thumb_ctrl",
																								actions	: ~[],
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								name	: ~"L_indexF_ctrl",
																								actions	: ~[],
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								name	: ~"L_middleF_ctrl",
																								actions	: ~[],
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								name	: ~"L_ringF_ctrl",
																								actions	: ~[],
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								name	: ~"L_pinkyF_ctrl",
																								actions	: ~[],
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																							})
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.00, 0.00, -0.00, 1.00],
																							pos	: [0.00, -0.00, 0.00],
																						},
																					})
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.00, -0.00, 0.00, 1.00],
																					pos	: [-0.00, 0.00, -0.00],
																				},
																			})
																		],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.26, -0.08, -0.07, 0.96],
																			pos	: [-0.00, 0.00, -0.00],
																		},
																	})
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.00, -0.00, 0.00, 1.00],
																	pos	: [-27.41, -60.54, 2.62],
																},
															})
														],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.26, 0.08, 0.07, 0.96],
															pos	: [0.00, -0.00, 0.00],
														},
													})
												],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [-0.26, -0.08, -0.07, 0.96],
													pos	: [27.41, 60.54, -2.62],
												},
											}),
											ChildNode(Node{
												name	: ~"c_hips_cntr_backup",
												actions	: ~[],
												children	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 46.47, 1.11],
												},
											}),
											ChildNode(Node{
												name	: ~"R_arm_IK_ctrl_grp",
												actions	: ~[],
												children	: ~[
													ChildNode(Node{
														name	: ~"R_arm_IK_ctrl_zero",
														actions	: ~[],
														children	: ~[
															ChildNode(Node{
																name	: ~"R_arm_IK_ctrl1",
																actions	: ~[],
																children	: ~[
																	ChildNode(Node{
																		name	: ~"ikHandle4",
																		actions	: ~[],
																		children	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, 0.00, 0.00, 1.00],
																			pos	: [-54.83, -0.00, 0.00],
																		},
																	}),
																	ChildNode(Node{
																		name	: ~"R_hand_grp",
																		actions	: ~[],
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"R_hand_zero",
																				actions	: ~[],
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"R_palm_ctrl",
																						actions	: ~[],
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"R_thumb_ctrl",
																								actions	: ~[],
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								name	: ~"R_indexF_ctrl",
																								actions	: ~[],
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								name	: ~"R_middleF_ctrl",
																								actions	: ~[],
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								name	: ~"R_ringF_ctrl",
																								actions	: ~[],
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								name	: ~"R_pinkyF_ctrl",
																								actions	: ~[],
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																							})
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.07, 0.96, -0.26, 0.08],
																							pos	: [0.00, 0.00, -0.00],
																						},
																					})
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																					pos	: [0.00, -0.00, 0.00],
																				},
																			})
																		],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.07, -0.96, 0.26, 0.08],
																			pos	: [-108.05, 6.80, 11.25],
																		},
																	})
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	pos	: [-27.33, -60.54, 2.62],
																},
															})
														],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [-0.00, -0.00, 0.00, 1.00],
															pos	: [0.00, 0.00, 0.00],
														},
													})
												],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [27.33, 60.54, -2.62],
												},
											}),
											ChildNode(Node{
												name	: ~"c_cog_ctrl_grp",
												actions	: ~[],
												children	: ~[
													ChildNode(Node{
														name	: ~"c_cog_ctrl_zero",
														actions	: ~[],
														children	: ~[
															ChildNode(Node{
																name	: ~"c_cog_ctrl",
																actions	: ~[],
																children	: ~[
																	ChildNode(Node{
																		name	: ~"c_hips_ctrl_grp",
																		actions	: ~[],
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"c_hips_ctrl_zero",
																				actions	: ~[],
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"c_hips_ctrl",
																						actions	: ~[],
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"group13",
																								actions	: ~[],
																								children	: ~[
																									ChildNode(Node{
																										name	: ~"cluster3Handle",
																										actions	: ~[],
																										children	: ~[],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																									}),
																									ChildNode(Node{
																										name	: ~"cluster2Handle",
																										actions	: ~[],
																										children	: ~[],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																									}),
																									ChildNode(Node{
																										name	: ~"cluster1Handle",
																										actions	: ~[],
																										children	: ~[],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																									})
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.46, 0.54, -0.46, 0.54],
																									pos	: [-46.65, -7.08, 0.00],
																								},
																							})
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.46, 0.54, -0.46, 0.54],
																							pos	: [0.00, -0.00, 0.00],
																						},
																					})
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																					pos	: [0.00, -0.00, -0.00],
																				},
																			})
																		],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			pos	: [-0.00, -0.04, -0.10],
																		},
																	}),
																	ChildNode(Node{
																		name	: ~"c_spine_01_ctrl_grp",
																		actions	: ~[],
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"c_spine_01_ctrl_zero",
																				actions	: ~[],
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"c_spine_01_ctrl",
																						actions	: ~[],
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"c_spine_03_ctrl_grp",
																								actions	: ~[],
																								children	: ~[
																									ChildNode(Node{
																										name	: ~"c_spine_03_ctrl_zero",
																										actions	: ~[],
																										children	: ~[
																											ChildNode(Node{
																												name	: ~"c_spine_03_ctrl",
																												actions	: ~[],
																												children	: ~[
																													ChildNode(Node{
																														name	: ~"group14",
																														actions	: ~[],
																														children	: ~[
																															ChildNode(Node{
																																name	: ~"cluster4Handle",
																																actions	: ~[],
																																children	: ~[],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [0.00, 0.00, -0.00, 1.00],
																																	pos	: [0.00, 0.00, 0.00],
																																},
																															})
																														],
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.52, 0.48, -0.52, 0.48],
																															pos	: [-50.08, 3.75, -0.04],
																														},
																													}),
																													ChildNode(Node{
																														name	: ~"c_spine_05_ctrl_grp",
																														actions	: ~[],
																														children	: ~[
																															ChildNode(Node{
																																name	: ~"c_spine_05_ctrl_zero",
																																actions	: ~[],
																																children	: ~[
																																	ChildNode(Node{
																																		name	: ~"c_spine_05_ctrl",
																																		actions	: ~[],
																																		children	: ~[
																																			ChildNode(Node{
																																				name	: ~"group12",
																																				actions	: ~[],
																																				children	: ~[
																																					ChildNode(Node{
																																						name	: ~"cluster6Handle",
																																						actions	: ~[],
																																						children	: ~[],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																							pos	: [-0.00, -0.00, 0.00],
																																						},
																																					}),
																																					ChildNode(Node{
																																						name	: ~"cluster5Handle",
																																						actions	: ~[],
																																						children	: ~[],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																							pos	: [-0.00, -0.00, 0.00],
																																						},
																																					})
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.57, 0.41, -0.57, 0.41],
																																					pos	: [-50.19, 17.53, -0.04],
																																				},
																																			}),
																																			ChildNode(Node{
																																				name	: ~"c_chest_ctrl_grp",
																																				actions	: ~[],
																																				children	: ~[
																																					ChildNode(Node{
																																						name	: ~"c_chest_ctrl_zero",
																																						actions	: ~[],
																																						children	: ~[
																																							ChildNode(Node{
																																								name	: ~"c_chest_ctrl",
																																								actions	: ~[],
																																								children	: ~[
																																									ChildNode(Node{
																																										name	: ~"group11",
																																										actions	: ~[],
																																										children	: ~[
																																											ChildNode(Node{
																																												name	: ~"cluster9Handle",
																																												actions	: ~[],
																																												children	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																											}),
																																											ChildNode(Node{
																																												name	: ~"cluster8Handle",
																																												actions	: ~[],
																																												children	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																											}),
																																											ChildNode(Node{
																																												name	: ~"cluster7Handle",
																																												actions	: ~[],
																																												children	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																											})
																																										],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.58, 0.41, -0.58, 0.41],
																																											pos	: [-56.22, 18.02, -0.04],
																																										},
																																									}),
																																									ChildNode(Node{
																																										name	: ~"c_breastControls_grp",
																																										actions	: ~[],
																																										children	: ~[
																																											ChildNode(Node{
																																												name	: ~"null1",
																																												actions	: ~[],
																																												children	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, 0.00, -0.00, 1.00],
																																													pos	: [0.00, -0.00, -0.00],
																																												},
																																											}),
																																											ChildNode(Node{
																																												name	: ~"R_breast_IK_control_grp",
																																												actions	: ~[],
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"R_breast_IK_cntr_zero",
																																														actions	: ~[],
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"R_breast_IK_ctrl",
																																																actions	: ~[],
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"ikHandle2",
																																																		actions	: ~[],
																																																		children	: ~[],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																	}),
																																																	ChildNode(Node{
																																																		name	: ~"R_breastTweak_cntr_grp",
																																																		actions	: ~[],
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"R_breastTweak_zero",
																																																				actions	: ~[],
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"R_breastTweak_ctrl",
																																																						actions	: ~[],
																																																						children	: ~[],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.00, 0.00, 0.00, 1.00],
																																																							pos	: [-0.00, 0.00, -0.00],
																																																						},
																																																					})
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [0.00, 0.00, -0.00, 1.00],
																																																					pos	: [-0.00, 0.00, -0.00],
																																																				},
																																																			})
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.03, -0.70, 0.06, 0.71],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																	})
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																																	pos	: [0.00, 0.00, -0.00],
																																																},
																																															})
																																														],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																													})
																																												],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.10, -0.08, -0.84, 0.52],
																																													pos	: [-2.31, -6.09, 4.23],
																																												},
																																											}),
																																											ChildNode(Node{
																																												name	: ~"L_breast_IK_control_grp",
																																												actions	: ~[],
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"L_breast_IK_cntr_zero",
																																														actions	: ~[],
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"L_breastTweak_cntr_grp",
																																																actions	: ~[],
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"L_breastTweak_zero",
																																																		actions	: ~[],
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"L_breastTweak_ctrl",
																																																				actions	: ~[],
																																																				children	: ~[],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.00, -0.00, -0.00, 1.00],
																																																					pos	: [0.00, -0.00, -0.00],
																																																				},
																																																			})
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																			pos	: [0.00, 0.00, 0.00],
																																																		},
																																																	})
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.03, 0.70, 0.06, 0.71],
																																																	pos	: [-0.00, 0.00, -0.00],
																																																},
																																															}),
																																															ChildNode(Node{
																																																name	: ~"L_breast_IK_ctrl",
																																																actions	: ~[],
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"ikHandle3",
																																																		actions	: ~[],
																																																		children	: ~[],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [-0.00, 0.00, -0.00, 1.00],
																																																			pos	: [0.00, -0.00, 0.00],
																																																		},
																																																	})
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.00, 0.00, 0.00, 1.00],
																																																	pos	: [-0.00, 0.00, -0.00],
																																																},
																																															})
																																														],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [0.00, 0.00, 0.00, 1.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																													})
																																												],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.10, 0.08, -0.84, 0.52],
																																													pos	: [-2.31, -6.09, -4.23],
																																												},
																																											})
																																										],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.00, 0.00, 1.00],
																																											pos	: [0.00, 0.00, 0.00],
																																										},
																																									}),
																																									ChildNode(Node{
																																										name	: ~"c_neck_01_ctrl_grp",
																																										actions	: ~[],
																																										children	: ~[
																																											ChildNode(Node{
																																												name	: ~"R_clav_ctrl_grp",
																																												actions	: ~[],
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"R_clav_ctrl_zero",
																																														actions	: ~[],
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"R_clav_ctrl",
																																																actions	: ~[],
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"R_mainSpaulder_ctrl_grp",
																																																		actions	: ~[],
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"R_mainSpaulder_ctrl_zero",
																																																				actions	: ~[],
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"R_mainSpaulder_ctrl",
																																																						actions	: ~[],
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								name	: ~"R_subSpaulder_ctrl_grp",
																																																								actions	: ~[],
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										name	: ~"R_subSpaulder_ctrl_zero",
																																																										actions	: ~[],
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												name	: ~"R_subSpaulder_ctrl",
																																																												actions	: ~[],
																																																												children	: ~[],
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [-0.00, 0.00, -0.04, 1.00],
																																																													pos	: [0.00, 0.00, -0.00],
																																																												},
																																																											})
																																																										],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.00, 0.00, 1.00],
																																																											pos	: [-0.00, 0.00, -0.00],
																																																										},
																																																									})
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [-0.00, -0.30, 0.00, 0.96],
																																																									pos	: [2.32, 0.02, 0.51],
																																																								},
																																																							})
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.00, 0.00, -0.03, 1.00],
																																																							pos	: [0.00, 0.00, -0.00],
																																																						},
																																																					})
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																																					pos	: [-0.00, 0.00, 0.00],
																																																				},
																																																			})
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.13, 0.32, -0.21, 0.91],
																																																			pos	: [4.89, -0.16, 0.38],
																																																		},
																																																	})
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, -0.12, 0.95, 0.28],
																																																	pos	: [0.00, -0.00, 0.00],
																																																},
																																															})
																																														],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																															pos	: [0.00, -0.00, -0.00],
																																														},
																																													})
																																												],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.19, -0.65, 0.22, 0.71],
																																													pos	: [-1.39, -3.03, 1.00],
																																												},
																																											}),
																																											ChildNode(Node{
																																												name	: ~"L_clav_ctrl_grp",
																																												actions	: ~[],
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"L_clav_ctrl_zero",
																																														actions	: ~[],
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"L_clav_ctrl",
																																																actions	: ~[],
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"L_mainSpaulder_ctrl_grp",
																																																		actions	: ~[],
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"L_mainSpaulder_ctrl_zero",
																																																				actions	: ~[],
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"L_mainSpaulder_ctrl",
																																																						actions	: ~[],
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								name	: ~"L_subSpaulder_ctrl_grp",
																																																								actions	: ~[],
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										name	: ~"L_subSpaulder_ctrl_zero",
																																																										actions	: ~[],
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												name	: ~"L_subSpaulder_ctrl",
																																																												actions	: ~[],
																																																												children	: ~[],
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [-0.00, -0.00, -0.04, 1.00],
																																																													pos	: [0.00, 0.00, -0.00],
																																																												},
																																																											})
																																																										],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.00, -0.00, 1.00],
																																																											pos	: [-0.00, 0.00, -0.00],
																																																										},
																																																									})
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.00, 0.30, 0.00, 0.96],
																																																									pos	: [2.32, 0.02, -0.51],
																																																								},
																																																							})
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.00, -0.00, -0.03, 1.00],
																																																							pos	: [0.00, -0.00, -0.00],
																																																						},
																																																					})
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [0.00, 0.00, 0.00, 1.00],
																																																					pos	: [0.00, -0.00, 0.00],
																																																				},
																																																			})
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [-0.09, -0.32, -0.22, 0.92],
																																																			pos	: [4.89, -0.12, -0.39],
																																																		},
																																																	})
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, -0.01, -0.28, 0.96],
																																																	pos	: [0.00, 0.00, -0.00],
																																																},
																																															})
																																														],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, -0.00, -0.00, 1.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																													})
																																												],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.16, 0.64, 0.24, 0.71],
																																													pos	: [-1.39, -3.03, -1.00],
																																												},
																																											}),
																																											ChildNode(Node{
																																												name	: ~"c_neck_01_ctrl_zero",
																																												actions	: ~[],
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"c_neck_01_ctrl",
																																														actions	: ~[],
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"c_neck_02_ctrl_grp",
																																																actions	: ~[],
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"c_neck_02_ctrl_zero",
																																																		actions	: ~[],
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"c_neck_02_ctrl",
																																																				actions	: ~[],
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"c_neck_03_ctrl_grp",
																																																						actions	: ~[],
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								name	: ~"c_neck_03_ctrl_zero",
																																																								actions	: ~[],
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										name	: ~"c_neck_03_ctrl",
																																																										actions	: ~[],
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												name	: ~"c_jaw_ctrl_grp",
																																																												actions	: ~[],
																																																												children	: ~[
																																																													ChildNode(Node{
																																																														name	: ~"c_jaw_ctrl_zero",
																																																														actions	: ~[],
																																																														children	: ~[
																																																															ChildNode(Node{
																																																																name	: ~"c_jaw_ctrl",
																																																																actions	: ~[],
																																																																children	: ~[],
																																																																space	: QuatSpace{
																																																																	scale	: 1.00,
																																																																	rot	: [-0.00, 0.00, -0.00, 1.00],
																																																																	pos	: [0.00, -0.00, 0.00],
																																																																},
																																																															})
																																																														],
																																																														space	: QuatSpace{
																																																															scale	: 1.00,
																																																															rot	: [-0.00, 0.00, -0.00, 1.00],
																																																															pos	: [-0.00, 0.00, 0.00],
																																																														},
																																																													})
																																																												],
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [0.00, 0.00, -0.82, 0.57],
																																																													pos	: [-0.41, -0.55, 0.00],
																																																												},
																																																											})
																																																										],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.00, 0.05, 1.00],
																																																											pos	: [0.00, 0.00, 0.00],
																																																										},
																																																									})
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																																									pos	: [0.00, 0.00, 0.00],
																																																								},
																																																							})
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.00, -0.00, -0.00, 1.00],
																																																							pos	: [2.37, 0.00, 0.00],
																																																						},
																																																					})
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.00, -0.00, 0.04, 1.00],
																																																					pos	: [0.00, 0.00, -0.00],
																																																				},
																																																			})
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [-0.00, -0.00, -0.00, 1.00],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																	})
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, 0.00, 0.02, 1.00],
																																																	pos	: [2.33, 0.01, -0.00],
																																																},
																																															})
																																														],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, -0.00, 0.06, 1.00],
																																															pos	: [0.00, 0.00, -0.00],
																																														},
																																													})
																																												],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, 0.00],
																																												},
																																											})
																																										],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.00, -0.23, 0.97],
																																											pos	: [6.82, -0.29, 0.00],
																																										},
																																									})
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																									pos	: [-0.00, 0.00, 0.00],
																																								},
																																							})
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [-0.00, -0.00, 0.00, 1.00],
																																							pos	: [0.00, 0.00, -0.00],
																																						},
																																					})
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																					pos	: [6.18, -0.00, -0.00],
																																				},
																																			})
																																		],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.57, 0.41, -0.57, 0.41],
																																			pos	: [0.00, -0.00, -0.00],
																																		},
																																	})
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																	pos	: [0.00, 0.00, 0.00],
																																},
																															})
																														],
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.00, 0.00, 0.13, 0.99],
																															pos	: [2.95, 0.06, -0.00],
																														},
																													})
																												],
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [-0.00, -0.00, -0.00, 1.00],
																													pos	: [-50.12, 3.07, 0.00],
																												},
																											})
																										],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.00, 0.00, -0.00, 1.00],
																											pos	: [0.00, 0.00, 0.00],
																										},
																									})
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.11, 0.99],
																									pos	: [3.03, 0.02, -0.00],
																								},
																							})
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.00, -0.00, 0.08, 1.00],
																							pos	: [-0.00, 0.00, 0.00],
																						},
																					})
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																					pos	: [0.00, -0.00, 0.00],
																				},
																			})
																		],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			pos	: [-0.04, -0.00, 0.00],
																		},
																	})
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, 0.00, 0.00, 1.00],
																	pos	: [-0.00, -47.22, -0.24],
																},
															})
														],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [-0.00, 0.00, -0.00, 1.00],
															pos	: [0.00, -0.00, 0.00],
														},
													})
												],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 47.22, 0.24],
												},
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									})
								],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 1.11, 0.00],
								},
							})
						],
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [-0.00, -1.11, 0.00],
						},
					}),
					ChildNode(Node{
						name	: ~"noTrasnform",
						actions	: ~[],
						children	: ~[
							ChildNode(Node{
								name	: ~"Body",
								actions	: ~[],
								children	: ~[
									ChildNode(Node{
										name	: ~"tongue_geo1",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"Tongue",
												mesh	: ~"tongue_geo1Shape@all",
												range	: [0, 528],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"polySurface172",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"cloak",
												mesh	: ~"polySurfaceShape174@all",
												range	: [0, 15252],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"topJaw_geo2",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"Teeth",
												mesh	: ~"topJaw_geo2Shape@all",
												range	: [0, 4656],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"lowerJaw_geo2",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"Teeth",
												mesh	: ~"lowerJaw_geo2Shape@all",
												range	: [0, 4248],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"L_upper_lash1",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"EyeLashes",
												mesh	: ~"L_upper_lash1Shape@all",
												range	: [0, 13716],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"L_lower_lash1",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"EyeLashes",
												mesh	: ~"L_lower_lash1Shape@all",
												range	: [0, 8964],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"R_upper_lash1",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"EyeLashes",
												mesh	: ~"R_upper_lash1Shape@all",
												range	: [0, 13716],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"R_lower_lash1",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"EyeLashes",
												mesh	: ~"R_lower_lash1Shape@all",
												range	: [0, 8964],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"L_eye_geo1",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"Eyes",
												mesh	: ~"L_eye_geo1Shape@all",
												range	: [0, 2784],
											}),
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"Pupil_SS",
												mesh	: ~"L_eye_geo1Shape@all",
												range	: [2784, 3264],
											}),
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"cornea",
												mesh	: ~"L_eye_geo1Shape@all",
												range	: [3264, 5568],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"R_eye_geo1",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"Eyes",
												mesh	: ~"R_eye_geo1Shape@all",
												range	: [0, 2784],
											}),
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"Pupil_SS",
												mesh	: ~"R_eye_geo1Shape@all",
												range	: [2784, 3264],
											}),
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"cornea",
												mesh	: ~"R_eye_geo1Shape@all",
												range	: [3264, 5568],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"Hair_Geo2",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"anisotropic1",
												mesh	: ~"Hair_Geo2Shape@all",
												range	: [0, 6954],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"Body_geo8",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"skin",
												mesh	: ~"Body_geo8Shape@all",
												range	: [0, 50496],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									})
								],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
							}),
							ChildNode(Node{
								name	: ~"Armor",
								actions	: ~[],
								children	: ~[
									ChildNode(Node{
										name	: ~"boots",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"armor",
												mesh	: ~"R_boot1Shape@all",
												range	: [0, 9042],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, -0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"backShealth1",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"armor",
												mesh	: ~"backShealth1Shape@all",
												range	: [0, 5550],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"skirt",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"armor",
												mesh	: ~"R_skirt_06Shape@all",
												range	: [0, 10236],
											}),
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"skin",
												mesh	: ~"R_skirt_06Shape@all",
												range	: [10236, 12102],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"bracket",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"skin",
												mesh	: ~"bracket_05_geo1Shape@all",
												range	: [0, 8448],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"bracers",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"armor",
												mesh	: ~"L_bracer1Shape@all",
												range	: [0, 2304],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										name	: ~"spaulders",
										actions	: ~[],
										children	: ~[
											ChildEntity(Entity{
												armature	: ~"",
												material	: ~"armor",
												mesh	: ~"R_subSpaulder1Shape@all",
												range	: [0, 6960],
											})
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									})
								],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
							}),
							ChildNode(Node{
								name	: ~"Eyes_Geo",
								actions	: ~[],
								children	: ~[],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
							})
						],
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
					})
				],
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.50, 0.50, 0.50, 0.50],
					pos	: [0.00, 0.00, 0.00],
				},
			}),
			ChildNode(Node{
				name	: ~"Lamp",
				actions	: ~[],
				children	: ~[
					ChildLight(Light{
						attenuation	: [0.00, 1.00],
						kind	: KindSpot(Spot{
							blend	: 0.15,
							size	: 1.31,
						}),
						energy	: 3.00,
						spherical	: false,
						name	: ~"Lamp",
						color	: [1.00, 1.00, 1.00],
						distance	: 100.00,
					})
				],
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.27, 0.31, 0.78, 0.47],
					pos	: [43.55, 25.15, 80.51],
				},
			})
		],
		global	: Global{
			gravity	: [0.00, 0.00, -9.81],
		},
	}}
