extends Control

onready var node_view := $Control/TextureViewer
onready var node_imp_format := $ConfirmationDialog/GridContainer/Format
onready var node_imp_palette := $ConfirmationDialog/GridContainer/Palette

var curfile = null

func set_file(file):
	curfile = file
	if file == null:
#		node_mesh.hide()
		node_view.texture = null
	else:
		var data = ChumReader.read_bitmap(file)
		if data == null:
			print("INVALID DATA")
		elif data["exists"]:
			print("LOADED: ", data)
			var img = data["bitmap"]
			var tex = ImageTexture.new()
			tex.create_from_image(img, 0)
			node_view.texture = tex
		else:
			print("DOES NOT EXIST")
			node_view.texture = null

func _on_Button_pressed():
	$ConfirmationDialog.popup_centered()

func _on_FileButton_pressed():
	$FileDialog.popup_centered()

func _on_FileDialog_file_selected(path):
	$ConfirmationDialog/GridContainer/File.text = path

func _on_Format_item_selected(id):
	id = $ConfirmationDialog/GridContainer/Format.get_item_id(id)
	if id == 1 or id == 2:
		$ConfirmationDialog/GridContainer/Palette.disabled = false
	else:
		$ConfirmationDialog/GridContainer/Palette.disabled = true

func _on_ConfirmationDialog_confirmed():
	if curfile != null:
		var path = $ConfirmationDialog/GridContainer/File.text
		var format = $ConfirmationDialog/GridContainer/Format.get_selected_id()
		var palette = $ConfirmationDialog/GridContainer/Palette.get_selected_id()
		print(format, palette)
		curfile.import_bitmap(path, format, palette)
		ChumReader.invalidate(curfile.get_hash_id())
		set_file(curfile)
