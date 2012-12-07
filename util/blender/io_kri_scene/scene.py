__author__ = ['Dzmitry Malyshau']
__bpydoc__ = 'Scene module of Claymore exporter.'

import mathutils
import json
from io_kri.common	import *
from io_kri.action	import *
from io_kri_arm.arm		import save_arm
from io_kri_mesh.mesh	import save_mesh


def cook_mat(mat):
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


def cook_node(ob,log):
	pos,rot,sca = ob.matrix_local.decompose()
	scale = (sca.x + sca.y + sca.z)/3.0
	if sca.x*sca.x+sca.y*sca.y+sca.z*sca.z > 0.01 + sca.x*sca.y+sca.y*sca.z+sca.z*sca.x:
		log.log(1,'w', 'Non-uniform scale: (%.1f,%.1f,%.1f)' % sca.to_tuple(1))
	return {
		'name'	: ob.name,
		'parent': ob.parent.name if ob.parent else '',
		'space'	: {
			'position'		: tuple(pos),
			'orientation'	: tuple(rot),
			'scale'			: scale
		}
	}

def cook_camera_proj(cam,log):
	return {	#todo: ortho
		'fov'	: cam.angle,
		'range'	: (cam.clip_start,cam.clip_end)
	}

def cook_light_proj(lamp,log):
	return {	#todo: non-spot
		'fov'	: lamp.spot_size if lamp.type=='SPOT' else 0,
		'range'	: (1,2*lamp.distance),
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
	log			= Logger(filename+'.log')
	out_mesh	= Writer(filename+'.k3mesh')
	out_arm		= Writer(filename+'.k3arm')
	sc = context.scene
	print('Exporting Scene...')
	log.logu(0,'Scene %s' % (filename))
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
		materials.append( cook_mat(mat) )
		#save_actions( mat, 'm','t' )
	# steady...
	for ob in sc.objects:
		if log.stop:	break
		node = cook_node(ob,log)
		if len(ob.modifiers):
			log.log(1,'w','Unapplied modifiers detected on object %s' % (ob.name))
		if ob.type == 'EMPTY':
			dummies.append(node)
		elif ob.type == 'MESH':
			out_mesh.begin('meta')
			out_mesh.text(ob.data.name)
			(_,face_num) = save_mesh(out_mesh,ob,log)
			out_mesh.end()
			offset = 0
			for fn,m in zip( face_num, ob.data.materials ):
				if not fn: break
				s = (m.name	if m else '')
				log.logu(1, '+entity: %d faces, [%s]' % (fn,s))
				entities.append({
					'node'		: node,
					'material'	: s,
					'mesh'		: ob.data.name,
					'range'		: (offset,offset+fn),
					'has_armature'	: ob.parent and ob.parent.type == 'ARMATURE'
					})
				offset += fn
		elif ob.type == 'ARMATURE':
			out_mesh.begin('meta')
			out_mesh.text(ob.data.name)
			save_arm(out_arm,ob,log)
			out_mesh.end()
			armatures.append({
				'node'	: node,
				'name'	: ob.data.name,
				'dual_quat'	: False,
				})
		elif ob.type == 'CAMERA':
			cameras.append({
				'node'	: node,
				'proj'	: cook_camera_proj(ob.data,log)
				})
		elif ob.type == 'LAMP':
			lights.append({
				'node'	: node,
				'proj'	: cook_light_proj(ob.data,log)
				})
	out_mesh.close()
	out_arm.close()
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
