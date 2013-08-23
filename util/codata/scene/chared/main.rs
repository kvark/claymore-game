use common::*;
pub fn load()-> Scene	{
	Scene{
		materials	: ~[
			Material{
				textures	: ~[],
				name	: ~"anisotropic1",
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.09, 0.09, 0.08]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[1.00, 1.00, 1.00]),
					DataVector(	~"SpecularParams",	[0.50, 50.00, 0.00, 1.00])
				],
			},
			Material{
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
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.80, 0.80, 0.80]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[1.00, 1.00, 1.00]),
					DataVector(	~"SpecularParams",	[0.50, 50.00, 0.00, 1.00])
				],
			},
			Material{
				textures	: ~[],
				name	: ~"cloak",
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.80, 0.80, 0.80]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[1.00, 1.00, 1.00]),
					DataVector(	~"SpecularParams",	[0.50, 50.00, 0.00, 1.00])
				],
			},
			Material{
				textures	: ~[],
				name	: ~"cornea",
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.00, 0.00, 0.00]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[0.50, 0.50, 0.50]),
					DataVector(	~"SpecularParams",	[0.50, 1.00, 0.00, 1.00])
				],
			},
			Material{
				textures	: ~[],
				name	: ~"EyeLashes",
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.00, 0.00, 0.00]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[1.00, 1.00, 1.00]),
					DataVector(	~"SpecularParams",	[0.50, 50.00, 0.00, 1.00])
				],
			},
			Material{
				textures	: ~[],
				name	: ~"Eyes",
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.44, 0.44, 0.54]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[0.49, 0.49, 0.49]),
					DataVector(	~"SpecularParams",	[0.50, 1.00, 0.00, 1.00])
				],
			},
			Material{
				textures	: ~[],
				name	: ~"Material",
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.80, 0.80, 0.80]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[1.00, 1.00, 1.00]),
					DataVector(	~"SpecularParams",	[0.50, 50.00, 0.00, 1.00])
				],
			},
			Material{
				textures	: ~[],
				name	: ~"Pupil_SS",
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.00, 0.00, 0.00]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[1.00, 1.00, 1.00]),
					DataVector(	~"SpecularParams",	[0.50, 50.00, 0.00, 1.00])
				],
			},
			Material{
				textures	: ~[
					Texture{
						wrap	: 0,
						name	: ~"Main",
						offset	: [0.00, 0.00, 0.00],
						filter	: 3,
						scale	: [1.00, 1.00, 1.00],
						path	: ~"//Skin_Diffuse.jpg",
					}
				],
				name	: ~"skin",
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.80, 0.80, 0.80]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[1.00, 1.00, 1.00]),
					DataVector(	~"SpecularParams",	[0.50, 1.00, 0.00, 1.00])
				],
			},
			Material{
				textures	: ~[],
				name	: ~"Teeth",
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.75, 0.75, 0.75]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[1.00, 1.00, 1.00]),
					DataVector(	~"SpecularParams",	[0.50, 50.00, 0.00, 1.00])
				],
			},
			Material{
				textures	: ~[],
				name	: ~"Tongue",
				kind	: KindPhong,
				data	: ~[
					DataScalar(	~"Ambient",	1.00),
					DataColor(	~"DiffuseColor",	[0.40, 0.08, 0.08]),
					DataVector(	~"DiffuseParams",	[0.80, 0.00, 0.00, 1.00]),
					DataColor(	~"SpecularColor",	[0.50, 0.50, 0.50]),
					DataVector(	~"SpecularParams",	[0.50, 1.00, 0.00, 1.00])
				],
			}
		],
		nodes	: ~[
			ChildNode(Node{
				name	: ~"Plane",
				space	: QuatSpace{
					scale	: 100.00,
					rot	: [1.00, 0.00, 0.00, 0.00],
					pos	: [0.00, 0.00, -1.00],
				},
				children	: ~[
					ChildEntity(Entity{
						material	: ~"Material",
						armature	: ~"",
						range	: [0, 6],
						mesh	: ~"Plane@",
					})
				],
			}),
			ChildNode(Node{
				name	: ~"Camera",
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.58, 0.41, 0.41, 0.58],
					pos	: [140.00, 0.00, 90.00],
				},
				children	: ~[
					ChildCamera(Camera{
						name	: ~"Camera",
						range	: [10.00, 300.00],
						fov_y	: 0.87,
					})
				],
			}),
			ChildNode(Node{
				name	: ~"Clare",
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.50, 0.50, 0.50, 0.50],
					pos	: [0.00, 0.00, 0.00],
				},
				children	: ~[
					ChildNode(Node{
						name	: ~"R_ik_foot_grp",
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [1.00, 0.00, 0.00, 0.00],
							pos	: [0.00, 0.00, 0.00],
						},
						children	: ~[
							ChildNode(Node{
								name	: ~"ikHandle8",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [1.00, 0.00, 0.00, 0.00],
									pos	: [0.00, 0.00, 0.00],
								},
								children	: ~[],
							})
						],
					}),
					ChildNode(Node{
						name	: ~"L_leg_ikHandle_zero.001",
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [1.00, 0.00, 0.00, 0.00],
							pos	: [0.00, 0.00, 0.00],
						},
						children	: ~[],
					}),
					ChildNode(Node{
						name	: ~"L_ik_foot_grp",
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [1.00, 0.00, 0.00, 0.00],
							pos	: [0.00, 0.00, 0.00],
						},
						children	: ~[
							ChildNode(Node{
								name	: ~"ikHandle7",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [1.00, 0.00, 0.00, 0.00],
									pos	: [0.00, 0.00, 0.00],
								},
								children	: ~[],
							})
						],
					}),
					ChildNode(Node{
						name	: ~"Transform",
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [1.00, 0.00, 0.00, 0.00],
							pos	: [-0.00, -1.11, 0.00],
						},
						children	: ~[
							ChildNode(Node{
								name	: ~"Controls",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [1.00, 0.00, 0.00, 0.00],
									pos	: [0.00, 1.11, 0.00],
								},
								children	: ~[
									ChildNode(Node{
										name	: ~"c_worldTransform_ctrl",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildNode(Node{
												name	: ~"Armature.002",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [1.00, 0.00, 0.00, 0.00],
													pos	: [0.00, 0.00, 0.00],
												},
												children	: ~[],
											}),
											ChildNode(Node{
												name	: ~"SKELETON",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [1.00, 0.00, 0.00, 0.00],
													pos	: [0.00, 0.00, 0.00],
												},
												children	: ~[],
											}),
											ChildNode(Node{
												name	: ~"c_eye_ctrl_grp",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [1.00, 0.00, 0.00, 0.00],
													pos	: [0.00, 0.00, 0.00],
												},
												children	: ~[
													ChildNode(Node{
														name	: ~"mainEye_ctrl_zero",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [1.00, 0.00, 0.00, 0.00],
															pos	: [-0.04, 71.88, 17.98],
														},
														children	: ~[
															ChildNode(Node{
																name	: ~"mainEye_ctrl",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [1.00, -0.00, -0.00, 0.00],
																	pos	: [0.04, -71.88, -17.98],
																},
																children	: ~[
																	ChildNode(Node{
																		name	: ~"R_eye_ctrl_grp",
																		space	: QuatSpace{
																			scale	: 0.72,
																			rot	: [1.00, 0.00, 0.00, -0.00],
																			pos	: [-0.95, -0.00, 0.00],
																		},
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"R_eye_ctrl_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [1.00, 0.00, 0.00, -0.00],
																					pos	: [-0.00, -0.00, 0.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"R_eye_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [1.00, 0.00, -0.00, -0.00],
																							pos	: [0.00, 0.00, -0.00],
																						},
																						children	: ~[],
																					})
																				],
																			})
																		],
																	}),
																	ChildNode(Node{
																		name	: ~"L_eye_ctrl_grp",
																		space	: QuatSpace{
																			scale	: 0.72,
																			rot	: [1.00, 0.00, 0.00, -0.00],
																			pos	: [0.95, 0.00, -0.00],
																		},
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"L_eye_ctrl_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [1.00, 0.00, 0.00, -0.00],
																					pos	: [0.00, 0.00, -0.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"L_eye_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [1.00, 0.00, 0.00, -0.00],
																							pos	: [0.00, 0.00, -0.00],
																						},
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
												name	: ~"Locators",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [1.00, 0.00, 0.00, 0.00],
													pos	: [0.00, 0.00, 0.00],
												},
												children	: ~[
													ChildNode(Node{
														name	: ~"L_eye_centerLocator",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [1.00, 0.00, 0.00, 0.00],
															pos	: [1.26, 71.88, 0.51],
														},
														children	: ~[],
													}),
													ChildNode(Node{
														name	: ~"R_eye_centerLocator",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [1.00, 0.00, 0.00, 0.00],
															pos	: [-1.26, 71.88, 0.51],
														},
														children	: ~[],
													})
												],
											}),
											ChildNode(Node{
												name	: ~"LegControls",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [1.00, 0.00, 0.00, 0.00],
													pos	: [0.00, 0.00, 0.00],
												},
												children	: ~[
													ChildNode(Node{
														name	: ~"L_leg_ikHandle_grp",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [1.00, 0.00, 0.00, 0.00],
															pos	: [3.00, -0.31, -3.16],
														},
														children	: ~[
															ChildNode(Node{
																name	: ~"L_leg_ikHandle_zero",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [1.00, -0.00, -0.00, -0.00],
																	pos	: [0.00, -0.00, 0.00],
																},
																children	: ~[
																	ChildNode(Node{
																		name	: ~"L_foot_ik_ctrl",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [1.00, -0.00, -0.00, 0.00],
																			pos	: [-0.00, 0.00, 0.00],
																		},
																		children	: ~[],
																	})
																],
															})
														],
													}),
													ChildNode(Node{
														name	: ~"R_leg_ikHandle_grp",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [1.00, 0.00, 0.00, 0.00],
															pos	: [-3.00, -0.31, -3.16],
														},
														children	: ~[
															ChildNode(Node{
																name	: ~"R_leg_ikHandle_zero",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [1.00, -0.00, -0.00, 0.00],
																	pos	: [-0.00, -0.00, 0.00],
																},
																children	: ~[
																	ChildNode(Node{
																		name	: ~"R_foot_ik_ctrl",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [1.00, -0.01, -0.00, 0.00],
																			pos	: [0.00, 0.00, 0.00],
																		},
																		children	: ~[],
																	})
																],
															})
														],
													}),
													ChildNode(Node{
														name	: ~"R_legPole_ctrl",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [1.00, 0.00, 0.00, 0.00],
															pos	: [-4.14, 43.04, -0.16],
														},
														children	: ~[
															ChildNode(Node{
																name	: ~"R_legPole_ctrl_zero",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [1.00, 0.00, -0.00, -0.00],
																	pos	: [0.57, -18.67, 10.59],
																},
																children	: ~[
																	ChildNode(Node{
																		name	: ~"R_legPole_ctrl.001",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [1.00, -0.00, -0.00, 0.00],
																			pos	: [0.00, 0.00, 0.00],
																		},
																		children	: ~[],
																	})
																],
															})
														],
													}),
													ChildNode(Node{
														name	: ~"L_legPole_ctrl_cons",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.51, -0.47, 0.53, -0.49],
															pos	: [4.10, 42.26, 0.61],
														},
														children	: ~[
															ChildNode(Node{
																name	: ~"L_legPole_ctrl_grp",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.51, 0.47, -0.53, 0.49],
																	pos	: [0.00, 0.00, -0.00],
																},
																children	: ~[
																	ChildNode(Node{
																		name	: ~"L_legPole_cntr_zero",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [1.00, 0.00, 0.00, -0.00],
																			pos	: [-0.55, -17.93, 9.74],
																		},
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"L_legPole_ctrl",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [1.00, -0.00, 0.00, 0.00],
																					pos	: [0.00, -0.00, -0.00],
																				},
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
												name	: ~"L_arm_IK_ctrl_grp",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.96, -0.26, -0.08, -0.07],
													pos	: [27.41, 60.54, -2.62],
												},
												children	: ~[
													ChildNode(Node{
														name	: ~"L_arm_IK_ctrl_zero",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.96, 0.26, 0.08, 0.07],
															pos	: [0.00, -0.00, 0.00],
														},
														children	: ~[
															ChildNode(Node{
																name	: ~"L_arm_IK_ctrl",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [1.00, 0.00, -0.00, 0.00],
																	pos	: [-27.41, -60.54, 2.62],
																},
																children	: ~[
																	ChildNode(Node{
																		name	: ~"L_armIK_handle",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [1.00, 0.00, 0.00, -0.00],
																			pos	: [-0.00, 0.00, -0.00],
																		},
																		children	: ~[],
																	}),
																	ChildNode(Node{
																		name	: ~"L_hand_grp",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.96, -0.26, -0.08, -0.07],
																			pos	: [-0.00, 0.00, -0.00],
																		},
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"L_hand_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [1.00, 0.00, -0.00, 0.00],
																					pos	: [-0.00, 0.00, -0.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"L_palm_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [1.00, -0.00, 0.00, -0.00],
																							pos	: [0.00, -0.00, 0.00],
																						},
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"L_thumb_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [1.00, 0.00, 0.00, -0.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								name	: ~"L_indexF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [1.00, 0.00, 0.00, -0.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								name	: ~"L_middleF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [1.00, 0.00, 0.00, -0.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								name	: ~"L_ringF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [1.00, 0.00, 0.00, -0.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								name	: ~"L_pinkyF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [1.00, 0.00, 0.00, -0.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
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
												name	: ~"c_hips_cntr_backup",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [1.00, 0.00, 0.00, 0.00],
													pos	: [0.00, 46.47, 1.11],
												},
												children	: ~[],
											}),
											ChildNode(Node{
												name	: ~"R_arm_IK_ctrl_grp",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [1.00, 0.00, 0.00, 0.00],
													pos	: [27.33, 60.54, -2.62],
												},
												children	: ~[
													ChildNode(Node{
														name	: ~"R_arm_IK_ctrl_zero",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [1.00, -0.00, -0.00, 0.00],
															pos	: [0.00, 0.00, 0.00],
														},
														children	: ~[
															ChildNode(Node{
																name	: ~"R_arm_IK_ctrl1",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [1.00, -0.00, -0.00, 0.00],
																	pos	: [-27.33, -60.54, 2.62],
																},
																children	: ~[
																	ChildNode(Node{
																		name	: ~"ikHandle4",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [1.00, 0.00, 0.00, 0.00],
																			pos	: [-54.83, -0.00, 0.00],
																		},
																		children	: ~[],
																	}),
																	ChildNode(Node{
																		name	: ~"R_hand_grp",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.08, 0.07, -0.96, 0.26],
																			pos	: [-108.05, 6.80, 11.25],
																		},
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"R_hand_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [1.00, -0.00, 0.00, 0.00],
																					pos	: [0.00, -0.00, 0.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"R_palm_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.08, -0.07, 0.96, -0.26],
																							pos	: [0.00, 0.00, -0.00],
																						},
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"R_thumb_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [1.00, -0.00, -0.00, 0.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								name	: ~"R_indexF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [1.00, -0.00, -0.00, 0.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								name	: ~"R_middleF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [1.00, -0.00, -0.00, 0.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								name	: ~"R_ringF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [1.00, -0.00, -0.00, 0.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																								children	: ~[],
																							}),
																							ChildNode(Node{
																								name	: ~"R_pinkyF_ctrl",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [1.00, -0.00, -0.00, 0.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
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
												name	: ~"c_cog_ctrl_grp",
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [1.00, 0.00, 0.00, 0.00],
													pos	: [0.00, 47.22, 0.24],
												},
												children	: ~[
													ChildNode(Node{
														name	: ~"c_cog_ctrl_zero",
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [1.00, -0.00, 0.00, -0.00],
															pos	: [0.00, -0.00, 0.00],
														},
														children	: ~[
															ChildNode(Node{
																name	: ~"c_cog_ctrl",
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [1.00, -0.00, 0.00, 0.00],
																	pos	: [-0.00, -47.22, -0.24],
																},
																children	: ~[
																	ChildNode(Node{
																		name	: ~"c_hips_ctrl_grp",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.54, -0.46, -0.54, 0.46],
																			pos	: [-0.00, -0.04, -0.10],
																		},
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"c_hips_ctrl_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [1.00, -0.00, -0.00, -0.00],
																					pos	: [0.00, -0.00, -0.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"c_hips_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.54, 0.46, 0.54, -0.46],
																							pos	: [0.00, -0.00, 0.00],
																						},
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"group13",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.54, 0.46, 0.54, -0.46],
																									pos	: [-46.65, -7.08, 0.00],
																								},
																								children	: ~[
																									ChildNode(Node{
																										name	: ~"cluster3Handle",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [1.00, 0.00, 0.00, -0.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																										children	: ~[],
																									}),
																									ChildNode(Node{
																										name	: ~"cluster2Handle",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [1.00, 0.00, 0.00, -0.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																										children	: ~[],
																									}),
																									ChildNode(Node{
																										name	: ~"cluster1Handle",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [1.00, 0.00, 0.00, -0.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
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
																		name	: ~"c_spine_01_ctrl_grp",
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.54, -0.46, -0.54, 0.46],
																			pos	: [-0.04, -0.00, 0.00],
																		},
																		children	: ~[
																			ChildNode(Node{
																				name	: ~"c_spine_01_ctrl_zero",
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [1.00, -0.00, -0.00, -0.00],
																					pos	: [0.00, -0.00, 0.00],
																				},
																				children	: ~[
																					ChildNode(Node{
																						name	: ~"c_spine_01_ctrl",
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [1.00, -0.00, -0.00, 0.08],
																							pos	: [-0.00, 0.00, 0.00],
																						},
																						children	: ~[
																							ChildNode(Node{
																								name	: ~"c_spine_03_ctrl_grp",
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.99, -0.00, 0.00, 0.11],
																									pos	: [3.03, 0.02, -0.00],
																								},
																								children	: ~[
																									ChildNode(Node{
																										name	: ~"c_spine_03_ctrl_zero",
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [1.00, -0.00, 0.00, -0.00],
																											pos	: [0.00, 0.00, 0.00],
																										},
																										children	: ~[
																											ChildNode(Node{
																												name	: ~"c_spine_03_ctrl",
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [1.00, -0.00, -0.00, -0.00],
																													pos	: [-50.12, 3.07, 0.00],
																												},
																												children	: ~[
																													ChildNode(Node{
																														name	: ~"group14",
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.48, 0.52, 0.48, -0.52],
																															pos	: [-50.08, 3.75, -0.04],
																														},
																														children	: ~[
																															ChildNode(Node{
																																name	: ~"cluster4Handle",
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [1.00, 0.00, 0.00, -0.00],
																																	pos	: [0.00, 0.00, 0.00],
																																},
																																children	: ~[],
																															})
																														],
																													}),
																													ChildNode(Node{
																														name	: ~"c_spine_05_ctrl_grp",
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.99, 0.00, 0.00, 0.13],
																															pos	: [2.95, 0.06, -0.00],
																														},
																														children	: ~[
																															ChildNode(Node{
																																name	: ~"c_spine_05_ctrl_zero",
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [1.00, -0.00, -0.00, -0.00],
																																	pos	: [0.00, 0.00, 0.00],
																																},
																																children	: ~[
																																	ChildNode(Node{
																																		name	: ~"c_spine_05_ctrl",
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.41, 0.57, 0.41, -0.57],
																																			pos	: [0.00, -0.00, -0.00],
																																		},
																																		children	: ~[
																																			ChildNode(Node{
																																				name	: ~"group12",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.41, 0.57, 0.41, -0.57],
																																					pos	: [-50.19, 17.53, -0.04],
																																				},
																																				children	: ~[
																																					ChildNode(Node{
																																						name	: ~"cluster6Handle",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [1.00, 0.00, -0.00, -0.00],
																																							pos	: [-0.00, -0.00, 0.00],
																																						},
																																						children	: ~[],
																																					}),
																																					ChildNode(Node{
																																						name	: ~"cluster5Handle",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [1.00, 0.00, -0.00, -0.00],
																																							pos	: [-0.00, -0.00, 0.00],
																																						},
																																						children	: ~[],
																																					})
																																				],
																																			}),
																																			ChildNode(Node{
																																				name	: ~"c_chest_ctrl_grp",
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [1.00, -0.00, -0.00, 0.00],
																																					pos	: [6.18, -0.00, -0.00],
																																				},
																																				children	: ~[
																																					ChildNode(Node{
																																						name	: ~"c_chest_ctrl_zero",
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [1.00, -0.00, -0.00, 0.00],
																																							pos	: [0.00, 0.00, -0.00],
																																						},
																																						children	: ~[
																																							ChildNode(Node{
																																								name	: ~"c_chest_ctrl",
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [1.00, -0.00, -0.00, -0.00],
																																									pos	: [-0.00, 0.00, 0.00],
																																								},
																																								children	: ~[
																																									ChildNode(Node{
																																										name	: ~"group11",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.41, 0.58, 0.41, -0.58],
																																											pos	: [-56.22, 18.02, -0.04],
																																										},
																																										children	: ~[
																																											ChildNode(Node{
																																												name	: ~"cluster9Handle",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [1.00, -0.00, -0.00, -0.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																												children	: ~[],
																																											}),
																																											ChildNode(Node{
																																												name	: ~"cluster8Handle",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [1.00, -0.00, -0.00, -0.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																												children	: ~[],
																																											}),
																																											ChildNode(Node{
																																												name	: ~"cluster7Handle",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [1.00, -0.00, -0.00, -0.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																												children	: ~[],
																																											})
																																										],
																																									}),
																																									ChildNode(Node{
																																										name	: ~"c_breastControls_grp",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [1.00, -0.00, 0.00, 0.00],
																																											pos	: [0.00, 0.00, 0.00],
																																										},
																																										children	: ~[
																																											ChildNode(Node{
																																												name	: ~"null1",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [1.00, -0.00, 0.00, -0.00],
																																													pos	: [0.00, -0.00, -0.00],
																																												},
																																												children	: ~[],
																																											}),
																																											ChildNode(Node{
																																												name	: ~"R_breast_IK_control_grp",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.52, -0.10, -0.08, -0.84],
																																													pos	: [-2.31, -6.09, 4.23],
																																												},
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"R_breast_IK_cntr_zero",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [1.00, 0.00, -0.00, -0.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"R_breast_IK_ctrl",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [1.00, -0.00, -0.00, -0.00],
																																																	pos	: [0.00, 0.00, -0.00],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"ikHandle2",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [1.00, 0.00, 0.00, 0.00],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																		children	: ~[],
																																																	}),
																																																	ChildNode(Node{
																																																		name	: ~"R_breastTweak_cntr_grp",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.71, 0.03, -0.70, 0.06],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"R_breastTweak_zero",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [1.00, 0.00, 0.00, -0.00],
																																																					pos	: [-0.00, 0.00, -0.00],
																																																				},
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"R_breastTweak_ctrl",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [1.00, -0.00, 0.00, 0.00],
																																																							pos	: [-0.00, 0.00, -0.00],
																																																						},
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
																																												name	: ~"L_breast_IK_control_grp",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.52, 0.10, 0.08, -0.84],
																																													pos	: [-2.31, -6.09, -4.23],
																																												},
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"L_breast_IK_cntr_zero",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [1.00, 0.00, 0.00, 0.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"L_breastTweak_cntr_grp",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.71, -0.03, 0.70, 0.06],
																																																	pos	: [-0.00, 0.00, -0.00],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"L_breastTweak_zero",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [1.00, 0.00, 0.00, 0.00],
																																																			pos	: [0.00, 0.00, 0.00],
																																																		},
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"L_breastTweak_ctrl",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [1.00, -0.00, -0.00, -0.00],
																																																					pos	: [0.00, -0.00, -0.00],
																																																				},
																																																				children	: ~[],
																																																			})
																																																		],
																																																	})
																																																],
																																															}),
																																															ChildNode(Node{
																																																name	: ~"L_breast_IK_ctrl",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [1.00, -0.00, 0.00, 0.00],
																																																	pos	: [-0.00, 0.00, -0.00],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"ikHandle3",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [1.00, -0.00, 0.00, -0.00],
																																																			pos	: [0.00, -0.00, 0.00],
																																																		},
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
																																										name	: ~"c_neck_01_ctrl_grp",
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.97, 0.00, 0.00, -0.23],
																																											pos	: [6.82, -0.29, 0.00],
																																										},
																																										children	: ~[
																																											ChildNode(Node{
																																												name	: ~"R_clav_ctrl_grp",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.71, -0.19, -0.65, 0.22],
																																													pos	: [-1.39, -3.03, 1.00],
																																												},
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"R_clav_ctrl_zero",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [1.00, 0.00, -0.00, -0.00],
																																															pos	: [0.00, -0.00, -0.00],
																																														},
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"R_clav_ctrl",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.28, 0.00, -0.12, 0.95],
																																																	pos	: [0.00, -0.00, 0.00],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"R_mainSpaulder_ctrl_grp",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.91, 0.13, 0.32, -0.21],
																																																			pos	: [4.89, -0.16, 0.38],
																																																		},
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"R_mainSpaulder_ctrl_zero",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [1.00, -0.00, -0.00, 0.00],
																																																					pos	: [-0.00, 0.00, 0.00],
																																																				},
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"R_mainSpaulder_ctrl",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [1.00, 0.00, 0.00, -0.03],
																																																							pos	: [0.00, 0.00, -0.00],
																																																						},
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								name	: ~"R_subSpaulder_ctrl_grp",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.96, -0.00, -0.30, 0.00],
																																																									pos	: [2.32, 0.02, 0.51],
																																																								},
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										name	: ~"R_subSpaulder_ctrl_zero",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [1.00, -0.00, -0.00, 0.00],
																																																											pos	: [-0.00, 0.00, -0.00],
																																																										},
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												name	: ~"R_subSpaulder_ctrl",
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [1.00, -0.00, 0.00, -0.04],
																																																													pos	: [0.00, 0.00, -0.00],
																																																												},
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
																																												name	: ~"L_clav_ctrl_grp",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.71, 0.16, 0.64, 0.24],
																																													pos	: [-1.39, -3.03, -1.00],
																																												},
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"L_clav_ctrl_zero",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [1.00, -0.00, -0.00, -0.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"L_clav_ctrl",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.96, 0.00, -0.01, -0.28],
																																																	pos	: [0.00, 0.00, -0.00],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"L_mainSpaulder_ctrl_grp",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.92, -0.09, -0.32, -0.22],
																																																			pos	: [4.89, -0.12, -0.39],
																																																		},
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"L_mainSpaulder_ctrl_zero",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [1.00, 0.00, 0.00, 0.00],
																																																					pos	: [0.00, -0.00, 0.00],
																																																				},
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"L_mainSpaulder_ctrl",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [1.00, -0.00, -0.00, -0.03],
																																																							pos	: [0.00, -0.00, -0.00],
																																																						},
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								name	: ~"L_subSpaulder_ctrl_grp",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.96, 0.00, 0.30, 0.00],
																																																									pos	: [2.32, 0.02, -0.51],
																																																								},
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										name	: ~"L_subSpaulder_ctrl_zero",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [1.00, -0.00, -0.00, -0.00],
																																																											pos	: [-0.00, 0.00, -0.00],
																																																										},
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												name	: ~"L_subSpaulder_ctrl",
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [1.00, -0.00, -0.00, -0.04],
																																																													pos	: [0.00, 0.00, -0.00],
																																																												},
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
																																												name	: ~"c_neck_01_ctrl_zero",
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [1.00, -0.00, -0.00, -0.00],
																																													pos	: [0.00, 0.00, 0.00],
																																												},
																																												children	: ~[
																																													ChildNode(Node{
																																														name	: ~"c_neck_01_ctrl",
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [1.00, -0.00, -0.00, 0.06],
																																															pos	: [0.00, 0.00, -0.00],
																																														},
																																														children	: ~[
																																															ChildNode(Node{
																																																name	: ~"c_neck_02_ctrl_grp",
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [1.00, 0.00, 0.00, 0.02],
																																																	pos	: [2.33, 0.01, -0.00],
																																																},
																																																children	: ~[
																																																	ChildNode(Node{
																																																		name	: ~"c_neck_02_ctrl_zero",
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [1.00, -0.00, -0.00, -0.00],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				name	: ~"c_neck_02_ctrl",
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [1.00, -0.00, -0.00, 0.04],
																																																					pos	: [0.00, 0.00, -0.00],
																																																				},
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						name	: ~"c_neck_03_ctrl_grp",
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [1.00, -0.00, -0.00, -0.00],
																																																							pos	: [2.37, 0.00, 0.00],
																																																						},
																																																						children	: ~[
																																																							ChildNode(Node{
																																																								name	: ~"c_neck_03_ctrl_zero",
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [1.00, 0.00, 0.00, 0.00],
																																																									pos	: [0.00, 0.00, 0.00],
																																																								},
																																																								children	: ~[
																																																									ChildNode(Node{
																																																										name	: ~"c_neck_03_ctrl",
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [1.00, -0.00, -0.00, 0.05],
																																																											pos	: [0.00, 0.00, 0.00],
																																																										},
																																																										children	: ~[
																																																											ChildNode(Node{
																																																												name	: ~"c_jaw_ctrl_grp",
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [0.57, 0.00, 0.00, -0.82],
																																																													pos	: [-0.41, -0.55, 0.00],
																																																												},
																																																												children	: ~[
																																																													ChildNode(Node{
																																																														name	: ~"c_jaw_ctrl_zero",
																																																														space	: QuatSpace{
																																																															scale	: 1.00,
																																																															rot	: [1.00, -0.00, 0.00, -0.00],
																																																															pos	: [-0.00, 0.00, 0.00],
																																																														},
																																																														children	: ~[
																																																															ChildNode(Node{
																																																																name	: ~"c_jaw_ctrl",
																																																																space	: QuatSpace{
																																																																	scale	: 1.00,
																																																																	rot	: [1.00, -0.00, 0.00, -0.00],
																																																																	pos	: [0.00, -0.00, 0.00],
																																																																},
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
						name	: ~"noTrasnform",
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [1.00, 0.00, 0.00, 0.00],
							pos	: [0.00, 0.00, 0.00],
						},
						children	: ~[
							ChildNode(Node{
								name	: ~"Body",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [1.00, 0.00, 0.00, 0.00],
									pos	: [0.00, 0.00, 0.00],
								},
								children	: ~[
									ChildNode(Node{
										name	: ~"tongue_geo1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Tongue",
												armature	: ~"",
												range	: [0, 528],
												mesh	: ~"tongue_geo1Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"polySurface172",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"cloak",
												armature	: ~"",
												range	: [0, 15252],
												mesh	: ~"polySurfaceShape174@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"topJaw_geo2",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Teeth",
												armature	: ~"",
												range	: [0, 4656],
												mesh	: ~"topJaw_geo2Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"lowerJaw_geo2",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"Teeth",
												armature	: ~"",
												range	: [0, 4248],
												mesh	: ~"lowerJaw_geo2Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"L_upper_lash1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 13716],
												mesh	: ~"L_upper_lash1Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"L_lower_lash1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 8964],
												mesh	: ~"L_lower_lash1Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"R_upper_lash1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 13716],
												mesh	: ~"R_upper_lash1Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"R_lower_lash1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"EyeLashes",
												armature	: ~"",
												range	: [0, 8964],
												mesh	: ~"R_lower_lash1Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"L_eye_geo1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
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
									}),
									ChildNode(Node{
										name	: ~"R_eye_geo1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
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
									}),
									ChildNode(Node{
										name	: ~"Hair_Geo2",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"anisotropic1",
												armature	: ~"",
												range	: [0, 6954],
												mesh	: ~"Hair_Geo2Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"Body_geo8",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"skin",
												armature	: ~"",
												range	: [0, 50496],
												mesh	: ~"Body_geo8Shape@",
											})
										],
									})
								],
							}),
							ChildNode(Node{
								name	: ~"Armor",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [1.00, 0.00, 0.00, 0.00],
									pos	: [0.00, 0.00, 0.00],
								},
								children	: ~[
									ChildNode(Node{
										name	: ~"boots",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, -0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 9042],
												mesh	: ~"R_boot1Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"backShealth1",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 5550],
												mesh	: ~"backShealth1Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"skirt",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
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
									}),
									ChildNode(Node{
										name	: ~"bracket",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"skin",
												armature	: ~"",
												range	: [0, 8448],
												mesh	: ~"bracket_05_geo1Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"bracers",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 2304],
												mesh	: ~"L_bracer1Shape@",
											})
										],
									}),
									ChildNode(Node{
										name	: ~"spaulders",
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [1.00, 0.00, 0.00, 0.00],
											pos	: [0.00, 0.00, 0.00],
										},
										children	: ~[
											ChildEntity(Entity{
												material	: ~"armor",
												armature	: ~"",
												range	: [0, 6960],
												mesh	: ~"R_subSpaulder1Shape@",
											})
										],
									})
								],
							}),
							ChildNode(Node{
								name	: ~"Eyes_Geo",
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [1.00, 0.00, 0.00, 0.00],
									pos	: [0.00, 0.00, 0.00],
								},
								children	: ~[],
							})
						],
					})
				],
			}),
			ChildNode(Node{
				name	: ~"Lamp",
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.47, 0.27, 0.31, 0.78],
					pos	: [43.55, 25.15, 80.51],
				},
				children	: ~[
					ChildLight(Light{
						attenuation	: [0.00, 1.00],
						color	: [1.00, 1.00, 1.00, 1.00],
						kind	: KindSpot(Spot{
							size	: 1.31,
							blend	: 0.15,
						}),
						spherical	: false,
						energy	: 3.00,
						distance	: 100.00,
					})
				],
			})
		],
		global	: Global{
			gravity	: [0.00, 0.00, -9.81],
		},
	}
}
