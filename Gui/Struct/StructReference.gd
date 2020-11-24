extends Control

signal modified(data)
var data: Dictionary

onready var node_value := $HBox/SpinBox
onready var node_menu := $HBox/MenuButton
onready var node_text := $Text
onready var node_goto := $HBox/Goto
onready var menu := node_menu.get_popup() as PopupMenu

func get_viewer():
	return get_tree().get_nodes_in_group("viewer")[0]

func get_archive():
	return get_viewer().archive

func try_get_name(id: int):
	return get_archive().maybe_get_name_from_hash(id)

func _ready():
	var err = menu.connect("index_pressed", self, "_on_index_pressed")
	if err != OK:
		push_warning("Connect failed")

func set_data(p_data: Dictionary):
	_disable_emit = true
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
	update_buttons()
	_disable_emit = false

func update_buttons():
	var id = int(node_value.value)
	if get_archive().get_file_from_hash(id) != null:
		node_goto.disabled = false
	else:
		node_goto.disabled = true

func _on_index_pressed(id: int):
	node_value.value = menu.get_item_id(id)
	update_buttons()

var _disable_emit := false
func _on_SpinBox_value_changed(value):
	if _disable_emit:
		return
	print("REFERENCE EMIT")
	data["value"] = int(value)
	emit_signal("modified", self.data)
	node_text.text = try_get_name(value)
	update_buttons()

func _on_Goto_pressed():
	var id = int(node_value.value)
	var file = get_archive().get_file_from_hash(id)
	var tree = get_viewer().node_tree
	var editor = get_viewer().node_editor
	editor.set_file(file)
	tree.set_selected(file)

func _on_Text_text_entered(new_text: String):
	if not get_archive().register_name(new_text):
		node_text.text = get_archive().maybe_get_name_from_hash(int(node_value.value))
	else:
		node_value.value = get_archive().get_hash_of_name(new_text)
