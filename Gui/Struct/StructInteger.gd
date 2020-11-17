extends SpinBox

signal modified(data)
var data: Dictionary

func set_data(p_data: Dictionary):
	self.data = p_data
	self.min_value = data["min"]
	self.max_value = data["max"]
	self.value = data["value"]
	self.editable = true

func _on_StructInteger_value_changed(value: float):
	data["value"] = int(value)
	emit_signal("modified", self.data)
