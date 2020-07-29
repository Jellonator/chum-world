extends TabContainer

const TAB_DEFAULT := 0
const TAB_VIEWER3D := 1
const TAB_TEXTURE := 2
const TAB_MATERIAL := 3
const TAB_SKIN := 4

func set_tab(id: int, file):
	get_child(id).set_file(file)
	for i in get_child_count():
		if i != id:
			get_child(i).set_file(null)
	set_current_tab(id)

func set_file(file):
	if file == null:
		set_tab(TAB_DEFAULT, file)
	else:
		match file.type:
			"MESH", "SURFACE", "LOD", "ROTSHAPE", "SPLINE":
				set_tab(TAB_VIEWER3D, file)
			"BITMAP":
				set_tab(TAB_TEXTURE, file)
			"MATERIAL", "MATERIALANIM":
				set_tab(TAB_MATERIAL, file)
			"SKIN":
				set_tab(TAB_SKIN, file)
			_:
				set_tab(TAB_DEFAULT, file)
# FILES WITHOUT VIEWERS:
# ANIMATION*
# CAMERA
# CAMERAZONE
# COLLISIONVOL
# GAMEOBJ
# HFOG
# LIGHT
# MATERIALOBJ
# NODE (appears in 3d view)
# OCCLUDER
# OMNI
# PARTICLES*
# RTC
# SOUND*
# TXT (appears in text editor)
# USERDEFINE (appears in text editor)
# WARP
# WORLD
# 
# * = files that could eventually get a custom viewer
