extends Control

const SCENE_EMPTYNODE = preload("res://Gui/Worldview/EmptyNode.tscn")

var archive = null
var archive_files = []
var node_draws = []

onready var node_surfaces := $Viewport/Surfaces
onready var node_camera := $Viewport/CameraViewer
onready var node_rect := $PanelContainer/TextureRect
onready var node_viewport := $Viewport
onready var node_speed := $PanelContainer/TextureRect/SpeedLabel
onready var node_draw := $Viewport/Draw

const MIN_SPEED = 0.0125
const MAX_SPEED = 128
const SPEED_MULT = 1.25

var speed = 2.0
var show_node_names := false

func get_simple_name(name: String, ftype: String) -> String:
	var a = name.find_last(">")
	if a != -1:
		name = name.substr(a+1, -1)
	var b = name.find(".")
	if b != -1:
		name = name.substr(0, b)
	if ftype != "":
		name = name + ":" + ftype
	return name

func try_add_node(nodedata: Dictionary, name: String):
	var node_base = Spatial.new()
	node_base.transform = nodedata["global_transform"]
	var ftype := ""
	var resid = nodedata["resource_id"]
	if resid != 0:
		var resfile = archive.get_file_from_hash(resid)
		if resfile == null:
			print("Could not load file ", resid, " from archive")
		else:
			ftype = resfile.type
			var child = MeshData.try_file_to_spatial(resfile)
			if child != null:
				node_base.add_child(child)
	node_surfaces.add_child(node_base)
	node_draws.append({
		"node": node_base,
		"name": get_simple_name(name, ftype),
		"type": ftype
	})

func reset_surfaces():
	node_draws.clear()
	for child in node_surfaces.get_children():
		child.queue_free()
	for file in archive_files:
		if file.type == "NODE":
			var node_data = ChumReader.read_node(file)
			if not node_data["exists"]:
				print("COULD NOT READ ", file.name)
			else:
				try_add_node(node_data["node"], file.name)

func set_archive(p_archive):
	self.archive = p_archive
	self.archive_files = archive.get_file_list()
	reset_surfaces()

func _on_TextureRect_item_rect_changed():
	node_viewport.size = node_rect.rect_size

func _input(event):
	if node_rect.has_focus():
		if Input.is_action_pressed("view_look"):
			if event is InputEventMouseMotion:
				node_camera.move_mouse(event.relative)
		if event.is_action_pressed("view_speed_increase"):
			speed = clamp(speed * SPEED_MULT, MIN_SPEED, MAX_SPEED)
			node_speed.text = "Speed: " + str(speed)
		if event.is_action_pressed("view_speed_decrease"):
			speed = clamp(speed / SPEED_MULT, MIN_SPEED, MAX_SPEED)
			node_speed.text = "Speed: " + str(speed)

func _physics_process(delta: float):
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
		node_draw.update()

const FONT := preload("res://Font/Base.tres")
const DIST_MAX := 35
const DIST_MIN := 25

func _on_Draw_draw():
	if not show_node_names:
		return
	var camera = node_viewport.get_camera()
	for data in node_draws:
		var node = data["node"]
		var distance = node.global_transform.origin.distance_to(camera.global_transform.origin)
#		if distance >= DIST_MAX:
#			continue
		var alpha = 1.0
		if distance > DIST_MIN:
			alpha = lerp(1.0, 0.0, (distance-DIST_MIN)/(DIST_MAX-DIST_MIN))
		var color = Color(1.0, 1.0, 1.0, alpha)
		if not camera.is_position_behind(node.transform.origin):
			var screen_pos = camera.unproject_position(node.transform.origin)
			node_draw.draw_string(FONT, screen_pos, data["name"], color)

func _on_CheckButton_toggled(button_pressed):
	show_node_names = button_pressed

func _ready():
	$PanelContainer/TextureRect/Controls/CheckButton.pressed = show_node_names
