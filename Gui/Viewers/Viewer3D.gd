extends Control

onready var node_viewport := $Viewport
onready var node_camera := $Viewport/Spatial/Camera
onready var node_rect := $TextureRect
onready var node_mesh := $Viewport/Spatial/MeshInstance
onready var mat = node_mesh.get_surface_material(0)

func _ready():
	node_viewport.size = node_rect.rect_size
	node_rect.connect("item_rect_changed", self, "_on_TextureRect_item_rect_changed")

func _on_TextureRect_item_rect_changed():
	node_viewport.size = node_rect.rect_size

func set_file(file):
	if file == null:
		node_mesh.hide()
	else:
		var data = ChumReader.read_tmesh(file.data)
		if data == null:
			print("INVALID DATA")
			node_mesh.hide()
		elif data["exists"]:
			print("LOADED: ", data)
			node_mesh.mesh = data["mesh"]
			node_mesh.show()
			for i in range(node_mesh.get_surface_material_count()):
				node_mesh.set_surface_material(i, mat)
		else:
			print("DOES NOT EXIST")
			node_mesh.hide()
