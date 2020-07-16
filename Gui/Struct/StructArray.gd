extends Control

signal modified(data)
var data: Dictionary

var array: Array
var can_resize: bool
var default_value: Dictionary

onready var node_elements := $VBox/Elements

func push_element(index: int, struct: Dictionary):
	var hbox := HBoxContainer.new()
	var label := Label.new()
	label.text = str(index)
	var instance = Structure.instance(struct)
	var btn_up := Button.new()
	btn_up.text = "^"
	var btn_down := Button.new()
	btn_down.text = "V"
	var btn_delete := Button.new()
	btn_delete.text = "X"
	if index == 0:
		btn_up.disabled = true
	hbox.add_child(label)
	hbox.add_child(instance)
	hbox.add_child(btn_up)
	hbox.add_child(btn_down)
	hbox.add_child(btn_delete)
	if not can_resize:
		btn_delete.hide()
	hbox.set_meta("label", label)
	hbox.set_meta("index", index)
	hbox.set_meta("btn_up", btn_up)
	hbox.set_meta("btn_down", btn_down)
	btn_up.connect("pressed", self, "_on_btnup_pressed", [hbox])
	btn_down.connect("pressed", self, "_on_btndown_pressed", [hbox])
	btn_delete.connect("pressed", self, "_on_delete_pressed", [hbox])
	node_elements.add_child(hbox)
	instance.set_data(struct)
	instance.connect("modified", self, "_on_modified")
	return hbox

func swap_elements(a: int, b: int):
	if a == b:
		return
	var num = node_elements.get_child_count()
	var hbox_a = node_elements.get_child(a)
	var hbox_b = node_elements.get_child(b)
	# Move elements (apparently this just works)
	node_elements.move_child(hbox_a, b)
	node_elements.move_child(hbox_b, a)
	# Set index metavalues
	hbox_a.set_meta("index", b)
	hbox_b.set_meta("index", a)
	hbox_a.get_meta("label").text = str(b)
	hbox_b.get_meta("label").text = str(a)
	hbox_a.get_meta("btn_up").disabled = (b == 0)
	hbox_b.get_meta("btn_up").disabled = (a == 0)
	hbox_a.get_meta("btn_down").disabled = (b == num-1)
	hbox_b.get_meta("btn_down").disabled = (a == num-1)
	# Swap array values
	var temp = self.array[a]
	self.array[a] = self.array[b]
	self.array[b] = temp

func _on_modified(_data):
	emit_signal("modified", self.data)

func _on_btnup_pressed(hbox):
	var index = hbox.get_meta("index")
	var other = index - 1
	swap_elements(other, index)
	emit_signal("modified", self.data)

func _on_btndown_pressed(hbox):
	var index = hbox.get_meta("index")
	var other = index + 1
	swap_elements(index, other)
	emit_signal("modified", self.data)

func _on_delete_pressed(hbox):
	var index = hbox.get_meta("index")
	var num = node_elements.get_child_count()
	for i in range(index + 1, num):
		var child = node_elements.get_child(i)
		child.set_meta("index", child.get_meta("index") - 1)
	self.array.remove(index)
	hbox.queue_free()
	emit_signal("modified", self.data)

func set_data(data: Dictionary):
	self.data = data
	self.array = data["value"]
	self.can_resize = data["can_resize"]
	self.default_value = data["default"]
	for child in node_elements.get_children():
		child.queue_free()
	var i := 0
	var last = null
	for value in self.array:
		last = push_element(i, value)
		i += 1
	if last != null:
		last.get_meta("btn_down").disabled = true
	if can_resize:
		$VBox/Append.show()
	else:
		$VBox/Append.hide()

func _on_Append_pressed():
	var newdata = self.default_value.duplicate(true)
	var i := self.array.size()
	self.array.append(newdata)
	push_element(i, newdata)
	emit_signal("modified", self.data)
