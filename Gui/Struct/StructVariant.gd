extends PanelContainer

signal modified(data)
var data: Dictionary
var current_node: Control
var can_emit = false

func set_data(p_data: Dictionary):
	can_emit = false
	self.data = p_data
	current_node = Structure.instance(data["value"])
	$VBox.add_child(current_node)
	current_node.set_data(data["value"])
	current_node.connect("modified", self, "_on_modified")
	var names := data["order"] as Array
	for i in range(names.size()):
		var name := names[i] as String
		$VBox/Option.add_item(name, i)
	$VBox/Option.select(names.find(data["current"]))
	$VBox/Option.disabled = false
	can_emit = true

func _on_modified(_data):
	emit_signal("modified", self.data)

func _on_Option_item_selected(id: int):
	if can_emit:
		current_node.queue_free()
		var name = self.data["order"][id]
		self.data["current"] = name
		self.data["value"] = self.data["options"][name].generate()
		current_node = Structure.instance(data["value"])
		$VBox.add_child(current_node)
		current_node.set_data(data["value"])
		current_node.connect("modified", self, "_on_modified")
		emit_signal("modified", self.data)
