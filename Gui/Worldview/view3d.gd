extends Control

const SCENE_EMPTYNODE = preload("res://Gui/Worldview/EmptyNode.tscn")

var archive = null
var archive_files = []
var tnodes_by_id := {}
#var tnode_children := {}
var tnode_root = null
var selected_node = null

onready var node_surfaces := $Viewport/Surfaces
onready var node_camera := $Viewport/CameraViewer
onready var node_rect := $PanelContainer/TextureRect
onready var node_viewport := $Viewport
onready var node_speed := $PanelContainer/TextureRect/SpeedLabel
onready var node_draw := $Viewport/Draw
onready var node_tree := $Tree/Items
onready var node_temp := $Viewport/Temp

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

func try_add_node(nodedata: Dictionary, file):
	var node_base = Spatial.new()
	node_base.transform = nodedata["global_transform"]
	var ftype := ""
	var resid = nodedata["resource_id"]
	var meshes = []
	if resid != 0:
		var resfile = archive.get_file_from_hash(resid)
		if resfile == null:
			var child = MeshData.load_emptymesh(meshes)
			node_base.add_child(child)
			MessageOverlay.push_warn(
				"Could not load file %d from archive" % resid)
		else:
			ftype = resfile.type
			var child = MeshData.try_file_to_spatial(resfile, meshes)
			if child != null:
				node_base.add_child(child)
	else:
		var child = MeshData.load_emptymesh(meshes)
		node_base.add_child(child)
	node_surfaces.add_child(node_base)
	tnodes_by_id[file.get_hash_id()] = {
		"node": node_base,
		"name": get_simple_name(file.name, ftype),
		"type": ftype,
		"parent": nodedata["parent_id"],
		"id": file.get_hash_id(),
		"file": file,
		"children": [],
		"meshes": meshes
	}

func reset_surfaces():
	tnodes_by_id.clear()
	tnode_root = null
	selected_node = null
	for child in node_surfaces.get_children():
		child.queue_free()
	for file in archive_files:
		if file.type == "NODE":
			var node_data = ChumReader.read_node(file)
			if not node_data["exists"]:
				print("COULD NOT READ ", file.name)
			else:
				try_add_node(node_data["node"], file)
	for data in tnodes_by_id.values():
		var parentid = data["parent"]
		if parentid == 0:
			print("ROOT: ", data["file"].name)
			tnode_root = data
		elif parentid in tnodes_by_id:
			tnodes_by_id[parentid]["children"].append(data)
		else:
			print("INVALID PARENT ID ", parentid)
	node_tree.assemble_tree(tnode_root)

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
		if event.is_action_pressed("view_focus"):
			camera_focus_to(selected_node)
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
const DIST_MAX := 35.0
const DIST_MIN := 25.0

func draw_node_label(camera, text: String, position: Vector3, color: Color):
	if not camera.is_position_behind(position):
		var size = FONT.get_string_size(text)
		var screen_pos = camera.unproject_position(position)
		# draw BG
		var rect := Rect2(screen_pos - Vector2(size.x/2, 4), size)
		rect = rect.grow_individual(4.0, 0.0, 4.0, 0.0)
		var bgcolor := Color(0, 0, 0, 0.6 * color.a)
		node_draw.draw_rect(rect, bgcolor)
		# draw FG
		screen_pos.x -= size.x / 2
		screen_pos.y += size.y / 2
		node_draw.draw_string(FONT, screen_pos, text, color)

func _on_Draw_draw():
	var camera = node_viewport.get_camera()
	if not show_node_names:
		if selected_node != null:
			var color = Color(1.0, 1.0, 1.0, 1.0)
			var pos = selected_node["node"].global_transform.origin
			draw_node_label(camera, selected_node["name"], pos, color)
		return
	for data in tnodes_by_id.values():
		var node = data["node"]
		var distance = node.global_transform.origin.distance_to(camera.global_transform.origin)
		if distance >= DIST_MAX:
			continue
		var alpha = 1.0
		if distance > DIST_MIN:
			alpha = lerp(1.0, 0.0, (distance-DIST_MIN)/(DIST_MAX-DIST_MIN))
		var color = Color(1.0, 1.0, 1.0, alpha)
		draw_node_label(camera, data["name"], node.global_transform.origin, color)

func _on_CheckButton_toggled(button_pressed):
	show_node_names = button_pressed

func _ready():
	$PanelContainer/TextureRect/Controls/CheckButton.pressed = show_node_names

func _on_Items_node_selected(node):
	print("SELECTED ", node["name"])
	if selected_node != null:
		set_node_focus_material(selected_node, false)
	selected_node = node
	if selected_node != null:
		set_node_focus_material(selected_node, true)
	if node != null:
		camera_focus_to(node)

func set_node_focus_material(node, is_focused: bool):
	for meshdata in node["meshes"]:
		var mat = meshdata["original"]
		if is_focused:
			mat = meshdata["focus"]
		meshdata["mesh"].set_surface_material(
			meshdata["surface"], mat)

func camera_focus_to(node):
	node_camera.move_look(node["node"].global_transform, 2.0)
