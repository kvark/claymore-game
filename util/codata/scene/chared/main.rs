use common::*;
pub fn load()-> Scene	{Scene{
		materials	: ~[
			Material{
				name	: ~"anisotropic1",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.09, 0.09, 0.08])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[],
			},
			Material{
				name	: ~"armor",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[
					Texture{
						offset	: [0.00, 0.00, 0.00],
						name	: ~"Main",
						scale	: [1.00, 1.00, 1.00],
						filter	: 3,
						path	: ~"//Metal_R-Spec_G-Bump_B-Reflect.jpg",
						wrap	: 0,
					}
				],
			},
			Material{
				name	: ~"cloak",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[],
			},
			Material{
				name	: ~"cornea",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.50, 0.50, 0.50])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[],
			},
			Material{
				name	: ~"EyeLashes",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[],
			},
			Material{
				name	: ~"Eyes",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.44, 0.44, 0.54])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.49, 0.49, 0.49])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[],
			},
			Material{
				name	: ~"Material",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[],
			},
			Material{
				name	: ~"Pupil_SS",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[],
			},
			Material{
				name	: ~"skin",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[
					Texture{
						offset	: [0.00, 0.00, 0.00],
						name	: ~"Main.001",
						scale	: [1.00, 1.00, 1.00],
						filter	: 3,
						path	: ~"//Skin_Diffuse.jpg",
						wrap	: 0,
					}
				],
			},
			Material{
				name	: ~"Teeth",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.75, 0.75, 0.75])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[],
			},
			Material{
				name	: ~"Tongue",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.40, 0.08, 0.08])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.50, 0.50, 0.50])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				kind	: KindPhong,
				textures	: ~[],
			}
		],
		global	: Global{
			gravity	: [0.00, 0.00, -9.81],
		},
		nodes	: ~[
			ChildNode(Node{
				space	: QuatSpace{
					pos	: [0.00, 0.00, -1.00],
					rot	: [0.00, 0.00, 0.00, 1.00],
					scale	: 100.00,
				},
				name	: ~"Plane",
				children	: ~[
					ChildEntity(Entity{
						mesh	: ~"Plane@",
						material	: ~"Material",
						range	: [0, 6],
						armature	: ~"",
					})
				],
			}),
			ChildNode(Node{
				space	: QuatSpace{
					pos	: [140.00, 0.00, 90.00],
					rot	: [0.41, 0.41, 0.58, 0.58],
					scale	: 1.00,
				},
				name	: ~"Camera",
				children	: ~[
					ChildCamera(Camera{
						name	: ~"Camera",
						range	: [10.00, 300.00],
						fov_y	: 0.87,
					})
				],
			}),
			ChildNode(Node{
				space	: QuatSpace{
					pos	: [0.00, 0.00, 0.00],
					rot	: [0.50, 0.50, 0.50, 0.50],
					scale	: 1.00,
				},
				name	: ~"Clare",
				children	: ~[
					ChildNode(Node{
						space	: QuatSpace{
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
							scale	: 1.00,
						},
						name	: ~"R_ik_foot_grp",
						children	: ~[
							ChildNode(Node{
								space	: QuatSpace{
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
									scale	: 1.00,
								},
								name	: ~"ikHandle8",
								children	: ~[],
							})
						],
					}),
					ChildNode(Node{
						space	: QuatSpace{
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
							scale	: 1.00,
						},
						name	: ~"L_leg_ikHandle_zero.001",
						children	: ~[],
					}),
					ChildNode(Node{
						space	: QuatSpace{
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
							scale	: 1.00,
						},
						name	: ~"L_ik_foot_grp",
						children	: ~[
							ChildNode(Node{
								space	: QuatSpace{
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
									scale	: 1.00,
								},
								name	: ~"ikHandle7",
								children	: ~[],
							})
						],
					}),
					ChildNode(Node{
						space	: QuatSpace{
							pos	: [-0.00, -1.11, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
							scale	: 1.00,
						},
						name	: ~"Transform",
						children	: ~[
							ChildNode(Node{
								space	: QuatSpace{
									pos	: [0.00, 1.11, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
									scale	: 1.00,
								},
								name	: ~"Controls",
								children	: ~[
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"c_worldTransform_ctrl",
										children	: ~[
											ChildNode(Node{
												space	: QuatSpace{
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
													scale	: 1.00,
												},
												name	: ~"Armature.002",
												children	: ~[],
											}),
											ChildNode(Node{
												space	: QuatSpace{
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
													scale	: 1.00,
												},
												name	: ~"SKELETON",
												children	: ~[],
											}),
											ChildNode(Node{
												space	: QuatSpace{
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
													scale	: 1.00,
												},
												name	: ~"c_eye_ctrl_grp",
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															pos	: [-0.04, 71.88, 17.98],
															rot	: [0.00, 0.00, 0.00, 1.00],
															scale	: 1.00,
														},
														name	: ~"mainEye_ctrl_zero",
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	pos	: [0.04, -71.88, -17.98],
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	scale	: 1.00,
																},
																name	: ~"mainEye_ctrl",
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [-0.95, -0.00, 0.00],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			scale	: 0.72,
																		},
																		name	: ~"R_eye_ctrl_grp",
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					pos	: [-0.00, -0.00, 0.00],
																					rot	: [0.00, 0.00, -0.00, 1.00],
																					scale	: 1.00,
																				},
																				name	: ~"R_eye_ctrl_zero",
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							pos	: [0.00, 0.00, -0.00],
																							rot	: [0.00, -0.00, -0.00, 1.00],
																							scale	: 1.00,
																						},
																						name	: ~"R_eye_ctrl",
																						children	: ~[],
																					})
																				],
																			})
																		],
																	}),
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [0.95, 0.00, -0.00],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			scale	: 0.72,
																		},
																		name	: ~"L_eye_ctrl_grp",
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					pos	: [0.00, 0.00, -0.00],
																					rot	: [0.00, 0.00, -0.00, 1.00],
																					scale	: 1.00,
																				},
																				name	: ~"L_eye_ctrl_zero",
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							pos	: [0.00, 0.00, -0.00],
																							rot	: [0.00, 0.00, -0.00, 1.00],
																							scale	: 1.00,
																						},
																						name	: ~"L_eye_ctrl",
																						children	: ~[],
																					})
																				],
																			})
																		],
																	})
																],
															})
														],
													})
												],
											}),
											ChildNode(Node{
												space	: QuatSpace{
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
													scale	: 1.00,
												},
												name	: ~"Locators",
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															pos	: [1.26, 71.88, 0.51],
															rot	: [0.00, 0.00, 0.00, 1.00],
															scale	: 1.00,
														},
														name	: ~"L_eye_centerLocator",
														children	: ~[],
													}),
													ChildNode(Node{
														space	: QuatSpace{
															pos	: [-1.26, 71.88, 0.51],
															rot	: [0.00, 0.00, 0.00, 1.00],
															scale	: 1.00,
														},
														name	: ~"R_eye_centerLocator",
														children	: ~[],
													})
												],
											}),
											ChildNode(Node{
												space	: QuatSpace{
													pos	: [0.00, 0.00, 0.00],
													rot	: [0.00, 0.00, 0.00, 1.00],
													scale	: 1.00,
												},
												name	: ~"LegControls",
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															pos	: [3.00, -0.31, -3.16],
															rot	: [0.00, 0.00, 0.00, 1.00],
															scale	: 1.00,
														},
														name	: ~"L_leg_ikHandle_grp",
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	pos	: [0.00, -0.00, 0.00],
																	rot	: [-0.00, -0.00, -0.00, 1.00],
																	scale	: 1.00,
																},
																name	: ~"L_leg_ikHandle_zero",
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [-0.00, 0.00, 0.00],
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																			scale	: 1.00,
																		},
																		name	: ~"L_foot_ik_ctrl",
																		children	: ~[],
																	})
																],
															})
														],
													}),
													ChildNode(Node{
														space	: QuatSpace{
															pos	: [-3.00, -0.31, -3.16],
															rot	: [0.00, 0.00, 0.00, 1.00],
															scale	: 1.00,
														},
														name	: ~"R_leg_ikHandle_grp",
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	pos	: [-0.00, -0.00, 0.00],
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	scale	: 1.00,
																},
																name	: ~"R_leg_ikHandle_zero",
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [0.00, 0.00, 0.00],
																			rot	: [-0.01, -0.00, 0.00, 1.00],
																			scale	: 1.00,
																		},
																		name	: ~"R_foot_ik_ctrl",
																		children	: ~[],
																	})
																],
															})
														],
													}),
													ChildNode(Node{
														space	: QuatSpace{
															pos	: [-4.14, 43.04, -0.16],
															rot	: [0.00, 0.00, 0.00, 1.00],
															scale	: 1.00,
														},
														name	: ~"R_legPole_ctrl",
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	pos	: [0.57, -18.67, 10.59],
																	rot	: [0.00, -0.00, -0.00, 1.00],
																	scale	: 1.00,
																},
																name	: ~"R_legPole_ctrl_zero",
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [0.00, 0.00, 0.00],
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																			scale	: 1.00,
																		},
																		name	: ~"R_legPole_ctrl.001",
																		children	: ~[],
																	})
																],
															})
														],
													}),
													ChildNode(Node{
														space	: QuatSpace{
															pos	: [4.10, 42.26, 0.61],
															rot	: [-0.47, 0.53, -0.49, 0.51],
															scale	: 1.00,
														},
														name	: ~"L_legPole_ctrl_cons",
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	pos	: [0.00, 0.00, -0.00],
																	rot	: [0.47, -0.53, 0.49, 0.51],
																	scale	: 1.00,
																},
																name	: ~"L_legPole_ctrl_grp",
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [-0.55, -17.93, 9.74],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			scale	: 1.00,
																		},
																		name	: ~"L_legPole_cntr_zero",
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					pos	: [0.00, -0.00, -0.00],
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																					scale	: 1.00,
																				},
																				name	: ~"L_legPole_ctrl",
																				children	: ~[],
																			})
																		],
																	})
																],
															})
														],
													})
												],
											}),
											ChildNode(Node{
												space	: QuatSpace{
													pos	: [27.41, 60.54, -2.62],
													rot	: [-0.26, -0.08, -0.07, 0.96],
													scale	: 1.00,
												},
												name	: ~"L_arm_IK_ctrl_grp",
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															pos	: [0.00, -0.00, 0.00],
															rot	: [0.26, 0.08, 0.07, 0.96],
															scale	: 1.00,
														},
														name	: ~"L_arm_IK_ctrl_zero",
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	pos	: [-27.41, -60.54, 2.62],
																	rot	: [0.00, -0.00, 0.00, 1.00],
																	scale	: 1.00,
																},
																name	: ~"L_arm_IK_ctrl",
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [-0.00, 0.00, -0.00],
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			scale	: 1.00,
																		},
																		name	: ~"L_armIK_handle",
																		children	: ~[],
																	}),
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [-0.00, 0.00, -0.00],
																			rot	: [-0.26, -0.08, -0.07, 0.96],
																			scale	: 1.00,
																		},
																		name	: ~"L_hand_grp",
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					pos	: [-0.00, 0.00, -0.00],
																					rot	: [0.00, -0.00, 0.00, 1.00],
																					scale	: 1.00,
																				},
																				name	: ~"L_hand_zero",
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							pos	: [0.00, -0.00, 0.00],
																							rot	: [-0.00, 0.00, -0.00, 1.00],
																							scale	: 1.00,
																						},
																						name	: ~"L_palm_ctrl",
																						children	: ~[
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									scale	: 1.00,
																								},
																								name	: ~"L_thumb_ctrl",
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									scale	: 1.00,
																								},
																								name	: ~"L_indexF_ctrl",
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									scale	: 1.00,
																								},
																								name	: ~"L_middleF_ctrl",
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									scale	: 1.00,
																								},
																								name	: ~"L_ringF_ctrl",
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-0.00, -0.00, -0.00],
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									scale	: 1.00,
																								},
																								name	: ~"L_pinkyF_ctrl",
																								children	: ~[],
																							})
																						],
																					})
																				],
																			})
																		],
																	})
																],
															})
														],
													})
												],
											}),
											ChildNode(Node{
												space	: QuatSpace{
													pos	: [0.00, 46.47, 1.11],
													rot	: [0.00, 0.00, 0.00, 1.00],
													scale	: 1.00,
												},
												name	: ~"c_hips_cntr_backup",
												children	: ~[],
											}),
											ChildNode(Node{
												space	: QuatSpace{
													pos	: [27.33, 60.54, -2.62],
													rot	: [0.00, 0.00, 0.00, 1.00],
													scale	: 1.00,
												},
												name	: ~"R_arm_IK_ctrl_grp",
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															pos	: [0.00, 0.00, 0.00],
															rot	: [-0.00, -0.00, 0.00, 1.00],
															scale	: 1.00,
														},
														name	: ~"R_arm_IK_ctrl_zero",
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	pos	: [-27.33, -60.54, 2.62],
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	scale	: 1.00,
																},
																name	: ~"R_arm_IK_ctrl1",
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [-54.83, -0.00, 0.00],
																			rot	: [0.00, 0.00, 0.00, 1.00],
																			scale	: 1.00,
																		},
																		name	: ~"ikHandle4",
																		children	: ~[],
																	}),
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [-108.05, 6.80, 11.25],
																			rot	: [0.07, -0.96, 0.26, 0.08],
																			scale	: 1.00,
																		},
																		name	: ~"R_hand_grp",
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					pos	: [0.00, -0.00, 0.00],
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																					scale	: 1.00,
																				},
																				name	: ~"R_hand_zero",
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							pos	: [0.00, 0.00, -0.00],
																							rot	: [-0.07, 0.96, -0.26, 0.08],
																							scale	: 1.00,
																						},
																						name	: ~"R_palm_ctrl",
																						children	: ~[
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									scale	: 1.00,
																								},
																								name	: ~"R_thumb_ctrl",
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									scale	: 1.00,
																								},
																								name	: ~"R_indexF_ctrl",
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									scale	: 1.00,
																								},
																								name	: ~"R_middleF_ctrl",
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									scale	: 1.00,
																								},
																								name	: ~"R_ringF_ctrl",
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-0.00, 0.00, -0.00],
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									scale	: 1.00,
																								},
																								name	: ~"R_pinkyF_ctrl",
																								children	: ~[],
																							})
																						],
																					})
																				],
																			})
																		],
																	})
																],
															})
														],
													})
												],
											}),
											ChildNode(Node{
												space	: QuatSpace{
													pos	: [0.00, 47.22, 0.24],
													rot	: [0.00, 0.00, 0.00, 1.00],
													scale	: 1.00,
												},
												name	: ~"c_cog_ctrl_grp",
												children	: ~[
													ChildNode(Node{
														space	: QuatSpace{
															pos	: [0.00, -0.00, 0.00],
															rot	: [-0.00, 0.00, -0.00, 1.00],
															scale	: 1.00,
														},
														name	: ~"c_cog_ctrl_zero",
														children	: ~[
															ChildNode(Node{
																space	: QuatSpace{
																	pos	: [-0.00, -47.22, -0.24],
																	rot	: [-0.00, 0.00, 0.00, 1.00],
																	scale	: 1.00,
																},
																name	: ~"c_cog_ctrl",
																children	: ~[
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [-0.00, -0.04, -0.10],
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			scale	: 1.00,
																		},
																		name	: ~"c_hips_ctrl_grp",
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					pos	: [0.00, -0.00, -0.00],
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																					scale	: 1.00,
																				},
																				name	: ~"c_hips_ctrl_zero",
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							pos	: [0.00, -0.00, 0.00],
																							rot	: [0.46, 0.54, -0.46, 0.54],
																							scale	: 1.00,
																						},
																						name	: ~"c_hips_ctrl",
																						children	: ~[
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [-46.65, -7.08, 0.00],
																									rot	: [0.46, 0.54, -0.46, 0.54],
																									scale	: 1.00,
																								},
																								name	: ~"group13",
																								children	: ~[
																									ChildNode(Node{
																										space	: QuatSpace{
																											pos	: [-0.00, -0.00, 0.00],
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											scale	: 1.00,
																										},
																										name	: ~"cluster3Handle",
																										children	: ~[],
																									}),
																									ChildNode(Node{
																										space	: QuatSpace{
																											pos	: [-0.00, -0.00, 0.00],
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											scale	: 1.00,
																										},
																										name	: ~"cluster2Handle",
																										children	: ~[],
																									}),
																									ChildNode(Node{
																										space	: QuatSpace{
																											pos	: [-0.00, -0.00, 0.00],
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											scale	: 1.00,
																										},
																										name	: ~"cluster1Handle",
																										children	: ~[],
																									})
																								],
																							})
																						],
																					})
																				],
																			})
																		],
																	}),
																	ChildNode(Node{
																		space	: QuatSpace{
																			pos	: [-0.04, -0.00, 0.00],
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			scale	: 1.00,
																		},
																		name	: ~"c_spine_01_ctrl_grp",
																		children	: ~[
																			ChildNode(Node{
																				space	: QuatSpace{
																					pos	: [0.00, -0.00, 0.00],
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																					scale	: 1.00,
																				},
																				name	: ~"c_spine_01_ctrl_zero",
																				children	: ~[
																					ChildNode(Node{
																						space	: QuatSpace{
																							pos	: [-0.00, 0.00, 0.00],
																							rot	: [-0.00, -0.00, 0.08, 1.00],
																							scale	: 1.00,
																						},
																						name	: ~"c_spine_01_ctrl",
																						children	: ~[
																							ChildNode(Node{
																								space	: QuatSpace{
																									pos	: [3.03, 0.02, -0.00],
																									rot	: [-0.00, 0.00, 0.11, 0.99],
																									scale	: 1.00,
																								},
																								name	: ~"c_spine_03_ctrl_grp",
																								children	: ~[
																									ChildNode(Node{
																										space	: QuatSpace{
																											pos	: [0.00, 0.00, 0.00],
																											rot	: [-0.00, 0.00, -0.00, 1.00],
																											scale	: 1.00,
																										},
																										name	: ~"c_spine_03_ctrl_zero",
																										children	: ~[
																											ChildNode(Node{
																												space	: QuatSpace{
																													pos	: [-50.12, 3.07, 0.00],
																													rot	: [-0.00, -0.00, -0.00, 1.00],
																													scale	: 1.00,
																												},
																												name	: ~"c_spine_03_ctrl",
																												children	: ~[
																													ChildNode(Node{
																														space	: QuatSpace{
																															pos	: [-50.08, 3.75, -0.04],
																															rot	: [0.52, 0.48, -0.52, 0.48],
																															scale	: 1.00,
																														},
																														name	: ~"group14",
																														children	: ~[
																															ChildNode(Node{
																																space	: QuatSpace{
																																	pos	: [0.00, 0.00, 0.00],
																																	rot	: [0.00, 0.00, -0.00, 1.00],
																																	scale	: 1.00,
																																},
																																name	: ~"cluster4Handle",
																																children	: ~[],
																															})
																														],
																													}),
																													ChildNode(Node{
																														space	: QuatSpace{
																															pos	: [2.95, 0.06, -0.00],
																															rot	: [0.00, 0.00, 0.13, 0.99],
																															scale	: 1.00,
																														},
																														name	: ~"c_spine_05_ctrl_grp",
																														children	: ~[
																															ChildNode(Node{
																																space	: QuatSpace{
																																	pos	: [0.00, 0.00, 0.00],
																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																	scale	: 1.00,
																																},
																																name	: ~"c_spine_05_ctrl_zero",
																																children	: ~[
																																	ChildNode(Node{
																																		space	: QuatSpace{
																																			pos	: [0.00, -0.00, -0.00],
																																			rot	: [0.57, 0.41, -0.57, 0.41],
																																			scale	: 1.00,
																																		},
																																		name	: ~"c_spine_05_ctrl",
																																		children	: ~[
																																			ChildNode(Node{
																																				space	: QuatSpace{
																																					pos	: [-50.19, 17.53, -0.04],
																																					rot	: [0.57, 0.41, -0.57, 0.41],
																																					scale	: 1.00,
																																				},
																																				name	: ~"group12",
																																				children	: ~[
																																					ChildNode(Node{
																																						space	: QuatSpace{
																																							pos	: [-0.00, -0.00, 0.00],
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																							scale	: 1.00,
																																						},
																																						name	: ~"cluster6Handle",
																																						children	: ~[],
																																					}),
																																					ChildNode(Node{
																																						space	: QuatSpace{
																																							pos	: [-0.00, -0.00, 0.00],
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																							scale	: 1.00,
																																						},
																																						name	: ~"cluster5Handle",
																																						children	: ~[],
																																					})
																																				],
																																			}),
																																			ChildNode(Node{
																																				space	: QuatSpace{
																																					pos	: [6.18, -0.00, -0.00],
																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																					scale	: 1.00,
																																				},
																																				name	: ~"c_chest_ctrl_grp",
																																				children	: ~[
																																					ChildNode(Node{
																																						space	: QuatSpace{
																																							pos	: [0.00, 0.00, -0.00],
																																							rot	: [-0.00, -0.00, 0.00, 1.00],
																																							scale	: 1.00,
																																						},
																																						name	: ~"c_chest_ctrl_zero",
																																						children	: ~[
																																							ChildNode(Node{
																																								space	: QuatSpace{
																																									pos	: [-0.00, 0.00, 0.00],
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																									scale	: 1.00,
																																								},
																																								name	: ~"c_chest_ctrl",
																																								children	: ~[
																																									ChildNode(Node{
																																										space	: QuatSpace{
																																											pos	: [-56.22, 18.02, -0.04],
																																											rot	: [0.58, 0.41, -0.58, 0.41],
																																											scale	: 1.00,
																																										},
																																										name	: ~"group11",
																																										children	: ~[
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													pos	: [0.00, 0.00, -0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													scale	: 1.00,
																																												},
																																												name	: ~"cluster9Handle",
																																												children	: ~[],
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													pos	: [0.00, 0.00, -0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													scale	: 1.00,
																																												},
																																												name	: ~"cluster8Handle",
																																												children	: ~[],
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													pos	: [0.00, 0.00, -0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													scale	: 1.00,
																																												},
																																												name	: ~"cluster7Handle",
																																												children	: ~[],
																																											})
																																										],
																																									}),
																																									ChildNode(Node{
																																										space	: QuatSpace{
																																											pos	: [0.00, 0.00, 0.00],
																																											rot	: [-0.00, 0.00, 0.00, 1.00],
																																											scale	: 1.00,
																																										},
																																										name	: ~"c_breastControls_grp",
																																										children	: ~[
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													pos	: [0.00, -0.00, -0.00],
																																													rot	: [-0.00, 0.00, -0.00, 1.00],
																																													scale	: 1.00,
																																												},
																																												name	: ~"null1",
																																												children	: ~[],
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													pos	: [-2.31, -6.09, 4.23],
																																													rot	: [-0.10, -0.08, -0.84, 0.52],
																																													scale	: 1.00,
																																												},
																																												name	: ~"R_breast_IK_control_grp",
																																												children	: ~[
																																													ChildNode(Node{
																																														space	: QuatSpace{
																																															pos	: [0.00, 0.00, 0.00],
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																															scale	: 1.00,
																																														},
																																														name	: ~"R_breast_IK_cntr_zero",
																																														children	: ~[
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	pos	: [0.00, 0.00, -0.00],
																																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																																	scale	: 1.00,
																																																},
																																																name	: ~"R_breast_IK_ctrl",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			pos	: [0.00, 0.00, -0.00],
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																			scale	: 1.00,
																																																		},
																																																		name	: ~"ikHandle2",
																																																		children	: ~[],
																																																	}),
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			pos	: [0.00, 0.00, -0.00],
																																																			rot	: [0.03, -0.70, 0.06, 0.71],
																																																			scale	: 1.00,
																																																		},
																																																		name	: ~"R_breastTweak_cntr_grp",
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				space	: QuatSpace{
																																																					pos	: [-0.00, 0.00, -0.00],
																																																					rot	: [0.00, 0.00, -0.00, 1.00],
																																																					scale	: 1.00,
																																																				},
																																																				name	: ~"R_breastTweak_zero",
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						space	: QuatSpace{
																																																							pos	: [-0.00, 0.00, -0.00],
																																																							rot	: [-0.00, 0.00, 0.00, 1.00],
																																																							scale	: 1.00,
																																																						},
																																																						name	: ~"R_breastTweak_ctrl",
																																																						children	: ~[],
																																																					})
																																																				],
																																																			})
																																																		],
																																																	})
																																																],
																																															})
																																														],
																																													})
																																												],
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													pos	: [-2.31, -6.09, -4.23],
																																													rot	: [0.10, 0.08, -0.84, 0.52],
																																													scale	: 1.00,
																																												},
																																												name	: ~"L_breast_IK_control_grp",
																																												children	: ~[
																																													ChildNode(Node{
																																														space	: QuatSpace{
																																															pos	: [0.00, 0.00, 0.00],
																																															rot	: [0.00, 0.00, 0.00, 1.00],
																																															scale	: 1.00,
																																														},
																																														name	: ~"L_breast_IK_cntr_zero",
																																														children	: ~[
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	pos	: [-0.00, 0.00, -0.00],
																																																	rot	: [-0.03, 0.70, 0.06, 0.71],
																																																	scale	: 1.00,
																																																},
																																																name	: ~"L_breastTweak_cntr_grp",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			pos	: [0.00, 0.00, 0.00],
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																			scale	: 1.00,
																																																		},
																																																		name	: ~"L_breastTweak_zero",
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				space	: QuatSpace{
																																																					pos	: [0.00, -0.00, -0.00],
																																																					rot	: [-0.00, -0.00, -0.00, 1.00],
																																																					scale	: 1.00,
																																																				},
																																																				name	: ~"L_breastTweak_ctrl",
																																																				children	: ~[],
																																																			})
																																																		],
																																																	})
																																																],
																																															}),
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	pos	: [-0.00, 0.00, -0.00],
																																																	rot	: [-0.00, 0.00, 0.00, 1.00],
																																																	scale	: 1.00,
																																																},
																																																name	: ~"L_breast_IK_ctrl",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			pos	: [0.00, -0.00, 0.00],
																																																			rot	: [-0.00, 0.00, -0.00, 1.00],
																																																			scale	: 1.00,
																																																		},
																																																		name	: ~"ikHandle3",
																																																		children	: ~[],
																																																	})
																																																],
																																															})
																																														],
																																													})
																																												],
																																											})
																																										],
																																									}),
																																									ChildNode(Node{
																																										space	: QuatSpace{
																																											pos	: [6.82, -0.29, 0.00],
																																											rot	: [0.00, 0.00, -0.23, 0.97],
																																											scale	: 1.00,
																																										},
																																										name	: ~"c_neck_01_ctrl_grp",
																																										children	: ~[
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													pos	: [-1.39, -3.03, 1.00],
																																													rot	: [-0.19, -0.65, 0.22, 0.71],
																																													scale	: 1.00,
																																												},
																																												name	: ~"R_clav_ctrl_grp",
																																												children	: ~[
																																													ChildNode(Node{
																																														space	: QuatSpace{
																																															pos	: [0.00, -0.00, -0.00],
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																															scale	: 1.00,
																																														},
																																														name	: ~"R_clav_ctrl_zero",
																																														children	: ~[
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	pos	: [0.00, -0.00, 0.00],
																																																	rot	: [0.00, -0.12, 0.95, 0.28],
																																																	scale	: 1.00,
																																																},
																																																name	: ~"R_clav_ctrl",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			pos	: [4.89, -0.16, 0.38],
																																																			rot	: [0.13, 0.32, -0.21, 0.91],
																																																			scale	: 1.00,
																																																		},
																																																		name	: ~"R_mainSpaulder_ctrl_grp",
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				space	: QuatSpace{
																																																					pos	: [-0.00, 0.00, 0.00],
																																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																																					scale	: 1.00,
																																																				},
																																																				name	: ~"R_mainSpaulder_ctrl_zero",
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						space	: QuatSpace{
																																																							pos	: [0.00, 0.00, -0.00],
																																																							rot	: [0.00, 0.00, -0.03, 1.00],
																																																							scale	: 1.00,
																																																						},
																																																						name	: ~"R_mainSpaulder_ctrl",
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								space	: QuatSpace{
																																																									pos	: [2.32, 0.02, 0.51],
																																																									rot	: [-0.00, -0.30, 0.00, 0.96],
																																																									scale	: 1.00,
																																																								},
																																																								name	: ~"R_subSpaulder_ctrl_grp",
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										space	: QuatSpace{
																																																											pos	: [-0.00, 0.00, -0.00],
																																																											rot	: [-0.00, -0.00, 0.00, 1.00],
																																																											scale	: 1.00,
																																																										},
																																																										name	: ~"R_subSpaulder_ctrl_zero",
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												space	: QuatSpace{
																																																													pos	: [0.00, 0.00, -0.00],
																																																													rot	: [-0.00, 0.00, -0.04, 1.00],
																																																													scale	: 1.00,
																																																												},
																																																												name	: ~"R_subSpaulder_ctrl",
																																																												children	: ~[],
																																																											})
																																																										],
																																																									})
																																																								],
																																																							})
																																																						],
																																																					})
																																																				],
																																																			})
																																																		],
																																																	})
																																																],
																																															})
																																														],
																																													})
																																												],
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													pos	: [-1.39, -3.03, -1.00],
																																													rot	: [0.16, 0.64, 0.24, 0.71],
																																													scale	: 1.00,
																																												},
																																												name	: ~"L_clav_ctrl_grp",
																																												children	: ~[
																																													ChildNode(Node{
																																														space	: QuatSpace{
																																															pos	: [0.00, 0.00, 0.00],
																																															rot	: [-0.00, -0.00, -0.00, 1.00],
																																															scale	: 1.00,
																																														},
																																														name	: ~"L_clav_ctrl_zero",
																																														children	: ~[
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	pos	: [0.00, 0.00, -0.00],
																																																	rot	: [0.00, -0.01, -0.28, 0.96],
																																																	scale	: 1.00,
																																																},
																																																name	: ~"L_clav_ctrl",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			pos	: [4.89, -0.12, -0.39],
																																																			rot	: [-0.09, -0.32, -0.22, 0.92],
																																																			scale	: 1.00,
																																																		},
																																																		name	: ~"L_mainSpaulder_ctrl_grp",
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				space	: QuatSpace{
																																																					pos	: [0.00, -0.00, 0.00],
																																																					rot	: [0.00, 0.00, 0.00, 1.00],
																																																					scale	: 1.00,
																																																				},
																																																				name	: ~"L_mainSpaulder_ctrl_zero",
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						space	: QuatSpace{
																																																							pos	: [0.00, -0.00, -0.00],
																																																							rot	: [-0.00, -0.00, -0.03, 1.00],
																																																							scale	: 1.00,
																																																						},
																																																						name	: ~"L_mainSpaulder_ctrl",
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								space	: QuatSpace{
																																																									pos	: [2.32, 0.02, -0.51],
																																																									rot	: [0.00, 0.30, 0.00, 0.96],
																																																									scale	: 1.00,
																																																								},
																																																								name	: ~"L_subSpaulder_ctrl_grp",
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										space	: QuatSpace{
																																																											pos	: [-0.00, 0.00, -0.00],
																																																											rot	: [-0.00, -0.00, -0.00, 1.00],
																																																											scale	: 1.00,
																																																										},
																																																										name	: ~"L_subSpaulder_ctrl_zero",
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												space	: QuatSpace{
																																																													pos	: [0.00, 0.00, -0.00],
																																																													rot	: [-0.00, -0.00, -0.04, 1.00],
																																																													scale	: 1.00,
																																																												},
																																																												name	: ~"L_subSpaulder_ctrl",
																																																												children	: ~[],
																																																											})
																																																										],
																																																									})
																																																								],
																																																							})
																																																						],
																																																					})
																																																				],
																																																			})
																																																		],
																																																	})
																																																],
																																															})
																																														],
																																													})
																																												],
																																											}),
																																											ChildNode(Node{
																																												space	: QuatSpace{
																																													pos	: [0.00, 0.00, 0.00],
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													scale	: 1.00,
																																												},
																																												name	: ~"c_neck_01_ctrl_zero",
																																												children	: ~[
																																													ChildNode(Node{
																																														space	: QuatSpace{
																																															pos	: [0.00, 0.00, -0.00],
																																															rot	: [-0.00, -0.00, 0.06, 1.00],
																																															scale	: 1.00,
																																														},
																																														name	: ~"c_neck_01_ctrl",
																																														children	: ~[
																																															ChildNode(Node{
																																																space	: QuatSpace{
																																																	pos	: [2.33, 0.01, -0.00],
																																																	rot	: [0.00, 0.00, 0.02, 1.00],
																																																	scale	: 1.00,
																																																},
																																																name	: ~"c_neck_02_ctrl_grp",
																																																children	: ~[
																																																	ChildNode(Node{
																																																		space	: QuatSpace{
																																																			pos	: [0.00, 0.00, -0.00],
																																																			rot	: [-0.00, -0.00, -0.00, 1.00],
																																																			scale	: 1.00,
																																																		},
																																																		name	: ~"c_neck_02_ctrl_zero",
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				space	: QuatSpace{
																																																					pos	: [0.00, 0.00, -0.00],
																																																					rot	: [-0.00, -0.00, 0.04, 1.00],
																																																					scale	: 1.00,
																																																				},
																																																				name	: ~"c_neck_02_ctrl",
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						space	: QuatSpace{
																																																							pos	: [2.37, 0.00, 0.00],
																																																							rot	: [-0.00, -0.00, -0.00, 1.00],
																																																							scale	: 1.00,
																																																						},
																																																						name	: ~"c_neck_03_ctrl_grp",
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								space	: QuatSpace{
																																																									pos	: [0.00, 0.00, 0.00],
																																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																																									scale	: 1.00,
																																																								},
																																																								name	: ~"c_neck_03_ctrl_zero",
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										space	: QuatSpace{
																																																											pos	: [0.00, 0.00, 0.00],
																																																											rot	: [-0.00, -0.00, 0.05, 1.00],
																																																											scale	: 1.00,
																																																										},
																																																										name	: ~"c_neck_03_ctrl",
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												space	: QuatSpace{
																																																													pos	: [-0.41, -0.55, 0.00],
																																																													rot	: [0.00, 0.00, -0.82, 0.57],
																																																													scale	: 1.00,
																																																												},
																																																												name	: ~"c_jaw_ctrl_grp",
																																																												children	: ~[
																																																													ChildNode(Node{
																																																														space	: QuatSpace{
																																																															pos	: [-0.00, 0.00, 0.00],
																																																															rot	: [-0.00, 0.00, -0.00, 1.00],
																																																															scale	: 1.00,
																																																														},
																																																														name	: ~"c_jaw_ctrl_zero",
																																																														children	: ~[
																																																															ChildNode(Node{
																																																																space	: QuatSpace{
																																																																	pos	: [0.00, -0.00, 0.00],
																																																																	rot	: [-0.00, 0.00, -0.00, 1.00],
																																																																	scale	: 1.00,
																																																																},
																																																																name	: ~"c_jaw_ctrl",
																																																																children	: ~[],
																																																															})
																																																														],
																																																													})
																																																												],
																																																											})
																																																										],
																																																									})
																																																								],
																																																							})
																																																						],
																																																					})
																																																				],
																																																			})
																																																		],
																																																	})
																																																],
																																															})
																																														],
																																													})
																																												],
																																											})
																																										],
																																									})
																																								],
																																							})
																																						],
																																					})
																																				],
																																			})
																																		],
																																	})
																																],
																															})
																														],
																													})
																												],
																											})
																										],
																									})
																								],
																							})
																						],
																					})
																				],
																			})
																		],
																	})
																],
															})
														],
													})
												],
											})
										],
									})
								],
							})
						],
					}),
					ChildNode(Node{
						space	: QuatSpace{
							pos	: [0.00, 0.00, 0.00],
							rot	: [0.00, 0.00, 0.00, 1.00],
							scale	: 1.00,
						},
						name	: ~"noTrasnform",
						children	: ~[
							ChildNode(Node{
								space	: QuatSpace{
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
									scale	: 1.00,
								},
								name	: ~"Body",
								children	: ~[
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"tongue_geo1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"tongue_geo1Shape@",
												material	: ~"Tongue",
												range	: [0, 528],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"polySurface172",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"polySurfaceShape174@",
												material	: ~"cloak",
												range	: [0, 15252],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"topJaw_geo2",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"topJaw_geo2Shape@",
												material	: ~"Teeth",
												range	: [0, 4656],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"lowerJaw_geo2",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"lowerJaw_geo2Shape@",
												material	: ~"Teeth",
												range	: [0, 4248],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"L_upper_lash1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"L_upper_lash1Shape@",
												material	: ~"EyeLashes",
												range	: [0, 13716],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"L_lower_lash1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"L_lower_lash1Shape@",
												material	: ~"EyeLashes",
												range	: [0, 8964],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"R_upper_lash1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_upper_lash1Shape@",
												material	: ~"EyeLashes",
												range	: [0, 13716],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"R_lower_lash1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_lower_lash1Shape@",
												material	: ~"EyeLashes",
												range	: [0, 8964],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"L_eye_geo1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"L_eye_geo1Shape@",
												material	: ~"Eyes",
												range	: [0, 2784],
												armature	: ~"",
											}),
											ChildEntity(Entity{
												mesh	: ~"L_eye_geo1Shape@",
												material	: ~"Pupil_SS",
												range	: [2784, 3264],
												armature	: ~"",
											}),
											ChildEntity(Entity{
												mesh	: ~"L_eye_geo1Shape@",
												material	: ~"cornea",
												range	: [3264, 5568],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"R_eye_geo1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_eye_geo1Shape@",
												material	: ~"Eyes",
												range	: [0, 2784],
												armature	: ~"",
											}),
											ChildEntity(Entity{
												mesh	: ~"R_eye_geo1Shape@",
												material	: ~"Pupil_SS",
												range	: [2784, 3264],
												armature	: ~"",
											}),
											ChildEntity(Entity{
												mesh	: ~"R_eye_geo1Shape@",
												material	: ~"cornea",
												range	: [3264, 5568],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"Hair_Geo2",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"Hair_Geo2Shape@",
												material	: ~"anisotropic1",
												range	: [0, 6954],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"Body_geo8",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"Body_geo8Shape@",
												material	: ~"skin",
												range	: [0, 50496],
												armature	: ~"",
											})
										],
									})
								],
							}),
							ChildNode(Node{
								space	: QuatSpace{
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
									scale	: 1.00,
								},
								name	: ~"Armor",
								children	: ~[
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, -0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"boots",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_boot1Shape@",
												material	: ~"armor",
												range	: [0, 9042],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"backShealth1",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"backShealth1Shape@",
												material	: ~"armor",
												range	: [0, 5550],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"skirt",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_skirt_06Shape@",
												material	: ~"armor",
												range	: [0, 10236],
												armature	: ~"",
											}),
											ChildEntity(Entity{
												mesh	: ~"R_skirt_06Shape@",
												material	: ~"skin",
												range	: [10236, 12102],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"bracket",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"bracket_05_geo1Shape@",
												material	: ~"skin",
												range	: [0, 8448],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"bracers",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"L_bracer1Shape@",
												material	: ~"armor",
												range	: [0, 2304],
												armature	: ~"",
											})
										],
									}),
									ChildNode(Node{
										space	: QuatSpace{
											pos	: [0.00, 0.00, 0.00],
											rot	: [0.00, 0.00, 0.00, 1.00],
											scale	: 1.00,
										},
										name	: ~"spaulders",
										children	: ~[
											ChildEntity(Entity{
												mesh	: ~"R_subSpaulder1Shape@",
												material	: ~"armor",
												range	: [0, 6960],
												armature	: ~"",
											})
										],
									})
								],
							}),
							ChildNode(Node{
								space	: QuatSpace{
									pos	: [0.00, 0.00, 0.00],
									rot	: [0.00, 0.00, 0.00, 1.00],
									scale	: 1.00,
								},
								name	: ~"Eyes_Geo",
								children	: ~[],
							})
						],
					})
				],
			}),
			ChildNode(Node{
				space	: QuatSpace{
					pos	: [43.55, 25.15, 80.51],
					rot	: [0.27, 0.31, 0.78, 0.47],
					scale	: 1.00,
				},
				name	: ~"Lamp",
				children	: ~[
					ChildLight(Light{
						name	: ~"Lamp",
						attenuation	: [0.00, 1.00],
						kind	: KindSpot(Spot{
							size	: 1.31,
							blend	: 0.15,
						}),
						distance	: 100.00,
						spherical	: false,
						energy	: 3.00,
						color	: [1.00, 1.00, 1.00],
					})
				],
			})
		],
	}}
