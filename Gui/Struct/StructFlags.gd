extends VBoxContainer

signal modified(data)
var data: Dictionary

func set_data(p_data: Dictionary):
	self.data = p_data
	for child in self.get_children():
		child.queue_free()
	var names := data["names"] as PoolStringArray
	var i := 1
	var value = data["value"]
	for name in names:
		var checkbox := CheckBox.new()
		checkbox.text = name
		if value & i != 0:
			checkbox.pressed = true
		add_child(checkbox)
		var err = checkbox.connect("toggled", self, "_on_checkbox_toggle", [i])
		if err != OK:
			push_warning("Connect failed")
		i = i << 1

func _on_checkbox_toggle(enabled: bool, index: int):
	var value = data["value"]
	if enabled:
		data["value"] = value | index
	else:
		data["value"] = value & ~index
	emit_signal("modified", self.data)
