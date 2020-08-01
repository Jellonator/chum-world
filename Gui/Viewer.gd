extends Control

const ChumArchive := preload("res://gdchum/ChumArchive.gdns")
const MENU_FILE_OPEN := 0
const MENU_FILE_EXIT := 1
const MENU_FILE_SAVE_AS := 2

const MENU_ARCHIVE_BULKEXPORT := 0

const MENU_VIEW_CULLING := 0
const MENU_VIEW_ALPHA := 1

var archive: ChumArchive
var should_3dview_reload := false

onready var node_menu_file := $VBox/Menu/File
onready var node_tree := $VBox/Tabs/Files/VBox/Tree
onready var node_editor := $VBox/Tabs/Files/EditorList
onready var node_view3d := $"VBox/Tabs/3D View"
onready var node_tabs := $VBox/Tabs

func _ready():
	archive = ChumArchive.new()
	$VBox/Menu/File.get_popup().connect(
		"id_pressed", self, "_on_menu_file_select")
	$VBox/Menu/Archive.get_popup().connect(
		"id_pressed", self, "_on_menu_archive_select")

func _on_menu_file_select(id):
	match id:
		MENU_FILE_OPEN:
			$ArchiveFileSelector.popup_centered()
		MENU_FILE_EXIT:
			get_tree().quit(0)
		MENU_FILE_SAVE_AS:
			$ArchiveFileSaver.popup_centered()

func _on_menu_archive_select(id):
	match id:
		MENU_ARCHIVE_BULKEXPORT:
			if archive != null:
				$BulkExport.show_with_archive(archive)

func load_archive(ngc: String, dgc: String, ftype: String):
	node_editor.set_file(null)
	var err = archive.load(ngc, dgc, ftype)
	var popup = node_menu_file.get_popup()
	if err != OK:
		show_err("Could not open files %d" % [err])
		popup.set_item_disabled(popup.get_item_id(MENU_FILE_SAVE_AS), true)
	else:
		popup.set_item_disabled(popup.get_item_id(MENU_FILE_SAVE_AS), false)
	ChumReader.clear_cache()
	node_tree.set_archive(archive)
	should_3dview_reload = true
	_on_Tabs_tab_changed(node_tabs.current_tab)

func _on_ArchiveFileSelector_files_selected(ngc: String, dgc: String, ftype: String):
	load_archive(ngc, dgc, ftype)

func show_err(text: String):
	$ErrDialog.dialog_text = text
	$ErrDialog.popup_centered()

func _on_Tree_file_selected(file):
	node_editor.set_file(file)

var export_mode := 0
var export_file = null
func _on_SaveDialog_file_selected(path):
	export_file.export_to(export_mode, path)

func _on_ArchiveFileSaver_files_selected(ngc, dgc, ftype):
	archive.save(ngc, dgc)

func _on_Tabs_tab_changed(tab):
	if tab == 1 and should_3dview_reload:
		should_3dview_reload = false
		node_view3d.set_archive(archive)
