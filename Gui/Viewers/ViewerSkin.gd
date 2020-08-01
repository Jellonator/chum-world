extends Control

const MIN_SPEED = 0.0125
const MAX_SPEED = 128
const SPEED_MULT = 1.25

var speed = 2.0

const MAT_BLANK := preload("res://Shader/blank.tres")

var cfile = null

onready var node_items := $HSplitContainer/ItemList
onready var node_meshes := $Viewport/Meshes
onready var node_viewport := $Viewport
onready var node_rect := $HSplitContainer/TextureRect
onready var node_camera := $Viewport/CameraViewer
onready var node_speed := $HSplitContainer/TextureRect/SpeedLabel
var meshes = []

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

func set_meshes(mesh_ids: Array):
	for child in node_meshes.get_children():
		child.queue_free()
	meshes.clear()
	var archive = cfile.get_archive()
	for id in mesh_ids:
		var mesh_file = archive.get_file_from_hash(id)
		var mesh_data = ChumReader.read_tmesh(mesh_file)
		if mesh_data == null or not mesh_data["exists"]:
			meshes.append(null)
			print("COULD NOT LOAD")
		else:
			var mesh := MeshInstance.new()
			mesh.mesh = mesh_data["mesh"]
#			for i in range(mesh.get_surface_material_count()):
#				mesh.set_surface_material(i, MAT_BLANK)
			node_meshes.add_child(mesh)
			meshes.append({
				"mesh": mesh,
				"surfaces": mesh_data["surfaces"]
			})

func set_groups(group_values: Dictionary):
	node_items.clear()
	var archive = cfile.get_archive()
	for id in group_values:
		var group = group_values[id]
		var name = archive.maybe_get_name_from_hash(id)
		var idx = node_items.get_item_count()
		node_items.add_item(name)
		node_items.set_item_metadata(idx, group)
	node_items.sort_items_by_text()

func set_file(file):
	node_camera.reset_transform()
	cfile = file
	if file == null:
		pass
	else:
		var data = ChumReader.read_skin(file)
		if data == null:
			print("INVALID DATA")
		elif data["exists"]:
			print("LOADED SKIN")
			var skin = data["skin"]
			var archive = file.get_archive()
			set_meshes(skin["meshes"])
			set_groups(skin["groups"])
		else:
			print("DOES NOT EXIST")

func _on_TextureRect_item_rect_changed():
	node_viewport.size = node_rect.rect_size * 2
	node_viewport.set_size_override(true, node_rect.rect_size)

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

func set_group(groupdata: Dictionary):
	for mesh_id in len(self.meshes):
		if self.meshes[mesh_id] == null:
			continue
		var section_vertices = {}
		if mesh_id in groupdata:
			var section = groupdata[mesh_id]
			section_vertices = section["vertices"]
		var mesh = self.meshes[mesh_id]["mesh"]
		var mesh_surfaces = self.meshes[mesh_id]["surfaces"]
		var new_mesh = ArrayMesh.new()
		for surface_id in range(mesh_surfaces.size()):
			var surface = mesh.mesh.surface_get_arrays(surface_id)
			var surface_vertices = mesh_surfaces[surface_id]["vertices"]
			var colors := PoolColorArray()
			for index in surface_vertices:
				var weight = 0.0
				if index in section_vertices:
					weight = section_vertices[index]
				colors.push_back(Color(1.0-weight, 1.0-weight, 1.0, 1.0))
			surface[Mesh.ARRAY_COLOR] = colors
			new_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, surface)
		mesh.mesh = new_mesh
		for i in range(mesh.get_surface_material_count()):
			mesh.set_surface_material(i, MAT_BLANK)

func _on_ItemList_item_selected(index: int):
	set_group(node_items.get_item_metadata(index))
