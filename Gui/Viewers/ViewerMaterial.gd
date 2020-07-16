extends Control

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
			$Viewport/MeshInstance.set_surface_material(0, mat)
			$TextureRect.show()
		else:
			print("DOES NOT EXIST")
			$TextureRect.hide()
