extends HBoxContainer

signal modified(data)
var data: Dictionary

func set_data(p_data: Dictionary):
	self.data = p_data
	$X.value = data["value"].x
	$Y.value = data["value"].y
	$X.editable = true
	$Y.editable = true

func _on_X_value_changed(value):
	data["value"].x = value
	emit_signal("modified", self.data)

func _on_Y_value_changed(value):
	data["value"].y = value
	emit_signal("modified", self.data)
