extends Control

const ChumArchive := preload("res://gdchum/ChumArchive.gdns")
const MENU_FILE_OPEN := 0
const MENU_FILE_EXIT := 1
const MENU_FILE_SAVE_AS := 2

const MENU_EXPORT_BULK_EXPORT := 0
const MENU_EXPORT_SCENE_EXPORT := 1

const MENU_VIEW_CULLING := 0
const MENU_VIEW_ALPHA := 1

const MENU_HELP_GUIDE := 0
const MENU_HELP_ABOUT := 1

var archive: ChumArchive
var should_3dview_reload := false

onready var node_menu_file := $VBox/Panel/Menu/File
onready var node_menu_export := $VBox/Panel/Menu/Export
onready var node_menu_view := $VBox/Panel/Menu/View
onready var node_menu_help := $VBox/Panel/Menu/Help
onready var node_tree := $VBox/Tabs/Files/VBox/Tree
onready var node_editor := $VBox/Tabs/Files/EditorList
onready var node_view3d := $"VBox/Tabs/3D View"
onready var node_tabs := $VBox/Tabs as TabContainer
onready var node_about_dialog := $AboutDialog
onready var node_scene_export_dialog := $SceneExportDialog
onready var node_global_viewport := $Viewport

func _ready():
	print(get_path())
	node_view3d.set_active(false)
	archive = ChumArchive.new()
	node_menu_file.get_popup().connect(
		"id_pressed", self, "_on_menu_file_select")
	node_menu_export.get_popup().connect(
		"id_pressed", self, "_on_menu_export_select")
	node_menu_help.get_popup().connect(
		"id_pressed", self, "_on_menu_help_select")

func _on_menu_file_select(id):
	match id:
		MENU_FILE_OPEN:
			$ArchiveFileSelector.popup_centered()
		MENU_FILE_EXIT:
			$ExitDialogue.popup_centered()
		MENU_FILE_SAVE_AS:
			$ArchiveFileSaver.popup_centered()

func _on_menu_export_select(id):
	if archive == null:
		return
	match id:
		MENU_EXPORT_BULK_EXPORT:
			$BulkExport.show_with_archive(archive)
		MENU_EXPORT_SCENE_EXPORT:
			node_scene_export_dialog.set_archive(archive)
			node_scene_export_dialog.popup_centered()

func _on_menu_help_select(id):
	match id:
		MENU_HELP_ABOUT:
			node_about_dialog.popup_centered()
		MENU_HELP_GUIDE:
			OS.shell_open("https://github.com/Jellonator/chum-world/wiki/User-Guide")

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

func _on_ArchiveFileSaver_files_selected(ngc, dgc, _ftype):
	archive.save(ngc, dgc)

func _on_Tabs_tab_changed(tab):
	if tab == 1:
		if should_3dview_reload:
			should_3dview_reload = false
			node_view3d.set_archive(archive)
		node_view3d.set_active(true)
	else:
		node_view3d.set_active(false)
