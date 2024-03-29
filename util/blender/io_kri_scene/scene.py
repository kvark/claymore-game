__author__ = ['Dzmitry Malyshau']
__bpydoc__ = 'Scene module of Claymore exporter.'

import mathutils
import math
from io_kri.common	import *
from io_kri.action	import *
from io_kri_mesh.mesh	import *


def cook_mat(mat,log):
	textures = []
	for mt in mat.texture_slots:
		if mt==None: continue
		it = mt.texture
		if it==None: continue
		if it.type != 'IMAGE':
			log.log(2,'w','Texture "%s": type is not IMAGE' % (it.name))
			continue
		if it.image==None:
			log.log(2,'w','Texture "%s": image is not assigned' % (it.name))
			continue
		textures.append(('Texture',{
			'name'	: mt.name,
			'path'	: it.image.filepath,
			'filter': (1,(2,3)[it.use_mipmap])[it.use_interpolation],
			'wrap'	: 0,
			'scale'	: list(mt.scale),
			'offset': list(mt.offset)
			}))
	kind = 'phong'
	if mat.use_shadeless:
		kind = 'flat'
	elif mat.use_tangent_shading:
		kind = 'anisotropic'
	diff_params = [mat.diffuse_intensity,float(mat.emit),0.0,mat.alpha]
	spec_params = [mat.specular_intensity,float(mat.specular_hardness),0.0,mat.specular_alpha]
	return ('Material',{
		'name'		: mat.name,
		'shader'	: kind,
		'data'		: [
			((), 'Ambient',			('DataScalar',	mat.ambient )),
			((), 'DiffuseColor',	('DataColor',	list(mat.diffuse_color) )),
			((), 'DiffuseParams',	('DataVector',	diff_params )),
			((), 'SpecularColor',	('DataColor',	list(mat.specular_color) )),
			((), 'SpecularParams',	('DataVector',	spec_params )),
		],
		'textures'	: textures
		})


def cook_space(matrix):
	pos,rot,sca = matrix.decompose()
	scale = (sca.x + sca.y + sca.z)/3.0
	if sca.x*sca.x+sca.y*sca.y+sca.z*sca.z > 0.01 + sca.x*sca.y+sca.y*sca.z+sca.z*sca.x:
		log.log(1,'w', 'Non-uniform scale: (%.1f,%.1f,%.1f)' % sca.to_tuple(1))
	return ('Space',{
		'pos'	: list(pos),
		'rot'	: [rot.x,rot.y,rot.z,rot.w],
		'scale'	: scale
		})

def cook_node(ob,log):
	return ('ChildNode','Node',{
		'name'		: ob.name,
		'space'		: cook_space(ob.matrix_local),
		'children'	: [],
		'actions'	: [],
		})

def cook_camera(cam,log):
	return ('ChildCamera','Camera',{	#todo: ortho
		'name'	: cam.name,
		'fov_y'	: cam.angle,	#todo: make sure it's vfov
		'range'	: [cam.clip_start,cam.clip_end]
		})

def cook_lamp(lamp,log):
	attenu = [lamp.linear_attenuation,lamp.quadratic_attenuation]
	sphere = False
	kind = ()
	if lamp.type in ('SPOT','POINT'):
		sphere = lamp.use_sphere
	if lamp.type == 'SPOT':
		kind = ('KindSpot','Spot',{
			'size'	: lamp.spot_size,
			'blend'	: lamp.spot_blend
			})
	elif lamp.type == 'POINT':
		kind = ('KindOmni','Omni',{})
	elif lamp.type == 'AREA':
		#kind = ('KindOmni')
		params = [lamp.size,lamp.size_y,0.1]
	return ('ChildLight','Light',{
		'name'			: lamp.name,
		'kind'			: kind,
		'color'			: list(lamp.color),
		'energy'		: lamp.energy,
		'attenuation'	: attenu,
		'distance'		: lamp.distance,
		'spherical'		: sphere,
		})

def cook_armature(arm,log):
	root = ('Bone',{ 'children':[] })
	bones = { '':root }
	for b in arm.bones:
		par = bones['']
		mx = b.matrix_local
		if b.parent:
			par = bones[b.parent.name]
			mx = b.parent.matrix_local.copy().inverted() * mx
		ob = ('Bone',{
			'name'		: b.name,
			'space'		: cook_space(mx),
			'children'	: []
			})
		par[1]['children'].append(ob)
		bones[b.name] = ob
	return ('ChildArmature','Armature',{
		'name'		: arm.name,
		'dual_quat'	: False,
		'bones'		: root[1]['children'],
		'actions'	: []
		})


def export_value(elem,ofile,num_format,level):
	import collections
	#print('Exporting:',str(elem))
	new_line = '\n%s' % (level * '\t')
	tip = type(elem)
	if tip is tuple:
		last = elem[len(elem)-1]
		if type(last) is dict:	# object
			assert len(elem) <= 3
			name = elem[0]
			if len(elem) == 3:
				name = elem[1]
				ofile.write( elem[0] + '(' )
			ofile.write(name)
			if len(last):
				ofile.write( '{' )
				for key,val in last.items():
					ofile.write( '%s\t%s\t: ' % (new_line, key))
					export_value( val, ofile, num_format, level+1 )
					ofile.write( ',' )
				ofile.write( new_line + '}' )
			if len(elem) == 3:
				ofile.write( ')' )
		else:
			if type(elem[0]) is str:	# enum element
				ofile.write( elem[0] )
			if len(elem)>1:
				ofile.write( '(\t' )
				for sub in elem[1:]:
					export_value( sub, ofile, num_format, level+1 )
					if not (sub is last):
						ofile.write(',\t')
				ofile.write( ')' )
		#else:
			#raise Exception( 'Composite element %s is unknown' % (str(elem)))
	elif tip is bool:
		ofile.write( ('false','true')[elem] )
	elif tip is int:
		ofile.write( str(elem) )
	elif tip is float:
		ofile.write( num_format % (elem) )
	elif tip is str:
		ofile.write( '~"%s"' % (elem) )
	elif tip is list:
		if len(elem):
			subtip = type(elem[0])
			is_class = subtip in (tuple,dict,list,str)
			ofile.write( ('[','~[')[is_class] )
			for i,sub in enumerate(elem):
				assert type(sub) == subtip
				if is_class:
					ofile.write( new_line + '\t' )
				export_value( sub, ofile, num_format, level+1)
				if i+1 != len(elem):
					ofile.write( (', ',',')[is_class] )
			if is_class:
				ofile.write( new_line )
			ofile.write(']')
		else:
			ofile.write('~[]')
	else:
		ofile.write( '0' )
		raise Exception( 'Element %s is unknown' % (str(type(elem))) )


def export_doc(document,filepath,num_format):
	ofile = open(filepath+'.rs','w')
	ofile.write('use common::*;\n')
	ofile.write('pub fn load()-> Scene	{')
	export_value(document, ofile, num_format, 1)
	ofile.write('}\n')
	ofile.close()


def export_json(document,filepath,num_format):
	import json
	class KriEncoder(json.JSONEncoder):
		def default(self,obj):
			if isinstance(obj,float):
				return num_format % obj
			return json.JSONEncoder.default(self,obj)
	json.encoder.FLOAT_REPR = lambda o: num_format % (o)
	text = json.dumps(document, indent="\t", allow_nan=False, cls=KriEncoder)
	file = open(filepath+'.json','w')
	file.write(text)
	file.close()


def save_scene(filepath,context,export_meshes,export_actions,precision):
	glob		= ('Global',{})
	materials	= []
	nodes		= []
	# ready...
	import os
	if not os.path.exists(filepath):
		os.makedirs(filepath)
	log	= Logger(filepath+'.log')
	out_mesh,out_act_node,out_act_arm = None,None,None
	collection_mesh, collection_node_anim = 'all','nodes'
	if export_meshes:
		out_mesh	= Writer('%s/%s.k3mesh' % (filepath,collection_mesh))
		out_mesh.begin('*mesh')
	if export_actions:
		out_act_node= Writer('%s/%s.k3act' % (filepath,collection_node_anim))
		out_act_node.begin('*action')
	sc = context.scene
	print('Exporting Scene...')
	log.logu(0,'Scene %s' % (filepath))
	# -globals
	bDegrees = (sc.unit_settings.system_rotation == 'DEGREES')
	if not bDegrees:
		#it's easier to convert on loading than here
		log.log(1,'w','Radians are not supported')
	if sc.use_gravity:
		gv = sc.gravity
		log.log(1,'i', 'gravity: (%.1f,%.1f,%.1f)' % (gv.x,gv.y,gv.z))
		glob[1]['gravity'] = list(sc.gravity)
	else:
		glob[1]['gravity'] = [0,0,0]
	# -materials
	for mat in context.blend_data.materials:
		if log.stop:	break
		materials.append( cook_mat(mat,log) )
		#save_actions( mat, 'm','t' )
	# steady...
	node_map = {}
	for ob in sc.objects:
		if log.stop:	break
		if len(ob.modifiers):
			log.log(1,'w','Unapplied modifiers detected on object %s' % (ob.name))
		node_map[ob.name] = node = cook_node(ob,log)
		anims = save_actions_int( out_act_node,ob,None,log )
		for ani in anims:
			node[2]['actions'].append( '%s@%s' % (ani,collection_node_anim) )
		if ob.parent == None:
			nodes.append(node)
		else:
			node_map[ob.parent.name][2]['children'].append(node)
		children = node[2]['children']
		# parse node
		if ob.type == 'MESH':
			if out_mesh != None:
				(_,face_num) = save_mesh(out_mesh,ob,log)
			else:
				(_,face_num) = collect_attributes(ob.data,None,ob.vertex_groups,True,log)
			offset = 0
			for fn,m in zip( face_num, ob.data.materials ):
				if not fn: break
				s = (m.name	if m else '')
				log.logu(1, '+entity: %d faces, [%s]' % (fn,s))
				has_arm = ob.parent and ob.parent.type == 'ARMATURE'
				arm_name = ob.parent.data.name if has_arm else ''
				children.append(('ChildEntity','Entity',{
					'material'	: s,
					'mesh'		: '%s@%s' % (ob.data.name,collection_mesh),
					'range'		: [3*offset,3*(offset+fn)],
					'armature'	: arm_name
					}))
				offset += fn
		elif ob.type == 'ARMATURE':
			arm = cook_armature(ob.data,log)
			children.append(arm)
			name = ob.data.name
			ani_path = (None, '%s/%s' % (filepath,name))[export_actions]
			anims = save_actions_ext( ani_path,ob,'pose',log )
			for ani in anims:
				arm[2]['actions'].append( '%s@%s' % (ani,name) )
		elif ob.type == 'CAMERA':
			children.append( cook_camera(ob.data,log) )
		elif ob.type == 'LAMP':
			children.append( cook_lamp(ob.data,log) )
	if out_mesh != None:
		out_mesh.end()
		out_mesh.close()
	if out_act_node != None:
		out_act_node.end()
		out_act_node.close()
	# animations
	# go!
	document = ('Scene',{
		'global'	: glob,
		'materials'	: materials,
		'nodes'		: nodes,
	})
	num_format = '%' + ('.%df' % precision)
	export_doc(document, filepath, num_format)
	print('Done.')
	log.conclude()
