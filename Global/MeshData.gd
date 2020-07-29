extends Node

const SCENE_EMPTYNODE = preload("res://Gui/Worldview/EmptyNode.tscn")
var EMPTYNODE_MESH = null

func get_emptynode_mesh():
	if EMPTYNODE_MESH != null:
		return EMPTYNODE_MESH
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_LINES)
	st.add_color(Color.red)
	st.set_material(preload("res://Shader/unshaded.tres"))
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(1, 0, 0))
	st.add_color(Color.blue)
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(0, 0, 1))
	st.add_color(Color.green)
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(0, 1, 0))
	st.add_color(Color(0.2, 0.2, 0.2))
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(-1, 0, 0))
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(0, 0, -1))
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(0, -1, 0))
	EMPTYNODE_MESH = st.commit()
	return EMPTYNODE_MESH

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
		_:
			return SCENE_EMPTYNODE.instance()
