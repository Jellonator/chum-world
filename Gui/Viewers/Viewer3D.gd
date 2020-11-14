extends Control

onready var node_viewport := $Viewport
onready var node_camera := $Viewport/Spatial/Camera
onready var node_rect := $TextureRect
onready var node_mesh := $Viewport/Spatial/MeshInstance
onready var node_spatial := $Viewport/Spatial
onready var mat = node_mesh.get_surface_material(0)

const MIN_SPEED = 0.0125
const MAX_SPEED = 128
const SPEED_MULT = 1.25

var speed = 2.0

#var surfaces = []

func _ready():
	node_viewport.size = node_rect.rect_size
	var err = node_rect.connect("item_rect_changed", self, "_on_TextureRect_item_rect_changed")
	if err != OK:
		push_warning("Connect failed")

func _on_TextureRect_item_rect_changed():
	node_viewport.size = node_rect.rect_size * GlobalConfig.viewport_scale
	node_viewport.set_size_override(true, node_rect.rect_size)

func set_file(file):
	node_camera.reset_transform()
	if node_mesh != null:
		node_mesh.queue_free()
		node_mesh = null
	if file != null:
		node_mesh = MeshData.try_file_to_spatial(file)
		node_spatial.add_child(node_mesh)

func _input(event):
	if node_rect.has_focus():
		if Input.is_action_pressed("view_look"):
			if event is InputEventMouseMotion:
				node_camera.move_mouse(event.relative)
		if event.is_action_pressed("view_speed_increase"):
			speed = clamp(speed * SPEED_MULT, MIN_SPEED, MAX_SPEED)
			$SpeedLabel.text = "Speed: " + str(speed)
		if event.is_action_pressed("view_speed_decrease"):
			speed = clamp(speed / SPEED_MULT, MIN_SPEED, MAX_SPEED)
			$SpeedLabel.text = "Speed: " + str(speed)

func _physics_process(delta):
	if node_rect.has_focus():
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
