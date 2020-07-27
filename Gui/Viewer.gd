extends Control

const ChumArchive := preload("res://gdchum/ChumArchive.gdns")
const MENU_FILE_OPEN := 0
const MENU_FILE_EXIT := 1
const MENU_FILE_SAVE_AS := 2

const MENU_ARCHIVE_BULKEXPORT := 0

var archive: ChumArchive

onready var node_menu_file := $VBox/Menu/File
onready var node_tree := $VBox/Tabs/Files/VBox/Tree
onready var node_editor := $VBox/Tabs/Files/EditorList
onready var node_view3d := $"VBox/Tabs/3D View"

func _ready():
#	node_editor.set_file(null)
	archive = ChumArchive.new()
	$VBox/Menu/File.get_popup().connect(
		"id_pressed", self, "_on_menu_file_select")
	$VBox/Menu/Archive.get_popup().connect(
		"id_pressed", self, "_on_menu_archive_select")
	var tx = Transform2D()
	tx.x = Vector2(8.99999976e-1, -3.93402502e-8)
	tx.y = Vector2(3.93402502e-8, 8.99999976e-1)
	tx.origin = Vector2(-2.50000000e-1, 5.00000417e-2)
	print(tx)
	var tx_off = Transform2D().translated(Vector2(-0.5, -0.5))
	var tx_off2 = Transform2D().translated(Vector2(0.5, 0.5))
	print(tx_off * tx * tx_off2)
	print(Transform2D()\
		* tx_off2\
		* Transform2D().translated(Vector2(-0.3, 0.0))\
		* Transform2D().scaled(Vector2(0.9, 0.9))\
		* tx_off\
		)

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
	node_view3d.set_archive(archive)

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
