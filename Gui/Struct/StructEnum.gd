extends OptionButton

signal modified(data)
var data: Dictionary
var can_emit = false

func set_data(p_data: Dictionary):
	can_emit = false
	self.data = p_data
	self.clear()
	var names := data["names"] as PoolStringArray
	for i in range(names.size()):
		var name := names[i]
		self.add_item(name, i)
	self.select(data["value"])
	self.disabled = false
	can_emit = true

func _on_StructEnum_item_selected(id):
	if can_emit:
		data["value"] = id
		emit_signal("modified", self.data)
