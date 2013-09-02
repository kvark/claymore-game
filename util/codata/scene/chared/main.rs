use common::*;
pub fn load()-> Scene	{Scene{
		global	: Global{
			gravity	: [0.00, 0.00, -9.81],
		},
		materials	: ~[
			Material{
				textures	: ~[],
				shader	: ~"phong",
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
				textures	: ~[
					Texture{
						path	: ~"//Metal_R-Spec_G-Bump_B-Reflect.jpg",
						filter	: 3,
						offset	: [0.00, 0.00, 0.00],
						name	: ~"Main",
						wrap	: 0,
						scale	: [1.00, 1.00, 1.00],
					}
				],
				shader	: ~"phong",
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
				textures	: ~[],
				shader	: ~"phong",
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
				textures	: ~[],
				shader	: ~"phong",
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
				textures	: ~[],
				shader	: ~"phong",
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
				textures	: ~[],
				shader	: ~"phong",
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
				textures	: ~[],
				shader	: ~"phong",
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
				textures	: ~[],
				shader	: ~"phong",
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
				textures	: ~[
					Texture{
						path	: ~"//Skin_Diffuse.jpg",
						filter	: 3,
						offset	: [0.00, 0.00, 0.00],
						name	: ~"Main.001",
						wrap	: 0,
						scale	: [1.00, 1.00, 1.00],
					}
				],
				shader	: ~"phong",
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
				textures	: ~[],
				shader	: ~"phong",
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
				textures	: ~[],
				shader	: ~"phong",
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
				space	: QuatSpace{
					scale	: 100.00,
					pos	: [0.00, 0.00, -1.00],
					rot	: [0.00, 0.00, 0.00, 1.00],
				},
				children	: ~[
					ChildEntity(Entity{
						material	: ~"Material",
						mesh	: ~"Plane@",
						armature	: ~"",
						range	: [0, 6],
					})
				],
				name	: ~"Plane",
			}),
			ChildNode(Node{
				space	: QuatSpace{
					scale	: 1.00,
					pos	: [140.00, 0.00, 90.00],
					rot	: [0.41, 0.41, 0.58, 0.58],
				},
				children	: ~[
					ChildCamera(Camera{
						range	: [10.00, 300.00],
						fov_y	: 0.87,
						name	: ~"Camera",
					})
				],
				name	: ~"Camera",
			}),
			ChildNode(Node{
				space	: QuatSpace{
					scale	: 1.00,
					pos	: [0.00, 0.00, 0.00],
					rot	: [0.50, 0.50, 0.50, 0.50],
				},
				children	: ~[
					ChildNode(Node{
						space	: QuatSpace{
							scale	: 1.00,
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						children	: ~[
							ChildNode(Node{
								space	: QuatSpace{
									scale	: 1.00,
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								children	: ~[],
								name	: ~"ikHandle8",
							})
						],
						name	: ~"R_ik_foot_grp",
					}),
					ChildNode(Node{
						space	: QuatSpace{
							scale	: 1.00,
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						children	: ~[],
						name	: ~"L_leg_ikHandle_zero.001",
					}),
					ChildNode(Node{
						space	: QuatSpace{
							scale	: 1.00,
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						children	: ~[
							ChildNode(Node{
								space	: QuatSpace{
									scale	: 1.00,
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								children	: ~[],
								name	: ~"ikHandle7",
							})
						],
						name	: ~"L_ik_foot_grp",
					}),
					ChildNode(Node{
						space	: QuatSpace{
							scale	: 1.00,
							pos	: [-0.00, -1.11, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						children	: ~[
							ChildNode(Node{
								space	: QuatSpace{
									scale	: 1.00,
									pos	: [0.00, 1.11, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								children	: ~[
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildNode(Node{
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												children	: ~[
													ChildArmature(Armature{
														bones	: ~[
															Bone{
																space	: QuatSpace{
																	scale	: 1.00,
																	pos	: [0.00, 47.22, 0.24],
																	rot	: [0.71, 0.00, 0.00, 0.71],
																},
																children	: ~[
																	Bone{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [-0.04, 0.00, -0.00],
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																		},
																		children	: ~[
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [0.00, 1.49, 0.00],
																					rot	: [-0.00, 0.71, 0.00, 0.71],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [-0.00, 1.54, 0.00],
																							rot	: [-0.10, 0.00, -0.00, 0.99],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 1.45, 0.00],
																									rot	: [-0.02, 0.00, -0.00, 1.00],
																								},
																								children	: ~[
																									Bone{
																										space	: QuatSpace{
																											scale	: 1.00,
																											pos	: [-0.00, 1.50, 0.00],
																											rot	: [-0.11, 0.00, -0.00, 0.99],
																										},
																										children	: ~[
																											Bone{
																												space	: QuatSpace{
																													scale	: 1.00,
																													pos	: [0.00, 6.18, -0.00],
																													rot	: [-0.00, 0.00, 0.00, 1.00],
																												},
																												children	: ~[
																													Bone{
																														space	: QuatSpace{
																															scale	: 1.00,
																															pos	: [0.00, 6.39, -0.06],
																															rot	: [-0.00, 1.00, -0.00, -0.00],
																														},
																														children	: ~[
																															Bone{
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	pos	: [0.00, 0.44, -0.29],
																																	rot	: [0.00, -0.00, 0.00, 1.00],
																																},
																																children	: ~[
																																	Bone{
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			pos	: [-0.00, 2.10, -1.00],
																																			rot	: [-0.00, 0.98, -0.21, -0.00],
																																		},
																																		children	: ~[
																																			Bone{
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					pos	: [0.00, 2.37, -0.00],
																																					rot	: [-0.00, 0.00, 0.00, 1.00],
																																				},
																																				children	: ~[
																																					Bone{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [-0.00, 6.34, 0.04],
																																							rot	: [-0.04, 0.00, 0.00, 1.00],
																																						},
																																						children	: ~[
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [1.26, -4.62, 3.06],
																																									rot	: [0.71, -0.01, 0.01, 0.71],
																																								},
																																								children	: ~[
																																									Bone{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [0.00, 0.69, 0.00],
																																											rot	: [0.00, 0.71, -0.00, 0.70],
																																										},
																																										children	: ~[],
																																										name	: ~"L_eye_end_joint",
																																									}
																																								],
																																								name	: ~"L_eye_joint",
																																							},
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [-1.26, -4.62, 3.06],
																																									rot	: [0.71, 0.01, -0.01, 0.71],
																																								},
																																								children	: ~[
																																									Bone{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [0.00, 0.69, -0.00],
																																											rot	: [-0.00, 0.70, -0.00, 0.71],
																																										},
																																										children	: ~[],
																																										name	: ~"R_eye_end_joint",
																																									}
																																								],
																																								name	: ~"R_eye_joint",
																																							},
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [-1.26, -4.62, 3.06],
																																									rot	: [0.77, -0.00, 0.00, 0.64],
																																								},
																																								children	: ~[
																																									Bone{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [-0.00, 0.50, -0.00],
																																											rot	: [0.00, 0.71, -0.00, 0.71],
																																										},
																																										children	: ~[],
																																										name	: ~"R_eye_blink_01_joint",
																																									}
																																								],
																																								name	: ~"R_eye_blink_base_joint",
																																							},
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [1.26, -4.62, 3.06],
																																									rot	: [0.77, -0.00, 0.00, 0.64],
																																								},
																																								children	: ~[
																																									Bone{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [0.00, 0.50, -0.00],
																																											rot	: [0.00, 0.71, -0.00, 0.71],
																																										},
																																										children	: ~[],
																																										name	: ~"L_eye_blink_01_joint",
																																									}
																																								],
																																								name	: ~"L_eye_blink_base_joint",
																																							},
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [-1.26, -4.62, 3.06],
																																									rot	: [0.60, -0.00, 0.00, 0.80],
																																								},
																																								children	: ~[
																																									Bone{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [-0.00, 0.67, -0.00],
																																											rot	: [-0.00, 0.71, -0.00, 0.71],
																																										},
																																										children	: ~[],
																																										name	: ~"R_eye_blink_02_joint",
																																									}
																																								],
																																								name	: ~"R_eye_blink_02_base_joint",
																																							},
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [1.26, -4.62, 3.06],
																																									rot	: [0.60, -0.00, 0.00, 0.80],
																																								},
																																								children	: ~[
																																									Bone{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [-0.00, 0.67, -0.00],
																																											rot	: [0.00, 0.71, 0.00, 0.71],
																																										},
																																										children	: ~[],
																																										name	: ~"L_eye_blink_02_joint",
																																									}
																																								],
																																								name	: ~"L_eye_blink_02_base_joint",
																																							}
																																						],
																																						name	: ~"head_end",
																																					},
																																					Bone{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [-0.00, -0.41, 0.55],
																																							rot	: [0.82, 0.00, 0.00, 0.57],
																																						},
																																						children	: ~[
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [0.00, 3.68, -0.00],
																																									rot	: [-0.00, 0.71, 0.00, 0.71],
																																								},
																																								children	: ~[],
																																								name	: ~"jaw_end_joint",
																																							}
																																						],
																																						name	: ~"jaw_joint",
																																					}
																																				],
																																				name	: ~"head_joint",
																																			}
																																		],
																																		name	: ~"c_neck_02_joint",
																																	}
																																],
																																name	: ~"c_neck_01_joint",
																															},
																															Bone{
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	pos	: [-0.00, 0.04, 2.55],
																																	rot	: [-0.00, 0.16, 0.99, 0.00],
																																},
																																children	: ~[
																																	Bone{
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			pos	: [0.00, 1.81, 0.00],
																																			rot	: [0.00, 1.00, -0.00, -0.00],
																																		},
																																		children	: ~[],
																																		name	: ~"c_shealth_end_joint",
																																	}
																																],
																																name	: ~"c_shealth_01_joint",
																															},
																															Bone{
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	pos	: [-1.00, -2.13, -2.41],
																																	rot	: [-0.31, 0.71, 0.50, 0.39],
																																},
																																children	: ~[
																																	Bone{
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			pos	: [-0.00, 5.59, -0.00],
																																			rot	: [-0.00, -0.93, -0.00, 0.36],
																																		},
																																		children	: ~[
																																			Bone{
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					pos	: [-0.52, 0.95, -0.03],
																																					rot	: [-0.15, 0.73, 0.20, 0.63],
																																				},
																																				children	: ~[
																																					Bone{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [0.00, 3.42, 0.00],
																																							rot	: [0.00, 0.91, 0.00, 0.40],
																																						},
																																						children	: ~[
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [0.00, 3.58, 0.00],
																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																								},
																																								children	: ~[
																																									Bone{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [-0.00, 3.95, -0.00],
																																											rot	: [0.24, 0.27, 0.02, 0.93],
																																										},
																																										children	: ~[
																																											Bone{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [0.00, 2.77, 0.00],
																																													rot	: [0.00, 0.01, 0.00, 1.00],
																																												},
																																												children	: ~[
																																													Bone{
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															pos	: [-0.00, 2.64, -0.00],
																																															rot	: [-0.00, -0.02, 0.02, 1.00],
																																														},
																																														children	: ~[
																																															Bone{
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	pos	: [0.00, 5.42, -0.00],
																																																	rot	: [-0.00, 0.02, 0.00, 1.00],
																																																},
																																																children	: ~[
																																																	Bone{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [-0.06, 2.48, -0.37],
																																																			rot	: [0.11, -0.29, 0.75, 0.59],
																																																		},
																																																		children	: ~[
																																																			Bone{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [-0.54, -0.80, -1.23],
																																																					rot	: [-0.37, 0.10, -0.71, 0.59],
																																																				},
																																																				children	: ~[
																																																					Bone{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [-0.00, 1.18, 0.00],
																																																							rot	: [0.25, 0.56, -0.16, 0.77],
																																																						},
																																																						children	: ~[
																																																							Bone{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 0.75, 0.00],
																																																									rot	: [0.12, 0.34, 0.07, 0.93],
																																																								},
																																																								children	: ~[
																																																									Bone{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 0.96, 0.00],
																																																											rot	: [0.00, 0.94, 0.00, 0.35],
																																																										},
																																																										children	: ~[],
																																																										name	: ~"L_pinkyFinger_04_joint",
																																																									}
																																																								],
																																																								name	: ~"L_pinkyFinger_03_joint",
																																																							}
																																																						],
																																																						name	: ~"L_pinkyFinger_02_joint",
																																																					}
																																																				],
																																																				name	: ~"L_pinkyFinger_01_joint",
																																																			},
																																																			Bone{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [0.26, -0.29, -0.98],
																																																					rot	: [-0.54, -0.07, -0.59, 0.60],
																																																				},
																																																				children	: ~[
																																																					Bone{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.63, 0.00],
																																																							rot	: [0.27, 0.81, -0.24, 0.46],
																																																						},
																																																						children	: ~[
																																																							Bone{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [0.00, 0.95, -0.00],
																																																									rot	: [0.12, 0.37, 0.10, 0.92],
																																																								},
																																																								children	: ~[
																																																									Bone{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 1.00, 0.00],
																																																											rot	: [0.00, 0.96, -0.00, 0.29],
																																																										},
																																																										children	: ~[],
																																																										name	: ~"L_ringFinger_04_joint",
																																																									}
																																																								],
																																																								name	: ~"L_ringFinger_03_joint",
																																																							}
																																																						],
																																																						name	: ~"L_ringFinger_02_joint",
																																																					}
																																																				],
																																																				name	: ~"L_ringFinger_01_joint",
																																																			},
																																																			Bone{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [0.92, 0.08, -0.55],
																																																					rot	: [-0.58, -0.15, -0.55, 0.58],
																																																				},
																																																				children	: ~[
																																																					Bone{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.64, 0.00],
																																																							rot	: [0.16, 0.83, -0.31, 0.44],
																																																						},
																																																						children	: ~[
																																																							Bone{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [0.00, 1.01, -0.00],
																																																									rot	: [0.07, 0.19, 0.02, 0.98],
																																																								},
																																																								children	: ~[
																																																									Bone{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [0.00, 1.24, 0.00],
																																																											rot	: [-0.00, 0.24, 0.00, 0.97],
																																																										},
																																																										children	: ~[],
																																																										name	: ~"L_middleFinger_04_joint",
																																																									}
																																																								],
																																																								name	: ~"L_middleFinger_03_joint",
																																																							}
																																																						],
																																																						name	: ~"L_middleFinger_02_joint",
																																																					}
																																																				],
																																																				name	: ~"L_middleFinger_01_joint",
																																																			},
																																																			Bone{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [1.60, 0.39, 0.17],
																																																					rot	: [-0.63, -0.19, -0.46, 0.59],
																																																				},
																																																				children	: ~[
																																																					Bone{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.48, -0.00],
																																																							rot	: [-0.05, 0.73, -0.36, 0.58],
																																																						},
																																																						children	: ~[
																																																							Bone{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 1.03, -0.00],
																																																									rot	: [0.14, 0.36, -0.05, 0.92],
																																																								},
																																																								children	: ~[
																																																									Bone{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 1.09, -0.00],
																																																											rot	: [-0.00, 0.99, -0.00, 0.17],
																																																										},
																																																										children	: ~[],
																																																										name	: ~"L_indexFinger_04_joint",
																																																									}
																																																								],
																																																								name	: ~"L_indexFinger_03_joint",
																																																							}
																																																						],
																																																						name	: ~"L_indexFinger_02_joint",
																																																					}
																																																				],
																																																				name	: ~"L_indexFinger_01_joint",
																																																			}
																																																		],
																																																		name	: ~"L_wrist_end_joint",
																																																	},
																																																	Bone{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [-0.43, 1.21, 0.79],
																																																			rot	: [0.38, 0.51, 0.19, 0.75],
																																																		},
																																																		children	: ~[
																																																			Bone{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [0.00, 1.91, 0.00],
																																																					rot	: [-0.06, -0.32, -0.13, 0.94],
																																																				},
																																																				children	: ~[
																																																					Bone{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [-0.00, 1.10, -0.00],
																																																							rot	: [0.01, 0.12, 0.02, 0.99],
																																																						},
																																																						children	: ~[
																																																							Bone{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [0.00, 1.17, 0.00],
																																																									rot	: [0.00, -0.82, 0.00, 0.57],
																																																								},
																																																								children	: ~[],
																																																								name	: ~"L_thumb_04_joint",
																																																							}
																																																						],
																																																						name	: ~"L_thumb_03_joint",
																																																					}
																																																				],
																																																				name	: ~"L_thumb_02_joint",
																																																			}
																																																		],
																																																		name	: ~"L_thumb_01_joint",
																																																	}
																																																],
																																																name	: ~"L_wrist_joint",
																																															}
																																														],
																																														name	: ~"L_forearm_02_joint",
																																													}
																																												],
																																												name	: ~"L_forearm_01_joint",
																																											}
																																										],
																																										name	: ~"L_elbow_joint",
																																									}
																																								],
																																								name	: ~"L_arm_02_joint",
																																							}
																																						],
																																						name	: ~"L_arm_01_joint",
																																					}
																																				],
																																				name	: ~"L_shoulder_joint",
																																			},
																																			Bone{
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					pos	: [-0.31, -0.70, 0.27],
																																					rot	: [-0.37, 0.26, -0.12, 0.89],
																																				},
																																				children	: ~[
																																					Bone{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [0.00, 3.22, 0.00],
																																							rot	: [0.33, -0.01, 0.91, 0.25],
																																						},
																																						children	: ~[
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [0.00, 1.03, -0.00],
																																									rot	: [0.24, 0.35, -0.81, 0.40],
																																								},
																																								children	: ~[
																																									Bone{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [0.00, 2.16, -0.00],
																																											rot	: [0.00, 0.87, 0.00, 0.50],
																																										},
																																										children	: ~[],
																																										name	: ~"L_subSpaulder_end_joint",
																																									}
																																								],
																																								name	: ~"L_subSpaulder_joint",
																																							}
																																						],
																																						name	: ~"L_mainSpaulder_end_joint",
																																					}
																																				],
																																				name	: ~"L_mainSpaulder_joint",
																																			},
																																			Bone{
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					pos	: [-0.52, 0.95, -0.03],
																																					rot	: [-0.15, 0.73, 0.20, 0.63],
																																				},
																																				children	: ~[
																																					Bone{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [0.00, 10.95, 0.00],
																																							rot	: [0.13, 0.96, -0.21, 0.13],
																																						},
																																						children	: ~[
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [-0.00, 10.84, -0.00],
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																								},
																																								children	: ~[],
																																								name	: ~"L_armIK_03_joint",
																																							}
																																						],
																																						name	: ~"L_armIK_02_joint",
																																					}
																																				],
																																				name	: ~"L_armIK_01_joint",
																																			}
																																		],
																																		name	: ~"L_clav_end_joint",
																																	}
																																],
																																name	: ~"L_clav_joint",
																															},
																															Bone{
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	pos	: [1.00, -2.13, -2.41],
																																	rot	: [-0.32, -0.71, -0.50, 0.38],
																																},
																																children	: ~[
																																	Bone{
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			pos	: [0.00, 5.59, -0.00],
																																			rot	: [0.00, -0.72, -0.00, 0.70],
																																		},
																																		children	: ~[
																																			Bone{
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					pos	: [-0.31, 0.95, 0.42],
																																					rot	: [0.11, 0.20, 0.23, 0.95],
																																				},
																																				children	: ~[
																																					Bone{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [0.00, 3.42, -0.00],
																																							rot	: [0.00, -0.21, 0.00, 0.98],
																																						},
																																						children	: ~[
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [0.00, 3.58, -0.00],
																																									rot	: [0.00, -0.00, -0.00, 1.00],
																																								},
																																								children	: ~[
																																									Bone{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [0.00, 3.95, -0.00],
																																											rot	: [0.24, -0.27, -0.02, 0.93],
																																										},
																																										children	: ~[
																																											Bone{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [0.00, 2.77, 0.00],
																																													rot	: [-0.00, -0.01, -0.00, 1.00],
																																												},
																																												children	: ~[
																																													Bone{
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															pos	: [0.00, 2.64, -0.00],
																																															rot	: [-0.00, 0.02, -0.02, 1.00],
																																														},
																																														children	: ~[
																																															Bone{
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	pos	: [0.00, 5.42, -0.00],
																																																	rot	: [0.00, -0.02, 0.00, 1.00],
																																																},
																																																children	: ~[
																																																	Bone{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [0.43, 1.21, 0.79],
																																																			rot	: [0.38, -0.51, -0.19, 0.75],
																																																		},
																																																		children	: ~[
																																																			Bone{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [0.00, 1.91, -0.00],
																																																					rot	: [-0.06, 0.32, 0.13, 0.94],
																																																				},
																																																				children	: ~[
																																																					Bone{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.10, -0.00],
																																																							rot	: [0.01, -0.12, -0.02, 0.99],
																																																						},
																																																						children	: ~[
																																																							Bone{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 1.17, 0.00],
																																																									rot	: [-0.00, 0.82, -0.00, 0.57],
																																																								},
																																																								children	: ~[],
																																																								name	: ~"R_thumb_04_joint",
																																																							}
																																																						],
																																																						name	: ~"R_thumb_03_joint",
																																																					}
																																																				],
																																																				name	: ~"R_thumb_02_joint",
																																																			}
																																																		],
																																																		name	: ~"R_thumb_01_joint",
																																																	},
																																																	Bone{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [0.06, 2.48, -0.37],
																																																			rot	: [0.11, 0.29, -0.75, 0.59],
																																																		},
																																																		children	: ~[
																																																			Bone{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [-1.60, 0.39, 0.17],
																																																					rot	: [-0.45, 0.13, 0.60, 0.65],
																																																				},
																																																				children	: ~[
																																																					Bone{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [-0.00, 1.48, -0.00],
																																																							rot	: [0.07, -0.67, 0.35, 0.65],
																																																						},
																																																						children	: ~[
																																																							Bone{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [0.00, 1.03, -0.00],
																																																									rot	: [0.15, -0.31, 0.02, 0.94],
																																																								},
																																																								children	: ~[
																																																									Bone{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 1.09, -0.00],
																																																											rot	: [-0.00, -0.99, 0.00, 0.17],
																																																										},
																																																										children	: ~[],
																																																										name	: ~"R_indexFinger_04_joint",
																																																									}
																																																								],
																																																								name	: ~"R_indexFinger_03_joint",
																																																							}
																																																						],
																																																						name	: ~"R_indexFinger_02_joint",
																																																					}
																																																				],
																																																				name	: ~"R_indexFinger_01_joint",
																																																			},
																																																			Bone{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [-0.92, 0.08, -0.55],
																																																					rot	: [-0.58, 0.15, 0.55, 0.58],
																																																				},
																																																				children	: ~[
																																																					Bone{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.64, 0.00],
																																																							rot	: [0.16, -0.83, 0.31, 0.44],
																																																						},
																																																						children	: ~[
																																																							Bone{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 1.01, -0.00],
																																																									rot	: [0.07, -0.19, -0.02, 0.98],
																																																								},
																																																								children	: ~[
																																																									Bone{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 1.24, -0.00],
																																																											rot	: [0.00, -0.24, -0.00, 0.97],
																																																										},
																																																										children	: ~[],
																																																										name	: ~"R_middleFinger_04_joint",
																																																									}
																																																								],
																																																								name	: ~"R_middleFinger_03_joint",
																																																							}
																																																						],
																																																						name	: ~"R_middleFinger_02_joint",
																																																					}
																																																				],
																																																				name	: ~"R_middleFinger_01_joint",
																																																			},
																																																			Bone{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [-0.26, -0.29, -0.98],
																																																					rot	: [-0.62, 0.09, 0.52, 0.57],
																																																				},
																																																				children	: ~[
																																																					Bone{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [-0.00, 1.63, -0.00],
																																																							rot	: [0.23, -0.84, 0.28, 0.40],
																																																						},
																																																						children	: ~[
																																																							Bone{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 0.95, -0.00],
																																																									rot	: [0.12, -0.41, -0.09, 0.90],
																																																								},
																																																								children	: ~[
																																																									Bone{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [0.00, 1.00, 0.00],
																																																											rot	: [-0.00, -0.96, -0.00, 0.29],
																																																										},
																																																										children	: ~[],
																																																										name	: ~"R_ringFinger_04_joint",
																																																									}
																																																								],
																																																								name	: ~"R_ringFinger_03_joint",
																																																							}
																																																						],
																																																						name	: ~"R_ringFinger_02_joint",
																																																					}
																																																				],
																																																				name	: ~"R_ringFinger_01_joint",
																																																			},
																																																			Bone{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [0.54, -0.80, -1.23],
																																																					rot	: [-0.57, -0.04, 0.62, 0.53],
																																																				},
																																																				children	: ~[
																																																					Bone{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 1.18, 0.00],
																																																							rot	: [0.21, -0.68, 0.22, 0.67],
																																																						},
																																																						children	: ~[
																																																							Bone{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [-0.00, 0.75, 0.00],
																																																									rot	: [0.12, -0.42, -0.07, 0.90],
																																																								},
																																																								children	: ~[
																																																									Bone{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [0.00, 0.96, 0.00],
																																																											rot	: [0.00, -0.94, 0.00, 0.35],
																																																										},
																																																										children	: ~[],
																																																										name	: ~"R_pinkyFinger_04_joint",
																																																									}
																																																								],
																																																								name	: ~"R_pinkyFinger_03_joint",
																																																							}
																																																						],
																																																						name	: ~"R_pinkyFinger_02_joint",
																																																					}
																																																				],
																																																				name	: ~"R_pinkyFinger_01_joint",
																																																			}
																																																		],
																																																		name	: ~"R_wrist_end_joint",
																																																	}
																																																],
																																																name	: ~"R_wrist_joint",
																																															}
																																														],
																																														name	: ~"R_forearm_02_joint",
																																													}
																																												],
																																												name	: ~"R_forearm_01_joint",
																																											}
																																										],
																																										name	: ~"R_elbow_joint",
																																									}
																																								],
																																								name	: ~"R_arm_02_joint",
																																							}
																																						],
																																						name	: ~"R_arm_01_joint",
																																					}
																																				],
																																				name	: ~"R_shoulder_joint",
																																			},
																																			Bone{
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					pos	: [-0.40, -0.70, 0.06],
																																					rot	: [0.38, 0.65, -0.05, 0.65],
																																				},
																																				children	: ~[
																																					Bone{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [0.00, 3.22, 0.00],
																																							rot	: [-0.54, 0.21, -0.80, 0.14],
																																						},
																																						children	: ~[
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [-0.00, 1.03, -0.00],
																																									rot	: [0.24, -0.35, 0.81, 0.40],
																																								},
																																								children	: ~[
																																									Bone{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [-0.00, 2.16, 0.00],
																																											rot	: [-0.00, 0.38, 0.00, 0.93],
																																										},
																																										children	: ~[],
																																										name	: ~"R_subSpaulder_end_joint",
																																									}
																																								],
																																								name	: ~"R_subSpaulder_joint",
																																							}
																																						],
																																						name	: ~"R_mainSpaulder_end_joint",
																																					}
																																				],
																																				name	: ~"R_mainSpaulder_joint",
																																			},
																																			Bone{
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					pos	: [-0.31, 0.95, 0.42],
																																					rot	: [0.11, 0.20, 0.23, 0.95],
																																				},
																																				children	: ~[
																																					Bone{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [0.00, 10.95, -0.00],
																																							rot	: [0.24, -0.46, 0.02, 0.85],
																																						},
																																						children	: ~[
																																							Bone{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [0.00, 10.84, 0.00],
																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																								},
																																								children	: ~[],
																																								name	: ~"R_armIK_03_joint",
																																							}
																																						],
																																						name	: ~"R_armIK_02_joint",
																																					}
																																				],
																																				name	: ~"R_armIK_01_joint",
																																			}
																																		],
																																		name	: ~"R_clav_end_joint",
																																	}
																																],
																																name	: ~"R_clav_joint",
																															}
																														],
																														name	: ~"c_spine_007_joint",
																													},
																													Bone{
																														space	: QuatSpace{
																															scale	: 1.00,
																															pos	: [3.20, -0.56, 2.70],
																															rot	: [0.84, 0.12, -0.06, 0.52],
																														},
																														children	: ~[
																															Bone{
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	pos	: [-0.00, 3.94, -0.00],
																																	rot	: [-0.00, 0.58, -0.00, 0.81],
																																},
																																children	: ~[
																																	Bone{
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			pos	: [0.00, 0.00, 0.00],
																																			rot	: [0.00, -0.71, 0.00, 0.71],
																																		},
																																		children	: ~[],
																																		name	: ~"L_breast_end_joint",
																																	}
																																],
																																name	: ~"L_breast_joint",
																															}
																														],
																														name	: ~"L_breast_base_joint",
																													},
																													Bone{
																														space	: QuatSpace{
																															scale	: 1.00,
																															pos	: [-3.20, -0.56, 2.70],
																															rot	: [0.84, -0.12, 0.06, 0.52],
																														},
																														children	: ~[
																															Bone{
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	pos	: [-0.00, 3.94, -0.00],
																																	rot	: [-0.00, -0.58, 0.00, 0.81],
																																},
																																children	: ~[
																																	Bone{
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			pos	: [0.00, -0.00, -0.00],
																																			rot	: [0.00, 0.71, -0.00, 0.71],
																																		},
																																		children	: ~[],
																																		name	: ~"R_breast_end_joint1",
																																	}
																																],
																																name	: ~"R_breast_joint1",
																															}
																														],
																														name	: ~"R_breast_base_joint1",
																													}
																												],
																												name	: ~"c_spine_006_joint",
																											}
																										],
																										name	: ~"c_spine_005_joint",
																									}
																								],
																								name	: ~"c_spine_004_joint",
																							}
																						],
																						name	: ~"c_spine_003_joint",
																					}
																				],
																				name	: ~"c_spine_002_joint",
																			}
																		],
																		name	: ~"c_spine_001_joint",
																	},
																	Bone{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [0.00, -0.10, 0.04],
																			rot	: [0.00, -0.71, 0.00, 0.71],
																		},
																		children	: ~[
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [4.99, 0.48, 2.82],
																					rot	: [-0.69, -0.14, -0.69, 0.14],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [-0.00, 4.60, -0.00],
																							rot	: [-0.00, 0.71, 0.00, 0.71],
																						},
																						children	: ~[],
																						name	: ~"R_hip_joint",
																					}
																				],
																				name	: ~"R_hip_base_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [4.92, 0.46, 4.10],
																					rot	: [0.51, -0.51, 0.49, 0.49],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [-0.19, -6.70, -0.11],
																							rot	: [0.85, -0.02, 0.53, -0.00],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [0.00, 5.50, -0.00],
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																								},
																								children	: ~[
																									Bone{
																										space	: QuatSpace{
																											scale	: 1.00,
																											pos	: [0.00, 4.95, 0.00],
																											rot	: [0.00, -0.00, 0.00, 1.00],
																										},
																										children	: ~[],
																										name	: ~"R_kneePivot_01_joint",
																									}
																								],
																								name	: ~"R_kneePivot_02_joint",
																							}
																						],
																						name	: ~"R_kneePivot_03_joint",
																					},
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [-0.53, -18.67, -0.32],
																							rot	: [0.22, -0.06, -0.97, 0.00],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 19.52, 0.00],
																									rot	: [-0.56, -0.19, -0.14, 0.80],
																								},
																								children	: ~[
																									Bone{
																										space	: QuatSpace{
																											scale	: 1.00,
																											pos	: [-0.00, 8.00, -0.00],
																											rot	: [-0.23, 0.02, 0.01, 0.97],
																										},
																										children	: ~[
																											Bone{
																												space	: QuatSpace{
																													scale	: 1.00,
																													pos	: [0.00, 2.80, -0.00],
																													rot	: [-0.00, -0.72, 0.00, 0.70],
																												},
																												children	: ~[],
																												name	: ~"R_toe_joint",
																											}
																										],
																										name	: ~"R_ball_joint",
																									}
																								],
																								name	: ~"R_ankle_joint",
																							}
																						],
																						name	: ~"R_knee_01_joint",
																					}
																				],
																				name	: ~"R_leg_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [4.99, 0.48, -2.82],
																					rot	: [-0.69, -0.14, -0.69, 0.14],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [-0.00, 4.60, 0.00],
																							rot	: [0.00, 0.71, 0.00, 0.71],
																						},
																						children	: ~[],
																						name	: ~"L_hip_joint",
																					}
																				],
																				name	: ~"L_hip_base_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [4.92, 0.46, -4.10],
																					rot	: [0.49, -0.49, 0.51, 0.51],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.19, -6.70, -0.06],
																							rot	: [0.95, 0.01, -0.30, -0.00],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 5.50, 0.00],
																									rot	: [0.00, -0.00, 0.00, 1.00],
																								},
																								children	: ~[
																									Bone{
																										space	: QuatSpace{
																											scale	: 1.00,
																											pos	: [0.00, 4.95, -0.00],
																											rot	: [-0.00, 0.00, 0.00, 1.00],
																										},
																										children	: ~[],
																										name	: ~"L_kneePivot_01_joint",
																									}
																								],
																								name	: ~"L_kneePivot_02_joint",
																							}
																						],
																						name	: ~"L_kneePivot_03_joint",
																					},
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.53, -18.67, -0.16],
																							rot	: [0.25, 0.05, 0.97, 0.00],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [0.00, 19.52, 0.00],
																									rot	: [-0.55, 0.25, 0.14, 0.79],
																								},
																								children	: ~[
																									Bone{
																										space	: QuatSpace{
																											scale	: 1.00,
																											pos	: [-0.00, 8.00, 0.00],
																											rot	: [-0.23, -0.06, -0.02, 0.97],
																										},
																										children	: ~[
																											Bone{
																												space	: QuatSpace{
																													scale	: 1.00,
																													pos	: [0.00, 2.80, -0.00],
																													rot	: [0.00, -0.69, 0.00, 0.72],
																												},
																												children	: ~[],
																												name	: ~"L_toe_joint",
																											}
																										],
																										name	: ~"L_ball_joint",
																									}
																								],
																								name	: ~"L_ankle_joint",
																							}
																						],
																						name	: ~"L_knee_01_joint",
																					}
																				],
																				name	: ~"L_leg_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [1.22, 4.37, 3.97],
																					rot	: [0.68, 0.70, -0.05, 0.19],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, -0.00],
																							rot	: [-0.07, 0.99, 0.04, 0.07],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 5.95, 0.00],
																									rot	: [0.00, -0.00, -0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"R_frontMid_skirtplate_03_joint",
																							}
																						],
																						name	: ~"R_frontMid_skirtplate_02_joint",
																					}
																				],
																				name	: ~"R_frontMid_skirtplate_01_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [2.94, 4.48, 2.04],
																					rot	: [-0.68, -0.73, -0.00, 0.00],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, -0.00],
																							rot	: [0.07, -0.93, 0.04, 0.35],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [0.00, 5.95, -0.00],
																									rot	: [0.00, -0.00, -0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"R_frontInner_skirtplate_03_joint",
																							}
																						],
																						name	: ~"R_frontInner_skirtplate_02_joint",
																					}
																				],
																				name	: ~"R_frontInner_skirtplate_01_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [2.94, 4.48, -2.04],
																					rot	: [-0.00, -0.00, -0.68, 0.73],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, 0.00],
																							rot	: [0.07, 0.93, -0.04, 0.35],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [0.00, 5.95, 0.00],
																									rot	: [0.00, -0.00, 0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"L_frontInner_skirtplate_03_joint",
																							}
																						],
																						name	: ~"L_frontInner_skirtplate_02_joint",
																					}
																				],
																				name	: ~"L_frontInner_skirtplate_01_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [-1.03, 1.62, -5.94],
																					rot	: [0.08, 0.39, -0.73, 0.56],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, -0.00],
																							rot	: [-0.13, 0.98, 0.14, 0.03],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 5.95, -0.00],
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"L_side_skirtplate_03_joint",
																							}
																						],
																						name	: ~"L_side_skirtplate_02_joint",
																					}
																				],
																				name	: ~"L_side_skirtplate_01_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [-0.29, -2.24, -3.34],
																					rot	: [-0.34, -0.02, -0.78, 0.53],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [-0.00, 4.80, 0.00],
																							rot	: [0.13, -0.96, -0.19, 0.17],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 5.95, 0.00],
																									rot	: [0.00, 0.00, 0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"L_rearSide_skirtplate_03_joint",
																							}
																						],
																						name	: ~"L_rearSide_skirtplate_02_joint",
																					}
																				],
																				name	: ~"L_rearSide_skirtplate_01_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [1.63, -3.54, -0.00],
																					rot	: [-0.61, -0.37, -0.60, 0.36],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [-0.00, 4.80, -0.00],
																							rot	: [0.01, 0.97, 0.25, 0.04],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 3.88, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"C_rear_skirtplate_03_joint",
																							}
																						],
																						name	: ~"C_rear_skirtplate_02_joint",
																					}
																				],
																				name	: ~"C_rear_skirtplate_01_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [-0.29, -2.24, 3.34],
																					rot	: [-0.78, -0.53, -0.34, 0.02],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, -0.00],
																							rot	: [0.13, 0.96, 0.19, 0.17],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 5.95, -0.00],
																									rot	: [0.00, -0.00, -0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"R_rearSide_skirtplate_03_joint",
																							}
																						],
																						name	: ~"R_rearSide_skirtplate_02_joint",
																					}
																				],
																				name	: ~"R_rearSide_skirtplate_01_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [-1.03, 1.62, 5.94],
																					rot	: [0.73, 0.56, -0.08, 0.39],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, 0.00],
																							rot	: [-0.13, -0.98, -0.14, 0.03],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [0.00, 5.95, 0.00],
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"R_side_skirtplate_03_joint",
																							}
																						],
																						name	: ~"R_side_skirtplate_02_joint",
																					}
																				],
																				name	: ~"R_side_skirtplate_01_joint",
																			},
																			Bone{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [1.22, 4.37, -3.97],
																					rot	: [0.05, 0.19, -0.68, 0.70],
																				},
																				children	: ~[
																					Bone{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, 4.80, -0.00],
																							rot	: [-0.07, -0.99, -0.04, 0.07],
																						},
																						children	: ~[
																							Bone{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [0.00, 5.95, 0.00],
																									rot	: [0.00, 0.00, 0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"L_frontMid_skirtplate_03_joint",
																							}
																						],
																						name	: ~"L_frontMid_skirtplate_02_joint",
																					}
																				],
																				name	: ~"L_frontMid_skirtplate_01_joint",
																			}
																		],
																		name	: ~"C_hip_joint",
																	}
																],
																name	: ~"cog",
															}
														],
														dual_quat	: ~"false",
														name	: ~"Armature.002",
													})
												],
												name	: ~"Armature.002",
											}),
											ChildNode(Node{
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												children	: ~[],
												name	: ~"SKELETON",
											}),
											ChildNode(Node{
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															scale	: 1.00,
															pos	: [-0.04, 71.88, 17.98],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	scale	: 1.00,
																	pos	: [0.04, -71.88, -17.98],
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																},
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 0.72,
																			pos	: [-0.95, -0.00, 0.00],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																		},
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [-0.00, -0.00, 0.00],
																					rot	: [0.00, 0.00, -0.00, 1.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, 0.00, -0.00],
																							rot	: [0.00, -0.00, -0.00, 1.00],
																						},
																						children	: ~[],
																						name	: ~"R_eye_ctrl",
																					})
																				],
																				name	: ~"R_eye_ctrl_zero",
																			})
																		],
																		name	: ~"R_eye_ctrl_grp",
																	}),
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 0.72,
																			pos	: [0.95, 0.00, -0.00],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																		},
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [0.00, 0.00, -0.00],
																					rot	: [0.00, 0.00, -0.00, 1.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, 0.00, -0.00],
																							rot	: [0.00, 0.00, -0.00, 1.00],
																						},
																						children	: ~[],
																						name	: ~"L_eye_ctrl",
																					})
																				],
																				name	: ~"L_eye_ctrl_zero",
																			})
																		],
																		name	: ~"L_eye_ctrl_grp",
																	})
																],
																name	: ~"mainEye_ctrl",
															})
														],
														name	: ~"mainEye_ctrl_zero",
													})
												],
												name	: ~"c_eye_ctrl_grp",
											}),
											ChildNode(Node{
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															scale	: 1.00,
															pos	: [1.26, 71.88, 0.51],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														children	: ~[],
														name	: ~"L_eye_centerLocator",
													}),
													ChildNode(Node{
														space	: QuatSpace{
															scale	: 1.00,
															pos	: [-1.26, 71.88, 0.51],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														children	: ~[],
														name	: ~"R_eye_centerLocator",
													})
												],
												name	: ~"Locators",
											}),
											ChildNode(Node{
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															scale	: 1.00,
															pos	: [3.00, -0.31, -3.16],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	scale	: 1.00,
																	pos	: [0.00, -0.00, 0.00],
																	rot	: [-0.00, -0.00, -0.00, 1.00],
																},
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [-0.00, 0.00, 0.00],
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																		},
																		children	: ~[],
																		name	: ~"L_foot_ik_ctrl",
																	})
																],
																name	: ~"L_leg_ikHandle_zero",
															})
														],
														name	: ~"L_leg_ikHandle_grp",
													}),
													ChildNode(Node{
														space	: QuatSpace{
															scale	: 1.00,
															pos	: [-3.00, -0.31, -3.16],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	scale	: 1.00,
																	pos	: [-0.00, -0.00, 0.00],
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																},
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [0.00, 0.00, 0.00],
																			rot	: [-0.01, -0.00, 0.00, 1.00],
																		},
																		children	: ~[],
																		name	: ~"R_foot_ik_ctrl",
																	})
																],
																name	: ~"R_leg_ikHandle_zero",
															})
														],
														name	: ~"R_leg_ikHandle_grp",
													}),
													ChildNode(Node{
														space	: QuatSpace{
															scale	: 1.00,
															pos	: [-4.14, 43.04, -0.16],
															rot	: [0.00, 0.00, 0.00, 1.00],
														},
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	scale	: 1.00,
																	pos	: [0.57, -18.67, 10.59],
																	rot	: [0.00, -0.00, -0.00, 1.00],
																},
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [0.00, 0.00, 0.00],
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																		},
																		children	: ~[],
																		name	: ~"R_legPole_ctrl.001",
																	})
																],
																name	: ~"R_legPole_ctrl_zero",
															})
														],
														name	: ~"R_legPole_ctrl",
													}),
													ChildNode(Node{
														space	: QuatSpace{
															scale	: 1.00,
															pos	: [4.10, 42.26, 0.61],
															rot	: [-0.47, 0.53, -0.49, 0.51],
														},
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	scale	: 1.00,
																	pos	: [0.00, 0.00, -0.00],
																	rot	: [0.47, -0.53, 0.49, 0.51],
																},
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [-0.55, -17.93, 9.74],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																		},
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [0.00, -0.00, -0.00],
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																				},
																				children	: ~[],
																				name	: ~"L_legPole_ctrl",
																			})
																		],
																		name	: ~"L_legPole_cntr_zero",
																	})
																],
																name	: ~"L_legPole_ctrl_grp",
															})
														],
														name	: ~"L_legPole_ctrl_cons",
													})
												],
												name	: ~"LegControls",
											}),
											ChildNode(Node{
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [27.41, 60.54, -2.62],
													rot	: [-0.26, -0.08, -0.07, 0.96],
												},
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															scale	: 1.00,
															pos	: [0.00, -0.00, 0.00],
															rot	: [0.26, 0.08, 0.07, 0.96],
														},
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	scale	: 1.00,
																	pos	: [-27.41, -60.54, 2.62],
																	rot	: [0.00, -0.00, 0.00, 1.00],
																},
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [-0.00, 0.00, -0.00],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																		},
																		children	: ~[],
																		name	: ~"L_armIK_handle",
																	}),
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [-0.00, 0.00, -0.00],
																			rot	: [-0.26, -0.08, -0.07, 0.96],
																		},
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [-0.00, 0.00, -0.00],
																					rot	: [0.00, -0.00, 0.00, 1.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, -0.00, 0.00],
																							rot	: [-0.00, 0.00, -0.00, 1.00],
																						},
																						children	: ~[
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"L_thumb_ctrl",
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"L_indexF_ctrl",
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"L_middleF_ctrl",
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"L_ringF_ctrl",
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"L_pinkyF_ctrl",
																							})
																						],
																						name	: ~"L_palm_ctrl",
																					})
																				],
																				name	: ~"L_hand_zero",
																			})
																		],
																		name	: ~"L_hand_grp",
																	})
																],
																name	: ~"L_arm_IK_ctrl",
															})
														],
														name	: ~"L_arm_IK_ctrl_zero",
													})
												],
												name	: ~"L_arm_IK_ctrl_grp",
											}),
											ChildNode(Node{
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [0.00, 46.47, 1.11],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												children	: ~[],
												name	: ~"c_hips_cntr_backup",
											}),
											ChildNode(Node{
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [27.33, 60.54, -2.62],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															scale	: 1.00,
															pos	: [0.00, 0.00, 0.00],
															rot	: [-0.00, -0.00, 0.00, 1.00],
														},
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	scale	: 1.00,
																	pos	: [-27.33, -60.54, 2.62],
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																},
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [-54.83, -0.00, 0.00],
																			rot	: [0.00, 0.00, 0.00, 1.00],
																		},
																		children	: ~[],
																		name	: ~"ikHandle4",
																	}),
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [-108.05, 6.80, 11.25],
																			rot	: [0.07, -0.96, 0.26, 0.08],
																		},
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [0.00, -0.00, 0.00],
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, 0.00, -0.00],
																							rot	: [-0.07, 0.96, -0.26, 0.08],
																						},
																						children	: ~[
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"R_thumb_ctrl",
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"R_indexF_ctrl",
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"R_middleF_ctrl",
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"R_ringF_ctrl",
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																								},
																								children	: ~[],
																								name	: ~"R_pinkyF_ctrl",
																							})
																						],
																						name	: ~"R_palm_ctrl",
																					})
																				],
																				name	: ~"R_hand_zero",
																			})
																		],
																		name	: ~"R_hand_grp",
																	})
																],
																name	: ~"R_arm_IK_ctrl1",
															})
														],
														name	: ~"R_arm_IK_ctrl_zero",
													})
												],
												name	: ~"R_arm_IK_ctrl_grp",
											}),
											ChildNode(Node{
												space	: QuatSpace{
													scale	: 1.00,
													pos	: [0.00, 47.22, 0.24],
													rot	: [0.00, 0.00, 0.00, 1.00],
												},
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															scale	: 1.00,
															pos	: [0.00, -0.00, 0.00],
															rot	: [-0.00, 0.00, -0.00, 1.00],
														},
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	scale	: 1.00,
																	pos	: [-0.00, -47.22, -0.24],
																	rot	: [-0.00, 0.00, 0.00, 1.00],
																},
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [-0.00, -0.04, -0.10],
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																		},
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [0.00, -0.00, -0.00],
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [0.00, -0.00, 0.00],
																							rot	: [0.46, 0.54, -0.46, 0.54],
																						},
																						children	: ~[
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [-46.65, -7.08, 0.00],
																									rot	: [0.46, 0.54, -0.46, 0.54],
																								},
																								children	: ~[
																									ChildNode(Node{
																										space	: QuatSpace{
																											scale	: 1.00,
																											pos	: [-0.00, -0.00, 0.00],
																											rot	: [0.00, 0.00, -0.00, 1.00],
																										},
																										children	: ~[],
																										name	: ~"cluster3Handle",
																									}),
																									ChildNode(Node{
																										space	: QuatSpace{
																											scale	: 1.00,
																											pos	: [-0.00, -0.00, 0.00],
																											rot	: [0.00, 0.00, -0.00, 1.00],
																										},
																										children	: ~[],
																										name	: ~"cluster2Handle",
																									}),
																									ChildNode(Node{
																										space	: QuatSpace{
																											scale	: 1.00,
																											pos	: [-0.00, -0.00, 0.00],
																											rot	: [0.00, 0.00, -0.00, 1.00],
																										},
																										children	: ~[],
																										name	: ~"cluster1Handle",
																									})
																								],
																								name	: ~"group13",
																							})
																						],
																						name	: ~"c_hips_ctrl",
																					})
																				],
																				name	: ~"c_hips_ctrl_zero",
																			})
																		],
																		name	: ~"c_hips_ctrl_grp",
																	}),
																	ChildNode(Node{
																		space	: QuatSpace{
																			scale	: 1.00,
																			pos	: [-0.04, -0.00, 0.00],
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																		},
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					scale	: 1.00,
																					pos	: [0.00, -0.00, 0.00],
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							scale	: 1.00,
																							pos	: [-0.00, 0.00, 0.00],
																							rot	: [-0.00, -0.00, 0.08, 1.00],
																						},
																						children	: ~[
																							ChildNode(Node{
																								space	: QuatSpace{
																									scale	: 1.00,
																									pos	: [3.03, 0.02, -0.00],
																									rot	: [-0.00, 0.00, 0.11, 0.99],
																								},
																								children	: ~[
																									ChildNode(Node{
																										space	: QuatSpace{
																											scale	: 1.00,
																											pos	: [0.00, 0.00, 0.00],
																											rot	: [-0.00, 0.00, -0.00, 1.00],
																										},
																										children	: ~[
																											ChildNode(Node{
																												space	: QuatSpace{
																													scale	: 1.00,
																													pos	: [-50.12, 3.07, 0.00],
																													rot	: [-0.00, -0.00, -0.00, 1.00],
																												},
																												children	: ~[
																													ChildNode(Node{
																														space	: QuatSpace{
																															scale	: 1.00,
																															pos	: [-50.08, 3.75, -0.04],
																															rot	: [0.52, 0.48, -0.52, 0.48],
																														},
																														children	: ~[
																															ChildNode(Node{
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	pos	: [0.00, 0.00, 0.00],
																																	rot	: [0.00, 0.00, -0.00, 1.00],
																																},
																																children	: ~[],
																																name	: ~"cluster4Handle",
																															})
																														],
																														name	: ~"group14",
																													}),
																													ChildNode(Node{
																														space	: QuatSpace{
																															scale	: 1.00,
																															pos	: [2.95, 0.06, -0.00],
																															rot	: [0.00, 0.00, 0.13, 0.99],
																														},
																														children	: ~[
																															ChildNode(Node{
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	pos	: [0.00, 0.00, 0.00],
																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																},
																																children	: ~[
																																	ChildNode(Node{
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			pos	: [0.00, -0.00, -0.00],
																																			rot	: [0.57, 0.41, -0.57, 0.41],
																																		},
																																		children	: ~[
																																			ChildNode(Node{
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					pos	: [-50.19, 17.53, -0.04],
																																					rot	: [0.57, 0.41, -0.57, 0.41],
																																				},
																																				children	: ~[
																																					ChildNode(Node{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [-0.00, -0.00, 0.00],
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																						},
																																						children	: ~[],
																																						name	: ~"cluster6Handle",
																																					}),
																																					ChildNode(Node{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [-0.00, -0.00, 0.00],
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																						},
																																						children	: ~[],
																																						name	: ~"cluster5Handle",
																																					})
																																				],
																																				name	: ~"group12",
																																			}),
																																			ChildNode(Node{
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					pos	: [6.18, -0.00, -0.00],
																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																				},
																																				children	: ~[
																																					ChildNode(Node{
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							pos	: [0.00, 0.00, -0.00],
																																							rot	: [-0.00, -0.00, 0.00, 1.00],
																																						},
																																						children	: ~[
																																							ChildNode(Node{
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									pos	: [-0.00, 0.00, 0.00],
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																								},
																																								children	: ~[
																																									ChildNode(Node{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [-56.22, 18.02, -0.04],
																																											rot	: [0.58, 0.41, -0.58, 0.41],
																																										},
																																										children	: ~[
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [0.00, 0.00, -0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																												},
																																												children	: ~[],
																																												name	: ~"cluster9Handle",
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [0.00, 0.00, -0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																												},
																																												children	: ~[],
																																												name	: ~"cluster8Handle",
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [0.00, 0.00, -0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																												},
																																												children	: ~[],
																																												name	: ~"cluster7Handle",
																																											})
																																										],
																																										name	: ~"group11",
																																									}),
																																									ChildNode(Node{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [0.00, 0.00, 0.00],
																																											rot	: [-0.00, 0.00, 0.00, 1.00],
																																										},
																																										children	: ~[
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [0.00, -0.00, -0.00],
																																													rot	: [-0.00, 0.00, -0.00, 1.00],
																																												},
																																												children	: ~[],
																																												name	: ~"null1",
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [-2.31, -6.09, 4.23],
																																													rot	: [-0.10, -0.08, -0.84, 0.52],
																																												},
																																												children	: ~[
																																													ChildNode(Node{
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															pos	: [0.00, 0.00, 0.00],
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																														},
																																														children	: ~[
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	pos	: [0.00, 0.00, -0.00],
																																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [0.00, 0.00, -0.00],
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																		},
																																																		children	: ~[],
																																																		name	: ~"ikHandle2",
																																																	}),
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [0.00, 0.00, -0.00],
																																																			rot	: [0.03, -0.70, 0.06, 0.71],
																																																		},
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [-0.00, 0.00, -0.00],
																																																					rot	: [0.00, 0.00, -0.00, 1.00],
																																																				},
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [-0.00, 0.00, -0.00],
																																																							rot	: [-0.00, 0.00, 0.00, 1.00],
																																																						},
																																																						children	: ~[],
																																																						name	: ~"R_breastTweak_ctrl",
																																																					})
																																																				],
																																																				name	: ~"R_breastTweak_zero",
																																																			})
																																																		],
																																																		name	: ~"R_breastTweak_cntr_grp",
																																																	})
																																																],
																																																name	: ~"R_breast_IK_ctrl",
																																															})
																																														],
																																														name	: ~"R_breast_IK_cntr_zero",
																																													})
																																												],
																																												name	: ~"R_breast_IK_control_grp",
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [-2.31, -6.09, -4.23],
																																													rot	: [0.10, 0.08, -0.84, 0.52],
																																												},
																																												children	: ~[
																																													ChildNode(Node{
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															pos	: [0.00, 0.00, 0.00],
																																															rot	: [0.00, 0.00, 0.00, 1.00],
																																														},
																																														children	: ~[
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	pos	: [-0.00, 0.00, -0.00],
																																																	rot	: [-0.03, 0.70, 0.06, 0.71],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [0.00, 0.00, 0.00],
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																		},
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [0.00, -0.00, -0.00],
																																																					rot	: [-0.00, -0.00, -0.00, 1.00],
																																																				},
																																																				children	: ~[],
																																																				name	: ~"L_breastTweak_ctrl",
																																																			})
																																																		],
																																																		name	: ~"L_breastTweak_zero",
																																																	})
																																																],
																																																name	: ~"L_breastTweak_cntr_grp",
																																															}),
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	pos	: [-0.00, 0.00, -0.00],
																																																	rot	: [-0.00, 0.00, 0.00, 1.00],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [0.00, -0.00, 0.00],
																																																			rot	: [-0.00, 0.00, -0.00, 1.00],
																																																		},
																																																		children	: ~[],
																																																		name	: ~"ikHandle3",
																																																	})
																																																],
																																																name	: ~"L_breast_IK_ctrl",
																																															})
																																														],
																																														name	: ~"L_breast_IK_cntr_zero",
																																													})
																																												],
																																												name	: ~"L_breast_IK_control_grp",
																																											})
																																										],
																																										name	: ~"c_breastControls_grp",
																																									}),
																																									ChildNode(Node{
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											pos	: [6.82, -0.29, 0.00],
																																											rot	: [0.00, 0.00, -0.23, 0.97],
																																										},
																																										children	: ~[
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [-1.39, -3.03, 1.00],
																																													rot	: [-0.19, -0.65, 0.22, 0.71],
																																												},
																																												children	: ~[
																																													ChildNode(Node{
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															pos	: [0.00, -0.00, -0.00],
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																														},
																																														children	: ~[
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	pos	: [0.00, -0.00, 0.00],
																																																	rot	: [0.00, -0.12, 0.95, 0.28],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [4.89, -0.16, 0.38],
																																																			rot	: [0.13, 0.32, -0.21, 0.91],
																																																		},
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [-0.00, 0.00, 0.00],
																																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																																				},
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [0.00, 0.00, -0.00],
																																																							rot	: [0.00, 0.00, -0.03, 1.00],
																																																						},
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [2.32, 0.02, 0.51],
																																																									rot	: [-0.00, -0.30, 0.00, 0.96],
																																																								},
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 0.00, -0.00],
																																																											rot	: [-0.00, -0.00, 0.00, 1.00],
																																																										},
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													pos	: [0.00, 0.00, -0.00],
																																																													rot	: [-0.00, 0.00, -0.04, 1.00],
																																																												},
																																																												children	: ~[],
																																																												name	: ~"R_subSpaulder_ctrl",
																																																											})
																																																										],
																																																										name	: ~"R_subSpaulder_ctrl_zero",
																																																									})
																																																								],
																																																								name	: ~"R_subSpaulder_ctrl_grp",
																																																							})
																																																						],
																																																						name	: ~"R_mainSpaulder_ctrl",
																																																					})
																																																				],
																																																				name	: ~"R_mainSpaulder_ctrl_zero",
																																																			})
																																																		],
																																																		name	: ~"R_mainSpaulder_ctrl_grp",
																																																	})
																																																],
																																																name	: ~"R_clav_ctrl",
																																															})
																																														],
																																														name	: ~"R_clav_ctrl_zero",
																																													})
																																												],
																																												name	: ~"R_clav_ctrl_grp",
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [-1.39, -3.03, -1.00],
																																													rot	: [0.16, 0.64, 0.24, 0.71],
																																												},
																																												children	: ~[
																																													ChildNode(Node{
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															pos	: [0.00, 0.00, 0.00],
																																															rot	: [-0.00, -0.00, -0.00, 1.00],
																																														},
																																														children	: ~[
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	pos	: [0.00, 0.00, -0.00],
																																																	rot	: [0.00, -0.01, -0.28, 0.96],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [4.89, -0.12, -0.39],
																																																			rot	: [-0.09, -0.32, -0.22, 0.92],
																																																		},
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [0.00, -0.00, 0.00],
																																																					rot	: [0.00, 0.00, 0.00, 1.00],
																																																				},
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [0.00, -0.00, -0.00],
																																																							rot	: [-0.00, -0.00, -0.03, 1.00],
																																																						},
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [2.32, 0.02, -0.51],
																																																									rot	: [0.00, 0.30, 0.00, 0.96],
																																																								},
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [-0.00, 0.00, -0.00],
																																																											rot	: [-0.00, -0.00, -0.00, 1.00],
																																																										},
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													pos	: [0.00, 0.00, -0.00],
																																																													rot	: [-0.00, -0.00, -0.04, 1.00],
																																																												},
																																																												children	: ~[],
																																																												name	: ~"L_subSpaulder_ctrl",
																																																											})
																																																										],
																																																										name	: ~"L_subSpaulder_ctrl_zero",
																																																									})
																																																								],
																																																								name	: ~"L_subSpaulder_ctrl_grp",
																																																							})
																																																						],
																																																						name	: ~"L_mainSpaulder_ctrl",
																																																					})
																																																				],
																																																				name	: ~"L_mainSpaulder_ctrl_zero",
																																																			})
																																																		],
																																																		name	: ~"L_mainSpaulder_ctrl_grp",
																																																	})
																																																],
																																																name	: ~"L_clav_ctrl",
																																															})
																																														],
																																														name	: ~"L_clav_ctrl_zero",
																																													})
																																												],
																																												name	: ~"L_clav_ctrl_grp",
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													pos	: [0.00, 0.00, 0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																												},
																																												children	: ~[
																																													ChildNode(Node{
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															pos	: [0.00, 0.00, -0.00],
																																															rot	: [-0.00, -0.00, 0.06, 1.00],
																																														},
																																														children	: ~[
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	pos	: [2.33, 0.01, -0.00],
																																																	rot	: [0.00, 0.00, 0.02, 1.00],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			pos	: [0.00, 0.00, -0.00],
																																																			rot	: [-0.00, -0.00, -0.00, 1.00],
																																																		},
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					pos	: [0.00, 0.00, -0.00],
																																																					rot	: [-0.00, -0.00, 0.04, 1.00],
																																																				},
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							pos	: [2.37, 0.00, 0.00],
																																																							rot	: [-0.00, -0.00, -0.00, 1.00],
																																																						},
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									pos	: [0.00, 0.00, 0.00],
																																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																																								},
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											pos	: [0.00, 0.00, 0.00],
																																																											rot	: [-0.00, -0.00, 0.05, 1.00],
																																																										},
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													pos	: [-0.41, -0.55, 0.00],
																																																													rot	: [0.00, 0.00, -0.82, 0.57],
																																																												},
																																																												children	: ~[
																																																													ChildNode(Node{
																																																														space	: QuatSpace{
																																																															scale	: 1.00,
																																																															pos	: [-0.00, 0.00, 0.00],
																																																															rot	: [-0.00, 0.00, -0.00, 1.00],
																																																														},
																																																														children	: ~[
																																																															ChildNode(Node{
																																																																space	: QuatSpace{
																																																																	scale	: 1.00,
																																																																	pos	: [0.00, -0.00, 0.00],
																																																																	rot	: [-0.00, 0.00, -0.00, 1.00],
																																																																},
																																																																children	: ~[],
																																																																name	: ~"c_jaw_ctrl",
																																																															})
																																																														],
																																																														name	: ~"c_jaw_ctrl_zero",
																																																													})
																																																												],
																																																												name	: ~"c_jaw_ctrl_grp",
																																																											})
																																																										],
																																																										name	: ~"c_neck_03_ctrl",
																																																									})
																																																								],
																																																								name	: ~"c_neck_03_ctrl_zero",
																																																							})
																																																						],
																																																						name	: ~"c_neck_03_ctrl_grp",
																																																					})
																																																				],
																																																				name	: ~"c_neck_02_ctrl",
																																																			})
																																																		],
																																																		name	: ~"c_neck_02_ctrl_zero",
																																																	})
																																																],
																																																name	: ~"c_neck_02_ctrl_grp",
																																															})
																																														],
																																														name	: ~"c_neck_01_ctrl",
																																													})
																																												],
																																												name	: ~"c_neck_01_ctrl_zero",
																																											})
																																										],
																																										name	: ~"c_neck_01_ctrl_grp",
																																									})
																																								],
																																								name	: ~"c_chest_ctrl",
																																							})
																																						],
																																						name	: ~"c_chest_ctrl_zero",
																																					})
																																				],
																																				name	: ~"c_chest_ctrl_grp",
																																			})
																																		],
																																		name	: ~"c_spine_05_ctrl",
																																	})
																																],
																																name	: ~"c_spine_05_ctrl_zero",
																															})
																														],
																														name	: ~"c_spine_05_ctrl_grp",
																													})
																												],
																												name	: ~"c_spine_03_ctrl",
																											})
																										],
																										name	: ~"c_spine_03_ctrl_zero",
																									})
																								],
																								name	: ~"c_spine_03_ctrl_grp",
																							})
																						],
																						name	: ~"c_spine_01_ctrl",
																					})
																				],
																				name	: ~"c_spine_01_ctrl_zero",
																			})
																		],
																		name	: ~"c_spine_01_ctrl_grp",
																	})
																],
																name	: ~"c_cog_ctrl",
															})
														],
														name	: ~"c_cog_ctrl_zero",
													})
												],
												name	: ~"c_cog_ctrl_grp",
											})
										],
										name	: ~"c_worldTransform_ctrl",
									})
								],
								name	: ~"Controls",
							})
						],
						name	: ~"Transform",
					}),
					ChildNode(Node{
						space	: QuatSpace{
							scale	: 1.00,
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
						},
						children	: ~[
							ChildNode(Node{
								space	: QuatSpace{
									scale	: 1.00,
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								children	: ~[
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Tongue",
												mesh	: ~"tongue_geo1Shape@",
												armature	: ~"",
												range	: [0, 528],
											})
										],
										name	: ~"tongue_geo1",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"cloak",
												mesh	: ~"polySurfaceShape174@",
												armature	: ~"",
												range	: [0, 15252],
											})
										],
										name	: ~"polySurface172",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Teeth",
												mesh	: ~"topJaw_geo2Shape@",
												armature	: ~"",
												range	: [0, 4656],
											})
										],
										name	: ~"topJaw_geo2",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Teeth",
												mesh	: ~"lowerJaw_geo2Shape@",
												armature	: ~"",
												range	: [0, 4248],
											})
										],
										name	: ~"lowerJaw_geo2",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												mesh	: ~"L_upper_lash1Shape@",
												armature	: ~"",
												range	: [0, 13716],
											})
										],
										name	: ~"L_upper_lash1",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												mesh	: ~"L_lower_lash1Shape@",
												armature	: ~"",
												range	: [0, 8964],
											})
										],
										name	: ~"L_lower_lash1",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												mesh	: ~"R_upper_lash1Shape@",
												armature	: ~"",
												range	: [0, 13716],
											})
										],
										name	: ~"R_upper_lash1",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												mesh	: ~"R_lower_lash1Shape@",
												armature	: ~"",
												range	: [0, 8964],
											})
										],
										name	: ~"R_lower_lash1",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Eyes",
												mesh	: ~"L_eye_geo1Shape@",
												armature	: ~"",
												range	: [0, 2784],
											}),
											ChildEntity(Entity{
												material	: ~"Pupil_SS",
												mesh	: ~"L_eye_geo1Shape@",
												armature	: ~"",
												range	: [2784, 3264],
											}),
											ChildEntity(Entity{
												material	: ~"cornea",
												mesh	: ~"L_eye_geo1Shape@",
												armature	: ~"",
												range	: [3264, 5568],
											})
										],
										name	: ~"L_eye_geo1",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Eyes",
												mesh	: ~"R_eye_geo1Shape@",
												armature	: ~"",
												range	: [0, 2784],
											}),
											ChildEntity(Entity{
												material	: ~"Pupil_SS",
												mesh	: ~"R_eye_geo1Shape@",
												armature	: ~"",
												range	: [2784, 3264],
											}),
											ChildEntity(Entity{
												material	: ~"cornea",
												mesh	: ~"R_eye_geo1Shape@",
												armature	: ~"",
												range	: [3264, 5568],
											})
										],
										name	: ~"R_eye_geo1",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"anisotropic1",
												mesh	: ~"Hair_Geo2Shape@",
												armature	: ~"",
												range	: [0, 6954],
											})
										],
										name	: ~"Hair_Geo2",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"skin",
												mesh	: ~"Body_geo8Shape@",
												armature	: ~"",
												range	: [0, 50496],
											})
										],
										name	: ~"Body_geo8",
									})
								],
								name	: ~"Body",
							}),
							ChildNode(Node{
								space	: QuatSpace{
									scale	: 1.00,
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								children	: ~[
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, -0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												mesh	: ~"R_boot1Shape@",
												armature	: ~"",
												range	: [0, 9042],
											})
										],
										name	: ~"boots",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												mesh	: ~"backShealth1Shape@",
												armature	: ~"",
												range	: [0, 5550],
											})
										],
										name	: ~"backShealth1",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												mesh	: ~"R_skirt_06Shape@",
												armature	: ~"",
												range	: [0, 10236],
											}),
											ChildEntity(Entity{
												material	: ~"skin",
												mesh	: ~"R_skirt_06Shape@",
												armature	: ~"",
												range	: [10236, 12102],
											})
										],
										name	: ~"skirt",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"skin",
												mesh	: ~"bracket_05_geo1Shape@",
												armature	: ~"",
												range	: [0, 8448],
											})
										],
										name	: ~"bracket",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												mesh	: ~"L_bracer1Shape@",
												armature	: ~"",
												range	: [0, 2304],
											})
										],
										name	: ~"bracers",
									}),
									ChildNode(Node{
										space	: QuatSpace{
											scale	: 1.00,
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												mesh	: ~"R_subSpaulder1Shape@",
												armature	: ~"",
												range	: [0, 6960],
											})
										],
										name	: ~"spaulders",
									})
								],
								name	: ~"Armor",
							}),
							ChildNode(Node{
								space	: QuatSpace{
									scale	: 1.00,
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
								},
								children	: ~[],
								name	: ~"Eyes_Geo",
							})
						],
						name	: ~"noTrasnform",
					})
				],
				name	: ~"Clare",
			}),
			ChildNode(Node{
				space	: QuatSpace{
					scale	: 1.00,
					pos	: [43.55, 25.15, 80.51],
					rot	: [0.27, 0.31, 0.78, 0.47],
				},
				children	: ~[
					ChildLight(Light{
						attenuation	: [0.00, 1.00],
						kind	: KindSpot(Spot{
							size	: 1.31,
							blend	: 0.15,
						}),
						spherical	: false,
						name	: ~"Lamp",
						energy	: 3.00,
						distance	: 100.00,
						color	: [1.00, 1.00, 1.00],
					})
				],
				name	: ~"Lamp",
			})
		],
	}}
