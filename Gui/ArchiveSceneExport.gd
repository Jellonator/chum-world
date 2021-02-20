extends ConfirmationDialog

const SceneData := preload("res://gdchum/SceneData.gdns")

var archive

func set_archive(p_archive):
	self.archive = p_archive

onready var node_surface_root := $VBox/SurfaceQuality
onready var node_surface_slider := $VBox/SurfaceQuality/Slider
onready var node_surface_spinbox := $VBox/SurfaceQuality/Label
onready var node_surface_label := $VBox/SurfaceLabel

var _lock = false
func _on_Label_value_changed(value):
	if _lock:
		return
	_lock = true
	node_surface_slider.value = value
	_lock = false

func _on_Slider_value_changed(value):
	if _lock:
		return
	_lock = true
	node_surface_spinbox.value = value
	_lock = false

func _on_Surface_toggled(button_pressed: bool):
	node_surface_slider.editable = button_pressed
	node_surface_spinbox.editable = button_pressed
	if button_pressed:
		node_surface_label.modulate = Color.white
		node_surface_root.modulate = Color.white
	else:
		node_surface_root.modulate = Color.gray
		node_surface_label.modulate = Color.darkgray


func create_node(scene, node_name: String, node_view) -> Array:
	if node_view.parent_id == 0:
		return []
	var parent_fh = archive.get_file_from_hash(node_view.parent_id)
	var parent_view = ChumReader.get_node_view(parent_fh)
	var arr = create_node(scene, parent_fh.name, parent_view)
	if not scene.has_node(arr + [node_name]):
		print("CREATING NODE:  ", arr)
		scene.add_node(arr, node_name, node_view.local_transform)
	else:
		print("ALREADY EXISTS: ", arr)
	arr.push_back(node_name)
	return arr

func _on_SceneExportDialog_confirmed():
#	var dict := {
	var file_name = $VBox/File/FileString.text
#		"include": {
	var include_mesh = $VBox/Include/Mesh.pressed
	var include_skin = $VBox/Include/Skin.pressed
	var include_lod = $VBox/Include/Lod.pressed
	var include_surface = $VBox/Include/Surface.pressed
	var include_rotshape = $VBox/Include/Rotshape.pressed
	var include_light = $VBox/Include/Light.pressed
#		},
	var surface_quality := int(node_surface_spinbox.value)
#	}
	var scene = SceneData.new()
	for file in archive.get_file_list():
		if file.type == "NODE":
			var node_name = file.name
			var node_view = ChumReader.get_node_view(file)
			var resid = node_view.resource_id
			if resid == 0:
				continue
			var resfile = archive.get_file_from_hash(resid)
			if resfile == null:
				continue
			if include_mesh and resfile.type == "MESH":
				var path = create_node(scene, node_name, node_view)
				scene.set_node_mesh(path, resfile)
			if include_surface and resfile.type == "SURFACE":
				var path = create_node(scene, node_name, node_view)
				scene.set_node_surface(path, resfile, surface_quality)
			if include_lod and resfile.type == "LOD":
				var path = create_node(scene, node_name, node_view)
				scene.set_node_lod(path, resfile, surface_quality, false)
			if include_skin and resfile.type == "SKIN":
				var path = create_node(scene, node_name, node_view)
				scene.set_node_skin(path, resfile, surface_quality, false)
			if include_rotshape and resfile.type == "ROTSHAPE":
				pass
			if include_light and resfile.type == "LIGHT":
				pass
			if include_light and resfile.type == "OMNI":
				pass
	scene.add_required_materials(archive)
	scene.export_to(file_name)

func _on_FileSelect_pressed():
	$FileDialog.popup_centered()

func _on_FileDialog_file_selected(path: String):
	$VBox/File/FileString.text = path
