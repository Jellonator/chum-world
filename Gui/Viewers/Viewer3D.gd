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
	node_rect.connect("item_rect_changed", self, "_on_TextureRect_item_rect_changed")

func _on_TextureRect_item_rect_changed():
	node_viewport.size = node_rect.rect_size

func set_file(file):
#	for surf in surfaces:
#		surf.queue_free()
#	surfaces.clear()
	node_camera.reset_transform()
	if node_mesh != null:
		node_mesh.queue_free()
		node_mesh = null
	if file != null:
		node_mesh = MeshData.try_file_to_spatial(file)
		node_spatial.add_child(node_mesh)
#	print("========================================")
#	if file == null:
#		node_mesh.hide()
#	elif file.type == "MESH":
#		var data = ChumReader.read_tmesh(file)
#		if data == null:
#			print("INVALID DATA")
#			node_mesh.hide()
#		elif data["exists"]:
#			print("LOADED: ")
#			node_mesh.mesh = data["mesh"]
#			print("TX: ", data["transform"])
#			print("UNK1: ", data["unk1"].size())
#			for x in data["unk1"]:
#				print("\t", x)
#			print("UNK2: ", data["unk2"].size())
#			for x in data["unk2"]:
#				print("\t", x)
#			print("UNK3: ", data["unk3"].size())
#			for x in data["unk3"]:
#				print("\t", x)
#			node_mesh.transform = Transform()
#			node_mesh.show()
#		else:
#			print("DOES NOT EXIST")
#			node_mesh.hide()
#	elif file.type == "SURFACE":
#		var data = ChumReader.read_surface(file)
#		if data == null:
#			print("INVALID DATA")
#			node_mesh.hide()
#		elif data["exists"]:
#			print("LOADED: ", data)
#			for mesh in data["surfaces"]:
#				var instance = MeshInstance.new()
#				instance.mesh = mesh
#				node_viewport.add_child(instance)
#				surfaces.append(instance)
##			node_mesh.mesh = data["mesh"]
##			node_mesh.transform = Transform()
#			node_mesh.hide()
#		else:
#			print("DOES NOT EXIST")
#			node_mesh.hide()
#	else:
#		node_mesh.hide()
#		print("UNRECOGNIZED TYPE ", file.type)

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
