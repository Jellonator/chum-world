extends Control

const ChumArchive := preload("res://gdchum/ChumArchive.gdns")
const MENU_FILE_OPEN := 0
const MENU_FILE_EXIT := 1

var archive: ChumArchive

onready var node_tree := $VBox/Tabs/Files/VBox/Tree
onready var node_editor := $VBox/Tabs/Files/EditorList

func _ready():
#	node_editor.set_file(null)
	archive = ChumArchive.new()
	$VBox/Menu/File.get_popup().connect("id_pressed", self, "_on_menu_file_select")

func _on_menu_file_select(id):
	match id:
		MENU_FILE_OPEN:
			$ArchiveFileSelector.popup_centered()
		MENU_FILE_EXIT:
			get_tree().quit(0)

func load_archive(ngc: String, dgc: String, ftype: String):
	node_editor.set_file(null)
	var err = archive.load(ngc, dgc, ftype)
	if err != OK:
		show_err("Could not open files %d" % [err])
	node_tree.set_archive(archive)

func _on_ArchiveFileSelector_files_selected(ngc: String, dgc: String, ftype: String):
	load_archive(ngc, dgc, ftype)

func show_err(text: String):
	$ErrDialog.dialog_text = text
	$ErrDialog.popup_centered()

func _on_Tree_file_selected(file):
	node_editor.set_file(file)
