use common::*;
pub fn load()-> Scene	{Scene{
		nodes	: ~[
			ChildNode(Node{
				children	: ~[
					ChildEntity(Entity{
						range	: [0, 6],
						material	: ~"Material",
						armature	: ~"",
						mesh	: ~"Plane@all",
					})
				],
				actions	: ~[],
				space	: QuatSpace{
					scale	: 100.00,
					rot	: [0.00, 0.00, 0.00, 1.00],
					pos	: [0.00, 0.00, -1.00],
				},
				name	: ~"Plane",
			}),
			ChildNode(Node{
				children	: ~[
					ChildCamera(Camera{
						range	: [10.00, 300.00],
						fov_y	: 0.87,
						name	: ~"Camera",
					})
				],
				actions	: ~[],
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.41, 0.41, 0.58, 0.58],
					pos	: [140.00, 0.00, 90.00],
				},
				name	: ~"Camera",
			}),
			ChildNode(Node{
				children	: ~[
					ChildNode(Node{
						children	: ~[
							ChildNode(Node{
								children	: ~[],
								actions	: ~[],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
								name	: ~"ikHandle8",
							})
						],
						actions	: ~[],
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
						name	: ~"R_ik_foot_grp",
					}),
					ChildNode(Node{
						children	: ~[],
						actions	: ~[],
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
						name	: ~"L_leg_ikHandle_zero.001",
					}),
					ChildNode(Node{
						children	: ~[
							ChildNode(Node{
								children	: ~[],
								actions	: ~[],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
								name	: ~"ikHandle7",
							})
						],
						actions	: ~[],
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
						name	: ~"L_ik_foot_grp",
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
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, -0.00, 0.70],
																																											pos	: [0.00, 0.69, 0.00],
																																										},
																																										name	: ~"L_eye_end_joint",
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.71, -0.01, 0.01, 0.71],
																																									pos	: [1.26, -4.62, 3.06],
																																								},
																																								name	: ~"L_eye_joint",
																																							},
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.70, -0.00, 0.71],
																																											pos	: [0.00, 0.69, -0.00],
																																										},
																																										name	: ~"R_eye_end_joint",
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.71, 0.01, -0.01, 0.71],
																																									pos	: [-1.26, -4.62, 3.06],
																																								},
																																								name	: ~"R_eye_joint",
																																							},
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, -0.00, 0.71],
																																											pos	: [-0.00, 0.50, -0.00],
																																										},
																																										name	: ~"R_eye_blink_01_joint",
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.77, -0.00, 0.00, 0.64],
																																									pos	: [-1.26, -4.62, 3.06],
																																								},
																																								name	: ~"R_eye_blink_base_joint",
																																							},
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, -0.00, 0.71],
																																											pos	: [0.00, 0.50, -0.00],
																																										},
																																										name	: ~"L_eye_blink_01_joint",
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.77, -0.00, 0.00, 0.64],
																																									pos	: [1.26, -4.62, 3.06],
																																								},
																																								name	: ~"L_eye_blink_base_joint",
																																							},
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.71, -0.00, 0.71],
																																											pos	: [-0.00, 0.67, -0.00],
																																										},
																																										name	: ~"R_eye_blink_02_joint",
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.60, -0.00, 0.00, 0.80],
																																									pos	: [-1.26, -4.62, 3.06],
																																								},
																																								name	: ~"R_eye_blink_02_base_joint",
																																							},
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.71, 0.00, 0.71],
																																											pos	: [-0.00, 0.67, -0.00],
																																										},
																																										name	: ~"L_eye_blink_02_joint",
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.60, -0.00, 0.00, 0.80],
																																									pos	: [1.26, -4.62, 3.06],
																																								},
																																								name	: ~"L_eye_blink_02_base_joint",
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [-0.04, 0.00, 0.00, 1.00],
																																							pos	: [-0.00, 6.34, 0.04],
																																						},
																																						name	: ~"head_end",
																																					},
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [-0.00, 0.71, 0.00, 0.71],
																																									pos	: [0.00, 3.68, -0.00],
																																								},
																																								name	: ~"jaw_end_joint",
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.82, 0.00, 0.00, 0.57],
																																							pos	: [-0.00, -0.41, 0.55],
																																						},
																																						name	: ~"jaw_joint",
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.00, 0.00, 0.00, 1.00],
																																					pos	: [0.00, 2.37, -0.00],
																																				},
																																				name	: ~"head_joint",
																																			}
																																		],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [-0.00, 0.98, -0.21, -0.00],
																																			pos	: [-0.00, 2.10, -1.00],
																																		},
																																		name	: ~"c_neck_02_joint",
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [0.00, -0.00, 0.00, 1.00],
																																	pos	: [0.00, 0.44, -0.29],
																																},
																																name	: ~"c_neck_01_joint",
																															},
																															Bone{
																																children	: ~[
																																	Bone{
																																		children	: ~[],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, 1.00, -0.00, -0.00],
																																			pos	: [0.00, 1.81, 0.00],
																																		},
																																		name	: ~"c_shealth_end_joint",
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, 0.16, 0.99, 0.00],
																																	pos	: [-0.00, 0.04, 2.55],
																																},
																																name	: ~"c_shealth_01_joint",
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
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, 0.94, 0.00, 0.35],
																																																											pos	: [-0.00, 0.96, 0.00],
																																																										},
																																																										name	: ~"L_pinkyFinger_04_joint",
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, 0.34, 0.07, 0.93],
																																																									pos	: [-0.00, 0.75, 0.00],
																																																								},
																																																								name	: ~"L_pinkyFinger_03_joint",
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.25, 0.56, -0.16, 0.77],
																																																							pos	: [-0.00, 1.18, 0.00],
																																																						},
																																																						name	: ~"L_pinkyFinger_02_joint",
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.37, 0.10, -0.71, 0.59],
																																																					pos	: [-0.54, -0.80, -1.23],
																																																				},
																																																				name	: ~"L_pinkyFinger_01_joint",
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, 0.96, -0.00, 0.29],
																																																											pos	: [-0.00, 1.00, 0.00],
																																																										},
																																																										name	: ~"L_ringFinger_04_joint",
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, 0.37, 0.10, 0.92],
																																																									pos	: [0.00, 0.95, -0.00],
																																																								},
																																																								name	: ~"L_ringFinger_03_joint",
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.27, 0.81, -0.24, 0.46],
																																																							pos	: [0.00, 1.63, 0.00],
																																																						},
																																																						name	: ~"L_ringFinger_02_joint",
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.54, -0.07, -0.59, 0.60],
																																																					pos	: [0.26, -0.29, -0.98],
																																																				},
																																																				name	: ~"L_ringFinger_01_joint",
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, 0.24, 0.00, 0.97],
																																																											pos	: [0.00, 1.24, 0.00],
																																																										},
																																																										name	: ~"L_middleFinger_04_joint",
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.07, 0.19, 0.02, 0.98],
																																																									pos	: [0.00, 1.01, -0.00],
																																																								},
																																																								name	: ~"L_middleFinger_03_joint",
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.16, 0.83, -0.31, 0.44],
																																																							pos	: [0.00, 1.64, 0.00],
																																																						},
																																																						name	: ~"L_middleFinger_02_joint",
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.58, -0.15, -0.55, 0.58],
																																																					pos	: [0.92, 0.08, -0.55],
																																																				},
																																																				name	: ~"L_middleFinger_01_joint",
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, 0.99, -0.00, 0.17],
																																																											pos	: [-0.00, 1.09, -0.00],
																																																										},
																																																										name	: ~"L_indexFinger_04_joint",
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.14, 0.36, -0.05, 0.92],
																																																									pos	: [-0.00, 1.03, -0.00],
																																																								},
																																																								name	: ~"L_indexFinger_03_joint",
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.05, 0.73, -0.36, 0.58],
																																																							pos	: [0.00, 1.48, -0.00],
																																																						},
																																																						name	: ~"L_indexFinger_02_joint",
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.63, -0.19, -0.46, 0.59],
																																																					pos	: [1.60, 0.39, 0.17],
																																																				},
																																																				name	: ~"L_indexFinger_01_joint",
																																																			}
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.11, -0.29, 0.75, 0.59],
																																																			pos	: [-0.06, 2.48, -0.37],
																																																		},
																																																		name	: ~"L_wrist_end_joint",
																																																	},
																																																	Bone{
																																																		children	: ~[
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.00, -0.82, 0.00, 0.57],
																																																									pos	: [0.00, 1.17, 0.00],
																																																								},
																																																								name	: ~"L_thumb_04_joint",
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.01, 0.12, 0.02, 0.99],
																																																							pos	: [-0.00, 1.10, -0.00],
																																																						},
																																																						name	: ~"L_thumb_03_joint",
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.06, -0.32, -0.13, 0.94],
																																																					pos	: [0.00, 1.91, 0.00],
																																																				},
																																																				name	: ~"L_thumb_02_joint",
																																																			}
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.38, 0.51, 0.19, 0.75],
																																																			pos	: [-0.43, 1.21, 0.79],
																																																		},
																																																		name	: ~"L_thumb_01_joint",
																																																	}
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.00, 0.02, 0.00, 1.00],
																																																	pos	: [0.00, 5.42, -0.00],
																																																},
																																																name	: ~"L_wrist_joint",
																																															}
																																														],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, -0.02, 0.02, 1.00],
																																															pos	: [-0.00, 2.64, -0.00],
																																														},
																																														name	: ~"L_forearm_02_joint",
																																													}
																																												],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.00, 0.01, 0.00, 1.00],
																																													pos	: [0.00, 2.77, 0.00],
																																												},
																																												name	: ~"L_forearm_01_joint",
																																											}
																																										],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.24, 0.27, 0.02, 0.93],
																																											pos	: [-0.00, 3.95, -0.00],
																																										},
																																										name	: ~"L_elbow_joint",
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																									pos	: [0.00, 3.58, 0.00],
																																								},
																																								name	: ~"L_arm_02_joint",
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, 0.91, 0.00, 0.40],
																																							pos	: [0.00, 3.42, 0.00],
																																						},
																																						name	: ~"L_arm_01_joint",
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.15, 0.73, 0.20, 0.63],
																																					pos	: [-0.52, 0.95, -0.03],
																																				},
																																				name	: ~"L_shoulder_joint",
																																			},
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.87, 0.00, 0.50],
																																											pos	: [0.00, 2.16, -0.00],
																																										},
																																										name	: ~"L_subSpaulder_end_joint",
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.24, 0.35, -0.81, 0.40],
																																									pos	: [0.00, 1.03, -0.00],
																																								},
																																								name	: ~"L_subSpaulder_joint",
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.33, -0.01, 0.91, 0.25],
																																							pos	: [0.00, 3.22, 0.00],
																																						},
																																						name	: ~"L_mainSpaulder_end_joint",
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.37, 0.26, -0.12, 0.89],
																																					pos	: [-0.31, -0.70, 0.27],
																																				},
																																				name	: ~"L_mainSpaulder_joint",
																																			},
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																									pos	: [-0.00, 10.84, -0.00],
																																								},
																																								name	: ~"L_armIK_03_joint",
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.13, 0.96, -0.21, 0.13],
																																							pos	: [0.00, 10.95, 0.00],
																																						},
																																						name	: ~"L_armIK_02_joint",
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.15, 0.73, 0.20, 0.63],
																																					pos	: [-0.52, 0.95, -0.03],
																																				},
																																				name	: ~"L_armIK_01_joint",
																																			}
																																		],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [-0.00, -0.93, -0.00, 0.36],
																																			pos	: [-0.00, 5.59, -0.00],
																																		},
																																		name	: ~"L_clav_end_joint",
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.31, 0.71, 0.50, 0.39],
																																	pos	: [-1.00, -2.13, -2.41],
																																},
																																name	: ~"L_clav_joint",
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
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [-0.00, 0.82, -0.00, 0.57],
																																																									pos	: [-0.00, 1.17, 0.00],
																																																								},
																																																								name	: ~"R_thumb_04_joint",
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.01, -0.12, -0.02, 0.99],
																																																							pos	: [0.00, 1.10, -0.00],
																																																						},
																																																						name	: ~"R_thumb_03_joint",
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.06, 0.32, 0.13, 0.94],
																																																					pos	: [0.00, 1.91, -0.00],
																																																				},
																																																				name	: ~"R_thumb_02_joint",
																																																			}
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.38, -0.51, -0.19, 0.75],
																																																			pos	: [0.43, 1.21, 0.79],
																																																		},
																																																		name	: ~"R_thumb_01_joint",
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
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.99, 0.00, 0.17],
																																																											pos	: [-0.00, 1.09, -0.00],
																																																										},
																																																										name	: ~"R_indexFinger_04_joint",
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.15, -0.31, 0.02, 0.94],
																																																									pos	: [0.00, 1.03, -0.00],
																																																								},
																																																								name	: ~"R_indexFinger_03_joint",
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.07, -0.67, 0.35, 0.65],
																																																							pos	: [-0.00, 1.48, -0.00],
																																																						},
																																																						name	: ~"R_indexFinger_02_joint",
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.45, 0.13, 0.60, 0.65],
																																																					pos	: [-1.60, 0.39, 0.17],
																																																				},
																																																				name	: ~"R_indexFinger_01_joint",
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, -0.24, -0.00, 0.97],
																																																											pos	: [-0.00, 1.24, -0.00],
																																																										},
																																																										name	: ~"R_middleFinger_04_joint",
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.07, -0.19, -0.02, 0.98],
																																																									pos	: [-0.00, 1.01, -0.00],
																																																								},
																																																								name	: ~"R_middleFinger_03_joint",
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.16, -0.83, 0.31, 0.44],
																																																							pos	: [0.00, 1.64, 0.00],
																																																						},
																																																						name	: ~"R_middleFinger_02_joint",
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.58, 0.15, 0.55, 0.58],
																																																					pos	: [-0.92, 0.08, -0.55],
																																																				},
																																																				name	: ~"R_middleFinger_01_joint",
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.96, -0.00, 0.29],
																																																											pos	: [0.00, 1.00, 0.00],
																																																										},
																																																										name	: ~"R_ringFinger_04_joint",
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, -0.41, -0.09, 0.90],
																																																									pos	: [-0.00, 0.95, -0.00],
																																																								},
																																																								name	: ~"R_ringFinger_03_joint",
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.23, -0.84, 0.28, 0.40],
																																																							pos	: [-0.00, 1.63, -0.00],
																																																						},
																																																						name	: ~"R_ringFinger_02_joint",
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.62, 0.09, 0.52, 0.57],
																																																					pos	: [-0.26, -0.29, -0.98],
																																																				},
																																																				name	: ~"R_ringFinger_01_joint",
																																																			},
																																																			Bone{
																																																				children	: ~[
																																																					Bone{
																																																						children	: ~[
																																																							Bone{
																																																								children	: ~[
																																																									Bone{
																																																										children	: ~[],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [0.00, -0.94, 0.00, 0.35],
																																																											pos	: [0.00, 0.96, 0.00],
																																																										},
																																																										name	: ~"R_pinkyFinger_04_joint",
																																																									}
																																																								],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.12, -0.42, -0.07, 0.90],
																																																									pos	: [-0.00, 0.75, 0.00],
																																																								},
																																																								name	: ~"R_pinkyFinger_03_joint",
																																																							}
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.21, -0.68, 0.22, 0.67],
																																																							pos	: [0.00, 1.18, 0.00],
																																																						},
																																																						name	: ~"R_pinkyFinger_02_joint",
																																																					}
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.57, -0.04, 0.62, 0.53],
																																																					pos	: [0.54, -0.80, -1.23],
																																																				},
																																																				name	: ~"R_pinkyFinger_01_joint",
																																																			}
																																																		],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.11, 0.29, -0.75, 0.59],
																																																			pos	: [0.06, 2.48, -0.37],
																																																		},
																																																		name	: ~"R_wrist_end_joint",
																																																	}
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, -0.02, 0.00, 1.00],
																																																	pos	: [0.00, 5.42, -0.00],
																																																},
																																																name	: ~"R_wrist_joint",
																																															}
																																														],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, 0.02, -0.02, 1.00],
																																															pos	: [0.00, 2.64, -0.00],
																																														},
																																														name	: ~"R_forearm_02_joint",
																																													}
																																												],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.01, -0.00, 1.00],
																																													pos	: [0.00, 2.77, 0.00],
																																												},
																																												name	: ~"R_forearm_01_joint",
																																											}
																																										],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.24, -0.27, -0.02, 0.93],
																																											pos	: [0.00, 3.95, -0.00],
																																										},
																																										name	: ~"R_elbow_joint",
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.00, -0.00, -0.00, 1.00],
																																									pos	: [0.00, 3.58, -0.00],
																																								},
																																								name	: ~"R_arm_02_joint",
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, -0.21, 0.00, 0.98],
																																							pos	: [0.00, 3.42, -0.00],
																																						},
																																						name	: ~"R_arm_01_joint",
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.11, 0.20, 0.23, 0.95],
																																					pos	: [-0.31, 0.95, 0.42],
																																				},
																																				name	: ~"R_shoulder_joint",
																																			},
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[
																																									Bone{
																																										children	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.38, 0.00, 0.93],
																																											pos	: [-0.00, 2.16, 0.00],
																																										},
																																										name	: ~"R_subSpaulder_end_joint",
																																									}
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.24, -0.35, 0.81, 0.40],
																																									pos	: [-0.00, 1.03, -0.00],
																																								},
																																								name	: ~"R_subSpaulder_joint",
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [-0.54, 0.21, -0.80, 0.14],
																																							pos	: [0.00, 3.22, 0.00],
																																						},
																																						name	: ~"R_mainSpaulder_end_joint",
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.38, 0.65, -0.05, 0.65],
																																					pos	: [-0.40, -0.70, 0.06],
																																				},
																																				name	: ~"R_mainSpaulder_joint",
																																			},
																																			Bone{
																																				children	: ~[
																																					Bone{
																																						children	: ~[
																																							Bone{
																																								children	: ~[],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																									pos	: [0.00, 10.84, 0.00],
																																								},
																																								name	: ~"R_armIK_03_joint",
																																							}
																																						],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.24, -0.46, 0.02, 0.85],
																																							pos	: [0.00, 10.95, -0.00],
																																						},
																																						name	: ~"R_armIK_02_joint",
																																					}
																																				],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.11, 0.20, 0.23, 0.95],
																																					pos	: [-0.31, 0.95, 0.42],
																																				},
																																				name	: ~"R_armIK_01_joint",
																																			}
																																		],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, -0.72, -0.00, 0.70],
																																			pos	: [0.00, 5.59, -0.00],
																																		},
																																		name	: ~"R_clav_end_joint",
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.32, -0.71, -0.50, 0.38],
																																	pos	: [1.00, -2.13, -2.41],
																																},
																																name	: ~"R_clav_joint",
																															}
																														],
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [-0.00, 1.00, -0.00, -0.00],
																															pos	: [0.00, 6.39, -0.06],
																														},
																														name	: ~"c_spine_007_joint",
																													},
																													Bone{
																														children	: ~[
																															Bone{
																																children	: ~[
																																	Bone{
																																		children	: ~[],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, -0.71, 0.00, 0.71],
																																			pos	: [0.00, 0.00, 0.00],
																																		},
																																		name	: ~"L_breast_end_joint",
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, 0.58, -0.00, 0.81],
																																	pos	: [-0.00, 3.94, -0.00],
																																},
																																name	: ~"L_breast_joint",
																															}
																														],
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.84, 0.12, -0.06, 0.52],
																															pos	: [3.20, -0.56, 2.70],
																														},
																														name	: ~"L_breast_base_joint",
																													},
																													Bone{
																														children	: ~[
																															Bone{
																																children	: ~[
																																	Bone{
																																		children	: ~[],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.00, 0.71, -0.00, 0.71],
																																			pos	: [0.00, -0.00, -0.00],
																																		},
																																		name	: ~"R_breast_end_joint1",
																																	}
																																],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, -0.58, 0.00, 0.81],
																																	pos	: [-0.00, 3.94, -0.00],
																																},
																																name	: ~"R_breast_joint1",
																															}
																														],
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.84, -0.12, 0.06, 0.52],
																															pos	: [-3.20, -0.56, 2.70],
																														},
																														name	: ~"R_breast_base_joint1",
																													}
																												],
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [-0.00, 0.00, 0.00, 1.00],
																													pos	: [0.00, 6.18, -0.00],
																												},
																												name	: ~"c_spine_006_joint",
																											}
																										],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.11, 0.00, -0.00, 0.99],
																											pos	: [-0.00, 1.50, 0.00],
																										},
																										name	: ~"c_spine_005_joint",
																									}
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.02, 0.00, -0.00, 1.00],
																									pos	: [-0.00, 1.45, 0.00],
																								},
																								name	: ~"c_spine_004_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.10, 0.00, -0.00, 0.99],
																							pos	: [-0.00, 1.54, 0.00],
																						},
																						name	: ~"c_spine_003_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, 0.71, 0.00, 0.71],
																					pos	: [0.00, 1.49, 0.00],
																				},
																				name	: ~"c_spine_002_joint",
																			}
																		],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			pos	: [-0.04, 0.00, -0.00],
																		},
																		name	: ~"c_spine_001_joint",
																	},
																	Bone{
																		children	: ~[
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.00, 0.71, 0.00, 0.71],
																							pos	: [-0.00, 4.60, -0.00],
																						},
																						name	: ~"R_hip_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.69, -0.14, -0.69, 0.14],
																					pos	: [4.99, 0.48, 2.82],
																				},
																				name	: ~"R_hip_base_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[
																									Bone{
																										children	: ~[],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, -0.00, 0.00, 1.00],
																											pos	: [0.00, 4.95, 0.00],
																										},
																										name	: ~"R_kneePivot_01_joint",
																									}
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																									pos	: [0.00, 5.50, -0.00],
																								},
																								name	: ~"R_kneePivot_02_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.85, -0.02, 0.53, -0.00],
																							pos	: [-0.19, -6.70, -0.11],
																						},
																						name	: ~"R_kneePivot_03_joint",
																					},
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[
																									Bone{
																										children	: ~[
																											Bone{
																												children	: ~[],
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [-0.00, -0.72, 0.00, 0.70],
																													pos	: [0.00, 2.80, -0.00],
																												},
																												name	: ~"R_toe_joint",
																											}
																										],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.23, 0.02, 0.01, 0.97],
																											pos	: [-0.00, 8.00, -0.00],
																										},
																										name	: ~"R_ball_joint",
																									}
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.56, -0.19, -0.14, 0.80],
																									pos	: [-0.00, 19.52, 0.00],
																								},
																								name	: ~"R_ankle_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.22, -0.06, -0.97, 0.00],
																							pos	: [-0.53, -18.67, -0.32],
																						},
																						name	: ~"R_knee_01_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.51, -0.51, 0.49, 0.49],
																					pos	: [4.92, 0.46, 4.10],
																				},
																				name	: ~"R_leg_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.00, 0.71, 0.00, 0.71],
																							pos	: [-0.00, 4.60, 0.00],
																						},
																						name	: ~"L_hip_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.69, -0.14, -0.69, 0.14],
																					pos	: [4.99, 0.48, -2.82],
																				},
																				name	: ~"L_hip_base_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[
																									Bone{
																										children	: ~[],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.00, 0.00, 0.00, 1.00],
																											pos	: [0.00, 4.95, -0.00],
																										},
																										name	: ~"L_kneePivot_01_joint",
																									}
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 5.50, 0.00],
																								},
																								name	: ~"L_kneePivot_02_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.95, 0.01, -0.30, -0.00],
																							pos	: [0.19, -6.70, -0.06],
																						},
																						name	: ~"L_kneePivot_03_joint",
																					},
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[
																									Bone{
																										children	: ~[
																											Bone{
																												children	: ~[],
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [0.00, -0.69, 0.00, 0.72],
																													pos	: [0.00, 2.80, -0.00],
																												},
																												name	: ~"L_toe_joint",
																											}
																										],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.23, -0.06, -0.02, 0.97],
																											pos	: [-0.00, 8.00, 0.00],
																										},
																										name	: ~"L_ball_joint",
																									}
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.55, 0.25, 0.14, 0.79],
																									pos	: [0.00, 19.52, 0.00],
																								},
																								name	: ~"L_ankle_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.25, 0.05, 0.97, 0.00],
																							pos	: [0.53, -18.67, -0.16],
																						},
																						name	: ~"L_knee_01_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.49, -0.49, 0.51, 0.51],
																					pos	: [4.92, 0.46, -4.10],
																				},
																				name	: ~"L_leg_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, -0.00, 1.00],
																									pos	: [-0.00, 5.95, 0.00],
																								},
																								name	: ~"R_frontMid_skirtplate_03_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.07, 0.99, 0.04, 0.07],
																							pos	: [0.00, 4.80, -0.00],
																						},
																						name	: ~"R_frontMid_skirtplate_02_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.68, 0.70, -0.05, 0.19],
																					pos	: [1.22, 4.37, 3.97],
																				},
																				name	: ~"R_frontMid_skirtplate_01_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, -0.00, 1.00],
																									pos	: [0.00, 5.95, -0.00],
																								},
																								name	: ~"R_frontInner_skirtplate_03_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.07, -0.93, 0.04, 0.35],
																							pos	: [0.00, 4.80, -0.00],
																						},
																						name	: ~"R_frontInner_skirtplate_02_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.68, -0.73, -0.00, 0.00],
																					pos	: [2.94, 4.48, 2.04],
																				},
																				name	: ~"R_frontInner_skirtplate_01_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, 0.00, 1.00],
																									pos	: [0.00, 5.95, 0.00],
																								},
																								name	: ~"L_frontInner_skirtplate_03_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.07, 0.93, -0.04, 0.35],
																							pos	: [0.00, 4.80, 0.00],
																						},
																						name	: ~"L_frontInner_skirtplate_02_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, -0.00, -0.68, 0.73],
																					pos	: [2.94, 4.48, -2.04],
																				},
																				name	: ~"L_frontInner_skirtplate_01_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																									pos	: [-0.00, 5.95, -0.00],
																								},
																								name	: ~"L_side_skirtplate_03_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.13, 0.98, 0.14, 0.03],
																							pos	: [0.00, 4.80, -0.00],
																						},
																						name	: ~"L_side_skirtplate_02_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.08, 0.39, -0.73, 0.56],
																					pos	: [-1.03, 1.62, -5.94],
																				},
																				name	: ~"L_side_skirtplate_01_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, 0.00, 1.00],
																									pos	: [-0.00, 5.95, 0.00],
																								},
																								name	: ~"L_rearSide_skirtplate_03_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.13, -0.96, -0.19, 0.17],
																							pos	: [-0.00, 4.80, 0.00],
																						},
																						name	: ~"L_rearSide_skirtplate_02_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.34, -0.02, -0.78, 0.53],
																					pos	: [-0.29, -2.24, -3.34],
																				},
																				name	: ~"L_rearSide_skirtplate_01_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, 3.88, -0.00],
																								},
																								name	: ~"C_rear_skirtplate_03_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.01, 0.97, 0.25, 0.04],
																							pos	: [-0.00, 4.80, -0.00],
																						},
																						name	: ~"C_rear_skirtplate_02_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.61, -0.37, -0.60, 0.36],
																					pos	: [1.63, -3.54, -0.00],
																				},
																				name	: ~"C_rear_skirtplate_01_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, -0.00, -0.00, 1.00],
																									pos	: [-0.00, 5.95, -0.00],
																								},
																								name	: ~"R_rearSide_skirtplate_03_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.13, 0.96, 0.19, 0.17],
																							pos	: [0.00, 4.80, -0.00],
																						},
																						name	: ~"R_rearSide_skirtplate_02_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.78, -0.53, -0.34, 0.02],
																					pos	: [-0.29, -2.24, 3.34],
																				},
																				name	: ~"R_rearSide_skirtplate_01_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.00, 1.00],
																									pos	: [0.00, 5.95, 0.00],
																								},
																								name	: ~"R_side_skirtplate_03_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.13, -0.98, -0.14, 0.03],
																							pos	: [0.00, 4.80, 0.00],
																						},
																						name	: ~"R_side_skirtplate_02_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.73, 0.56, -0.08, 0.39],
																					pos	: [-1.03, 1.62, 5.94],
																				},
																				name	: ~"R_side_skirtplate_01_joint",
																			},
																			Bone{
																				children	: ~[
																					Bone{
																						children	: ~[
																							Bone{
																								children	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, 0.00, 1.00],
																									pos	: [0.00, 5.95, 0.00],
																								},
																								name	: ~"L_frontMid_skirtplate_03_joint",
																							}
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.07, -0.99, -0.04, 0.07],
																							pos	: [0.00, 4.80, -0.00],
																						},
																						name	: ~"L_frontMid_skirtplate_02_joint",
																					}
																				],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.05, 0.19, -0.68, 0.70],
																					pos	: [1.22, 4.37, -3.97],
																				},
																				name	: ~"L_frontMid_skirtplate_01_joint",
																			}
																		],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, -0.71, 0.00, 0.71],
																			pos	: [0.00, -0.10, 0.04],
																		},
																		name	: ~"C_hip_joint",
																	}
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.71, 0.00, 0.00, 0.71],
																	pos	: [0.00, 47.22, 0.24],
																},
																name	: ~"cog",
															}
														],
														dual_quat	: false,
														actions	: ~[
															~"Armature.002Action@Armature.002"
														],
														name	: ~"Armature.002",
													})
												],
												actions	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
												name	: ~"Armature.002",
											}),
											ChildNode(Node{
												children	: ~[],
												actions	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
												name	: ~"SKELETON",
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
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.00, -0.00, -0.00, 1.00],
																							pos	: [0.00, 0.00, -0.00],
																						},
																						name	: ~"R_eye_ctrl",
																					})
																				],
																				actions	: ~[],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.00, 0.00, -0.00, 1.00],
																					pos	: [-0.00, -0.00, 0.00],
																				},
																				name	: ~"R_eye_ctrl_zero",
																			})
																		],
																		actions	: ~[],
																		space	: QuatSpace{
																			scale	: 0.72,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [-0.95, -0.00, 0.00],
																		},
																		name	: ~"R_eye_ctrl_grp",
																	}),
																	ChildNode(Node{
																		children	: ~[
																			ChildNode(Node{
																				children	: ~[
																					ChildNode(Node{
																						children	: ~[],
																						actions	: ~[],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.00, 0.00, -0.00, 1.00],
																							pos	: [0.00, 0.00, -0.00],
																						},
																						name	: ~"L_eye_ctrl",
																					})
																				],
																				actions	: ~[],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.00, 0.00, -0.00, 1.00],
																					pos	: [0.00, 0.00, -0.00],
																				},
																				name	: ~"L_eye_ctrl_zero",
																			})
																		],
																		actions	: ~[],
																		space	: QuatSpace{
																			scale	: 0.72,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [0.95, 0.00, -0.00],
																		},
																		name	: ~"L_eye_ctrl_grp",
																	})
																],
																actions	: ~[
																	~"mainEye_ctrlAction@nodes"
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	pos	: [0.04, -71.88, -17.98],
																},
																name	: ~"mainEye_ctrl",
															})
														],
														actions	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-0.04, 71.88, 17.98],
														},
														name	: ~"mainEye_ctrl_zero",
													})
												],
												actions	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
												name	: ~"c_eye_ctrl_grp",
											}),
											ChildNode(Node{
												children	: ~[
													ChildNode(Node{
														children	: ~[],
														actions	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [1.26, 71.88, 0.51],
														},
														name	: ~"L_eye_centerLocator",
													}),
													ChildNode(Node{
														children	: ~[],
														actions	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-1.26, 71.88, 0.51],
														},
														name	: ~"R_eye_centerLocator",
													})
												],
												actions	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
												name	: ~"Locators",
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
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																			pos	: [-0.00, 0.00, 0.00],
																		},
																		name	: ~"L_foot_ik_ctrl",
																	})
																],
																actions	: ~[],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, -0.00, 1.00],
																	pos	: [0.00, -0.00, 0.00],
																},
																name	: ~"L_leg_ikHandle_zero",
															})
														],
														actions	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [3.00, -0.31, -3.16],
														},
														name	: ~"L_leg_ikHandle_grp",
													}),
													ChildNode(Node{
														children	: ~[
															ChildNode(Node{
																children	: ~[
																	ChildNode(Node{
																		children	: ~[],
																		actions	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.01, -0.00, 0.00, 1.00],
																			pos	: [0.00, 0.00, 0.00],
																		},
																		name	: ~"R_foot_ik_ctrl",
																	})
																],
																actions	: ~[],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	pos	: [-0.00, -0.00, 0.00],
																},
																name	: ~"R_leg_ikHandle_zero",
															})
														],
														actions	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-3.00, -0.31, -3.16],
														},
														name	: ~"R_leg_ikHandle_grp",
													}),
													ChildNode(Node{
														children	: ~[
															ChildNode(Node{
																children	: ~[
																	ChildNode(Node{
																		children	: ~[],
																		actions	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.00, -0.00, 0.00, 1.00],
																			pos	: [0.00, 0.00, 0.00],
																		},
																		name	: ~"R_legPole_ctrl.001",
																	})
																],
																actions	: ~[],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.00, -0.00, -0.00, 1.00],
																	pos	: [0.57, -18.67, 10.59],
																},
																name	: ~"R_legPole_ctrl_zero",
															})
														],
														actions	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.00, 0.00, 0.00, 1.00],
															pos	: [-4.14, 43.04, -0.16],
														},
														name	: ~"R_legPole_ctrl",
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
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																					pos	: [0.00, -0.00, -0.00],
																				},
																				name	: ~"L_legPole_ctrl",
																			})
																		],
																		actions	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [-0.55, -17.93, 9.74],
																		},
																		name	: ~"L_legPole_cntr_zero",
																	})
																],
																actions	: ~[],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.47, -0.53, 0.49, 0.51],
																	pos	: [0.00, 0.00, -0.00],
																},
																name	: ~"L_legPole_ctrl_grp",
															})
														],
														actions	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [-0.47, 0.53, -0.49, 0.51],
															pos	: [4.10, 42.26, 0.61],
														},
														name	: ~"L_legPole_ctrl_cons",
													})
												],
												actions	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 0.00, 0.00],
												},
												name	: ~"LegControls",
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
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, 0.00, -0.00, 1.00],
																			pos	: [-0.00, 0.00, -0.00],
																		},
																		name	: ~"L_armIK_handle",
																	}),
																	ChildNode(Node{
																		children	: ~[
																			ChildNode(Node{
																				children	: ~[
																					ChildNode(Node{
																						children	: ~[
																							ChildNode(Node{
																								children	: ~[],
																								actions	: ~[
																									~"L_thumb_ctrlAction@nodes"
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																								name	: ~"L_thumb_ctrl",
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: ~[
																									~"L_indexF_ctrlAction@nodes"
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																								name	: ~"L_indexF_ctrl",
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: ~[
																									~"L_middleF_ctrlAction@nodes"
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																								name	: ~"L_middleF_ctrl",
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: ~[
																									~"L_ringF_ctrlAction@nodes"
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																								name	: ~"L_ringF_ctrl",
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: ~[
																									~"L_pinkyF_ctrlAction@nodes"
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.00, 0.00, -0.00, 1.00],
																									pos	: [-0.00, -0.00, -0.00],
																								},
																								name	: ~"L_pinkyF_ctrl",
																							})
																						],
																						actions	: ~[
																							~"L_palm_ctrlAction@nodes"
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.00, 0.00, -0.00, 1.00],
																							pos	: [0.00, -0.00, 0.00],
																						},
																						name	: ~"L_palm_ctrl",
																					})
																				],
																				actions	: ~[],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [0.00, -0.00, 0.00, 1.00],
																					pos	: [-0.00, 0.00, -0.00],
																				},
																				name	: ~"L_hand_zero",
																			})
																		],
																		actions	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.26, -0.08, -0.07, 0.96],
																			pos	: [-0.00, 0.00, -0.00],
																		},
																		name	: ~"L_hand_grp",
																	})
																],
																actions	: ~[
																	~"L_arm_IK_ctrlAction@nodes"
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [0.00, -0.00, 0.00, 1.00],
																	pos	: [-27.41, -60.54, 2.62],
																},
																name	: ~"L_arm_IK_ctrl",
															})
														],
														actions	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [0.26, 0.08, 0.07, 0.96],
															pos	: [0.00, -0.00, 0.00],
														},
														name	: ~"L_arm_IK_ctrl_zero",
													})
												],
												actions	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [-0.26, -0.08, -0.07, 0.96],
													pos	: [27.41, 60.54, -2.62],
												},
												name	: ~"L_arm_IK_ctrl_grp",
											}),
											ChildNode(Node{
												children	: ~[],
												actions	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 46.47, 1.11],
												},
												name	: ~"c_hips_cntr_backup",
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
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.00, 0.00, 0.00, 1.00],
																			pos	: [-54.83, -0.00, 0.00],
																		},
																		name	: ~"ikHandle4",
																	}),
																	ChildNode(Node{
																		children	: ~[
																			ChildNode(Node{
																				children	: ~[
																					ChildNode(Node{
																						children	: ~[
																							ChildNode(Node{
																								children	: ~[],
																								actions	: ~[
																									~"R_thumb_ctrlAction@nodes"
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																								name	: ~"R_thumb_ctrl",
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: ~[
																									~"R_indexF_ctrlAction@nodes"
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																								name	: ~"R_indexF_ctrl",
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: ~[
																									~"R_middleF_ctrlAction@nodes"
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																								name	: ~"R_middleF_ctrl",
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: ~[
																									~"R_ringF_ctrlAction@nodes"
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																								name	: ~"R_ringF_ctrl",
																							}),
																							ChildNode(Node{
																								children	: ~[],
																								actions	: ~[
																									~"R_pinkyF_ctrlAction@nodes"
																								],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, -0.00, 0.00, 1.00],
																									pos	: [-0.00, 0.00, -0.00],
																								},
																								name	: ~"R_pinkyF_ctrl",
																							})
																						],
																						actions	: ~[
																							~"R_palm_ctrlAction@nodes"
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.07, 0.96, -0.26, 0.08],
																							pos	: [0.00, 0.00, -0.00],
																						},
																						name	: ~"R_palm_ctrl",
																					})
																				],
																				actions	: ~[],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, 0.00, 0.00, 1.00],
																					pos	: [0.00, -0.00, 0.00],
																				},
																				name	: ~"R_hand_zero",
																			})
																		],
																		actions	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [0.07, -0.96, 0.26, 0.08],
																			pos	: [-108.05, 6.80, 11.25],
																		},
																		name	: ~"R_hand_grp",
																	})
																],
																actions	: ~[
																	~"R_arm_IK_ctrl1Action@nodes"
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, -0.00, 0.00, 1.00],
																	pos	: [-27.33, -60.54, 2.62],
																},
																name	: ~"R_arm_IK_ctrl1",
															})
														],
														actions	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [-0.00, -0.00, 0.00, 1.00],
															pos	: [0.00, 0.00, 0.00],
														},
														name	: ~"R_arm_IK_ctrl_zero",
													})
												],
												actions	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [27.33, 60.54, -2.62],
												},
												name	: ~"R_arm_IK_ctrl_grp",
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
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																										name	: ~"cluster3Handle",
																									}),
																									ChildNode(Node{
																										children	: ~[],
																										actions	: ~[],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																										name	: ~"cluster2Handle",
																									}),
																									ChildNode(Node{
																										children	: ~[],
																										actions	: ~[],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [0.00, 0.00, -0.00, 1.00],
																											pos	: [-0.00, -0.00, 0.00],
																										},
																										name	: ~"cluster1Handle",
																									})
																								],
																								actions	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [0.46, 0.54, -0.46, 0.54],
																									pos	: [-46.65, -7.08, 0.00],
																								},
																								name	: ~"group13",
																							})
																						],
																						actions	: ~[
																							~"c_hips_ctrlAction@nodes"
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [0.46, 0.54, -0.46, 0.54],
																							pos	: [0.00, -0.00, 0.00],
																						},
																						name	: ~"c_hips_ctrl",
																					})
																				],
																				actions	: ~[],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																					pos	: [0.00, -0.00, -0.00],
																				},
																				name	: ~"c_hips_ctrl_zero",
																			})
																		],
																		actions	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			pos	: [-0.00, -0.04, -0.10],
																		},
																		name	: ~"c_hips_ctrl_grp",
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
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [0.00, 0.00, -0.00, 1.00],
																																	pos	: [0.00, 0.00, 0.00],
																																},
																																name	: ~"cluster4Handle",
																															})
																														],
																														actions	: ~[],
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.52, 0.48, -0.52, 0.48],
																															pos	: [-50.08, 3.75, -0.04],
																														},
																														name	: ~"group14",
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
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																							pos	: [-0.00, -0.00, 0.00],
																																						},
																																						name	: ~"cluster6Handle",
																																					}),
																																					ChildNode(Node{
																																						children	: ~[],
																																						actions	: ~[],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [0.00, -0.00, -0.00, 1.00],
																																							pos	: [-0.00, -0.00, 0.00],
																																						},
																																						name	: ~"cluster5Handle",
																																					})
																																				],
																																				actions	: ~[],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [0.57, 0.41, -0.57, 0.41],
																																					pos	: [-50.19, 17.53, -0.04],
																																				},
																																				name	: ~"group12",
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
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																												name	: ~"cluster9Handle",
																																											}),
																																											ChildNode(Node{
																																												children	: ~[],
																																												actions	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																												name	: ~"cluster8Handle",
																																											}),
																																											ChildNode(Node{
																																												children	: ~[],
																																												actions	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, -0.00],
																																												},
																																												name	: ~"cluster7Handle",
																																											})
																																										],
																																										actions	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.58, 0.41, -0.58, 0.41],
																																											pos	: [-56.22, 18.02, -0.04],
																																										},
																																										name	: ~"group11",
																																									}),
																																									ChildNode(Node{
																																										children	: ~[
																																											ChildNode(Node{
																																												children	: ~[],
																																												actions	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, 0.00, -0.00, 1.00],
																																													pos	: [0.00, -0.00, -0.00],
																																												},
																																												name	: ~"null1",
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
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																		name	: ~"ikHandle2",
																																																	}),
																																																	ChildNode(Node{
																																																		children	: ~[
																																																			ChildNode(Node{
																																																				children	: ~[
																																																					ChildNode(Node{
																																																						children	: ~[],
																																																						actions	: ~[
																																																							~"R_breastTweak_ctrlAction@nodes"
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.00, 0.00, 0.00, 1.00],
																																																							pos	: [-0.00, 0.00, -0.00],
																																																						},
																																																						name	: ~"R_breastTweak_ctrl",
																																																					})
																																																				],
																																																				actions	: ~[],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [0.00, 0.00, -0.00, 1.00],
																																																					pos	: [-0.00, 0.00, -0.00],
																																																				},
																																																				name	: ~"R_breastTweak_zero",
																																																			})
																																																		],
																																																		actions	: ~[],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.03, -0.70, 0.06, 0.71],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																		name	: ~"R_breastTweak_cntr_grp",
																																																	})
																																																],
																																																actions	: ~[
																																																	~"R_breast_IK_ctrlAction@nodes"
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																																	pos	: [0.00, 0.00, -0.00],
																																																},
																																																name	: ~"R_breast_IK_ctrl",
																																															})
																																														],
																																														actions	: ~[],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																														name	: ~"R_breast_IK_cntr_zero",
																																													})
																																												],
																																												actions	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.10, -0.08, -0.84, 0.52],
																																													pos	: [-2.31, -6.09, 4.23],
																																												},
																																												name	: ~"R_breast_IK_control_grp",
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
																																																				actions	: ~[
																																																					~"L_breastTweak_ctrlAction@nodes"
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.00, -0.00, -0.00, 1.00],
																																																					pos	: [0.00, -0.00, -0.00],
																																																				},
																																																				name	: ~"L_breastTweak_ctrl",
																																																			})
																																																		],
																																																		actions	: ~[],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.00, 0.00, 0.00, 1.00],
																																																			pos	: [0.00, 0.00, 0.00],
																																																		},
																																																		name	: ~"L_breastTweak_zero",
																																																	})
																																																],
																																																actions	: ~[],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.03, 0.70, 0.06, 0.71],
																																																	pos	: [-0.00, 0.00, -0.00],
																																																},
																																																name	: ~"L_breastTweak_cntr_grp",
																																															}),
																																															ChildNode(Node{
																																																children	: ~[
																																																	ChildNode(Node{
																																																		children	: ~[],
																																																		actions	: ~[],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [-0.00, 0.00, -0.00, 1.00],
																																																			pos	: [0.00, -0.00, 0.00],
																																																		},
																																																		name	: ~"ikHandle3",
																																																	})
																																																],
																																																actions	: ~[
																																																	~"L_breast_IK_ctrlAction@nodes"
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [-0.00, 0.00, 0.00, 1.00],
																																																	pos	: [-0.00, 0.00, -0.00],
																																																},
																																																name	: ~"L_breast_IK_ctrl",
																																															})
																																														],
																																														actions	: ~[],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [0.00, 0.00, 0.00, 1.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																														name	: ~"L_breast_IK_cntr_zero",
																																													})
																																												],
																																												actions	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.10, 0.08, -0.84, 0.52],
																																													pos	: [-2.31, -6.09, -4.23],
																																												},
																																												name	: ~"L_breast_IK_control_grp",
																																											})
																																										],
																																										actions	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [-0.00, 0.00, 0.00, 1.00],
																																											pos	: [0.00, 0.00, 0.00],
																																										},
																																										name	: ~"c_breastControls_grp",
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
																																																												actions	: ~[
																																																													~"R_subSpaulder_ctrlAction@nodes"
																																																												],
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [-0.00, 0.00, -0.04, 1.00],
																																																													pos	: [0.00, 0.00, -0.00],
																																																												},
																																																												name	: ~"R_subSpaulder_ctrl",
																																																											})
																																																										],
																																																										actions	: ~[],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.00, 0.00, 1.00],
																																																											pos	: [-0.00, 0.00, -0.00],
																																																										},
																																																										name	: ~"R_subSpaulder_ctrl_zero",
																																																									})
																																																								],
																																																								actions	: ~[],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [-0.00, -0.30, 0.00, 0.96],
																																																									pos	: [2.32, 0.02, 0.51],
																																																								},
																																																								name	: ~"R_subSpaulder_ctrl_grp",
																																																							})
																																																						],
																																																						actions	: ~[
																																																							~"R_mainSpaulder_ctrlAction@nodes"
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [0.00, 0.00, -0.03, 1.00],
																																																							pos	: [0.00, 0.00, -0.00],
																																																						},
																																																						name	: ~"R_mainSpaulder_ctrl",
																																																					})
																																																				],
																																																				actions	: ~[],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																																					pos	: [-0.00, 0.00, 0.00],
																																																				},
																																																				name	: ~"R_mainSpaulder_ctrl_zero",
																																																			})
																																																		],
																																																		actions	: ~[],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [0.13, 0.32, -0.21, 0.91],
																																																			pos	: [4.89, -0.16, 0.38],
																																																		},
																																																		name	: ~"R_mainSpaulder_ctrl_grp",
																																																	})
																																																],
																																																actions	: ~[
																																																	~"R_clav_ctrlAction@nodes"
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, -0.12, 0.95, 0.28],
																																																	pos	: [0.00, -0.00, 0.00],
																																																},
																																																name	: ~"R_clav_ctrl",
																																															})
																																														],
																																														actions	: ~[],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [0.00, -0.00, -0.00, 1.00],
																																															pos	: [0.00, -0.00, -0.00],
																																														},
																																														name	: ~"R_clav_ctrl_zero",
																																													})
																																												],
																																												actions	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.19, -0.65, 0.22, 0.71],
																																													pos	: [-1.39, -3.03, 1.00],
																																												},
																																												name	: ~"R_clav_ctrl_grp",
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
																																																												actions	: ~[
																																																													~"L_subSpaulder_ctrlAction@nodes"
																																																												],
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [-0.00, -0.00, -0.04, 1.00],
																																																													pos	: [0.00, 0.00, -0.00],
																																																												},
																																																												name	: ~"L_subSpaulder_ctrl",
																																																											})
																																																										],
																																																										actions	: ~[],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.00, -0.00, 1.00],
																																																											pos	: [-0.00, 0.00, -0.00],
																																																										},
																																																										name	: ~"L_subSpaulder_ctrl_zero",
																																																									})
																																																								],
																																																								actions	: ~[],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.00, 0.30, 0.00, 0.96],
																																																									pos	: [2.32, 0.02, -0.51],
																																																								},
																																																								name	: ~"L_subSpaulder_ctrl_grp",
																																																							})
																																																						],
																																																						actions	: ~[
																																																							~"L_mainSpaulder_ctrlAction@nodes"
																																																						],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.00, -0.00, -0.03, 1.00],
																																																							pos	: [0.00, -0.00, -0.00],
																																																						},
																																																						name	: ~"L_mainSpaulder_ctrl",
																																																					})
																																																				],
																																																				actions	: ~[],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [0.00, 0.00, 0.00, 1.00],
																																																					pos	: [0.00, -0.00, 0.00],
																																																				},
																																																				name	: ~"L_mainSpaulder_ctrl_zero",
																																																			})
																																																		],
																																																		actions	: ~[],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [-0.09, -0.32, -0.22, 0.92],
																																																			pos	: [4.89, -0.12, -0.39],
																																																		},
																																																		name	: ~"L_mainSpaulder_ctrl_grp",
																																																	})
																																																],
																																																actions	: ~[
																																																	~"L_clav_ctrlAction@nodes"
																																																],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, -0.01, -0.28, 0.96],
																																																	pos	: [0.00, 0.00, -0.00],
																																																},
																																																name	: ~"L_clav_ctrl",
																																															})
																																														],
																																														actions	: ~[],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, -0.00, -0.00, 1.00],
																																															pos	: [0.00, 0.00, 0.00],
																																														},
																																														name	: ~"L_clav_ctrl_zero",
																																													})
																																												],
																																												actions	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [0.16, 0.64, 0.24, 0.71],
																																													pos	: [-1.39, -3.03, -1.00],
																																												},
																																												name	: ~"L_clav_ctrl_grp",
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
																																																																actions	: ~[
																																																																	~"c_jaw_ctrlAction@nodes"
																																																																],
																																																																space	: QuatSpace{
																																																																	scale	: 1.00,
																																																																	rot	: [-0.00, 0.00, -0.00, 1.00],
																																																																	pos	: [0.00, -0.00, 0.00],
																																																																},
																																																																name	: ~"c_jaw_ctrl",
																																																															})
																																																														],
																																																														actions	: ~[],
																																																														space	: QuatSpace{
																																																															scale	: 1.00,
																																																															rot	: [-0.00, 0.00, -0.00, 1.00],
																																																															pos	: [-0.00, 0.00, 0.00],
																																																														},
																																																														name	: ~"c_jaw_ctrl_zero",
																																																													})
																																																												],
																																																												actions	: ~[],
																																																												space	: QuatSpace{
																																																													scale	: 1.00,
																																																													rot	: [0.00, 0.00, -0.82, 0.57],
																																																													pos	: [-0.41, -0.55, 0.00],
																																																												},
																																																												name	: ~"c_jaw_ctrl_grp",
																																																											})
																																																										],
																																																										actions	: ~[
																																																											~"c_neck_03_ctrlAction@nodes"
																																																										],
																																																										space	: QuatSpace{
																																																											scale	: 1.00,
																																																											rot	: [-0.00, -0.00, 0.05, 1.00],
																																																											pos	: [0.00, 0.00, 0.00],
																																																										},
																																																										name	: ~"c_neck_03_ctrl",
																																																									})
																																																								],
																																																								actions	: ~[],
																																																								space	: QuatSpace{
																																																									scale	: 1.00,
																																																									rot	: [0.00, 0.00, 0.00, 1.00],
																																																									pos	: [0.00, 0.00, 0.00],
																																																								},
																																																								name	: ~"c_neck_03_ctrl_zero",
																																																							})
																																																						],
																																																						actions	: ~[],
																																																						space	: QuatSpace{
																																																							scale	: 1.00,
																																																							rot	: [-0.00, -0.00, -0.00, 1.00],
																																																							pos	: [2.37, 0.00, 0.00],
																																																						},
																																																						name	: ~"c_neck_03_ctrl_grp",
																																																					})
																																																				],
																																																				actions	: ~[
																																																					~"c_neck_02_ctrlAction@nodes"
																																																				],
																																																				space	: QuatSpace{
																																																					scale	: 1.00,
																																																					rot	: [-0.00, -0.00, 0.04, 1.00],
																																																					pos	: [0.00, 0.00, -0.00],
																																																				},
																																																				name	: ~"c_neck_02_ctrl",
																																																			})
																																																		],
																																																		actions	: ~[],
																																																		space	: QuatSpace{
																																																			scale	: 1.00,
																																																			rot	: [-0.00, -0.00, -0.00, 1.00],
																																																			pos	: [0.00, 0.00, -0.00],
																																																		},
																																																		name	: ~"c_neck_02_ctrl_zero",
																																																	})
																																																],
																																																actions	: ~[],
																																																space	: QuatSpace{
																																																	scale	: 1.00,
																																																	rot	: [0.00, 0.00, 0.02, 1.00],
																																																	pos	: [2.33, 0.01, -0.00],
																																																},
																																																name	: ~"c_neck_02_ctrl_grp",
																																															})
																																														],
																																														actions	: ~[
																																															~"c_neck_01_ctrlAction@nodes"
																																														],
																																														space	: QuatSpace{
																																															scale	: 1.00,
																																															rot	: [-0.00, -0.00, 0.06, 1.00],
																																															pos	: [0.00, 0.00, -0.00],
																																														},
																																														name	: ~"c_neck_01_ctrl",
																																													})
																																												],
																																												actions	: ~[],
																																												space	: QuatSpace{
																																													scale	: 1.00,
																																													rot	: [-0.00, -0.00, -0.00, 1.00],
																																													pos	: [0.00, 0.00, 0.00],
																																												},
																																												name	: ~"c_neck_01_ctrl_zero",
																																											})
																																										],
																																										actions	: ~[],
																																										space	: QuatSpace{
																																											scale	: 1.00,
																																											rot	: [0.00, 0.00, -0.23, 0.97],
																																											pos	: [6.82, -0.29, 0.00],
																																										},
																																										name	: ~"c_neck_01_ctrl_grp",
																																									})
																																								],
																																								actions	: ~[
																																									~"c_chest_ctrlAction@nodes"
																																								],
																																								space	: QuatSpace{
																																									scale	: 1.00,
																																									rot	: [-0.00, -0.00, -0.00, 1.00],
																																									pos	: [-0.00, 0.00, 0.00],
																																								},
																																								name	: ~"c_chest_ctrl",
																																							})
																																						],
																																						actions	: ~[],
																																						space	: QuatSpace{
																																							scale	: 1.00,
																																							rot	: [-0.00, -0.00, 0.00, 1.00],
																																							pos	: [0.00, 0.00, -0.00],
																																						},
																																						name	: ~"c_chest_ctrl_zero",
																																					})
																																				],
																																				actions	: ~[],
																																				space	: QuatSpace{
																																					scale	: 1.00,
																																					rot	: [-0.00, -0.00, 0.00, 1.00],
																																					pos	: [6.18, -0.00, -0.00],
																																				},
																																				name	: ~"c_chest_ctrl_grp",
																																			})
																																		],
																																		actions	: ~[
																																			~"c_spine_05_ctrlAction@nodes"
																																		],
																																		space	: QuatSpace{
																																			scale	: 1.00,
																																			rot	: [0.57, 0.41, -0.57, 0.41],
																																			pos	: [0.00, -0.00, -0.00],
																																		},
																																		name	: ~"c_spine_05_ctrl",
																																	})
																																],
																																actions	: ~[],
																																space	: QuatSpace{
																																	scale	: 1.00,
																																	rot	: [-0.00, -0.00, -0.00, 1.00],
																																	pos	: [0.00, 0.00, 0.00],
																																},
																																name	: ~"c_spine_05_ctrl_zero",
																															})
																														],
																														actions	: ~[],
																														space	: QuatSpace{
																															scale	: 1.00,
																															rot	: [0.00, 0.00, 0.13, 0.99],
																															pos	: [2.95, 0.06, -0.00],
																														},
																														name	: ~"c_spine_05_ctrl_grp",
																													})
																												],
																												actions	: ~[
																													~"c_spine_03_ctrlAction@nodes"
																												],
																												space	: QuatSpace{
																													scale	: 1.00,
																													rot	: [-0.00, -0.00, -0.00, 1.00],
																													pos	: [-50.12, 3.07, 0.00],
																												},
																												name	: ~"c_spine_03_ctrl",
																											})
																										],
																										actions	: ~[],
																										space	: QuatSpace{
																											scale	: 1.00,
																											rot	: [-0.00, 0.00, -0.00, 1.00],
																											pos	: [0.00, 0.00, 0.00],
																										},
																										name	: ~"c_spine_03_ctrl_zero",
																									})
																								],
																								actions	: ~[],
																								space	: QuatSpace{
																									scale	: 1.00,
																									rot	: [-0.00, 0.00, 0.11, 0.99],
																									pos	: [3.03, 0.02, -0.00],
																								},
																								name	: ~"c_spine_03_ctrl_grp",
																							})
																						],
																						actions	: ~[
																							~"c_spine_01_ctrlAction@nodes"
																						],
																						space	: QuatSpace{
																							scale	: 1.00,
																							rot	: [-0.00, -0.00, 0.08, 1.00],
																							pos	: [-0.00, 0.00, 0.00],
																						},
																						name	: ~"c_spine_01_ctrl",
																					})
																				],
																				actions	: ~[],
																				space	: QuatSpace{
																					scale	: 1.00,
																					rot	: [-0.00, -0.00, -0.00, 1.00],
																					pos	: [0.00, -0.00, 0.00],
																				},
																				name	: ~"c_spine_01_ctrl_zero",
																			})
																		],
																		actions	: ~[],
																		space	: QuatSpace{
																			scale	: 1.00,
																			rot	: [-0.46, -0.54, 0.46, 0.54],
																			pos	: [-0.04, -0.00, 0.00],
																		},
																		name	: ~"c_spine_01_ctrl_grp",
																	})
																],
																actions	: ~[
																	~"c_cog_ctrlAction@nodes"
																],
																space	: QuatSpace{
																	scale	: 1.00,
																	rot	: [-0.00, 0.00, 0.00, 1.00],
																	pos	: [-0.00, -47.22, -0.24],
																},
																name	: ~"c_cog_ctrl",
															})
														],
														actions	: ~[],
														space	: QuatSpace{
															scale	: 1.00,
															rot	: [-0.00, 0.00, -0.00, 1.00],
															pos	: [0.00, -0.00, 0.00],
														},
														name	: ~"c_cog_ctrl_zero",
													})
												],
												actions	: ~[],
												space	: QuatSpace{
													scale	: 1.00,
													rot	: [0.00, 0.00, 0.00, 1.00],
													pos	: [0.00, 47.22, 0.24],
												},
												name	: ~"c_cog_ctrl_grp",
											})
										],
										actions	: ~[
											~"c_worldTransform_ctrlAction@nodes"
										],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"c_worldTransform_ctrl",
									})
								],
								actions	: ~[],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 1.11, 0.00],
								},
								name	: ~"Controls",
							})
						],
						actions	: ~[],
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [-0.00, -1.11, 0.00],
						},
						name	: ~"Transform",
					}),
					ChildNode(Node{
						children	: ~[
							ChildNode(Node{
								children	: ~[
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 528],
												material	: ~"Tongue",
												armature	: ~"",
												mesh	: ~"tongue_geo1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"tongue_geo1",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 15252],
												material	: ~"cloak",
												armature	: ~"",
												mesh	: ~"polySurfaceShape174@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"polySurface172",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 4656],
												material	: ~"Teeth",
												armature	: ~"",
												mesh	: ~"topJaw_geo2Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"topJaw_geo2",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 4248],
												material	: ~"Teeth",
												armature	: ~"",
												mesh	: ~"lowerJaw_geo2Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"lowerJaw_geo2",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 13716],
												material	: ~"EyeLashes",
												armature	: ~"",
												mesh	: ~"L_upper_lash1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"L_upper_lash1",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 8964],
												material	: ~"EyeLashes",
												armature	: ~"",
												mesh	: ~"L_lower_lash1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"L_lower_lash1",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 13716],
												material	: ~"EyeLashes",
												armature	: ~"",
												mesh	: ~"R_upper_lash1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"R_upper_lash1",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 8964],
												material	: ~"EyeLashes",
												armature	: ~"",
												mesh	: ~"R_lower_lash1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"R_lower_lash1",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 2784],
												material	: ~"Eyes",
												armature	: ~"",
												mesh	: ~"L_eye_geo1Shape@all",
											}),
											ChildEntity(Entity{
												range	: [2784, 3264],
												material	: ~"Pupil_SS",
												armature	: ~"",
												mesh	: ~"L_eye_geo1Shape@all",
											}),
											ChildEntity(Entity{
												range	: [3264, 5568],
												material	: ~"cornea",
												armature	: ~"",
												mesh	: ~"L_eye_geo1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"L_eye_geo1",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 2784],
												material	: ~"Eyes",
												armature	: ~"",
												mesh	: ~"R_eye_geo1Shape@all",
											}),
											ChildEntity(Entity{
												range	: [2784, 3264],
												material	: ~"Pupil_SS",
												armature	: ~"",
												mesh	: ~"R_eye_geo1Shape@all",
											}),
											ChildEntity(Entity{
												range	: [3264, 5568],
												material	: ~"cornea",
												armature	: ~"",
												mesh	: ~"R_eye_geo1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"R_eye_geo1",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 6954],
												material	: ~"anisotropic1",
												armature	: ~"",
												mesh	: ~"Hair_Geo2Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"Hair_Geo2",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 50496],
												material	: ~"skin",
												armature	: ~"",
												mesh	: ~"Body_geo8Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"Body_geo8",
									})
								],
								actions	: ~[],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
								name	: ~"Body",
							}),
							ChildNode(Node{
								children	: ~[
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 9042],
												material	: ~"armor",
												armature	: ~"",
												mesh	: ~"R_boot1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, -0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"boots",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 5550],
												material	: ~"armor",
												armature	: ~"",
												mesh	: ~"backShealth1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"backShealth1",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 10236],
												material	: ~"armor",
												armature	: ~"",
												mesh	: ~"R_skirt_06Shape@all",
											}),
											ChildEntity(Entity{
												range	: [10236, 12102],
												material	: ~"skin",
												armature	: ~"",
												mesh	: ~"R_skirt_06Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"skirt",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 8448],
												material	: ~"skin",
												armature	: ~"",
												mesh	: ~"bracket_05_geo1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"bracket",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 2304],
												material	: ~"armor",
												armature	: ~"",
												mesh	: ~"L_bracer1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"bracers",
									}),
									ChildNode(Node{
										children	: ~[
											ChildEntity(Entity{
												range	: [0, 6960],
												material	: ~"armor",
												armature	: ~"",
												mesh	: ~"R_subSpaulder1Shape@all",
											})
										],
										actions	: ~[],
										space	: QuatSpace{
											scale	: 1.00,
											rot	: [0.00, 0.00, 0.00, 1.00],
											pos	: [0.00, 0.00, 0.00],
										},
										name	: ~"spaulders",
									})
								],
								actions	: ~[],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
								name	: ~"Armor",
							}),
							ChildNode(Node{
								children	: ~[],
								actions	: ~[],
								space	: QuatSpace{
									scale	: 1.00,
									rot	: [0.00, 0.00, 0.00, 1.00],
									pos	: [0.00, 0.00, 0.00],
								},
								name	: ~"Eyes_Geo",
							})
						],
						actions	: ~[],
						space	: QuatSpace{
							scale	: 1.00,
							rot	: [0.00, 0.00, 0.00, 1.00],
							pos	: [0.00, 0.00, 0.00],
						},
						name	: ~"noTrasnform",
					})
				],
				actions	: ~[],
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.50, 0.50, 0.50, 0.50],
					pos	: [0.00, 0.00, 0.00],
				},
				name	: ~"Clare",
			}),
			ChildNode(Node{
				children	: ~[
					ChildLight(Light{
						kind	: KindSpot(Spot{
							size	: 1.31,
							blend	: 0.15,
						}),
						attenuation	: [0.00, 1.00],
						spherical	: false,
						distance	: 100.00,
						name	: ~"Lamp",
						energy	: 3.00,
						color	: [1.00, 1.00, 1.00],
					})
				],
				actions	: ~[],
				space	: QuatSpace{
					scale	: 1.00,
					rot	: [0.27, 0.31, 0.78, 0.47],
					pos	: [43.55, 25.15, 80.51],
				},
				name	: ~"Lamp",
			})
		],
		global	: Global{
			gravity	: [0.00, 0.00, -9.81],
		},
		materials	: ~[
			Material{
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.09, 0.09, 0.08])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				name	: ~"anisotropic1",
			},
			Material{
				textures	: ~[
					Texture{
						scale	: [1.00, 1.00, 1.00],
						filter	: 3,
						offset	: [0.00, 0.00, 0.00],
						name	: ~"Main",
						wrap	: 0,
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
				name	: ~"armor",
			},
			Material{
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				name	: ~"cloak",
			},
			Material{
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.50, 0.50, 0.50])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				name	: ~"cornea",
			},
			Material{
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				name	: ~"EyeLashes",
			},
			Material{
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.44, 0.44, 0.54])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.49, 0.49, 0.49])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				name	: ~"Eyes",
			},
			Material{
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.80, 0.80, 0.80])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				name	: ~"Material",
			},
			Material{
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.00, 0.00, 0.00])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				name	: ~"Pupil_SS",
			},
			Material{
				textures	: ~[
					Texture{
						scale	: [1.00, 1.00, 1.00],
						filter	: 3,
						offset	: [0.00, 0.00, 0.00],
						name	: ~"Main.001",
						wrap	: 0,
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
				name	: ~"skin",
			},
			Material{
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.75, 0.75, 0.75])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[1.00, 1.00, 1.00])),
					(	~"SpecularParams",	DataVector(	[0.50, 50.00, 0.00, 1.00]))
				],
				name	: ~"Teeth",
			},
			Material{
				textures	: ~[],
				shader	: ~"phong",
				data	: ~[
					(	~"Ambient",	DataScalar(	1.00)),
					(	~"DiffuseColor",	DataColor(	[0.40, 0.08, 0.08])),
					(	~"DiffuseParams",	DataVector(	[0.80, 0.00, 0.00, 1.00])),
					(	~"SpecularColor",	DataColor(	[0.50, 0.50, 0.50])),
					(	~"SpecularParams",	DataVector(	[0.50, 1.00, 0.00, 1.00]))
				],
				name	: ~"Tongue",
			}
		],
	}}
