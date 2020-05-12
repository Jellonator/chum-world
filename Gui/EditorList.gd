extends VBoxContainer

var cfile = null

func set_tab(id: int, file):
	$TabContainer.get_child(id).set_file(file)
	for i in $TabContainer.get_child_count():
		if i != id:
			$TabContainer.get_child(i).set_file(null)

func set_file(file):
	cfile = file
	set_tab($TabContainer.current_tab, file)

func _on_TabContainer_tab_changed(tab):
	set_tab(tab, cfile)
