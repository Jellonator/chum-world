extends Control

signal modified(data)
var data: Dictionary

onready var node_value := $HBox/SpinBox
onready var node_menu := $HBox/MenuButton
onready var node_text := $Text
onready var menu := node_menu.get_popup() as PopupMenu

func get_archive():
	return get_tree().get_nodes_in_group("viewer")[0].archive

func try_get_name(id: int):
	return get_archive().maybe_get_name_from_hash(id)

func _ready():
	var err = menu.connect("index_pressed", self, "_on_index_pressed")
	if err != OK:
		push_warning("Connect failed")

func set_data(p_data: Dictionary):
	self.data = p_data
	node_value.value = data["value"]
	node_value.editable = true
	node_menu.disabled = false
	var typename = data["reference"]
	menu.clear()
	var archive = get_archive()
	var list = archive.get_file_list()
	for cfile in list:
		if typename == null or typename == cfile.type:
			var index = menu.get_item_count()
			menu.add_item(cfile.name, cfile.get_hash_id())
			menu.set_item_id(index, cfile.get_hash_id())
	node_text.text = try_get_name(data["value"])

func _on_index_pressed(id: int):
	node_value.value = menu.get_item_id(id)

func _on_SpinBox_value_changed(value):
	print("REFERENCE EMIT")
	data["value"] = int(value)
	emit_signal("modified", self.data)
	node_text.text = try_get_name(value)
