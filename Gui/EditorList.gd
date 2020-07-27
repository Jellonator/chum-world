extends VBoxContainer

var cfile = null

onready var button_export := $HBoxContainer/ExportButton
onready var node_tabs := $Split/TabContainer
onready var node_struct := $Split/Margin/Panel/Scroll/VBox
onready var node_margin := $Split/Margin

func set_exportbutton(file):
	if file == null:
		button_export.disabled = true
		return
	else:
		button_export.disabled = false
	var typename = file.get_type()
	var popup = button_export.get_popup()
	for i in range(1, popup.get_item_count()):
		popup.set_item_disabled(i,
		not typename in ExportData.VALID_EXPORTS[popup.get_item_id(i)])

func set_tab(id: int, file):
	node_tabs.get_child(id).set_file(file)
	for i in node_tabs.get_child_count():
		if i != id:
			node_tabs.get_child(i).set_file(null)

func set_file(file):
	cfile = file
	set_tab(node_tabs.current_tab, file)
	set_exportbutton(file)
	for child in node_struct.get_children():
		child.queue_free()
	if file != null:
		var struct = file.read_structure()
		if struct != null:
			var instance = Structure.instance(struct)
			node_struct.add_child(instance)
			instance.set_data(struct)
			instance.connect("modified", self, "_on_struct_modified")
			node_margin.show()
		else:
			node_margin.hide()

func _on_struct_modified(data):
	print("REFRESH", data)
	cfile.import_structure(data)
#	ChumReader.invalidate(cfile.get_hash_id())
	ChumReader.clear_cache()
	refresh_viewer()

func refresh_viewer():
	node_tabs.get_child(node_tabs.current_tab).set_file(cfile)

func _on_TabContainer_tab_changed(tab):
	set_tab(tab, cfile)

func _ready():
	set_exportbutton(null)
	button_export.get_popup().connect("id_pressed", self, "_on_export_pressed")

func _on_export_pressed(id: int):
	if cfile == null:
		print("No file selected; could not export.")
	else:
		var dialog := owner.get_node("SaveDialog") as FileDialog
		dialog.filters = PoolStringArray([ExportData.EXPORT_FILE_FILTERS[id]])
		owner.export_file = cfile
		owner.export_mode = id
		dialog.popup_centered()
#		var fname = yield(dialog, "popup_hide")
#		print("ACCEPTED")
#		cfile.export_to(id, fname)
