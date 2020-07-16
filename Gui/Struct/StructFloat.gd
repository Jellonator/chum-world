extends SpinBox

signal modified(data)
var data: Dictionary

func set_data(data: Dictionary):
	self.data = data
	self.value = data["value"]
	self.editable = true

func _on_StructFloat_value_changed(value: float):
	data["value"] = value
	emit_signal("modified", self.data)
