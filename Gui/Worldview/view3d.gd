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

func try_make_child(resfile):
	match resfile.type:
		"SURFACE":
			var data = ChumReader.read_surface(resfile)
			if data == null:
				print("INVALID DATA")
			elif data["exists"]:
				var node_object = Spatial.new()
				for surf in data["surfaces"]:
					var node_mesh = MeshInstance.new()
					node_mesh.mesh = surf
					node_object.add_child(node_mesh)
				return node_object
#				node_mesh.transform = Transform()
#				return node_mesh
			else:
				print("DOES NOT EXIST")
		"MESH":
			var data = ChumReader.read_tmesh(resfile)
			if data == null:
				print("INVALID DATA")
			elif data["exists"]:
				var node_mesh = MeshInstance.new()
				node_mesh.mesh = data["mesh"]
				node_mesh.transform = Transform()
				return node_mesh
			else:
				print("DOES NOT EXIST")
		"SKIN":
			var data = ChumReader.read_skin(resfile)
			if data == null:
				print("INVALID DATA")
			elif data["exists"]:
				var skin = data["skin"]
				var parent = Spatial.new()
				for id in skin["meshes"]:
					var mesh_file = archive.get_file_from_hash(id)
					var mesh_data = ChumReader.read_tmesh(mesh_file)
					if mesh_data == null or not mesh_data["exists"]:
						print("COULD NOT LOAD")
					else:
						var mesh := MeshInstance.new()
						mesh.mesh = mesh_data["mesh"]
						parent.add_child(mesh)
				return parent
			else:
				print("DOES NOT EXIST")
		"LOD":
			var data = ChumReader.read_lod(resfile)
			if data == null:
				print("INVALID DATA ", resfile.name)
			elif data["exists"]:
				var lod = data["lod"]
				var parent = Spatial.new()
				for id in lod["skins"]:
					var skin_file = archive.get_file_from_hash(id)
					if skin_file == null:
						print("Could not load file ", id, " from archive")
					else:
						var child = try_make_child(skin_file)
						if child != null:
							parent.add_child(child)
				return parent
			else:
				print("DOES NOT EXIST ", resfile.name)
		_:
			return SCENE_EMPTYNODE.instance()

func get_simple_name(name: String) -> String:
	var a = name.find_last(">")
	if a != -1:
		name = name.substr(a+1, -1)
	var b = name.find(".")
	if b != -1:
		name = name.substr(0, b)
	return name

func try_add_node(nodedata: Dictionary, name: String):
	var node_base = Spatial.new()
	node_base.transform = nodedata["global_transform"]
#	node_base.scale = nodedata["local_scale"]
#	node_base.translate() = 
	var tx_rot = Transform(nodedata["local_rotation"])
	var tx = Transform().scaled(nodedata["local_scale"]) * tx_rot
	tx.origin += nodedata["local_translation"]
#	print(nodedata["global_transform"])
#	print(tx)
#	print(nodedata["local_transform"].affine_inverse())
#	print("--")
	var ftype := ""
	var resid = nodedata["resource_id"]
	if resid != 0:
		var resfile = archive.get_file_from_hash(resid)
		if resfile == null:
			print("Could not load file ", resid, " from archive")
		else:
			ftype = ":" + resfile.type
			var child = try_make_child(resfile)
			if child != null:
				node_base.add_child(child)
#	var text_child = SCENE_TEXT.instance()
#	node_base.add_child(text_child)
	node_surfaces.add_child(node_base)
	node_draws.append({
		"node": node_base,
		"name": get_simple_name(name) + ftype
	})
#	text_child.set_text(get_simple_name(name))

#func try_add_surface_from_file(file):
#	var data = ChumReader.read_surface(file)
#	if data == null:
#		print("INVALID DATA")
#	elif data["exists"]:
#		var node_mesh := MeshInstance.new()
#		node_mesh.mesh = data["mesh"]
##		node_mesh.transform = data["transform"].affine_inverse()
#		node_mesh.show()
#		node_surfaces.add_child(node_mesh)
#	else:
#		print("DOES NOT EXIST")

func reset_surfaces():
	node_draws.clear()
	for child in node_surfaces.get_children():
		child.queue_free()
	for file in archive_files:
		if file.type == "NODE":
#			print("===============================")
			var node_data = ChumReader.read_node(file)
			if not node_data["exists"]:
				print("COULD NOT READ ", file.name)
			else:
				try_add_node(node_data["node"], file.name)
#			prints(node_data["exists"], file.name)

func set_archive(p_archive):
	self.archive = p_archive
	self.archive_files = archive.get_file_list()
	reset_surfaces()

func _on_TextureRect_item_rect_changed():
	node_viewport.size = node_rect.rect_size

func _input(event):
	if self.has_focus() or node_rect.has_focus():
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

func _on_Draw_draw():
	if not show_node_names:
		return
	var camera = node_viewport.get_camera()
	for data in node_draws:
		var node = data["node"]
		if not camera.is_position_behind(node.transform.origin):
			var screen_pos = camera.unproject_position(node.transform.origin)
			node_draw.draw_string(FONT, screen_pos, data["name"])

func _on_CheckButton_toggled(button_pressed):
	show_node_names = button_pressed

func _ready():
	$PanelContainer/TextureRect/Controls/CheckButton.pressed = show_node_names
