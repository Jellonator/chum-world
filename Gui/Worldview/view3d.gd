extends Control

const SCENE_EMPTYNODE = preload("res://Gui/Worldview/EmptyNode.tscn")

var archive = null
var archive_files = []
var tnodes_by_id := {}
var tnode_root = null
var selected_node = null

onready var node_surfaces := $Viewport/Surfaces
onready var node_camera := $Viewport/CameraViewer
onready var node_rect := $PanelContainer/TextureRect
onready var node_viewport := $Viewport as Viewport
onready var node_speed := $PanelContainer/TextureRect/SpeedLabel
onready var node_draw := $Viewport/Draw
onready var node_tree := $Tree/VBox/Items
onready var node_temp := $Viewport/Temp
onready var node_popup_select := $PanelContainer/TextureRect/PopupSelector as PopupMenu

const MIN_SPEED = pow(2, -8)
const MAX_SPEED = pow(2, 8)
const SPEED_MULT = sqrt(2.0)

var speed = 2.0
var show_node_names := false

func get_simple_name(name: String) -> String:
	var a = name.find_last(">")
	if a != -1:
		name = name.substr(a+1, -1)
	var b = name.find(".")
	if b != -1:
		name = name.substr(0, b)
	return name

func try_add_node(nodedata: Dictionary, file):
	var node_base = Spatial.new()
	node_base.transform = nodedata["global_transform"]
	var resid = nodedata["resource_id"]
	var meshes = []
	var node_data = {
		"node": node_base,
		"name": get_simple_name(file.name),
		"type": "",
		"parent": nodedata["parent_id"],
		"id": file.get_hash_id(),
		"file": file,
		"children": [],
		"meshes": meshes
	}
	if resid != 0:
		var resfile = archive.get_file_from_hash(resid)
		if resfile == null:
			var child = MeshData.load_emptymesh(null, node_data, MeshData.ICON_UNKNOWN)
			node_base.add_child(child)
			MessageOverlay.push_warn(
				"Could not load file %d from archive" % resid)
		else:
			node_data["type"] = resfile.type
			node_data["name"] += ":" + resfile.type
			var child = MeshData.try_file_to_spatial(resfile, node_data)
			if child != null:
				node_base.add_child(child)
	else:
		var child = MeshData.load_emptymesh(null, node_data, MeshData.ICON_NODE)
		node_base.add_child(child)
	node_surfaces.add_child(node_base)
	tnodes_by_id[file.get_hash_id()] = node_data

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
		elif file.type == "WARP":
			var instance = MeshData.load_warp_from_file(file, null)
			node_surfaces.add_child(instance)
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
	update_shown_objects()

func set_archive(p_archive):
	self.archive = p_archive
	self.archive_files = archive.get_file_list()
	reset_surfaces()

func _on_TextureRect_item_rect_changed():
	node_viewport.size = node_rect.rect_size * GlobalConfig.viewport_scale
	node_viewport.set_size_override(true, node_rect.rect_size)

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
	if event.is_action_pressed("view_select"):
		var pos = node_rect.get_local_mouse_position()
		var rect = Rect2(Vector2(0, 0), node_rect.rect_size)
		if rect.has_point(pos):
			node_rect.grab_focus()

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
		if Input.is_action_just_pressed("view_select"):
			pick_node()

func pick_node():
	var camera := node_viewport.get_camera()
	var space_state := camera.get_world().direct_space_state
	var param := PhysicsShapeQueryParameters.new()
	var shape := RayShape.new()
	shape.length = 500
	var mouse_pos = node_rect.get_local_mouse_position() * GlobalConfig.viewport_scale
	var from = camera.project_ray_origin(mouse_pos)
	var to = from - camera.project_ray_normal(mouse_pos)
	param.transform = Transform()\
		.translated(from)\
		.looking_at(to, Vector3.UP)
	param.set_shape(shape)
	var result := space_state.intersect_shape(param, 16)
	if result.size() > 0:
		node_popup_select.clear()
		for value in result:
			var data = value["collider"].get_node_data()
			var name = data["name"]
			var i := node_popup_select.get_item_count()
			var icon = node_tree.get_node_icon(data)
			node_popup_select.add_icon_item(icon, name, i)
			node_popup_select.set_item_metadata(i, data)
		node_popup_select.rect_position = node_rect.get_global_mouse_position()
		node_popup_select.rect_size = Vector2.ZERO
		node_popup_select.popup()

const FONT_BIG := preload("res://Font/Big.tres")
const FONT_SMALL := preload("res://Font/Base.tres")
const DIST_MAX := 45.0
const DIST_MIN := 30.0

func draw_node_label(camera, text: String, position: Vector3, color: Color):
	if not camera.is_position_behind(position):
		var font = FONT_SMALL
		var size = font.get_string_size(text)
		var screen_pos = camera.unproject_position(position)
		# draw BG
		var rect := Rect2(screen_pos - Vector2(size.x/2, 4), size)
		rect = rect.grow_individual(4.0, 0.0, 4.0, 0.0)
		var bgcolor := Color(0, 0, 0, 0.6 * color.a)
		node_draw.draw_rect(rect, bgcolor)
		# draw FG
		screen_pos.x -= size.x / 2
		screen_pos.y += size.y / 2
		node_draw.draw_string(font, screen_pos, text, color)

func _on_Draw_draw():
	var camera = node_viewport.get_camera()
	node_draw.scale = GlobalConfig.viewport_scale
	if show_node_names:
		for data in tnodes_by_id.values():
			var node = data["node"]
			var pos = node.global_transform.origin
			var distance = pos.distance_to(camera.global_transform.origin)
			if distance >= DIST_MAX or data == selected_node:
				continue
			var alpha = 1.0
			if distance > DIST_MIN:
				alpha = range_lerp(distance, DIST_MIN, DIST_MAX, 1.0, 0.0)
	#			alpha = lerp(1.0, 0.0, (distance-DIST_MIN)/(DIST_MAX-DIST_MIN))
			var color = Color(1.0, 1.0, 1.0, alpha)
			draw_node_label(camera, data["name"], pos, color)
	if selected_node != null:
		var color = Color(1.0, 1.0, 1.0, 1.0)
		var pos = selected_node["node"].global_transform.origin
		draw_node_label(camera, selected_node["name"], pos, color)

func _on_CheckButton_toggled(button_pressed):
	show_node_names = button_pressed
	node_draw.update()

func _ready():
	$PanelContainer/TextureRect/Controls/NodeNames.pressed = show_node_names

func _on_Items_node_selected(node):
	if selected_node != null:
		set_node_focus_material(selected_node, false)
	selected_node = node
	if selected_node != null:
		set_node_focus_material(selected_node, true)
	node_draw.update()

func set_node_focus_material(node, is_focused: bool):
	for meshdata in node["meshes"]:
		var mat = meshdata["original"]
		if is_focused:
			mat = meshdata["focus"]
		match meshdata["surface"]:
			"sprite":
				meshdata["mesh"].material_override = mat
			var n:
				meshdata["mesh"].set_surface_material(n, mat)

func camera_focus_to(node):
	node_camera.move_look(node["node"].global_transform, 2.0)
	node_draw.update()

var option_show_shapes := false
var option_show_volumes := false
var option_show_nodes := false
var option_show_splines := false

func _on_ShowShapes_toggled(button_pressed):
	option_show_shapes = button_pressed
	get_tree().set_group("vis_collision", "visible", option_show_shapes)

func _on_ShowVolumes_toggled(button_pressed):
	option_show_volumes = button_pressed
	get_tree().set_group("vis_volume", "visible", option_show_volumes)

func _on_ShowNodes_toggled(button_pressed):
	option_show_nodes = button_pressed
	get_tree().set_group("vis_node", "visible", option_show_nodes)
	
func _on_ShowSplines_toggled(button_pressed):
	option_show_splines = button_pressed
	get_tree().set_group("vis_spline", "visible", option_show_splines)

func update_shown_objects():
	get_tree().set_group("vis_collision", "visible", option_show_shapes)
	get_tree().set_group("vis_volume", "visible", option_show_volumes)
	get_tree().set_group("vis_node", "visible", option_show_nodes)
	get_tree().set_group("vis_spline", "visible", option_show_splines)

func _on_PopupSelector_index_pressed(index):
	var id = node_popup_select.get_item_id(index)
	var item = node_popup_select.get_item_metadata(id)
	if selected_node != null:
		set_node_focus_material(selected_node, false)
	selected_node = item
	if selected_node != null:
		set_node_focus_material(selected_node, true)
	node_tree.try_select(item)

func _on_Button_pressed():
	reset_surfaces()
