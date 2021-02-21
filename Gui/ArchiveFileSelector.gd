extends ConfirmationDialog

signal files_selected(ngc, dgc, ftype)

func _on_NGCButton_pressed():
	$NGCDialog.popup_centered()

func _on_DGCButton_pressed():
	$DGCDialog.popup_centered()

func _on_NGCDialog_file_selected(path: String):
	$Grid/NGCLine.text = path
	if $Grid/AutoFile.pressed:
		var ext := path.get_extension()
		if ext.length() == 0:
			ext = "DGC"
		else:
			if ext[0] == ext[0].to_lower():
				ext = "d" + ext.substr(1)
			else:
				ext = "D" + ext.substr(1)
		$Grid/DGCLine.text = path.get_basename() + "." + ext

func _on_DGCDialog_file_selected(path: String):
	$Grid/DGCLine.text = path
	if $Grid/AutoFile.pressed:
		var ext := path.get_extension()
		if ext.length() == 0:
			ext = "NGC"
		else:
			if ext[0] == ext[0].to_lower():
				ext = "n" + ext.substr(1)
			else:
				ext = "N" + ext.substr(1)
		$Grid/NGCLine.text = path.get_basename() + "." + ext

func _on_ArchiveFileSelector_confirmed():
	var ftype = ""
	if $Grid/OptionButton.selected == 0:
		ftype = "NGC"
	elif $Grid/OptionButton.selected == 1:
		ftype = "PS2"
	emit_signal("files_selected", $Grid/NGCLine.text, $Grid/DGCLine.text, ftype)
