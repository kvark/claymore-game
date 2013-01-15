__author__ = ['Dzmitry Malyshau']
__bpydoc__ = 'Scene module of Claymore exporter.'

import mathutils
import json
import math
from io_kri.common	import *
from io_kri.action	import *
from io_kri_arm.arm		import *
from io_kri_mesh.mesh	import *


def cook_mat(mat):
	textures = []
	for mt in mat.texture_slots:
		if mt==None: continue
		it = mt.texture
		if it==None: continue
		textures.append({
			#'name'	:mt.name,
			'name'	:'Main',
			'path'	:it.image.filepath,
			'filter':(1,(2,3)[it.use_mipmap])[it.use_interpolation],
			'wrap'	:0,
			'scale'	:tuple(mt.scale),
			'offset':tuple(mt.offset)
			})
	kind = ('flat','phong')[len(textures)!=0]
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
		'space'	: {
			'position'		: tuple(pos),
			'orientation'	: tuple(rot),
			'scale'			: scale
		},
		'children'	: []
	}

def cook_camera_proj(cam,log):
	return {	#todo: ortho
		'fov'	: cam.angle,
		'range'	: (cam.clip_start,cam.clip_end)
	}


def save_scene(filename,context,export_meshes,export_armatures,precision):
	glob		= {}
	materials	= []
	nodes		= []
	entities	= []
	cameras		= []
	lights		= []
	# ready...
	log	= Logger(filename+'.log')
	if export_meshes:
		out_mesh	= Writer(filename+'.k3mesh')
		out_mesh.begin('*mesh')
	else:
		out_mesh	= None
	if export_armatures:
		out_arm		= Writer(filename+'.k3arm')
		out_arm.begin('*arm')
	else:
		out_arm		= None
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
	# nodes
	node_tree = {}
	for ob in sc.objects:
		node_tree[ob.name] = n = cook_node(ob,log)
		if ob.parent == None:
			nodes.append(n)
		else:
			node_tree[ob.parent.name]['children'].append(n)
	del node_tree
	# steady...
	for ob in sc.objects:
		if log.stop:	break
		if len(ob.modifiers):
			log.log(1,'w','Unapplied modifiers detected on object %s' % (ob.name))
		if ob.type == 'MESH':
			if out_mesh != None:
				out_mesh.begin('meta')
				out_mesh.text(ob.data.name)
				(_,face_num) = save_mesh(out_mesh,ob,log)
				out_mesh.end()
			else:
				(_,face_num) = collect_attributes(ob.data,None,ob.vertex_groups,True,log)
			offset = 0
			for fn,m in zip( face_num, ob.data.materials ):
				if not fn: break
				s = (m.name	if m else '')
				log.logu(1, '+entity: %d faces, [%s]' % (fn,s))
				has_arm = ob.parent and ob.parent.type == 'ARMATURE'
				arm_name = ob.parent.data.name if has_arm else ''
				entities.append({
					'node'		: ob.name,
					'material'	: s,
					'mesh'		: ob.data.name + '@',
					'range'		: (3*offset,3*(offset+fn)),
					'armature'	: arm_name
					})
				offset += fn
		elif ob.type == 'ARMATURE' and out_arm != None:
			out_arm.begin('meta')
			out_arm.text(ob.data.name)
			out_arm.text(ob.name)	# root node
			out_arm.pack('B', 0)	# dual-quat
			save_arm(out_arm,ob,log)
			out_arm.end()
		elif ob.type == 'CAMERA':
			cameras.append({
				'node'	: ob.name,
				'proj'	: cook_camera_proj(ob.data,log)
				})
		elif ob.type == 'LAMP':
			lamp = ob.data
			attenu = [lamp.linear_attenuation,lamp.quadratic_attenuation]
			params = []
			sphere = False
			if lamp.type in ('SPOT','POINT'):
				sphere = lamp.use_sphere
			if lamp.type == 'SPOT':
				params = [lamp.spot_size,lamp.spot_blend]
			elif lamp.type == 'AREA':
				params = [lamp.size,lamp.size_y,0.1]
			lights.append({
				'node'	: ob.name,
				'color'	: tuple(lamp.color),
				'distance'	: lamp.distance,
				'energy': lamp.energy,
				'attenu': attenu,
				'sphere': sphere,
				'kind'	: lamp.type,
				'params': params,
				})
	if out_mesh != None:
		out_mesh.end()
		out_mesh.close()
	if out_arm != None:
		out_arm.end()
		out_arm.close()
	# animations
	# go!
	document = {
		'global'	: glob,
		'materials'	: materials,
		'nodes'		: nodes,
		'entities'	: entities,
		'cameras'	: cameras,
		'lights'	: lights
	}
	num_format = '%' + ('.%df' % precision)
	class KriEncoder(json.JSONEncoder):
		def default(self,obj):
			if isinstance(obj,float):
				return num_format % obj
			return json.JSONEncoder.default(self,obj)
	json.encoder.FLOAT_REPR = lambda o: num_format % (o)
	text = json.dumps(document, indent="\t", cls=KriEncoder)
	file = open(filename+'.json','w')
	file.write(text)
	file.close()
	print('Done.')
	log.conclude()
