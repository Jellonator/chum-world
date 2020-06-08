extends Control

onready var node_viewport := $Viewport
onready var node_camera := $Viewport/Spatial/Camera
onready var node_rect := $TextureRect
onready var node_mesh := $Viewport/Spatial/MeshInstance
onready var mat = node_mesh.get_surface_material(0)

const MIN_SPEED = 0.0125
const MAX_SPEED = 128
const SPEED_MULT = 1.25

var speed = 2.0

func _ready():
	node_viewport.size = node_rect.rect_size
	node_rect.connect("item_rect_changed", self, "_on_TextureRect_item_rect_changed")

func _on_TextureRect_item_rect_changed():
	node_viewport.size = node_rect.rect_size

func set_file(file):
	node_camera.reset_transform()
	print("========================================")
	if file == null:
		node_mesh.hide()
	elif file.type == "MESH":
		var data = ChumReader.read_tmesh(file)
		if data == null:
			print("INVALID DATA")
			node_mesh.hide()
		elif data["exists"]:
			print("LOADED: ", data)
			node_mesh.mesh = data["mesh"]
			node_mesh.show()
		else:
			print("DOES NOT EXIST")
			node_mesh.hide()
	elif file.type == "SURFACE":
		var data = ChumReader.read_surface(file)
		if data == null:
			print("INVALID DATA")
			node_mesh.hide()
		elif data["exists"]:
			print("LOADED: ", data)
			node_mesh.mesh = data["mesh"]
			node_mesh.show()
		else:
			print("DOES NOT EXIST")
			node_mesh.hide()
	else:
		node_mesh.hide()
		print("UNRECOGNIZED TYPE ", file.type)

func _input(event):
	if self.has_focus() or node_rect.has_focus():
		if Input.is_action_pressed("view_look"):
			if event is InputEventMouseMotion:
				node_camera.move_mouse(event.relative)
		if event.is_action_pressed("view_speed_increase"):
			print(randi())
			speed = clamp(speed * SPEED_MULT, MIN_SPEED, MAX_SPEED)
			$SpeedLabel.text = "Speed: " + str(speed)
		if event.is_action_pressed("view_speed_decrease"):
			print(randi())
			speed = clamp(speed / SPEED_MULT, MIN_SPEED, MAX_SPEED)
			$SpeedLabel.text = "Speed: " + str(speed)

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
	node_camera.move_strafe(input_dir * delta * speed)
