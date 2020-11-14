extends Node

const SCENE_EMPTYNODE = preload("res://Gui/Worldview/EmptyNode.tscn")
const SCENE_SELECT_BODY := preload("res://Gui/Worldview/SelectBody.tscn")
var _EMPTYNODE_MESH = null
var _COLLISIONVOL_MESH = null

const ICON_ROOT := preload("res://Gui/Icon/root.png")
const ICON_NODE := preload("res://Gui/Icon/node.png")
const ICON_UNKNOWN := preload("res://Gui/Icon/unknown.png")
const TYPE_ICONS = {
	"ANIMATION": preload("res://Gui/Icon/animation.png"),
	"BITMAP": preload("res://Gui/Icon/bitmap.png"),
	"CAMERA": preload("res://Gui/Icon/camera.png"),
	"CAMERAZONE": preload("res://Gui/Icon/camerazone.png"),
	"COLLISIONVOL": preload("res://Gui/Icon/collisionvol.png"),
	"GAMEOBJ": preload("res://Gui/Icon/gameobj.png"),
	"HFOG": preload("res://Gui/Icon/hfog.png"),
	"LIGHT": preload("res://Gui/Icon/light.png"),
	"LOD": preload("res://Gui/Icon/lod.png"),
	"MATERIAL": preload("res://Gui/Icon/material.png"),
	"MATERIALANIM": preload("res://Gui/Icon/materialanim.png"),
	"MESH": preload("res://Gui/Icon/mesh.png"),
	"NODE": preload("res://Gui/Icon/node.png"),
	"OCCLUDER": preload("res://Gui/Icon/occluder.png"),
	"OMNI": preload("res://Gui/Icon/omni.png"),
	"PARTICLES": preload("res://Gui/Icon/particles.png"),
	"ROTSHAPE": preload("res://Gui/Icon/rotshape.png"),
	# RTC
	"SKIN": preload("res://Gui/Icon/skin.png"),
	"SOUND": preload("res://Gui/Icon/sound.png"),
	"SPLINE": preload("res://Gui/Icon/spline.png"),
	"SURFACE": preload("res://Gui/Icon/surface.png"),
	"TXT": preload("res://Gui/Icon/txt.png"),
	"USERDEFINE": preload("res://Gui/Icon/userdefine.png"),
	"WARP": preload("res://Gui/Icon/warp.png"),
	"WORLD": preload("res://Gui/Icon/world.png"),
}

func _maybe_add_line(st, a, b, check):
	if a in check:
		if b in check[a]:
			return
	if b in check:
		if a in check[b]:
			return
	if not a in check:
		check[a] = {}
	check[a][b] = true
	st.add_vertex(a)
	st.add_vertex(b)

func generate_wireframe(mesh: Mesh, tx: Transform) -> ArrayMesh:
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_LINES)
	var tris := mesh.get_faces()
	st.set_material(preload("res://Shader/unshaded.tres"))
	st.add_color(Color.black)
	var check = {}
	for i in range(0, tris.size(), 3):
		var a = tx.xform(tris[i])
		var b = tx.xform(tris[i+1])
		var c = tx.xform(tris[i+2])
		_maybe_add_line(st, a, b, check)
		_maybe_add_line(st, b, c, check)
		_maybe_add_line(st, a, c, check)
	return st.commit()

var _MESH_CYLINDER = null
func get_cylinder_mesh():
	if _MESH_CYLINDER != null:
		return _MESH_CYLINDER
	var cylinder := preload("res://Gui/Gizmos/Shapes/Cylinder.tscn").instance()
	var tx = Transform(
		Vector3(1, 0, 0),
		Vector3(0, 0, 1),
		Vector3(0, -1, 0),
		Vector3(0, 0, 0.5)
	)
	_MESH_CYLINDER = generate_wireframe(cylinder.mesh, tx)
	return _MESH_CYLINDER

var _MESH_SPHERE = null
func get_sphere_mesh():
	if _MESH_SPHERE != null:
		return _MESH_SPHERE
	var cylinder := preload("res://Gui/Gizmos/Shapes/Sphere.tscn").instance()
	_MESH_SPHERE = generate_wireframe(cylinder.mesh, Transform.IDENTITY)
	return _MESH_SPHERE

var _MESH_CUBE = null
func get_cube_mesh():
	if _MESH_CUBE != null:
		return _MESH_CUBE
	var cylinder := preload("res://Gui/Gizmos/Shapes/Cube.tscn").instance()
	var tx = Transform(
		Vector3(2, 0, 0),
		Vector3(0, 1, 0),
		Vector3(0, 0, 2),
		Vector3(0, 0.5, 0)
	)
	_MESH_CUBE = generate_wireframe(cylinder.mesh, tx)
	return _MESH_CUBE

func generate_surface_focus_material(shadermaterial: ShaderMaterial) -> Material:
	if shadermaterial == null:
		shadermaterial = ShaderMaterial.new()
		shadermaterial.shader = preload("res://Shader/material.shader")
	var material := shadermaterial.duplicate() as ShaderMaterial
	material.set_shader_param("do_highlight", 2)
	return material

func generate_mesh_focus_material(shadermaterial: ShaderMaterial) -> Material:
	var material := shadermaterial.duplicate() as ShaderMaterial
	material.set_shader_param("do_highlight", 1)
	return material

func _add_line(st, p1, p2):
	st.add_vertex(p1)
	st.add_vertex(p2)

func get_emptynode_mesh():
	if _EMPTYNODE_MESH != null:
		return _EMPTYNODE_MESH
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_LINES)
	st.add_color(Color(1.0, 0.01, 0.01))
	st.set_material(preload("res://Shader/unshaded.tres"))
	_add_line(st, Vector3(0, 0, 0), Vector3(1, 0, 0))
	st.add_color(Color(0.01, 0.01, 1.0))
	_add_line(st, Vector3(0, 0, 0), Vector3(0, 0, 1))
	st.add_color(Color(0.01, 1.0, 0.01))
	_add_line(st, Vector3(0, 0, 0), Vector3(0, 1, 0))
	st.add_color(Color(0.2, 0.2, 0.2))
	_add_line(st, Vector3(0, 0, 0), Vector3(-1, 0, 0))
	_add_line(st, Vector3(0, 0, 0), Vector3(0, 0, -1))
	_add_line(st, Vector3(0, 0, 0), Vector3(0, -1, 0))
	_EMPTYNODE_MESH = st.commit()
	return _EMPTYNODE_MESH

const COLLISIONVOL_GRID_SIZE := 8

const COLLISIONVOL_SIZE := 0.5

func get_collisionvol_mesh():
	if _COLLISIONVOL_MESH != null:
		return _COLLISIONVOL_MESH
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_LINES)
	st.set_material(preload("res://Shader/unshaded.tres"))
	st.add_color(Color(0.9, 0.4, 0.0))
	var s := COLLISIONVOL_SIZE
	for ix in range(COLLISIONVOL_GRID_SIZE+1):
		var x = range_lerp(ix, 0, COLLISIONVOL_GRID_SIZE, -s, s)
		for a in [-s, s]:
			_add_line(st, Vector3(x, a, s), Vector3(x, a, -s))
			_add_line(st, Vector3(a, x, s), Vector3(a, x, -s))
			_add_line(st, Vector3(x, s, a), Vector3(x, -s, a))
			_add_line(st, Vector3(a, s, x), Vector3(a, -s, x))
			_add_line(st, Vector3(s, x, a), Vector3(-s, x, a))
			_add_line(st, Vector3(s, a, x), Vector3(-s, a, x))
	_COLLISIONVOL_MESH = st.commit()
	return _COLLISIONVOL_MESH

var COOL_FORWARD := Vector3(-0.22471, 0.125, 1).normalized()

func load_mesh_from_file(file, node_owner):
	var data = ChumReader.read_tmesh(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var node_mesh = MeshInstance.new()
		node_mesh.mesh = data["mesh"]
		node_mesh.transform = Transform()
		if node_owner != null:
			var shape := SCENE_SELECT_BODY.instance()
			shape.set_node_data(node_owner)
			shape.add_shape(data["mesh"].create_convex_shape())
			node_mesh.add_child(shape)
			for surface in range(data["mesh"].get_surface_count()):
				var mat = data["mesh"].surface_get_material(surface)
				node_owner["meshes"].append({
					"mesh": node_mesh,
					"surface": surface,
					"original": mat,
					"focus": generate_mesh_focus_material(mat),
				})
		for unk in data["unk1"]:
			var sphere := MeshInstance.new()
			sphere.mesh = get_sphere_mesh()
			sphere.translation = unk["pos"]
			sphere.scale *= unk["radius"]
			sphere.add_to_group("vis_collision")
			node_mesh.add_child(sphere)
		for unk in data["unk2"]:
			var cube := MeshInstance.new()
			cube.mesh = get_cube_mesh()
			cube.transform = unk["transform"]
			cube.add_to_group("vis_collision")
			node_mesh.add_child(cube)
		for unk in data["unk3"]:
			var radius = unk["unk2"]
			var pos = Vector3(unk["unk1"][0], unk["unk1"][1], unk["unk1"][2])
			var height = unk["unk1"][3]
			var normal = unk["normal"]
			var sphere := MeshInstance.new()
			sphere.rotation = Vector3(
				asin(-normal.y),
				atan2(normal.x, normal.z),
				0)
			sphere.mesh = get_cylinder_mesh()
			sphere.scale = Vector3(radius, radius, height)
			sphere.transform.origin = pos
			sphere.add_to_group("vis_collision")
			node_mesh.add_child(sphere)
		return node_mesh
	else:
		print("DOES NOT EXIST ", file.name)

func load_surface_from_file(file, node_owner):
	var data = ChumReader.read_surface(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var shape
		var node_object = Spatial.new()
		if node_owner != null:
			shape = SCENE_SELECT_BODY.instance()
			shape.set_node_data(node_owner)
			node_object.add_child(shape)
		for surf in data["surfaces"]:
			var node_mesh = MeshInstance.new()
			node_mesh.mesh = surf
			node_object.add_child(node_mesh)
			var mat = surf.surface_get_material(0)
			if node_owner != null:
				shape.add_shape(surf.create_convex_shape())
				node_owner["meshes"].append({
					"mesh": node_mesh,
					"surface": 0,
					"original": mat,
					"focus": generate_surface_focus_material(mat),
				})
		return node_object
	else:
		print("DOES NOT EXIST ", file.name)

func load_skin_from_file(file, node_owner):
	var data = ChumReader.read_skin(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var skin = data["skin"]
		var parent = Spatial.new()
		var archive = file.get_archive()
		for id in skin["meshes"]:
			var mesh_file = archive.get_file_from_hash(id)
			var child = try_file_to_spatial(mesh_file, node_owner)
			if child != null:
				parent.add_child(child)
		return parent
	else:
		print("DOES NOT EXIST ", file.name)

func load_lod_from_file(file, node_owner):
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
				var child = try_file_to_spatial(skin_file, node_owner)
				if child != null:
					parent.add_child(child)
		return parent
	else:
		print("DOES NOT EXIST ", file.name)

func load_rotshape_from_file(file, node_owner):
	var data = ChumReader.read_rotshape(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var rotshape = data["rotshape"]
		var node := MeshInstance.new()
		node.mesh = rotshape["mesh"]
		var mat = rotshape["mesh"].surface_get_material(0)
		if node_owner != null:
			node_owner["meshes"].append({
				"mesh": node,
				"surface": 0,
				"original": mat,
				"focus": generate_mesh_focus_material(mat),
			})
			var shape := SCENE_SELECT_BODY.instance()
			shape.set_node_data(node_owner)
			var aabb = rotshape["mesh"].get_aabb()
			var center = aabb.position + aabb.size/2
			var radius = aabb.get_longest_axis_size()/2
			var sphere := SphereShape.new()
			sphere.radius = radius
			shape.add_shape(sphere, Transform().translated(center))
			node.add_child(shape)
		return node
	else:
		print("DOES NOT EXIST ", file.name)

const SPLINE_COLOR_A := Color.pink
const SPLINE_COLOR_B := Color.darkred

func load_spline_from_file(file, node_owner):
	var data = ChumReader.read_spline(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var spline = data["spline"]
		var parent := Spatial.new()
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
		if node_owner != null:
			node_owner["meshes"].append({
				"mesh": node,
				"surface": 0,
				"original": preload("res://Shader/unshaded.tres"),
				"focus": preload("res://Shader/unshaded_highlight.tres"),
			})
			node_owner["meshes"].append({
				"mesh": node,
				"surface": 1,
				"original": preload("res://Shader/unshaded.tres"),
				"focus": preload("res://Shader/unshaded_highlight.tres"),
			})
		node.add_to_group("vis_spline")
		var sprite = make_icon_billboard(file, node_owner, ICON_UNKNOWN)
		parent.add_child(sprite)
		parent.add_child(node)
		return parent
	else:
		print("DOES NOT EXIST ", file.name)

func load_collisionvol_from_file(file, node_owner):
	var data = ChumReader.read_collisionvol(file)
	if data == null:
		print("INVALID DATA ", file.name)
	elif data["exists"]:
		var volume = data["collisionvol"]
		var parent := Spatial.new()
		var mesh := MeshInstance.new()
		mesh.mesh = get_collisionvol_mesh()
		mesh.transform = volume["local_transform"]
		if node_owner != null:
			node_owner["meshes"].append({
				"mesh": mesh,
				"surface": 0,
				"original": preload("res://Shader/unshaded.tres"),
				"focus": preload("res://Shader/unshaded_highlight.tres"),
			})
			var shape := SCENE_SELECT_BODY.instance()
			shape.set_node_data(node_owner)
			var cube := BoxShape.new()
			cube.extents = Vector3(0.5, 0.5, 0.5)
			shape.add_shape(cube)
			mesh.add_child(shape)
		mesh.add_to_group("vis_volume")
		var sprite = make_icon_billboard(file, node_owner, ICON_UNKNOWN)
		parent.add_child(sprite)
		parent.add_child(mesh)
		return parent
	else:
		print("DOES NOT EXIST ", file.name)

func _load_material_from_id(archive, id):
	var file = archive.get_file_from_hash(id)
	prints("LOADING IMAGE", id, file.name)
	return ChumReader.read_material(file)

#const _warp_faces = [ 
#	[0, 1, 2, 3], # -Y (bottom)
#	[4, 5, 6, 7], # +Y (top)
#	[6, 5, 1, 2], # -Z (front)
#	[6, 7, 3, 2], # +X (right)
#	[5, 4, 0, 1], # -X (left)
#	[7, 4, 0, 3], # +Z (back)
#]

# - +
# - -
# + -
# + +

const _warp_faces = [ 
	[1, 2, 0, 3], # -Y (bottom)
	[4, 7, 5, 6], # +Y (top)
	[7, 4, 3, 0], # +Z (back)
	[6, 7, 2, 3], # +X (right)
	[4, 5, 0, 1], # -X (left)
	[5, 6, 1, 2], # -Z (front)
]

const _warp_texcoord_order = [0, 1, 2, 3]

func load_warp_from_file(file, _node_owner):
	var root := Spatial.new()
	var struct = file.read_structure()
#	print(JSON.print(struct,"\t"))
	var size = struct.value.size.value
	var points = [
		struct.value.vertices.value[0].value, # BBA -X -Y +Z
		struct.value.vertices.value[1].value, # BBB -X -Y -Z
		struct.value.vertices.value[2].value, # ABB +X -Y -Z
		struct.value.vertices.value[3].value, # ABA +X -Y +Z
		struct.value.vertices.value[4].value, # BAA -X +Y +Z
		struct.value.vertices.value[5].value, # BAB -X +Y -Z
		struct.value.vertices.value[6].value, # AAB +X +Y -Z
		struct.value.vertices.value[7].value, # AAA +X +Y +Z
	]
	var texcoords = [
		struct.value.texcoords.value[0].value,
		struct.value.texcoords.value[1].value,
		struct.value.texcoords.value[2].value,
		struct.value.texcoords.value[3].value
	]
	var materials = [
		struct.value.material_ids.value[0].value,
		struct.value.material_ids.value[1].value,
		struct.value.material_ids.value[2].value,
		struct.value.material_ids.value[3].value,
		struct.value.material_ids.value[4].value,
		struct.value.material_ids.value[5].value
	]
	var archive = file.get_archive()
	for i in range(6):
		var st := SurfaceTool.new()
		st.begin(Mesh.PRIMITIVE_TRIANGLE_STRIP)
		st.set_material(_load_material_from_id(archive, materials[i])["material"])
		st.add_uv(texcoords[_warp_texcoord_order[0]])
		st.add_vertex(points[_warp_faces[i][0]])
		st.add_uv(texcoords[_warp_texcoord_order[1]])
		st.add_vertex(points[_warp_faces[i][1]])
		st.add_uv(texcoords[_warp_texcoord_order[2]])
		st.add_vertex(points[_warp_faces[i][2]])
		st.add_uv(texcoords[_warp_texcoord_order[3]])
		st.add_vertex(points[_warp_faces[i][3]])
		var mesh = st.commit()
		var instance = MeshInstance.new()
		instance.cast_shadow = false
		instance.mesh = mesh
		root.add_child(instance)
	return root

func make_icon_billboard(file, node_owner, default_icon):
	var icon = default_icon
	if file != null:
		if file.type in MeshData.TYPE_ICONS:
			icon = MeshData.TYPE_ICONS[file.type]
		else:
			icon = ICON_UNKNOWN
	var sprite := Sprite3D.new()
	var matn = preload("res://Shader/sprite3d_normal.tres")
	var matf = preload("res://Shader/sprite3d_focus.tres")
	sprite.material_override = matn
	sprite.pixel_size = 0.05
	sprite.texture = icon
	if node_owner != null:
		node_owner["meshes"].append({
			"mesh": sprite,
			"surface": "sprite",
			"original": matn,
			"focus": matf,
		})
		var shape := SCENE_SELECT_BODY.instance()
		shape.set_node_data(node_owner)
		shape.add_shape(SphereShape.new())
		sprite.add_child(shape)
	sprite.add_to_group("vis_node")
	return sprite

func load_emptymesh(file, node_owner, default_icon):
	var mesh = SCENE_EMPTYNODE.instance()
	var sprite = make_icon_billboard(file, node_owner, default_icon)
	mesh.add_child(sprite)
	if node_owner != null:
		node_owner["meshes"].append({
			"mesh": mesh,
			"surface": 0,
			"original": preload("res://Shader/unshaded.tres"),
			"focus": preload("res://Shader/node_highlight.tres"),
		})
	mesh.add_to_group("vis_node")
	return mesh

func try_file_to_spatial(file, node_owner=null):
	if file == null:
		push_warning("Attempt to get spatial from NULL file")
		return
	match file.type:
		"SURFACE":
			return load_surface_from_file(file, node_owner)
		"MESH":
			return load_mesh_from_file(file, node_owner)
		"SKIN":
			return load_skin_from_file(file, node_owner)
		"LOD":
			return load_lod_from_file(file, node_owner)
		"ROTSHAPE":
			return load_rotshape_from_file(file, node_owner)
		"SPLINE":
			return load_spline_from_file(file, node_owner)
		"COLLISIONVOL":
			return load_collisionvol_from_file(file, node_owner)
		"WARP":
			return load_warp_from_file(file, node_owner)
		_:
			return load_emptymesh(file, node_owner, ICON_NODE)
