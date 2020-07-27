extends HBoxContainer

signal modified(data)
var data: Dictionary

func set_data(data: Dictionary):
	self.data = data
	$ColorPickerButton.color = data["value"]
	$ColorPickerButton.disabled = false

func _on_ColorPickerButton_color_changed(color):
	data["value"] = color
	emit_signal("modified", self.data)
