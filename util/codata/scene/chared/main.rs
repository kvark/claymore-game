use common::*;
pub fn load()-> Scene	{Scene{
		materials	: ~[
			Material{
				shader	: ~"phong",
				textures	: ~[],
				name	: ~"anisotropic1",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.09, 0.09, 0.08])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				shader	: ~"phong",
				textures	: ~[
					Texture{
						wrap	: 0,
						name	: ~"Main",
						offset	: [0.00, 0.00, 0.00],
						filter	: 3,
						scale	: [1.00, 1.00, 1.00],
						path	: ~"//Metal_R-Spec_G-Bump_B-Reflect.jpg",
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
			},
			Material{
				shader	: ~"phong",
				textures	: ~[],
				name	: ~"cloak",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				shader	: ~"phong",
				textures	: ~[],
				name	: ~"cornea",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.50, 0.50, 0.50])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
			},
			Material{
				shader	: ~"phong",
				textures	: ~[],
				name	: ~"EyeLashes",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				shader	: ~"phong",
				textures	: ~[],
				name	: ~"Eyes",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.44, 0.44, 0.54])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.49, 0.49, 0.49])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
			},
			Material{
				shader	: ~"phong",
				textures	: ~[],
				name	: ~"Material",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				shader	: ~"phong",
				textures	: ~[],
				name	: ~"Pupil_SS",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				shader	: ~"phong",
				textures	: ~[
					Texture{
						wrap	: 0,
						name	: ~"Main.001",
						offset	: [0.00, 0.00, 0.00],
						filter	: 3,
						scale	: [1.00, 1.00, 1.00],
						path	: ~"//Skin_Diffuse.jpg",
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
			},
			Material{
				shader	: ~"phong",
				textures	: ~[],
				name	: ~"Teeth",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.75, 0.75, 0.75])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
			},
			Material{
				shader	: ~"phong",
				textures	: ~[],
				name	: ~"Tongue",
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
				children	: ~[
					ChildEntity(Entity{
						material	: ~"Material",
						armature	: ~"",
						range	: [0, 6],
						mesh	: ~"Plane@",
					})
				],
				actions	: ~[],
				name	: ~"Plane",
				space	: QuatSpace{
					scale	: 100.00,
					rot	: [0.00, 0.00, 0.00, 1.00],
					pos	: [0.00, 0.00, -1.00],
				},
			}),
			ChildNode(Node{
				children	: ~[
					ChildCamera(Camera{
						name	: ~"Camera",
						range	: [10.00, 300.00],
						fov_y	: 0.87,
					})
				],
				actions	: ~[],
				name	: ~"Camera",
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.41, 0.41, 0.58, 0.58],
					pos	: [140.00, 0.00, 90.00],
				},
			}),
			ChildNode(Node{
				children	: ~[
					ChildNode(Node{
						children	: ~[
							ChildNode(Node{
								children	: ~[],
								actions	: ~[],
								name	: ~"ikHandle8",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
							})
						],
						actions	: ~[],
						name	: ~"R_ik_foot_grp",
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
					}),
					ChildNode(Node{
						children	: ~[],
						actions	: ~[],
						name	: ~"L_leg_ikHandle_zero.001",
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
					}),
					ChildNode(Node{
						children	: ~[
							ChildNode(Node{
								children	: ~[],
								actions	: ~[],
								name	: ~"ikHandle7",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
							})
						],
						actions	: ~[],
						name	: ~"L_ik_foot_grp",
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
					}),
					ChildNode(Node{
						children	: ~[
							ChildNode(Node{
								children	: ~[
									ChildNode(Node{
										children	: ~[
											ChildNode(Node{
												children	: ~[
													ChildArmature(Armature{
														actions	: [~"Armature.002Action"],
														name	: ~"Armature.002",
														bones	: ~[
															Bone{
																children	: ~[
																	Bone{
																		children	: ~[
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[
																									Bone{
																										children	: ~[
																											Bone{
																												children	: ~[
																													Bone{
																														children	: ~[
																															Bone{
																																children	: ~[
																																	Bone{
																																		children	: ~[
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										name	: ~"L_eye_end_joint",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, -0.00, 0.70],
																																											pos	: [0.00, 0.69, 0.00],
																																										},
																																									}
																																								],
																																								name	: ~"L_eye_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.71, -0.01, 0.01, 0.71],
																																									pos	: [1.26, -4.62, 3.06],
																																								},
																																							},
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										name	: ~"R_eye_end_joint",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.70, -0.00, 0.71],
																																											pos	: [0.00, 0.69, -0.00],
																																										},
																																									}
																																								],
																																								name	: ~"R_eye_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.71, 0.01, -0.01, 0.71],
																																									pos	: [-1.26, -4.62, 3.06],
																																								},
																																							},
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										name	: ~"R_eye_blink_01_joint",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, -0.00, 0.71],
																																											pos	: [-0.00, 0.50, -0.00],
																																										},
																																									}
																																								],
																																								name	: ~"R_eye_blink_base_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.77, -0.00, 0.00, 0.64],
																																									pos	: [-1.26, -4.62, 3.06],
																																								},
																																							},
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										name	: ~"L_eye_blink_01_joint",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, -0.00, 0.71],
																																											pos	: [0.00, 0.50, -0.00],
																																										},
																																									}
																																								],
																																								name	: ~"L_eye_blink_base_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.77, -0.00, 0.00, 0.64],
																																									pos	: [1.26, -4.62, 3.06],
																																								},
																																							},
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										name	: ~"R_eye_blink_02_joint",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.71, -0.00, 0.71],
																																											pos	: [-0.00, 0.67, -0.00],
																																										},
																																									}
																																								],
																																								name	: ~"R_eye_blink_02_base_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.60, -0.00, 0.00, 0.80],
																																									pos	: [-1.26, -4.62, 3.06],
																																								},
																																							},
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										name	: ~"L_eye_blink_02_joint",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, 0.00, 0.71],
																																											pos	: [-0.00, 0.67, -0.00],
																																										},
																																									}
																																								],
																																								name	: ~"L_eye_blink_02_base_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.60, -0.00, 0.00, 0.80],
																																									pos	: [1.26, -4.62, 3.06],
																																								},
																																							}
																																						],
																																						name	: ~"head_end",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [-0.04, 0.00, 0.00, 1.00],
																																							pos	: [-0.00, 6.34, 0.04],
																																						},
																																					},
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[],
																																								name	: ~"jaw_end_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [-0.00, 0.71, 0.00, 0.71],
																																									pos	: [0.00, 3.68, -0.00],
																																								},
																																							}
																																						],
																																						name	: ~"jaw_joint",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.82, 0.00, 0.00, 0.57],
																																							pos	: [-0.00, -0.41, 0.55],
																																						},
																																					}
																																				],
																																				name	: ~"head_joint",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.00, 0.00, 0.00, 1.00],
																																					pos	: [0.00, 2.37, -0.00],
																																				},
																																			}
																																		],
																																		name	: ~"c_neck_02_joint",
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [-0.00, 0.98, -0.21, -0.00],
																																			pos	: [-0.00, 2.10, -1.00],
																																		},
																																	}
																																],
																																name	: ~"c_neck_01_joint",
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [0.00, -0.00, 0.00, 1.00],
																																	pos	: [0.00, 0.44, -0.29],
																																},
																															},
																															Bone{
																																children	: ~[
																																	Bone{
																																		children	: ~[],
																																		name	: ~"c_shealth_end_joint",
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, 1.00, -0.00, -0.00],
																																			pos	: [0.00, 1.81, 0.00],
																																		},
																																	}
																																],
																																name	: ~"c_shealth_01_joint",
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, 0.16, 0.99, 0.00],
																																	pos	: [-0.00, 0.04, 2.55],
																																},
																															},
																															Bone{
																																children	: ~[
																																	Bone{
																																		children	: ~[
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[
																																											Bone{
																																												children	: ~[
																																													Bone{
																																														children	: ~[
																																															Bone{
																																																children	: ~[
																																																	Bone{
																																																		children	: ~[
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										name	: ~"L_pinkyFinger_04_joint",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, 0.94, 0.00, 0.35],
																																																											pos	: [-0.00, 0.96, 0.00],
																																																										},
																																																									}
																																																								],
																																																								name	: ~"L_pinkyFinger_03_joint",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, 0.34, 0.07, 0.93],
																																																									pos	: [-0.00, 0.75, 0.00],
																																																								},
																																																							}
																																																						],
																																																						name	: ~"L_pinkyFinger_02_joint",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.25, 0.56, -0.16, 0.77],
																																																							pos	: [-0.00, 1.18, 0.00],
																																																						},
																																																					}
																																																				],
																																																				name	: ~"L_pinkyFinger_01_joint",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.37, 0.10, -0.71, 0.59],
																																																					pos	: [-0.54, -0.80, -1.23],
																																																				},
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										name	: ~"L_ringFinger_04_joint",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, 0.96, -0.00, 0.29],
																																																											pos	: [-0.00, 1.00, 0.00],
																																																										},
																																																									}
																																																								],
																																																								name	: ~"L_ringFinger_03_joint",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, 0.37, 0.10, 0.92],
																																																									pos	: [0.00, 0.95, -0.00],
																																																								},
																																																							}
																																																						],
																																																						name	: ~"L_ringFinger_02_joint",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.27, 0.81, -0.24, 0.46],
																																																							pos	: [0.00, 1.63, 0.00],
																																																						},
																																																					}
																																																				],
																																																				name	: ~"L_ringFinger_01_joint",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.54, -0.07, -0.59, 0.60],
																																																					pos	: [0.26, -0.29, -0.98],
																																																				},
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										name	: ~"L_middleFinger_04_joint",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, 0.24, 0.00, 0.97],
																																																											pos	: [0.00, 1.24, 0.00],
																																																										},
																																																									}
																																																								],
																																																								name	: ~"L_middleFinger_03_joint",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.07, 0.19, 0.02, 0.98],
																																																									pos	: [0.00, 1.01, -0.00],
																																																								},
																																																							}
																																																						],
																																																						name	: ~"L_middleFinger_02_joint",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.16, 0.83, -0.31, 0.44],
																																																							pos	: [0.00, 1.64, 0.00],
																																																						},
																																																					}
																																																				],
																																																				name	: ~"L_middleFinger_01_joint",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.58, -0.15, -0.55, 0.58],
																																																					pos	: [0.92, 0.08, -0.55],
																																																				},
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										name	: ~"L_indexFinger_04_joint",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, 0.99, -0.00, 0.17],
																																																											pos	: [-0.00, 1.09, -0.00],
																																																										},
																																																									}
																																																								],
																																																								name	: ~"L_indexFinger_03_joint",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.14, 0.36, -0.05, 0.92],
																																																									pos	: [-0.00, 1.03, -0.00],
																																																								},
																																																							}
																																																						],
																																																						name	: ~"L_indexFinger_02_joint",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.05, 0.73, -0.36, 0.58],
																																																							pos	: [0.00, 1.48, -0.00],
																																																						},
																																																					}
																																																				],
																																																				name	: ~"L_indexFinger_01_joint",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.63, -0.19, -0.46, 0.59],
																																																					pos	: [1.60, 0.39, 0.17],
																																																				},
																																																			}
																																																		],
																																																		name	: ~"L_wrist_end_joint",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.11, -0.29, 0.75, 0.59],
																																																			pos	: [-0.06, 2.48, -0.37],
																																																		},
																																																	},
																																																	Bone{
																																																		children	: ~[
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[],
																																																								name	: ~"L_thumb_04_joint",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.00, -0.82, 0.00, 0.57],
																																																									pos	: [0.00, 1.17, 0.00],
																																																								},
																																																							}
																																																						],
																																																						name	: ~"L_thumb_03_joint",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.01, 0.12, 0.02, 0.99],
																																																							pos	: [-0.00, 1.10, -0.00],
																																																						},
																																																					}
																																																				],
																																																				name	: ~"L_thumb_02_joint",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.06, -0.32, -0.13, 0.94],
																																																					pos	: [0.00, 1.91, 0.00],
																																																				},
																																																			}
																																																		],
																																																		name	: ~"L_thumb_01_joint",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.38, 0.51, 0.19, 0.75],
																																																			pos	: [-0.43, 1.21, 0.79],
																																																		},
																																																	}
																																																],
																																																name	: ~"L_wrist_joint",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.00, 0.02, 0.00, 1.00],
																																																	pos	: [0.00, 5.42, -0.00],
																																																},
																																															}
																																														],
																																														name	: ~"L_forearm_02_joint",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, -0.02, 0.02, 1.00],
																																															pos	: [-0.00, 2.64, -0.00],
																																														},
																																													}
																																												],
																																												name	: ~"L_forearm_01_joint",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.00, 0.01, 0.00, 1.00],
																																													pos	: [0.00, 2.77, 0.00],
																																												},
																																											}
																																										],
																																										name	: ~"L_elbow_joint",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.24, 0.27, 0.02, 0.93],
																																											pos	: [-0.00, 3.95, -0.00],
																																										},
																																									}
																																								],
																																								name	: ~"L_arm_02_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																									pos	: [0.00, 3.58, 0.00],
																																								},
																																							}
																																						],
																																						name	: ~"L_arm_01_joint",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, 0.91, 0.00, 0.40],
																																							pos	: [0.00, 3.42, 0.00],
																																						},
																																					}
																																				],
																																				name	: ~"L_shoulder_joint",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.15, 0.73, 0.20, 0.63],
																																					pos	: [-0.52, 0.95, -0.03],
																																				},
																																			},
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										name	: ~"L_subSpaulder_end_joint",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.87, 0.00, 0.50],
																																											pos	: [0.00, 2.16, -0.00],
																																										},
																																									}
																																								],
																																								name	: ~"L_subSpaulder_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.24, 0.35, -0.81, 0.40],
																																									pos	: [0.00, 1.03, -0.00],
																																								},
																																							}
																																						],
																																						name	: ~"L_mainSpaulder_end_joint",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.33, -0.01, 0.91, 0.25],
																																							pos	: [0.00, 3.22, 0.00],
																																						},
																																					}
																																				],
																																				name	: ~"L_mainSpaulder_joint",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.37, 0.26, -0.12, 0.89],
																																					pos	: [-0.31, -0.70, 0.27],
																																				},
																																			},
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[],
																																								name	: ~"L_armIK_03_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																									pos	: [-0.00, 10.84, -0.00],
																																								},
																																							}
																																						],
																																						name	: ~"L_armIK_02_joint",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.13, 0.96, -0.21, 0.13],
																																							pos	: [0.00, 10.95, 0.00],
																																						},
																																					}
																																				],
																																				name	: ~"L_armIK_01_joint",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.15, 0.73, 0.20, 0.63],
																																					pos	: [-0.52, 0.95, -0.03],
																																				},
																																			}
																																		],
																																		name	: ~"L_clav_end_joint",
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [-0.00, -0.93, -0.00, 0.36],
																																			pos	: [-0.00, 5.59, -0.00],
																																		},
																																	}
																																],
																																name	: ~"L_clav_joint",
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.31, 0.71, 0.50, 0.39],
																																	pos	: [-1.00, -2.13, -2.41],
																																},
																															},
																															Bone{
																																children	: ~[
																																	Bone{
																																		children	: ~[
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[
																																											Bone{
																																												children	: ~[
																																													Bone{
																																														children	: ~[
																																															Bone{
																																																children	: ~[
																																																	Bone{
																																																		children	: ~[
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[],
																																																								name	: ~"R_thumb_04_joint",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [-0.00, 0.82, -0.00, 0.57],
																																																									pos	: [-0.00, 1.17, 0.00],
																																																								},
																																																							}
																																																						],
																																																						name	: ~"R_thumb_03_joint",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.01, -0.12, -0.02, 0.99],
																																																							pos	: [0.00, 1.10, -0.00],
																																																						},
																																																					}
																																																				],
																																																				name	: ~"R_thumb_02_joint",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.06, 0.32, 0.13, 0.94],
																																																					pos	: [0.00, 1.91, -0.00],
																																																				},
																																																			}
																																																		],
																																																		name	: ~"R_thumb_01_joint",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.38, -0.51, -0.19, 0.75],
																																																			pos	: [0.43, 1.21, 0.79],
																																																		},
																																																	},
																																																	Bone{
																																																		children	: ~[
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										name	: ~"R_indexFinger_04_joint",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.99, 0.00, 0.17],
																																																											pos	: [-0.00, 1.09, -0.00],
																																																										},
																																																									}
																																																								],
																																																								name	: ~"R_indexFinger_03_joint",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.15, -0.31, 0.02, 0.94],
																																																									pos	: [0.00, 1.03, -0.00],
																																																								},
																																																							}
																																																						],
																																																						name	: ~"R_indexFinger_02_joint",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.07, -0.67, 0.35, 0.65],
																																																							pos	: [-0.00, 1.48, -0.00],
																																																						},
																																																					}
																																																				],
																																																				name	: ~"R_indexFinger_01_joint",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.45, 0.13, 0.60, 0.65],
																																																					pos	: [-1.60, 0.39, 0.17],
																																																				},
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										name	: ~"R_middleFinger_04_joint",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, -0.24, -0.00, 0.97],
																																																											pos	: [-0.00, 1.24, -0.00],
																																																										},
																																																									}
																																																								],
																																																								name	: ~"R_middleFinger_03_joint",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.07, -0.19, -0.02, 0.98],
																																																									pos	: [-0.00, 1.01, -0.00],
																																																								},
																																																							}
																																																						],
																																																						name	: ~"R_middleFinger_02_joint",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.16, -0.83, 0.31, 0.44],
																																																							pos	: [0.00, 1.64, 0.00],
																																																						},
																																																					}
																																																				],
																																																				name	: ~"R_middleFinger_01_joint",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.58, 0.15, 0.55, 0.58],
																																																					pos	: [-0.92, 0.08, -0.55],
																																																				},
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										name	: ~"R_ringFinger_04_joint",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.96, -0.00, 0.29],
																																																											pos	: [0.00, 1.00, 0.00],
																																																										},
																																																									}
																																																								],
																																																								name	: ~"R_ringFinger_03_joint",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, -0.41, -0.09, 0.90],
																																																									pos	: [-0.00, 0.95, -0.00],
																																																								},
																																																							}
																																																						],
																																																						name	: ~"R_ringFinger_02_joint",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.23, -0.84, 0.28, 0.40],
																																																							pos	: [-0.00, 1.63, -0.00],
																																																						},
																																																					}
																																																				],
																																																				name	: ~"R_ringFinger_01_joint",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.62, 0.09, 0.52, 0.57],
																																																					pos	: [-0.26, -0.29, -0.98],
																																																				},
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										name	: ~"R_pinkyFinger_04_joint",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, -0.94, 0.00, 0.35],
																																																											pos	: [0.00, 0.96, 0.00],
																																																										},
																																																									}
																																																								],
																																																								name	: ~"R_pinkyFinger_03_joint",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, -0.42, -0.07, 0.90],
																																																									pos	: [-0.00, 0.75, 0.00],
																																																								},
																																																							}
																																																						],
																																																						name	: ~"R_pinkyFinger_02_joint",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.21, -0.68, 0.22, 0.67],
																																																							pos	: [0.00, 1.18, 0.00],
																																																						},
																																																					}
																																																				],
																																																				name	: ~"R_pinkyFinger_01_joint",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.57, -0.04, 0.62, 0.53],
																																																					pos	: [0.54, -0.80, -1.23],
																																																				},
																																																			}
																																																		],
																																																		name	: ~"R_wrist_end_joint",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.11, 0.29, -0.75, 0.59],
																																																			pos	: [0.06, 2.48, -0.37],
																																																		},
																																																	}
																																																],
																																																name	: ~"R_wrist_joint",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, -0.02, 0.00, 1.00],
																																																	pos	: [0.00, 5.42, -0.00],
																																																},
																																															}
																																														],
																																														name	: ~"R_forearm_02_joint",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, 0.02, -0.02, 1.00],
																																															pos	: [0.00, 2.64, -0.00],
																																														},
																																													}
																																												],
																																												name	: ~"R_forearm_01_joint",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.01, -0.00, 1.00],
																																													pos	: [0.00, 2.77, 0.00],
																																												},
																																											}
																																										],
																																										name	: ~"R_elbow_joint",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.24, -0.27, -0.02, 0.93],
																																											pos	: [0.00, 3.95, -0.00],
																																										},
																																									}
																																								],
																																								name	: ~"R_arm_02_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.00, -0.00, -0.00, 1.00],
																																									pos	: [0.00, 3.58, -0.00],
																																								},
																																							}
																																						],
																																						name	: ~"R_arm_01_joint",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, -0.21, 0.00, 0.98],
																																							pos	: [0.00, 3.42, -0.00],
																																						},
																																					}
																																				],
																																				name	: ~"R_shoulder_joint",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.11, 0.20, 0.23, 0.95],
																																					pos	: [-0.31, 0.95, 0.42],
																																				},
																																			},
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										name	: ~"R_subSpaulder_end_joint",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.38, 0.00, 0.93],
																																											pos	: [-0.00, 2.16, 0.00],
																																										},
																																									}
																																								],
																																								name	: ~"R_subSpaulder_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.24, -0.35, 0.81, 0.40],
																																									pos	: [-0.00, 1.03, -0.00],
																																								},
																																							}
																																						],
																																						name	: ~"R_mainSpaulder_end_joint",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [-0.54, 0.21, -0.80, 0.14],
																																							pos	: [0.00, 3.22, 0.00],
																																						},
																																					}
																																				],
																																				name	: ~"R_mainSpaulder_joint",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.38, 0.65, -0.05, 0.65],
																																					pos	: [-0.40, -0.70, 0.06],
																																				},
																																			},
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[],
																																								name	: ~"R_armIK_03_joint",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																									pos	: [0.00, 10.84, 0.00],
																																								},
																																							}
																																						],
																																						name	: ~"R_armIK_02_joint",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.24, -0.46, 0.02, 0.85],
																																							pos	: [0.00, 10.95, -0.00],
																																						},
																																					}
																																				],
																																				name	: ~"R_armIK_01_joint",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.11, 0.20, 0.23, 0.95],
																																					pos	: [-0.31, 0.95, 0.42],
																																				},
																																			}
																																		],
																																		name	: ~"R_clav_end_joint",
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, -0.72, -0.00, 0.70],
																																			pos	: [0.00, 5.59, -0.00],
																																		},
																																	}
																																],
																																name	: ~"R_clav_joint",
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.32, -0.71, -0.50, 0.38],
																																	pos	: [1.00, -2.13, -2.41],
																																},
																															}
																														],
																														name	: ~"c_spine_007_joint",
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [-0.00, 1.00, -0.00, -0.00],
																															pos	: [0.00, 6.39, -0.06],
																														},
																													},
																													Bone{
																														children	: ~[
																															Bone{
																																children	: ~[
																																	Bone{
																																		children	: ~[],
																																		name	: ~"L_breast_end_joint",
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, -0.71, 0.00, 0.71],
																																			pos	: [0.00, 0.00, 0.00],
																																		},
																																	}
																																],
																																name	: ~"L_breast_joint",
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, 0.58, -0.00, 0.81],
																																	pos	: [-0.00, 3.94, -0.00],
																																},
																															}
																														],
																														name	: ~"L_breast_base_joint",
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.84, 0.12, -0.06, 0.52],
																															pos	: [3.20, -0.56, 2.70],
																														},
																													},
																													Bone{
																														children	: ~[
																															Bone{
																																children	: ~[
																																	Bone{
																																		children	: ~[],
																																		name	: ~"R_breast_end_joint1",
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, 0.71, -0.00, 0.71],
																																			pos	: [0.00, -0.00, -0.00],
																																		},
																																	}
																																],
																																name	: ~"R_breast_joint1",
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, -0.58, 0.00, 0.81],
																																	pos	: [-0.00, 3.94, -0.00],
																																},
																															}
																														],
																														name	: ~"R_breast_base_joint1",
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.84, -0.12, 0.06, 0.52],
																															pos	: [-3.20, -0.56, 2.70],
																														},
																													}
																												],
																												name	: ~"c_spine_006_joint",
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [-0.00, 0.00, 0.00, 1.00],
																													pos	: [0.00, 6.18, -0.00],
																												},
																											}
																										],
																										name	: ~"c_spine_005_joint",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.11, 0.00, -0.00, 0.99],
																											pos	: [-0.00, 1.50, 0.00],
																										},
																									}
																								],
																								name	: ~"c_spine_004_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.02, 0.00, -0.00, 1.00],
																									pos	: [-0.00, 1.45, 0.00],
																								},
																							}
																						],
																						name	: ~"c_spine_003_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.10, 0.00, -0.00, 0.99],
																							pos	: [-0.00, 1.54, 0.00],
																						},
																					}
																				],
																				name	: ~"c_spine_002_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, 0.71, 0.00, 0.71],
																					pos	: [0.00, 1.49, 0.00],
																				},
																			}
																		],
																		name	: ~"c_spine_001_joint",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			pos	: [-0.04, 0.00, -0.00],
																		},
																	},
																	Bone{
																		children	: ~[
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[],
																						name	: ~"R_hip_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.00, 0.71, 0.00, 0.71],
																							pos	: [-0.00, 4.60, -0.00],
																						},
																					}
																				],
																				name	: ~"R_hip_base_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.69, -0.14, -0.69, 0.14],
																					pos	: [4.99, 0.48, 2.82],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[
																									Bone{
																										children	: ~[],
																										name	: ~"R_kneePivot_01_joint",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, -0.00, 0.00, 1.00],
																											pos	: [0.00, 4.95, 0.00],
																										},
																									}
																								],
																								name	: ~"R_kneePivot_02_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																									pos	: [0.00, 5.50, -0.00],
																								},
																							}
																						],
																						name	: ~"R_kneePivot_03_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.85, -0.02, 0.53, -0.00],
																							pos	: [-0.19, -6.70, -0.11],
																						},
																					},
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[
																									Bone{
																										children	: ~[
																											Bone{
																												children	: ~[],
																												name	: ~"R_toe_joint",
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [-0.00, -0.72, 0.00, 0.70],
																													pos	: [0.00, 2.80, -0.00],
																												},
																											}
																										],
																										name	: ~"R_ball_joint",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.23, 0.02, 0.01, 0.97],
																											pos	: [-0.00, 8.00, -0.00],
																										},
																									}
																								],
																								name	: ~"R_ankle_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.56, -0.19, -0.14, 0.80],
																									pos	: [-0.00, 19.52, 0.00],
																								},
																							}
																						],
																						name	: ~"R_knee_01_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.22, -0.06, -0.97, 0.00],
																							pos	: [-0.53, -18.67, -0.32],
																						},
																					}
																				],
																				name	: ~"R_leg_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.51, -0.51, 0.49, 0.49],
																					pos	: [4.92, 0.46, 4.10],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[],
																						name	: ~"L_hip_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.00, 0.71, 0.00, 0.71],
																							pos	: [-0.00, 4.60, 0.00],
																						},
																					}
																				],
																				name	: ~"L_hip_base_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.69, -0.14, -0.69, 0.14],
																					pos	: [4.99, 0.48, -2.82],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[
																									Bone{
																										children	: ~[],
																										name	: ~"L_kneePivot_01_joint",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.00, 0.00, 0.00, 1.00],
																											pos	: [0.00, 4.95, -0.00],
																										},
																									}
																								],
																								name	: ~"L_kneePivot_02_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 5.50, 0.00],
																								},
																							}
																						],
																						name	: ~"L_kneePivot_03_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.95, 0.01, -0.30, -0.00],
																							pos	: [0.19, -6.70, -0.06],
																						},
																					},
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[
																									Bone{
																										children	: ~[
																											Bone{
																												children	: ~[],
																												name	: ~"L_toe_joint",
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [0.00, -0.69, 0.00, 0.72],
																													pos	: [0.00, 2.80, -0.00],
																												},
																											}
																										],
																										name	: ~"L_ball_joint",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.23, -0.06, -0.02, 0.97],
																											pos	: [-0.00, 8.00, 0.00],
																										},
																									}
																								],
																								name	: ~"L_ankle_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.55, 0.25, 0.14, 0.79],
																									pos	: [0.00, 19.52, 0.00],
																								},
																							}
																						],
																						name	: ~"L_knee_01_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.25, 0.05, 0.97, 0.00],
																							pos	: [0.53, -18.67, -0.16],
																						},
																					}
																				],
																				name	: ~"L_leg_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.49, -0.49, 0.51, 0.51],
																					pos	: [4.92, 0.46, -4.10],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								name	: ~"R_frontMid_skirtplate_03_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, -0.00, 1.00],
																									pos	: [-0.00, 5.95, 0.00],
																								},
																							}
																						],
																						name	: ~"R_frontMid_skirtplate_02_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.07, 0.99, 0.04, 0.07],
																							pos	: [0.00, 4.80, -0.00],
																						},
																					}
																				],
																				name	: ~"R_frontMid_skirtplate_01_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.68, 0.70, -0.05, 0.19],
																					pos	: [1.22, 4.37, 3.97],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								name	: ~"R_frontInner_skirtplate_03_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, -0.00, 1.00],
																									pos	: [0.00, 5.95, -0.00],
																								},
																							}
																						],
																						name	: ~"R_frontInner_skirtplate_02_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.07, -0.93, 0.04, 0.35],
																							pos	: [0.00, 4.80, -0.00],
																						},
																					}
																				],
																				name	: ~"R_frontInner_skirtplate_01_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.68, -0.73, -0.00, 0.00],
																					pos	: [2.94, 4.48, 2.04],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								name	: ~"L_frontInner_skirtplate_03_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, 0.00, 1.00],
																									pos	: [0.00, 5.95, 0.00],
																								},
																							}
																						],
																						name	: ~"L_frontInner_skirtplate_02_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.07, 0.93, -0.04, 0.35],
																							pos	: [0.00, 4.80, 0.00],
																						},
																					}
																				],
																				name	: ~"L_frontInner_skirtplate_01_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, -0.00, -0.68, 0.73],
																					pos	: [2.94, 4.48, -2.04],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								name	: ~"L_side_skirtplate_03_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																									pos	: [-0.00, 5.95, -0.00],
																								},
																							}
																						],
																						name	: ~"L_side_skirtplate_02_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.13, 0.98, 0.14, 0.03],
																							pos	: [0.00, 4.80, -0.00],
																						},
																					}
																				],
																				name	: ~"L_side_skirtplate_01_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.08, 0.39, -0.73, 0.56],
																					pos	: [-1.03, 1.62, -5.94],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								name	: ~"L_rearSide_skirtplate_03_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, 0.00, 1.00],
																									pos	: [-0.00, 5.95, 0.00],
																								},
																							}
																						],
																						name	: ~"L_rearSide_skirtplate_02_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.13, -0.96, -0.19, 0.17],
																							pos	: [-0.00, 4.80, 0.00],
																						},
																					}
																				],
																				name	: ~"L_rearSide_skirtplate_01_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.34, -0.02, -0.78, 0.53],
																					pos	: [-0.29, -2.24, -3.34],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								name	: ~"C_rear_skirtplate_03_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, 3.88, -0.00],
																								},
																							}
																						],
																						name	: ~"C_rear_skirtplate_02_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.01, 0.97, 0.25, 0.04],
																							pos	: [-0.00, 4.80, -0.00],
																						},
																					}
																				],
																				name	: ~"C_rear_skirtplate_01_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.61, -0.37, -0.60, 0.36],
																					pos	: [1.63, -3.54, -0.00],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								name	: ~"R_rearSide_skirtplate_03_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, -0.00, 1.00],
																									pos	: [-0.00, 5.95, -0.00],
																								},
																							}
																						],
																						name	: ~"R_rearSide_skirtplate_02_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.13, 0.96, 0.19, 0.17],
																							pos	: [0.00, 4.80, -0.00],
																						},
																					}
																				],
																				name	: ~"R_rearSide_skirtplate_01_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.78, -0.53, -0.34, 0.02],
																					pos	: [-0.29, -2.24, 3.34],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								name	: ~"R_side_skirtplate_03_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																									pos	: [0.00, 5.95, 0.00],
																								},
																							}
																						],
																						name	: ~"R_side_skirtplate_02_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.13, -0.98, -0.14, 0.03],
																							pos	: [0.00, 4.80, 0.00],
																						},
																					}
																				],
																				name	: ~"R_side_skirtplate_01_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.73, 0.56, -0.08, 0.39],
																					pos	: [-1.03, 1.62, 5.94],
																				},
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								name	: ~"L_frontMid_skirtplate_03_joint",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, 0.00, 1.00],
																									pos	: [0.00, 5.95, 0.00],
																								},
																							}
																						],
																						name	: ~"L_frontMid_skirtplate_02_joint",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.07, -0.99, -0.04, 0.07],
																							pos	: [0.00, 4.80, -0.00],
																						},
																					}
																				],
																				name	: ~"L_frontMid_skirtplate_01_joint",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.05, 0.19, -0.68, 0.70],
																					pos	: [1.22, 4.37, -3.97],
																				},
																			}
																		],
																		name	: ~"C_hip_joint",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, -0.71, 0.00, 0.71],
																			pos	: [0.00, -0.10, 0.04],
																		},
																	}
																],
																name	: ~"cog",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.71, 0.00, 0.00, 0.71],
																	pos	: [0.00, 47.22, 0.24],
																},
															}
														],
														dual_quat	: ~"false",
													})
												],
												actions	: ~[],
												name	: ~"Armature.002",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
											}),
											ChildNode(Node{
												children	: ~[],
												actions	: ~[],
												name	: ~"SKELETON",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
											}),
											ChildNode(Node{
												children	: ~[
													ChildNode(Node{
														children	: ~[
															ChildNode(Node{
																children	: ~[
																	ChildNode(Node{
																		children	: ~[
																			ChildNode(Node{
																				children	: ~[
																					ChildNode(Node{
																						children	: ~[],
																						actions	: ~[],
																						name	: ~"R_eye_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.00, -0.00, -0.00, 1.00],
																							pos	: [0.00, 0.00, -0.00],
																						},
																					})
																				],
																				actions	: ~[],
																				name	: ~"R_eye_ctrl_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.00, 0.00, -0.00, 1.00],
																					pos	: [-0.00, -0.00, 0.00],
																				},
																			})
																		],
																		actions	: ~[],
																		name	: ~"R_eye_ctrl_grp",
																		space	: QuatSpace{
																			scale	: 0.72,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [-0.95, -0.00, 0.00],
																		},
																	}),
																	ChildNode(Node{
																		children	: ~[
																			ChildNode(Node{
																				children	: ~[
																					ChildNode(Node{
																						children	: ~[],
																						actions	: ~[],
																						name	: ~"L_eye_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.00, 0.00, -0.00, 1.00],
																							pos	: [0.00, 0.00, -0.00],
																						},
																					})
																				],
																				actions	: ~[],
																				name	: ~"L_eye_ctrl_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.00, 0.00, -0.00, 1.00],
																					pos	: [0.00, 0.00, -0.00],
																				},
																			})
																		],
																		actions	: ~[],
																		name	: ~"L_eye_ctrl_grp",
																		space	: QuatSpace{
																			scale	: 0.72,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [0.95, 0.00, -0.00],
																		},
																	})
																],
																actions	: [~"mainEye_ctrlAction"],
																name	: ~"mainEye_ctrl",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	pos	: [0.04, -71.88, -17.98],
																},
															})
														],
														actions	: ~[],
														name	: ~"mainEye_ctrl_zero",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-0.04, 71.88, 17.98],
														},
													})
												],
												actions	: ~[],
												name	: ~"c_eye_ctrl_grp",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
											}),
											ChildNode(Node{
												children	: ~[
													ChildNode(Node{
														children	: ~[],
														actions	: ~[],
														name	: ~"L_eye_centerLocator",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [1.26, 71.88, 0.51],
														},
													}),
													ChildNode(Node{
														children	: ~[],
														actions	: ~[],
														name	: ~"R_eye_centerLocator",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-1.26, 71.88, 0.51],
														},
													})
												],
												actions	: ~[],
												name	: ~"Locators",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
											}),
											ChildNode(Node{
												children	: ~[
													ChildNode(Node{
														children	: ~[
															ChildNode(Node{
																children	: ~[
																	ChildNode(Node{
																		children	: ~[],
																		actions	: ~[],
																		name	: ~"L_foot_ik_ctrl",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																			pos	: [-0.00, 0.00, 0.00],
																		},
																	})
																],
																actions	: ~[],
																name	: ~"L_leg_ikHandle_zero",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, -0.00, 1.00],
																	pos	: [0.00, -0.00, 0.00],
																},
															})
														],
														actions	: ~[],
														name	: ~"L_leg_ikHandle_grp",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [3.00, -0.31, -3.16],
														},
													}),
													ChildNode(Node{
														children	: ~[
															ChildNode(Node{
																children	: ~[
																	ChildNode(Node{
																		children	: ~[],
																		actions	: ~[],
																		name	: ~"R_foot_ik_ctrl",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.01, -0.00, 0.00, 1.00],
																			pos	: [0.00, 0.00, 0.00],
																		},
																	})
																],
																actions	: ~[],
																name	: ~"R_leg_ikHandle_zero",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	pos	: [-0.00, -0.00, 0.00],
																},
															})
														],
														actions	: ~[],
														name	: ~"R_leg_ikHandle_grp",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-3.00, -0.31, -3.16],
														},
													}),
													ChildNode(Node{
														children	: ~[
															ChildNode(Node{
																children	: ~[
																	ChildNode(Node{
																		children	: ~[],
																		actions	: ~[],
																		name	: ~"R_legPole_ctrl.001",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																			pos	: [0.00, 0.00, 0.00],
																		},
																	})
																],
																actions	: ~[],
																name	: ~"R_legPole_ctrl_zero",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.00, -0.00, -0.00, 1.00],
																	pos	: [0.57, -18.67, 10.59],
																},
															})
														],
														actions	: ~[],
														name	: ~"R_legPole_ctrl",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-4.14, 43.04, -0.16],
														},
													}),
													ChildNode(Node{
														children	: ~[
															ChildNode(Node{
																children	: ~[
																	ChildNode(Node{
																		children	: ~[
																			ChildNode(Node{
																				children	: ~[],
																				actions	: ~[],
																				name	: ~"L_legPole_ctrl",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																					pos	: [0.00, -0.00, -0.00],
																				},
																			})
																		],
																		actions	: ~[],
																		name	: ~"L_legPole_cntr_zero",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [-0.55, -17.93, 9.74],
																		},
																	})
																],
																actions	: ~[],
																name	: ~"L_legPole_ctrl_grp",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.47, -0.53, 0.49, 0.51],
																	pos	: [0.00, 0.00, -0.00],
																},
															})
														],
														actions	: ~[],
														name	: ~"L_legPole_ctrl_cons",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [-0.47, 0.53, -0.49, 0.51],
															pos	: [4.10, 42.26, 0.61],
														},
													})
												],
												actions	: ~[],
												name	: ~"LegControls",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
											}),
											ChildNode(Node{
												children	: ~[
													ChildNode(Node{
														children	: ~[
															ChildNode(Node{
																children	: ~[
																	ChildNode(Node{
																		children	: ~[],
																		actions	: ~[],
																		name	: ~"L_armIK_handle",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [-0.00, 0.00, -0.00],
																		},
																	}),
																	ChildNode(Node{
																		children	: ~[
																			ChildNode(Node{
																				children	: ~[
																					ChildNode(Node{
																						children	: ~[
																							ChildNode(Node{
																								children	: ~[],
																								actions	: [~"L_thumb_ctrlAction"],
																								name	: ~"L_thumb_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: [~"L_indexF_ctrlAction"],
																								name	: ~"L_indexF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: [~"L_middleF_ctrlAction"],
																								name	: ~"L_middleF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: [~"L_ringF_ctrlAction"],
																								name	: ~"L_ringF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: [~"L_pinkyF_ctrlAction"],
																								name	: ~"L_pinkyF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																							})
																						],
																						actions	: [~"L_palm_ctrlAction"],
																						name	: ~"L_palm_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.00, 0.00, -0.00, 1.00],
																							pos	: [0.00, -0.00, 0.00],
																						},
																					})
																				],
																				actions	: ~[],
																				name	: ~"L_hand_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.00, -0.00, 0.00, 1.00],
																					pos	: [-0.00, 0.00, -0.00],
																				},
																			})
																		],
																		actions	: ~[],
																		name	: ~"L_hand_grp",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.26, -0.08, -0.07, 0.96],
																			pos	: [-0.00, 0.00, -0.00],
																		},
																	})
																],
																actions	: [~"L_arm_IK_ctrlAction"],
																name	: ~"L_arm_IK_ctrl",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.00, -0.00, 0.00, 1.00],
																	pos	: [-27.41, -60.54, 2.62],
																},
															})
														],
														actions	: ~[],
														name	: ~"L_arm_IK_ctrl_zero",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.26, 0.08, 0.07, 0.96],
															pos	: [0.00, -0.00, 0.00],
														},
													})
												],
												actions	: ~[],
												name	: ~"L_arm_IK_ctrl_grp",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [-0.26, -0.08, -0.07, 0.96],
													pos	: [27.41, 60.54, -2.62],
												},
											}),
											ChildNode(Node{
												children	: ~[],
												actions	: ~[],
												name	: ~"c_hips_cntr_backup",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 46.47, 1.11],
												},
											}),
											ChildNode(Node{
												children	: ~[
													ChildNode(Node{
														children	: ~[
															ChildNode(Node{
																children	: ~[
																	ChildNode(Node{
																		children	: ~[],
																		actions	: ~[],
																		name	: ~"ikHandle4",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, 0.00, 0.00, 1.00],
																			pos	: [-54.83, -0.00, 0.00],
																		},
																	}),
																	ChildNode(Node{
																		children	: ~[
																			ChildNode(Node{
																				children	: ~[
																					ChildNode(Node{
																						children	: ~[
																							ChildNode(Node{
																								children	: ~[],
																								actions	: [~"R_thumb_ctrlAction"],
																								name	: ~"R_thumb_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: [~"R_indexF_ctrlAction"],
																								name	: ~"R_indexF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: [~"R_middleF_ctrlAction"],
																								name	: ~"R_middleF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: [~"R_ringF_ctrlAction"],
																								name	: ~"R_ringF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: [~"R_pinkyF_ctrlAction"],
																								name	: ~"R_pinkyF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																							})
																						],
																						actions	: [~"R_palm_ctrlAction"],
																						name	: ~"R_palm_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.07, 0.96, -0.26, 0.08],
																							pos	: [0.00, 0.00, -0.00],
																						},
																					})
																				],
																				actions	: ~[],
																				name	: ~"R_hand_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																					pos	: [0.00, -0.00, 0.00],
																				},
																			})
																		],
																		actions	: ~[],
																		name	: ~"R_hand_grp",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.07, -0.96, 0.26, 0.08],
																			pos	: [-108.05, 6.80, 11.25],
																		},
																	})
																],
																actions	: [~"R_arm_IK_ctrl1Action"],
																name	: ~"R_arm_IK_ctrl1",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	pos	: [-27.33, -60.54, 2.62],
																},
															})
														],
														actions	: ~[],
														name	: ~"R_arm_IK_ctrl_zero",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [-0.00, -0.00, 0.00, 1.00],
															pos	: [0.00, 0.00, 0.00],
														},
													})
												],
												actions	: ~[],
												name	: ~"R_arm_IK_ctrl_grp",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [27.33, 60.54, -2.62],
												},
											}),
											ChildNode(Node{
												children	: ~[
													ChildNode(Node{
														children	: ~[
															ChildNode(Node{
																children	: ~[
																	ChildNode(Node{
																		children	: ~[
																			ChildNode(Node{
																				children	: ~[
																					ChildNode(Node{
																						children	: ~[
																							ChildNode(Node{
																								children	: ~[
																									ChildNode(Node{
																										children	: ~[],
																										actions	: ~[],
																										name	: ~"cluster3Handle",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																									}),
																									ChildNode(Node{
																										children	: ~[],
																										actions	: ~[],
																										name	: ~"cluster2Handle",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																									}),
																									ChildNode(Node{
																										children	: ~[],
																										actions	: ~[],
																										name	: ~"cluster1Handle",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																									})
																								],
																								actions	: ~[],
																								name	: ~"group13",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.46, 0.54, -0.46, 0.54],
																									pos	: [-46.65, -7.08, 0.00],
																								},
																							})
																						],
																						actions	: [~"c_hips_ctrlAction"],
																						name	: ~"c_hips_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.46, 0.54, -0.46, 0.54],
																							pos	: [0.00, -0.00, 0.00],
																						},
																					})
																				],
																				actions	: ~[],
																				name	: ~"c_hips_ctrl_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																					pos	: [0.00, -0.00, -0.00],
																				},
																			})
																		],
																		actions	: ~[],
																		name	: ~"c_hips_ctrl_grp",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			pos	: [-0.00, -0.04, -0.10],
																		},
																	}),
																	ChildNode(Node{
																		children	: ~[
																			ChildNode(Node{
																				children	: ~[
																					ChildNode(Node{
																						children	: ~[
																							ChildNode(Node{
																								children	: ~[
																									ChildNode(Node{
																										children	: ~[
																											ChildNode(Node{
																												children	: ~[
																													ChildNode(Node{
																														children	: ~[
																															ChildNode(Node{
																																children	: ~[],
																																actions	: ~[],
																																name	: ~"cluster4Handle",
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [0.00, 0.00, -0.00, 1.00],
																																	pos	: [0.00, 0.00, 0.00],
																																},
																															})
																														],
																														actions	: ~[],
																														name	: ~"group14",
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.52, 0.48, -0.52, 0.48],
																															pos	: [-50.08, 3.75, -0.04],
																														},
																													}),
																													ChildNode(Node{
																														children	: ~[
																															ChildNode(Node{
																																children	: ~[
																																	ChildNode(Node{
																																		children	: ~[
																																			ChildNode(Node{
																																				children	: ~[
																																					ChildNode(Node{
																																						children	: ~[],
																																						actions	: ~[],
																																						name	: ~"cluster6Handle",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																							pos	: [-0.00, -0.00, 0.00],
																																						},
																																					}),
																																					ChildNode(Node{
																																						children	: ~[],
																																						actions	: ~[],
																																						name	: ~"cluster5Handle",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																							pos	: [-0.00, -0.00, 0.00],
																																						},
																																					})
																																				],
																																				actions	: ~[],
																																				name	: ~"group12",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.57, 0.41, -0.57, 0.41],
																																					pos	: [-50.19, 17.53, -0.04],
																																				},
																																			}),
																																			ChildNode(Node{
																																				children	: ~[
																																					ChildNode(Node{
																																						children	: ~[
																																							ChildNode(Node{
																																								children	: ~[
																																									ChildNode(Node{
																																										children	: ~[
																																											ChildNode(Node{
																																												children	: ~[],
																																												actions	: ~[],
																																												name	: ~"cluster9Handle",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																											}),
																																											ChildNode(Node{
																																												children	: ~[],
																																												actions	: ~[],
																																												name	: ~"cluster8Handle",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																											}),
																																											ChildNode(Node{
																																												children	: ~[],
																																												actions	: ~[],
																																												name	: ~"cluster7Handle",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																											})
																																										],
																																										actions	: ~[],
																																										name	: ~"group11",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.58, 0.41, -0.58, 0.41],
																																											pos	: [-56.22, 18.02, -0.04],
																																										},
																																									}),
																																									ChildNode(Node{
																																										children	: ~[
																																											ChildNode(Node{
																																												children	: ~[],
																																												actions	: ~[],
																																												name	: ~"null1",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, 0.00, -0.00, 1.00],
																																													pos	: [0.00, -0.00, -0.00],
																																												},
																																											}),
																																											ChildNode(Node{
																																												children	: ~[
																																													ChildNode(Node{
																																														children	: ~[
																																															ChildNode(Node{
																																																children	: ~[
																																																	ChildNode(Node{
																																																		children	: ~[],
																																																		actions	: ~[],
																																																		name	: ~"ikHandle2",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																	}),
																																																	ChildNode(Node{
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						children	: ~[],
																																																						actions	: [~"R_breastTweak_ctrlAction"],
																																																						name	: ~"R_breastTweak_ctrl",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.00, 0.00, 0.00, 1.00],
																																																							pos	: [-0.00, 0.00, -0.00],
																																																						},
																																																					})
																																																				],
																																																				actions	: ~[],
																																																				name	: ~"R_breastTweak_zero",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [0.00, 0.00, -0.00, 1.00],
																																																					pos	: [-0.00, 0.00, -0.00],
																																																				},
																																																			})
																																																		],
																																																		actions	: ~[],
																																																		name	: ~"R_breastTweak_cntr_grp",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.03, -0.70, 0.06, 0.71],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																	})
																																																],
																																																actions	: [~"R_breast_IK_ctrlAction"],
																																																name	: ~"R_breast_IK_ctrl",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																																	pos	: [0.00, 0.00, -0.00],
																																																},
																																															})
																																														],
																																														actions	: ~[],
																																														name	: ~"R_breast_IK_cntr_zero",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																													})
																																												],
																																												actions	: ~[],
																																												name	: ~"R_breast_IK_control_grp",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.10, -0.08, -0.84, 0.52],
																																													pos	: [-2.31, -6.09, 4.23],
																																												},
																																											}),
																																											ChildNode(Node{
																																												children	: ~[
																																													ChildNode(Node{
																																														children	: ~[
																																															ChildNode(Node{
																																																children	: ~[
																																																	ChildNode(Node{
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				children	: ~[],
																																																				actions	: [~"L_breastTweak_ctrlAction"],
																																																				name	: ~"L_breastTweak_ctrl",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.00, -0.00, -0.00, 1.00],
																																																					pos	: [0.00, -0.00, -0.00],
																																																				},
																																																			})
																																																		],
																																																		actions	: ~[],
																																																		name	: ~"L_breastTweak_zero",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																			pos	: [0.00, 0.00, 0.00],
																																																		},
																																																	})
																																																],
																																																actions	: ~[],
																																																name	: ~"L_breastTweak_cntr_grp",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.03, 0.70, 0.06, 0.71],
																																																	pos	: [-0.00, 0.00, -0.00],
																																																},
																																															}),
																																															ChildNode(Node{
																																																children	: ~[
																																																	ChildNode(Node{
																																																		children	: ~[],
																																																		actions	: ~[],
																																																		name	: ~"ikHandle3",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [-0.00, 0.00, -0.00, 1.00],
																																																			pos	: [0.00, -0.00, 0.00],
																																																		},
																																																	})
																																																],
																																																actions	: [~"L_breast_IK_ctrlAction"],
																																																name	: ~"L_breast_IK_ctrl",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.00, 0.00, 0.00, 1.00],
																																																	pos	: [-0.00, 0.00, -0.00],
																																																},
																																															})
																																														],
																																														actions	: ~[],
																																														name	: ~"L_breast_IK_cntr_zero",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [0.00, 0.00, 0.00, 1.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																													})
																																												],
																																												actions	: ~[],
																																												name	: ~"L_breast_IK_control_grp",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.10, 0.08, -0.84, 0.52],
																																													pos	: [-2.31, -6.09, -4.23],
																																												},
																																											})
																																										],
																																										actions	: ~[],
																																										name	: ~"c_breastControls_grp",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.00, 0.00, 1.00],
																																											pos	: [0.00, 0.00, 0.00],
																																										},
																																									}),
																																									ChildNode(Node{
																																										children	: ~[
																																											ChildNode(Node{
																																												children	: ~[
																																													ChildNode(Node{
																																														children	: ~[
																																															ChildNode(Node{
																																																children	: ~[
																																																	ChildNode(Node{
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												children	: ~[],
																																																												actions	: [~"R_subSpaulder_ctrlAction"],
																																																												name	: ~"R_subSpaulder_ctrl",
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [-0.00, 0.00, -0.04, 1.00],
																																																													pos	: [0.00, 0.00, -0.00],
																																																												},
																																																											})
																																																										],
																																																										actions	: ~[],
																																																										name	: ~"R_subSpaulder_ctrl_zero",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.00, 0.00, 1.00],
																																																											pos	: [-0.00, 0.00, -0.00],
																																																										},
																																																									})
																																																								],
																																																								actions	: ~[],
																																																								name	: ~"R_subSpaulder_ctrl_grp",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [-0.00, -0.30, 0.00, 0.96],
																																																									pos	: [2.32, 0.02, 0.51],
																																																								},
																																																							})
																																																						],
																																																						actions	: [~"R_mainSpaulder_ctrlAction"],
																																																						name	: ~"R_mainSpaulder_ctrl",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.00, 0.00, -0.03, 1.00],
																																																							pos	: [0.00, 0.00, -0.00],
																																																						},
																																																					})
																																																				],
																																																				actions	: ~[],
																																																				name	: ~"R_mainSpaulder_ctrl_zero",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																																					pos	: [-0.00, 0.00, 0.00],
																																																				},
																																																			})
																																																		],
																																																		actions	: ~[],
																																																		name	: ~"R_mainSpaulder_ctrl_grp",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.13, 0.32, -0.21, 0.91],
																																																			pos	: [4.89, -0.16, 0.38],
																																																		},
																																																	})
																																																],
																																																actions	: [~"R_clav_ctrlAction"],
																																																name	: ~"R_clav_ctrl",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, -0.12, 0.95, 0.28],
																																																	pos	: [0.00, -0.00, 0.00],
																																																},
																																															})
																																														],
																																														actions	: ~[],
																																														name	: ~"R_clav_ctrl_zero",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																															pos	: [0.00, -0.00, -0.00],
																																														},
																																													})
																																												],
																																												actions	: ~[],
																																												name	: ~"R_clav_ctrl_grp",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.19, -0.65, 0.22, 0.71],
																																													pos	: [-1.39, -3.03, 1.00],
																																												},
																																											}),
																																											ChildNode(Node{
																																												children	: ~[
																																													ChildNode(Node{
																																														children	: ~[
																																															ChildNode(Node{
																																																children	: ~[
																																																	ChildNode(Node{
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												children	: ~[],
																																																												actions	: [~"L_subSpaulder_ctrlAction"],
																																																												name	: ~"L_subSpaulder_ctrl",
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [-0.00, -0.00, -0.04, 1.00],
																																																													pos	: [0.00, 0.00, -0.00],
																																																												},
																																																											})
																																																										],
																																																										actions	: ~[],
																																																										name	: ~"L_subSpaulder_ctrl_zero",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.00, -0.00, 1.00],
																																																											pos	: [-0.00, 0.00, -0.00],
																																																										},
																																																									})
																																																								],
																																																								actions	: ~[],
																																																								name	: ~"L_subSpaulder_ctrl_grp",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.00, 0.30, 0.00, 0.96],
																																																									pos	: [2.32, 0.02, -0.51],
																																																								},
																																																							})
																																																						],
																																																						actions	: [~"L_mainSpaulder_ctrlAction"],
																																																						name	: ~"L_mainSpaulder_ctrl",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.00, -0.00, -0.03, 1.00],
																																																							pos	: [0.00, -0.00, -0.00],
																																																						},
																																																					})
																																																				],
																																																				actions	: ~[],
																																																				name	: ~"L_mainSpaulder_ctrl_zero",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [0.00, 0.00, 0.00, 1.00],
																																																					pos	: [0.00, -0.00, 0.00],
																																																				},
																																																			})
																																																		],
																																																		actions	: ~[],
																																																		name	: ~"L_mainSpaulder_ctrl_grp",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [-0.09, -0.32, -0.22, 0.92],
																																																			pos	: [4.89, -0.12, -0.39],
																																																		},
																																																	})
																																																],
																																																actions	: [~"L_clav_ctrlAction"],
																																																name	: ~"L_clav_ctrl",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, -0.01, -0.28, 0.96],
																																																	pos	: [0.00, 0.00, -0.00],
																																																},
																																															})
																																														],
																																														actions	: ~[],
																																														name	: ~"L_clav_ctrl_zero",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, -0.00, -0.00, 1.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																													})
																																												],
																																												actions	: ~[],
																																												name	: ~"L_clav_ctrl_grp",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.16, 0.64, 0.24, 0.71],
																																													pos	: [-1.39, -3.03, -1.00],
																																												},
																																											}),
																																											ChildNode(Node{
																																												children	: ~[
																																													ChildNode(Node{
																																														children	: ~[
																																															ChildNode(Node{
																																																children	: ~[
																																																	ChildNode(Node{
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												children	: ~[
																																																													ChildNode(Node{
																																																														children	: ~[
																																																															ChildNode(Node{
																																																																children	: ~[],
																																																																actions	: [~"c_jaw_ctrlAction"],
																																																																name	: ~"c_jaw_ctrl",
																																																																space	: QuatSpace{
																																																																	scale	: 1.00,
																																																																	rot	: [-0.00, 0.00, -0.00, 1.00],
																																																																	pos	: [0.00, -0.00, 0.00],
																																																																},
																																																															})
																																																														],
																																																														actions	: ~[],
																																																														name	: ~"c_jaw_ctrl_zero",
																																																														space	: QuatSpace{
																																																															scale	: 1.00,
																																																															rot	: [-0.00, 0.00, -0.00, 1.00],
																																																															pos	: [-0.00, 0.00, 0.00],
																																																														},
																																																													})
																																																												],
																																																												actions	: ~[],
																																																												name	: ~"c_jaw_ctrl_grp",
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [0.00, 0.00, -0.82, 0.57],
																																																													pos	: [-0.41, -0.55, 0.00],
																																																												},
																																																											})
																																																										],
																																																										actions	: [~"c_neck_03_ctrlAction"],
																																																										name	: ~"c_neck_03_ctrl",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.00, 0.05, 1.00],
																																																											pos	: [0.00, 0.00, 0.00],
																																																										},
																																																									})
																																																								],
																																																								actions	: ~[],
																																																								name	: ~"c_neck_03_ctrl_zero",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																																									pos	: [0.00, 0.00, 0.00],
																																																								},
																																																							})
																																																						],
																																																						actions	: ~[],
																																																						name	: ~"c_neck_03_ctrl_grp",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.00, -0.00, -0.00, 1.00],
																																																							pos	: [2.37, 0.00, 0.00],
																																																						},
																																																					})
																																																				],
																																																				actions	: [~"c_neck_02_ctrlAction"],
																																																				name	: ~"c_neck_02_ctrl",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.00, -0.00, 0.04, 1.00],
																																																					pos	: [0.00, 0.00, -0.00],
																																																				},
																																																			})
																																																		],
																																																		actions	: ~[],
																																																		name	: ~"c_neck_02_ctrl_zero",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [-0.00, -0.00, -0.00, 1.00],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																	})
																																																],
																																																actions	: ~[],
																																																name	: ~"c_neck_02_ctrl_grp",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, 0.00, 0.02, 1.00],
																																																	pos	: [2.33, 0.01, -0.00],
																																																},
																																															})
																																														],
																																														actions	: [~"c_neck_01_ctrlAction"],
																																														name	: ~"c_neck_01_ctrl",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, -0.00, 0.06, 1.00],
																																															pos	: [0.00, 0.00, -0.00],
																																														},
																																													})
																																												],
																																												actions	: ~[],
																																												name	: ~"c_neck_01_ctrl_zero",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, 0.00],
																																												},
																																											})
																																										],
																																										actions	: ~[],
																																										name	: ~"c_neck_01_ctrl_grp",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.00, -0.23, 0.97],
																																											pos	: [6.82, -0.29, 0.00],
																																										},
																																									})
																																								],
																																								actions	: [~"c_chest_ctrlAction"],
																																								name	: ~"c_chest_ctrl",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																									pos	: [-0.00, 0.00, 0.00],
																																								},
																																							})
																																						],
																																						actions	: ~[],
																																						name	: ~"c_chest_ctrl_zero",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [-0.00, -0.00, 0.00, 1.00],
																																							pos	: [0.00, 0.00, -0.00],
																																						},
																																					})
																																				],
																																				actions	: ~[],
																																				name	: ~"c_chest_ctrl_grp",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																					pos	: [6.18, -0.00, -0.00],
																																				},
																																			})
																																		],
																																		actions	: [~"c_spine_05_ctrlAction"],
																																		name	: ~"c_spine_05_ctrl",
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.57, 0.41, -0.57, 0.41],
																																			pos	: [0.00, -0.00, -0.00],
																																		},
																																	})
																																],
																																actions	: ~[],
																																name	: ~"c_spine_05_ctrl_zero",
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																	pos	: [0.00, 0.00, 0.00],
																																},
																															})
																														],
																														actions	: ~[],
																														name	: ~"c_spine_05_ctrl_grp",
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.00, 0.00, 0.13, 0.99],
																															pos	: [2.95, 0.06, -0.00],
																														},
																													})
																												],
																												actions	: [~"c_spine_03_ctrlAction"],
																												name	: ~"c_spine_03_ctrl",
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [-0.00, -0.00, -0.00, 1.00],
																													pos	: [-50.12, 3.07, 0.00],
																												},
																											})
																										],
																										actions	: ~[],
																										name	: ~"c_spine_03_ctrl_zero",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.00, 0.00, -0.00, 1.00],
																											pos	: [0.00, 0.00, 0.00],
																										},
																									})
																								],
																								actions	: ~[],
																								name	: ~"c_spine_03_ctrl_grp",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.11, 0.99],
																									pos	: [3.03, 0.02, -0.00],
																								},
																							})
																						],
																						actions	: [~"c_spine_01_ctrlAction"],
																						name	: ~"c_spine_01_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.00, -0.00, 0.08, 1.00],
																							pos	: [-0.00, 0.00, 0.00],
																						},
																					})
																				],
																				actions	: ~[],
																				name	: ~"c_spine_01_ctrl_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																					pos	: [0.00, -0.00, 0.00],
																				},
																			})
																		],
																		actions	: ~[],
																		name	: ~"c_spine_01_ctrl_grp",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			pos	: [-0.04, -0.00, 0.00],
																		},
																	})
																],
																actions	: [~"c_cog_ctrlAction"],
																name	: ~"c_cog_ctrl",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, 0.00, 0.00, 1.00],
																	pos	: [-0.00, -47.22, -0.24],
																},
															})
														],
														actions	: ~[],
														name	: ~"c_cog_ctrl_zero",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [-0.00, 0.00, -0.00, 1.00],
															pos	: [0.00, -0.00, 0.00],
														},
													})
												],
												actions	: ~[],
												name	: ~"c_cog_ctrl_grp",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 47.22, 0.24],
												},
											})
										],
										actions	: [~"c_worldTransform_ctrlAction"],
										name	: ~"c_worldTransform_ctrl",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									})
								],
								actions	: ~[],
								name	: ~"Controls",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 1.11, 0.00],
								},
							})
						],
						actions	: ~[],
						name	: ~"Transform",
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [-0.00, -1.11, 0.00],
						},
					}),
					ChildNode(Node{
						children	: ~[
							ChildNode(Node{
								children	: ~[
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Tongue",
												armature	: ~"",
												range	: [0, 528],
												mesh	: ~"tongue_geo1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"tongue_geo1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"cloak",
												armature	: ~"",
												range	: [0, 15252],
												mesh	: ~"polySurfaceShape174@",
											})
										],
										actions	: ~[],
										name	: ~"polySurface172",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Teeth",
												armature	: ~"",
												range	: [0, 4656],
												mesh	: ~"topJaw_geo2Shape@",
											})
										],
										actions	: ~[],
										name	: ~"topJaw_geo2",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Teeth",
												armature	: ~"",
												range	: [0, 4248],
												mesh	: ~"lowerJaw_geo2Shape@",
											})
										],
										actions	: ~[],
										name	: ~"lowerJaw_geo2",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 13716],
												mesh	: ~"L_upper_lash1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"L_upper_lash1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 8964],
												mesh	: ~"L_lower_lash1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"L_lower_lash1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 13716],
												mesh	: ~"R_upper_lash1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"R_upper_lash1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 8964],
												mesh	: ~"R_lower_lash1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"R_lower_lash1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Eyes",
												armature	: ~"",
												range	: [0, 2784],
												mesh	: ~"L_eye_geo1Shape@",
											}),
											ChildEntity(Entity{
												material	: ~"Pupil_SS",
												armature	: ~"",
												range	: [2784, 3264],
												mesh	: ~"L_eye_geo1Shape@",
											}),
											ChildEntity(Entity{
												material	: ~"cornea",
												armature	: ~"",
												range	: [3264, 5568],
												mesh	: ~"L_eye_geo1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"L_eye_geo1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Eyes",
												armature	: ~"",
												range	: [0, 2784],
												mesh	: ~"R_eye_geo1Shape@",
											}),
											ChildEntity(Entity{
												material	: ~"Pupil_SS",
												armature	: ~"",
												range	: [2784, 3264],
												mesh	: ~"R_eye_geo1Shape@",
											}),
											ChildEntity(Entity{
												material	: ~"cornea",
												armature	: ~"",
												range	: [3264, 5568],
												mesh	: ~"R_eye_geo1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"R_eye_geo1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"anisotropic1",
												armature	: ~"",
												range	: [0, 6954],
												mesh	: ~"Hair_Geo2Shape@",
											})
										],
										actions	: ~[],
										name	: ~"Hair_Geo2",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"skin",
												armature	: ~"",
												range	: [0, 50496],
												mesh	: ~"Body_geo8Shape@",
											})
										],
										actions	: ~[],
										name	: ~"Body_geo8",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									})
								],
								actions	: ~[],
								name	: ~"Body",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
							}),
							ChildNode(Node{
								children	: ~[
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 9042],
												mesh	: ~"R_boot1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"boots",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, -0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 5550],
												mesh	: ~"backShealth1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"backShealth1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 10236],
												mesh	: ~"R_skirt_06Shape@",
											}),
											ChildEntity(Entity{
												material	: ~"skin",
												armature	: ~"",
												range	: [10236, 12102],
												mesh	: ~"R_skirt_06Shape@",
											})
										],
										actions	: ~[],
										name	: ~"skirt",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"skin",
												armature	: ~"",
												range	: [0, 8448],
												mesh	: ~"bracket_05_geo1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"bracket",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 2304],
												mesh	: ~"L_bracer1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"bracers",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 6960],
												mesh	: ~"R_subSpaulder1Shape@",
											})
										],
										actions	: ~[],
										name	: ~"spaulders",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
									})
								],
								actions	: ~[],
								name	: ~"Armor",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
							}),
							ChildNode(Node{
								children	: ~[],
								actions	: ~[],
								name	: ~"Eyes_Geo",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
							})
						],
						actions	: ~[],
						name	: ~"noTrasnform",
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
					})
				],
				actions	: ~[],
				name	: ~"Clare",
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.50, 0.50, 0.50, 0.50],
					pos	: [0.00, 0.00, 0.00],
				},
			}),
			ChildNode(Node{
				children	: ~[
					ChildLight(Light{
						attenuation	: [0.00, 1.00],
						name	: ~"Lamp",
						color	: [1.00, 1.00, 1.00],
						kind	: KindSpot(Spot{
							size	: 1.31,
							blend	: 0.15,
						}),
						spherical	: false,
						energy	: 3.00,
						distance	: 100.00,
					})
				],
				actions	: ~[],
				name	: ~"Lamp",
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
