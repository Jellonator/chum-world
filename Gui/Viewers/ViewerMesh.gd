extends "res://Gui/Viewers/Viewer3D.gd"

const SceneData := preload("res://gdchum/SceneData.gdns")

const IMPORT_GLTF := 0

onready var node_menu_button := $HBox/Menu as MenuButton
onready var node_gltf_mesh_select := $Control/Confirm/Margin/VBox/Mesh as OptionButton
onready var popup_gltf_file := $Control/GLTFFileDialog
onready var popup_gltf_no_mesh := $Control/GLTFNoMeshes
onready var popup_gltf_select := $Control/Confirm

var scene: SceneData
var scene_meshes: Dictionary

func _ready():
	node_menu_button.get_popup().connect("id_pressed", self, "_on_import")

func _on_import(id: int):
	if id == IMPORT_GLTF:
		popup_gltf_file.popup_centered()

func _on_GLTFFileDialog_file_selected(path: String):
	scene = SceneData.new()
	scene.load(path)
	scene_meshes = scene.get_mesh_info()
	prints("MESHES", scene_meshes)
	if scene_meshes.size() == 0:
		popup_gltf_no_mesh.popup_centered()
		return
	node_gltf_mesh_select.clear()
	for name in scene_meshes:
		node_gltf_mesh_select.add_item(name)
		node_gltf_mesh_select.select(0)
	popup_gltf_select.popup_centered()

func _on_ConfirmationDialog_confirmed():
	var name = node_gltf_mesh_select.get_item_text(node_gltf_mesh_select.selected)
	var id = scene_meshes[name]["id"]
	scene.import_mesh(id, self.file)
	ChumReader.invalidate(self.file.get_hash_id())
	set_file(self.file)
