extends TabContainer

const TAB_DEFAULT := 0
const TAB_VIEWER3D := 1
const TAB_TEXTURE := 2

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
			"MESH":
				set_tab(TAB_VIEWER3D, file)
			"BITMAP":
				set_tab(TAB_TEXTURE, file)
			_:
				set_tab(TAB_DEFAULT, file)
