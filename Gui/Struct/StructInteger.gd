extends SpinBox

signal modified(data)
var data: Dictionary

func set_data(data: Dictionary):
	self.data = data
	self.value = data["value"]
	self.min_value = data["min"]
	self.max_value = data["max"]
	self.editable = true

func _on_StructInteger_value_changed(value: float):
	data["value"] = int(value)
	emit_signal("modified", self.data)
