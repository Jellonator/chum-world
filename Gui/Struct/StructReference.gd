extends HBoxContainer

signal modified(data)
var data: Dictionary

onready var node_value := $SpinBox
onready var node_menu := $MenuButton
onready var menu := node_menu.get_popup() as PopupMenu

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
	var archive = get_tree().get_nodes_in_group("viewer")[0].archive
	var list = archive.get_file_list()
	for cfile in list:
		if typename == null or typename == cfile.type:
			var index = menu.get_item_count()
			menu.add_item(cfile.name, cfile.get_hash_id())
			menu.set_item_id(index, cfile.get_hash_id())

func _on_index_pressed(id: int):
	node_value.value = menu.get_item_id(id)

func _on_SpinBox_value_changed(value):
	print("REFERENCE EMIT")
	data["value"] = int(value)
	emit_signal("modified", self.data)
