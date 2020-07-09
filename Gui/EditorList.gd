extends VBoxContainer

var cfile = null

onready var button_export := $HBoxContainer/ExportButton

const EXPORT_ID_BIN := 0
const EXPORT_ID_TEXT := 1
const EXPORT_ID_MODEL := 2
const EXPORT_ID_TEXTURE := 3

const VALID_EXPORTS := {
	EXPORT_ID_TEXT: ["TXT"],
	EXPORT_ID_MODEL: ["MESH", "SURFACE"],
	EXPORT_ID_TEXTURE: ["BITMAP"]
}

const EXPORT_FILE_FILTERS := {
	EXPORT_ID_BIN: "*.bin ; Binary Files",
	EXPORT_ID_TEXT: "*.txt ; Text Files",
	EXPORT_ID_MODEL: "*.obj ; Wavefront OBJ",
	EXPORT_ID_TEXTURE: "*.png ; PNG Images",
}

func set_exportbutton(file):
	if file == null:
		button_export.disabled = true
		return
	else:
		button_export.disabled = false
	var typename = file.get_type()
	var popup = button_export.get_popup()
	for i in range(1, popup.get_item_count()):
		popup.set_item_disabled(i, not typename in VALID_EXPORTS[popup.get_item_id(i)])

func set_tab(id: int, file):
	$TabContainer.get_child(id).set_file(file)
	for i in $TabContainer.get_child_count():
		if i != id:
			$TabContainer.get_child(i).set_file(null)

func set_file(file):
	cfile = file
	set_tab($TabContainer.current_tab, file)
	set_exportbutton(file)

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
		dialog.filters = PoolStringArray([EXPORT_FILE_FILTERS[id]])
		owner.export_file = cfile
		owner.export_mode = id
		dialog.popup_centered()
#		var fname = yield(dialog, "popup_hide")
#		print("ACCEPTED")
#		cfile.export_to(id, fname)
