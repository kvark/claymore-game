# <pep8 compliant>

bl_info = {
    'name': 'Claymore scene',
    'author': 'Dzmitry Malyshau',
    'version': (0, 1, 0),
    'blender': (2, 6, 2),
    'api': 36079,
    'location': 'File > Export > Claymore Scene',
    'description': 'Export the scene to Claymore engine (.json,.k3arm,.k3mesh)',
    'warning': '',
    'tracker_url': '',
    'category': 'Import-Export'}

# To support reload properly, try to access a package var, if it's there, reload everything
if 'bpy' in locals():
	import imp
	if 'export_kri_scene' in locals():
	        imp.reload(export_kri_scene)


import bpy
from bpy.props			import *
from bpy_extras.io_utils	import ImportHelper, ExportHelper
from io_kri.common		import Settings
from io_kri_scene.scene	import save_scene


class ExportScene( bpy.types.Operator, ExportHelper ):
	'''Export the scene to Claymore engine'''
	bl_idname	= 'export_scene.kri_scene'
	bl_label	= '-= Claymore scene =-'
	filename_ext	= ''

	# scene settings
	export_meshes	= BoolProperty( name='Export meshes',
		description='Dump mesh binary collection',
		default=True )
	export_arms		= BoolProperty( name='Export armatures',
		description='Dump armatures binary collection',
		default=True )
	num_precision	= IntProperty( name='Numeric precision',
		description='Number of digit past the dot for numerics',
		default=3, min=0, max=10)
	# general settings
	filepath	= StringProperty( name='File Path',
		description='Filepath used for exporting the Claymore scene',
		maxlen=1024, default='')
	show_info	= BoolProperty( name='Show infos',
		description='Print information messages (i)',
		default=Settings.showInfo )
	show_warn	= BoolProperty( name='Show warnings',
		description='Print warning messages (w)',
		default=Settings.showWarning )
	break_err	= BoolProperty( name='Break on error',
		description='Stop the process on first error',
		default=Settings.breakError )
	# mesh settings
	put_normal	= BoolProperty( name='Put normals',
		description='Export vertex normals',
		default=Settings.putNormal )
	put_tangent	= BoolProperty( name='Put tangents',
		description='Export vertex tangents',
		default=Settings.putTangent )
	put_quat	= BoolProperty( name='Put quaternions',
		description='Export vertex quaternions',
		default=Settings.putQuat )
	put_uv		= BoolProperty( name='Put UV layers',
		description='Export vertex UVs',
		default=Settings.putUv )
	norm_uv		= BoolProperty( name='Assume UV in range [0,1]',
		description='Assume UV layers are normalized. That allows using fixed-point data representation',
		default=Settings.normUv )
	put_color	= BoolProperty( name='Put color layers',
		description='Export vertex colors',
		default=Settings.putColor )
	quat_fake	= EnumProperty( name='Fake quaternions',
		description='Derive quaternions from normals only',
		items=(
			('Never','Never','Dont fake anything'),
			('Auto','Auto','Fake if no UV is given'),
			('Force','Force','Always fake')
			), default='Auto' )
	quat_int	= BoolProperty( name='Process quaternions',
		description='Prepare mesh quaternions for interpolation',
		default=Settings.doQuatInt )

	def execute(self, context):
		Settings.showInfo	= self.properties.show_info
		Settings.showWarning= self.properties.show_warn
		Settings.breakError	= self.properties.break_err
		Settings.putNormal	= self.properties.put_normal
		Settings.putTangent	= self.properties.put_tangent
		Settings.putQuat	= self.properties.put_quat
		Settings.putUv		= self.properties.put_uv
		Settings.normUv		= self.properties.norm_uv
		Settings.putColor	= self.properties.put_color
		Settings.doQuatInt	= self.properties.quat_int
		Settings.fakeQuat	= self.properties.quat_fake
		save_scene( self.properties.filepath, context,
			self.properties.export_meshes,
			self.properties.export_arms,
			self.properties.num_precision )
		return {'FINISHED'}


# Add to a menu
def menu_func(self, context):
	self.layout.operator( ExportScene.bl_idname, text=ExportScene.bl_label )

def register():
	bpy.utils.register_module(__name__)
	bpy.types.INFO_MT_file_export.append(menu_func)

def unregister():
	bpy.utils.unregister_module(__name__)
	bpy.types.INFO_MT_file_export.remove(menu_func)

if __name__ == '__main__':
	register()
