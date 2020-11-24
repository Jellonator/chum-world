extends Control

const SCENE_EMPTYNODE = preload("res://Gui/Worldview/EmptyNode.tscn")

var archive = null
var archive_files = []
var tnodes_by_id := {}
var tnode_root = null
var selected_node = null
var can_move_mouse := true
var do_move_children := true

onready var node_surfaces := $Viewport/Surfaces
onready var node_camera := $Viewport/CameraViewer
onready var node_rect := $PanelContainer/TextureRect
onready var node_viewport := $Viewport as Viewport
onready var node_speed := $PanelContainer/TextureRect/SpeedLabel
onready var node_draw := $Viewport/Draw
onready var node_tree := $Tree/VBox/Items
onready var node_temp := $Viewport/Temp
onready var node_transform_gizmo := $Viewport/TransformGizmo
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

func get_node_global_transform(node: Dictionary) -> Transform:
	return node["node"].transform

func get_node_local_transform(node: Dictionary) -> Transform:
	var parent_id := node["parent"] as int
	var gtx := node["view"].global_transform as Transform
#	var gtx := node["node"].global_transform as Transform
	if parent_id == 0:
		print("RETURNING GLOBAL")
		return gtx
	elif parent_id in tnodes_by_id:
		var other := tnodes_by_id[parent_id] as Dictionary
		var ptx := other["view"].global_transform as Transform
#		var ptx := other["node"].global_transform as Transform
		return ptx.affine_inverse() * gtx
	else:
		MessageOverlay.push_warn("Error: Node %d does not exist" % parent_id)
		return gtx

func try_add_node(node_view, file):
	var node_base = Spatial.new()
	node_base.transform = node_view.global_transform
	var resid = node_view.resource_id
	var meshes = []
	var node_data = {
		"view": node_view,
		"node": node_base,
		"name": get_simple_name(file.name),
		"type": "",
		"parent": node_view.parent_id,
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

func set_active(value: bool):
	node_transform_gizmo.set_active(value and selected_node != null)

func reset_surfaces():
	tnodes_by_id.clear()
	tnode_root = null
	selected_node = null
	for child in node_surfaces.get_children():
		child.queue_free()
	for file in archive_files:
		if file.type == "NODE":
			var node_view = ChumReader.get_node_view(file)
			if node_view != null:
				try_add_node(node_view, file)
#			var node_data = ChumReader.read_node(file)
#			if not node_data["exists"]:
#				print("COULD NOT READ ", file.name)
#			else:
#				try_add_node(node_data["node"], file)
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
	node_transform_gizmo.set_active(false)

func set_archive(p_archive):
	self.archive = p_archive
	self.archive_files = archive.get_file_list()
	reset_surfaces()

func _on_TextureRect_item_rect_changed():
	node_viewport.size = node_rect.rect_size * GlobalConfig.viewport_scale
	node_viewport.set_size_override(true, node_rect.rect_size)

func _input(event):
	if can_move_mouse:
		if node_rect.has_focus():
			if Input.is_action_pressed("view_look"):
				if event is InputEventMouseMotion:
					node_camera.move_mouse(event.relative)
			if event.is_action_pressed("view_focus") and selected_node != null:
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
	node_transform_gizmo._input(event)

func _physics_process(delta: float):
	if node_rect.has_focus():
		if can_move_mouse:
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
			if Input.is_action_just_pressed("view_select"):
				pick_node()
		node_draw.update()

func pick_node():
	var camera := node_viewport.get_camera()
	var space_state := camera.get_world().direct_space_state
	var param := PhysicsShapeQueryParameters.new()
	param.collision_mask = 1
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
		var item_set := {}
		for value in result:
			var data = value["collider"].get_node_data()
			if data in item_set:
				continue
			var name = data["name"]
			item_set[data] = data
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

func set_selected_node(node):
	if selected_node != null:
		set_node_focus_material(selected_node, false)
	selected_node = node
	if selected_node != null:
		set_node_focus_material(selected_node, true)
	node_draw.update()
	node_transform_gizmo.set_active(true)
	node_transform_gizmo.transform = selected_node["node"].transform

func _on_Items_node_selected(node):
	set_selected_node(node)
	print("SELECT FROM ITEMS")

func _on_PopupSelector_index_pressed(index):
	var id = node_popup_select.get_item_id(index)
	var item = node_popup_select.get_item_metadata(id)
	print("SELECT FROM POPUP")
	set_selected_node(item)
	node_tree.try_select(item)

func _on_Button_pressed():
	reset_surfaces()

func update_child_tranforms(parent_node: Dictionary, new_parent_transform: Transform, do_update_view: bool):
	for child_node in parent_node["children"]:
		var child_view = child_node["view"]
		if do_move_children:
			# Keep local transform, update global transform
			var new_transform = new_parent_transform * child_view.local_transform
			child_node["node"].transform = new_transform
			if do_update_view:
				child_view.global_transform = new_transform
				child_view.save(child_node["file"])
			update_child_tranforms(child_node, new_transform, do_update_view)
		else:
			# Keep global transform, update local transform
			# Children do not need to be modified since global transform is the same
			if do_update_view:
#				var new_local_tx = child_view.global_transform * new_parent_transform.affine_inverse()
				var new_local_tx = get_node_local_transform(child_node)
				child_view.local_transform = new_local_tx
				child_view.save(child_node["file"])

func _on_TransformGizmo_on_change_transform(tx):
	if selected_node != null:
#		var ogtx = selected_node["node"].transform
		selected_node["node"].transform = tx
		update_child_tranforms(selected_node, tx, false)

func _on_TransformGizmo_on_finalize_transform(tx):
	if selected_node != null:
		selected_node["node"].transform = tx
		var view = selected_node["view"]
		var file = selected_node["file"]
		view.global_transform = tx
		var localtx = get_node_local_transform(selected_node)
		view.local_transform = localtx
		update_child_tranforms(selected_node, tx, true)
		view.save(file)

func _on_MoveChildren_toggled(button_pressed: bool):
	do_move_children = button_pressed

func _on_OpenNodeInFiles_pressed():
	var id = int(selected_node["id"])
	var file = archive.get_file_from_hash(id)
	var viewer = owner
	var tree = viewer.node_tree
	var editor = viewer.node_editor
	viewer.node_tabs.current_tab = 0
	editor.set_file(file)
	tree.set_selected(file)
