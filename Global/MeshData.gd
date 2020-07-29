extends Node

const SCENE_EMPTYNODE = preload("res://Gui/Worldview/EmptyNode.tscn")
var _EMPTYNODE_MESH = null
var _COLLISIONVOL_MESH = null

func _add_line(st, p1, p2):
	st.add_vertex(p1)
	st.add_vertex(p2)

func get_emptynode_mesh():
	if _EMPTYNODE_MESH != null:
		return _EMPTYNODE_MESH
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_LINES)
	st.add_color(Color.red)
	st.set_material(preload("res://Shader/unshaded.tres"))
	_add_line(st, Vector3(0, 0, 0), Vector3(1, 0, 0))
	st.add_color(Color.blue)
	_add_line(st, Vector3(0, 0, 0), Vector3(0, 0, 1))
	st.add_color(Color.green)
	_add_line(st, Vector3(0, 0, 0), Vector3(0, 1, 0))
	st.add_color(Color(0.2, 0.2, 0.2))
	_add_line(st, Vector3(0, 0, 0), Vector3(-1, 0, 0))
	_add_line(st, Vector3(0, 0, 0), Vector3(0, 0, -1))
	_add_line(st, Vector3(0, 0, 0), Vector3(0, -1, 0))
	_EMPTYNODE_MESH = st.commit()
	return _EMPTYNODE_MESH

const COLLISIONVOL_GRID_SIZE := 8

func get_collisionvol_mesh():
	if _COLLISIONVOL_MESH != null:
		return _COLLISIONVOL_MESH
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_LINES)
	st.set_material(preload("res://Shader/unshaded.tres"))
	st.add_color(Color.yellow)
	for ix in range(COLLISIONVOL_GRID_SIZE+1):
		var x = range_lerp(ix, 0, COLLISIONVOL_GRID_SIZE, -1, 1)
		for a in [-1, 1]:
			_add_line(st, Vector3(x, a, 1), Vector3(x, a, -1))
			_add_line(st, Vector3(a, x, 1), Vector3(a, x, -1))
			_add_line(st, Vector3(x, 1, a), Vector3(x, -1, a))
			_add_line(st, Vector3(a, 1, x), Vector3(a, -1, x))
			_add_line(st, Vector3(1, x, a), Vector3(-1, x, a))
			_add_line(st, Vector3(1, a, x), Vector3(-1, a, x))
				
#			var b = range_lerp(ib, 0, COLLISIONVOL_GRID_SIZE, -1, 1)
#		_add_line(st, Vector3(a, -1, 1), Vector3(a, -1, 1))
	_COLLISIONVOL_MESH = st.commit()
	return _COLLISIONVOL_MESH

func load_mesh_from_file(file):
	var data = ChumReader.read_tmesh(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var node_mesh = MeshInstance.new()
		node_mesh.mesh = data["mesh"]
		node_mesh.transform = Transform()
		return node_mesh
	else:
		print("DOES NOT EXIST ", file.name)

func load_surface_from_file(file):
	var data = ChumReader.read_surface(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var node_object = Spatial.new()
		for surf in data["surfaces"]:
			var node_mesh = MeshInstance.new()
			node_mesh.mesh = surf
			node_object.add_child(node_mesh)
		return node_object
	else:
		print("DOES NOT EXIST ", file.name)

func load_skin_from_file(file):
	var data = ChumReader.read_skin(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var skin = data["skin"]
		var parent = Spatial.new()
		var archive = file.get_archive()
		for id in skin["meshes"]:
			var mesh_file = archive.get_file_from_hash(id)
			var child = try_file_to_spatial(mesh_file)
			if child != null:
				parent.add_child(child)
		return parent
	else:
		print("DOES NOT EXIST ", file.name)

func load_lod_from_file(file):
	var data = ChumReader.read_lod(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var lod = data["lod"]
		var parent = Spatial.new()
		var archive = file.get_archive()
		for id in lod["skins"]:
			var skin_file = archive.get_file_from_hash(id)
			if skin_file == null:
				print("Could not load file ", id, " from archive")
			else:
				var child = try_file_to_spatial(skin_file)
				if child != null:
					parent.add_child(child)
		return parent
	else:
		print("DOES NOT EXIST ", file.name)

func load_rotshape_from_file(file):
	var data = ChumReader.read_rotshape(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var rotshape = data["rotshape"]
		var node := MeshInstance.new()
		node.mesh = rotshape["mesh"]
		return node
	else:
		print("DOES NOT EXIST ", file.name)

const SPLINE_COLOR_A := Color.pink
const SPLINE_COLOR_B := Color.darkred

func load_spline_from_file(file):
	var data = ChumReader.read_spline(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var spline = data["spline"]
		var node := MeshInstance.new()
		var mesh := ArrayMesh.new()
		var st := SurfaceTool.new()
		st.begin(Mesh.PRIMITIVE_LINE_STRIP)
		var num = len(spline["vertices"])
		var i := 0
		for pos in spline["vertices"]:
			if i % 2 == 0 or i == num-1:
				st.add_color(SPLINE_COLOR_A)
			else:
				st.add_color(SPLINE_COLOR_B)
			st.add_vertex(pos)
			i += 1
		mesh.add_surface_from_arrays(Mesh.PRIMITIVE_LINE_STRIP, st.commit_to_arrays())
		st.begin(Mesh.PRIMITIVE_POINTS)
		st.add_color(Color.whitesmoke)
		for pos in spline["stops"]:
			st.add_vertex(pos)
		mesh.add_surface_from_arrays(Mesh.PRIMITIVE_POINTS, st.commit_to_arrays())
		node.mesh = mesh
		node.set_surface_material(0, preload("res://Shader/unshaded.tres"))
		node.set_surface_material(1, preload("res://Shader/unshaded.tres"))
		return node
	else:
		print("DOES NOT EXIST ", file.name)

func load_collisionvol_from_file(file):
	var mesh = MeshInstance.new()
	mesh.mesh = get_collisionvol_mesh()
	return mesh

func try_file_to_spatial(file):
	if file == null:
		push_warning("Attempt to get spatial from NULL file")
		return
	match file.type:
		"SURFACE":
			return load_surface_from_file(file)
		"MESH":
			return load_mesh_from_file(file)
		"SKIN":
			return load_skin_from_file(file)
		"LOD":
			return load_lod_from_file(file)
		"ROTSHAPE":
			return load_rotshape_from_file(file)
		"SPLINE":
			return load_spline_from_file(file)
		"COLLISIONVOL":
			return load_collisionvol_from_file(file)
		_:
			return SCENE_EMPTYNODE.instance()
