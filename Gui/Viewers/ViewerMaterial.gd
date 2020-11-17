extends Control

func _ready():
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_TRIANGLE_FAN)
	st.add_color(Color.white)
	st.add_uv(Vector2(0, 0))
	st.add_uv2(Vector2(0, 2))
	st.add_normal(Vector3(0, 0, 1))
	st.add_vertex(Vector3(-1, -1, 0)/2.0)
	st.add_uv(Vector2(0, 1))
	st.add_uv2(Vector2(0, 2))
	st.add_normal(Vector3(0, 0, 1))
	st.add_vertex(Vector3(-1, 1, 0)/2.0)
	st.add_uv(Vector2(1, 1))
	st.add_uv2(Vector2(0, 2))
	st.add_normal(Vector3(0, 0, 1))
	st.add_vertex(Vector3(1, 1, 0)/2.0)
	st.add_uv(Vector2(1, 0))
	st.add_uv2(Vector2(0, 2))
	st.add_normal(Vector3(0, 0, 1))
	st.add_vertex(Vector3(1, -1, 0)/2.0)
	$Viewport/MeshInstance.mesh = st.commit()

func set_file(file):
	if file == null:
		$TextureRect.hide()
	else:
		var data
		if file.type == "MATERIAL":
			data = ChumReader.read_material(file)
		elif file.type == "MATERIALANIM":
			data = ChumReader.read_materialanim(file)
		else:
			$TextureRect.hide()
			return
		if data == null:
			print("INVALID DATA")
			$TextureRect.hide()
		elif data["exists"]:
			var mat = data["material"]
#			mat.set_shader_param("alternative_alpha", false)
			$Viewport/MeshInstance.set_surface_material(0, mat)
			$TextureRect.show()
		else:
			print("DOES NOT EXIST")
			$TextureRect.hide()

func _on_TextureRect_item_rect_changed():
	var size = $TextureRect.rect_size
	if size.x < size.y:
		size.y = size.x
	elif size.y < size.x:
		size.x = size.y
	$Viewport.size = size
