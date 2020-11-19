extends GridContainer

var data: Dictionary

signal modified(data)

func set_data(p_data: Dictionary):
	self.data = p_data
	for child in self.get_children():
		child.queue_free()
	var names := data["order"] as PoolStringArray
	var values := data["value"] as Dictionary
#	print("STRUCT: ", names, " -> ", values)
	for name in names:
		var panel := PanelContainer.new()
		var hbox := MarginContainer.new()
		var label := Label.new()
		var btn := preload("res://Gui/Struct/StructStructHide.tscn").instance()
		label.text = "    " + name
		label.align = Label.ALIGN_LEFT
		hbox.add_child(btn)
		hbox.add_child(label)
#		hbox.alignment = BoxContainer.ALIGN_BEGIN
		panel.add_child(hbox)
		panel.add_stylebox_override("panel", preload("res://Gui/Struct/StructStructLabel.tres"))
		add_child(panel)
		var value = values[name]
		var instance = Structure.instance(value)
		btn.connect("toggled", self, "_hide_struct", [instance])
		var margin := MarginContainer.new()
#		margin.margin_left = 16
		margin.set("custom_constants/margin_left", 16)
		margin.add_child(instance)
		margin.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		add_child(margin)
		instance.set_data(value)
		instance.connect("modified", self, "_on_modified")

func _on_modified(_data):
	emit_signal("modified", self.data)

func _hide_struct(value: bool, instance: Control):
	instance.visible = not value
