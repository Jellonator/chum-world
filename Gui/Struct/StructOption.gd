extends PanelContainer

signal modified(data)
var data: Dictionary
#var default_value: Dictionary
var current_node: Control
var can_emit = false

func set_data(p_data: Dictionary):
	can_emit = false
	self.data = p_data
#	default_value = data["default"]
	if data.has("value"):
		$VBoxContainer/CheckButton.pressed = true
		create_node(data["value"])
		$VBoxContainer/CheckButton.text = "Enabled"
	else:
		$VBoxContainer/CheckButton.text = "Disabled"
	can_emit = true

func create_node(new_data):
	current_node = Structure.instance(new_data)
	$VBoxContainer.add_child(current_node)
	current_node.set_data(new_data)
	current_node.connect("modified", self, "_on_modified")
	$VBoxContainer/CheckButton.text = "Enabled"

func delete_node():
	current_node.queue_free()
	current_node = null
	$VBoxContainer/CheckButton.text = "Disabled"

func _on_CheckButton_toggled(button_pressed):
	if not can_emit:
		return
	if button_pressed:
		create_node(data["default"].generate())
	else:
		delete_node()
	emit_signal("modified", self.data)

func _on_modified(_data):
	emit_signal("modified", self.data)
