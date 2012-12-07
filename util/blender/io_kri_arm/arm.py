__author__ = ['Dzmitry Malyshau']
__bpydoc__ = 'Armature module of KRI exporter.'

import mathutils
from io_kri.common	import *
from io_kri.action	import *

def save_arm(out,ob,log):
	skel = ob.data
	print('Exporting Armature.')
	log.logu(0,'Armature %s' % (ob.data.name))
	out.begin('k3arm')
	nbon = len(skel.bones)
	log.logu(1,'%d bones' % (nbon))
	# go!
	out.pack('B', nbon)
	for bone in skel.bones:
		parid,par,mx = -1, bone.parent, bone.matrix_local.copy()
		if not (bone.use_inherit_scale and bone.use_deform):
			log.log(2,'w','weird bone: %s' % (bone.name))
		if par:
			parid = skel.bones.keys().index( par.name )
			mx = par.matrix_local.copy().inverted() * mx
		out.text( bone.name )
		out.pack('B', parid+1)
		save_matrix(out,mx)
	# animations
	save_actions(out,ob,log)
	# done
	out.end();	#k3arm
	print('Done.')
