extends ConfirmationDialog

signal files_selected(ngc, dgc)

func _on_NGCButton_pressed():
	$NGCDialog.popup_centered()

func _on_DGCButton_pressed():
	$DGCDialog.popup_centered()

func _on_NGCDialog_file_selected(path):
	$Grid/NGCLine.text = path

func _on_DGCDialog_file_selected(path):
	$Grid/DGCLine.text = path

func _on_ArchiveFileSelector_confirmed():
	emit_signal("files_selected", $Grid/NGCLine.text, $Grid/DGCLine.text)
