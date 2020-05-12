extends Control

const ChumArchive := preload("res://gdchum/ChumArchive.gdns")
const MENU_FILE_OPEN := 0
const MENU_FILE_EXIT := 1

var archive: ChumArchive

func _ready():
	archive = ChumArchive.new()
	$VBox/Menu/File.get_popup().connect("id_pressed", self, "_on_menu_file_select")

func _on_menu_file_select(id):
	match id:
		MENU_FILE_OPEN:
			$ArchiveFileSelector.popup_centered()
		MENU_FILE_EXIT:
			get_tree().quit(0)

func _on_ArchiveFileSelector_files_selected(ngc, dgc):
	var err = archive.load(ngc, dgc)
	if err != OK:
		show_err("Could not open files")

func show_err(text: String):
	$ErrDialog.dialog_text = text
	$ErrDialog.popup_centered()
