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
	node_camera.reset_transform()
	if file == null:
		node_mesh.hide()
	else:
		var data = ChumReader.read_tmesh(file)
		if data == null:
			print("INVALID DATA")
			node_mesh.hide()
		elif data["exists"]:
			print("LOADED: ", data)
			node_mesh.mesh = data["mesh"]
			node_mesh.show()
#			for i in range(node_mesh.get_surface_material_count()):
#				node_mesh.set_surface_material(i, mat)
		else:
			print("DOES NOT EXIST")
			node_mesh.hide()

func _input(event):
	if self.has_focus() or node_rect.has_focus() and Input.is_action_pressed("view_look"):
		if event is InputEventMouseMotion:
			node_camera.move_mouse(event.relative)

func _physics_process(delta):
	var tx = node_camera.get_camera_transform()
	var input_dir := Vector3()
	if Input.is_action_pressed("view_move_forward"):
		input_dir += -tx.basis.z
	if Input.is_action_pressed("view_move_backward"):
		input_dir += tx.basis.z
	if Input.is_action_pressed("view_move_left"):
		input_dir += -tx.basis.x
	if Input.is_action_pressed("view_move_right"):
		input_dir += tx.basis.x
	input_dir = input_dir.normalized()
	if Input.is_action_pressed("view_move_slow"):
		input_dir *= 0.5
	node_camera.move_strafe(input_dir * delta * 2)
