extends OptionButton

var data: Dictionary

func set_data(data: Dictionary):
	self.data = data
	self.clear()
	var names := data["names"] as PoolStringArray
	for i in range(names.size()):
		var name := names[i]
		self.add_item(name, i)
	self.select(data["value"])
	self.disabled = false

func _on_StructEnum_item_selected(id):
	data["value"] = id
