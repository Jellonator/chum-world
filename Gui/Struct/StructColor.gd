extends HBoxContainer

signal modified(data)
var data: Dictionary

func set_data(p_data: Dictionary):
	self.data = p_data
	$ColorPickerButton.color = data["value"]
	$ColorPickerButton.disabled = false
	$ColorPickerButton.edit_alpha = data["has_alpha"]

func _on_ColorPickerButton_color_changed(color):
	data["value"] = color
	emit_signal("modified", self.data)
