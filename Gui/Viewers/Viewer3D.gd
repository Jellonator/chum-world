extends Control

onready var node_viewport := $"/root/Viewer/Viewport"
onready var node_camera := $"/root/Viewer/Viewport/Spatial/Camera"
export var node_rect = NodePath()
export var speed_label = NodePath()
# onready var node_mesh := $"/root/GlobalViewport/Spatial/MeshInstance"
onready var node_meshes := $"/root/Viewer/Viewport/Spatial/Meshes"
# onready var mat = node_mesh.get_surface_material(0)

const MIN_SPEED = pow(2, -8)
const MAX_SPEED = pow(2, 8)
const SPEED_MULT = sqrt(2.0)

var speed = 2.0

var mesh = null
var file = null

func _ready():
	node_rect = get_node(node_rect)
	speed_label = get_node(speed_label)
	node_rect.texture = node_viewport.get_texture()
	node_rect.texture.flags = Texture.FLAG_FILTER
	var err = node_rect.connect("item_rect_changed", self, "_on_TextureRect_item_rect_changed")
	if err != OK:
		push_warning("Connect failed")

func _on_TextureRect_item_rect_changed():
	if visible:
		node_viewport.size = node_rect.rect_size * GlobalConfig.viewport_scale
		node_viewport.set_size_override(true, node_rect.rect_size)

func set_file(file):
	self.file = file
	_on_TextureRect_item_rect_changed()
#	for child in node_meshes.get_children():
#		child.queue_free()
	if mesh != null:
		mesh.queue_free()
		mesh = null
	node_camera.reset_transform()
	if file != null:
		mesh = MeshData.try_file_to_spatial(file)
		node_meshes.add_child(mesh)

func _input(event):
	if node_rect.has_focus():
		if Input.is_action_pressed("view_look"):
			if event is InputEventMouseMotion:
				node_camera.move_mouse(event.relative)
		if event.is_action_pressed("view_speed_increase"):
			speed = clamp(speed * SPEED_MULT, MIN_SPEED, MAX_SPEED)
			speed_label.text = "Speed: " + str(speed)
		if event.is_action_pressed("view_speed_decrease"):
			speed = clamp(speed / SPEED_MULT, MIN_SPEED, MAX_SPEED)
			speed_label.text = "Speed: " + str(speed)

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
