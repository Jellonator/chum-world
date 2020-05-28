extends TextureRect

func set_file(file):
	if file == null:
#		node_mesh.hide()
		texture = null
	else:
		var data = ChumReader.read_bitmap(file)
		if data == null:
			print("INVALID DATA")
		elif data["exists"]:
			print("LOADED: ", data)
			var img = data["bitmap"]
			var tex = ImageTexture.new()
			tex.create_from_image(img, 0)
			texture = tex
		else:
			print("DOES NOT EXIST")
			texture = null
