__author__ = ['Dzmitry Malyshau']
__bpydoc__ = 'Scene module of Claymore exporter.'

import mathutils
import json
from io_kri.common	import *
from io_kri.action	import *
from io_kri_arm.arm		import save_arm
from io_kri_mesh.mesh	import save_mesh


def save_mat(mat):
	textures = []
	for mt in mat.texture_slots:
		if mt==None: continue
		it = mt.texture
		if it==None: continue
		textures.append({
			'name'	:mt.name,
			'path'	:it.image.filepath,
			'filter':(1,(2,3)[it.use_mipmap])[it.use_interpolation],
			'wrap'	:0,
			'scale'	:tuple(mt.scale),
			'offset':tuple(mt.offset)
			})
	kind = 'phong'
	if mat.use_shadeless:
		kind = 'unshaded'
	elif mat.use_tangent_shading:
		kind = 'anisotropic'
	return {
		'name'	: mat.name,
		'kind' 	: kind,
		'data'		: [
			{
				'name'	: 'Ambient',
				'type'	: 'scalar',
				'value'	: mat.ambient
			},
			{
				'name'	: 'DiffuseColor',
				'type'	: 'vec3',
				'value'	: tuple(mat.diffuse_color)
			},
			{
				'name'	: 'DiffuseParams',
				'type'	: 'vec4',
				'value'	: (mat.diffuse_intensity,mat.emit,0,mat.alpha)
			},
			{
				'name'	: 'SpecularColor',
				'type'	: 'vec3',
				'value'	: tuple(mat.diffuse_color)
			},
			{
				'name'	: 'SpecularParams',
				'type'	: 'vec4',
				'value'	: (mat.specular_intensity,mat.specular_hardness,0,mat.specular_alpha)
			}],
		'textures'	: textures
	}


def save_scene(filename,context):
	glob		= {}
	materials	= []
	dummies		= []
	armatures	= []
	entities	= []
	cameras		= []
	lights		= []
	# ready...
	log			= Logger()
	oMesh		= Writer(filename+'.k3mesh')
	oArmature	= Writer(filename+'.k3arm')
	sc = context.scene
	# -globals
	bDegrees = (sc.unit_settings.system_rotation == 'DEGREES')
	if not bDegrees:
		#it's easier to convert on loading than here
		log.log(1,'w','Radians are not supported')
	if sc.use_gravity:
		gv = sc.gravity
		log.log(1,'i', 'gravity: (%.1f,%.1f,%.1f)' % (gv.x,gv.y,gv.z))
		glob['gravity'] = tuple(sc.gravity)
	else:
		glob['gravity'] = (0,0,0)
	# -materials
	for mat in context.blend_data.materials:
		if log.stop:	break
		materials.append( save_mat(mat) )
		#save_actions( mat, 'm','t' )
	# steady...
	# animations
	# go!
	document = {
		'global'	: glob,
		'materials'	: materials,
		'dummies'	: dummies,
		'armatures'	: armatures,
		'entities'	: entities,
		'cameras'	: cameras,
		'lights'	: lights
	}
	text = json.dumps(document, indent=2);
	file = open(filename+'.json','w')
	file.write(text)
	file.close()
	print('Done.')
	log.conclude()
