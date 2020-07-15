extends GridContainer

var data: Dictionary

signal modified(data)

func set_data(data: Dictionary):
	self.data = data
	for child in self.get_children():
		child.queue_free()
	var names := data["order"] as PoolStringArray
	var values := data["value"] as Dictionary
#	print("STRUCT: ", names, " -> ", values)
	for name in names:
		var label := Label.new()
		label.text = name
		add_child(label)
		var value = values[name]
		var instance = Structure.instance(value)
		add_child(instance)
		instance.connect("modified", self, "_on_modified")

func _on_modified(_data):
	emit_signal("modified", self.data)
