extends ConfirmationDialog

var carchive = null

var FTYPES = {}

onready var node_filetype := $GridContainer/FileType
onready var node_exporttype := $GridContainer/ExportType
onready var node_exportpath := $GridContainer/HBox/ExportPath
onready var node_folderdialog := $FolderDialog

func show_with_archive(archive):
	carchive = archive
	popup_centered()

func _ready():
	print(ExportData.EXPORTS_BY_TYPE)
	node_filetype.clear()
	FTYPES.clear()
	for name in ExportData.FILE_TYPES:
		var index = node_filetype.get_item_count()
		node_filetype.add_item(name)
		FTYPES[index] = {
			"export": ExportData.EXPORTS_BY_TYPE[name],
			"id": node_filetype.get_item_id(index),
			"name": name
		}
	node_filetype.select(0)
	update_export_type()

func update_export_type():
	var index = node_filetype.selected
	node_exporttype.clear()
	for id in ExportData.EXPORTS_BY_TYPE[FTYPES[index]["name"]]:
		node_exporttype.add_item(ExportData.EXPORT_NAMES[id], id)
	node_exporttype.select(node_exporttype.get_item_count() - 1)

func _on_BulkExport_confirmed():
	var index = node_filetype.selected
	var typename = FTYPES[index]["name"]
	var exportid = node_exporttype.get_selected_id()
	var exportpath = node_exportpath.text
	if not Directory.new().dir_exists(exportpath):
		print("Could not export!")
		return
	prints("EXPORT ALL", exportid)
	var extension = ExportData.EXPORT_EXTENSIONS[exportid]
	for file in carchive.get_file_list():
		if file.type == typename:
			var path := file.name as String
			var a = path.find_last(">")
			if a != -1:
				path = path.substr(a+1, -1)
			var b = path.find(".")
			if b != -1:
				path = path.substr(0, b)
			path = exportpath + "/" + path + "." + extension
			file.export_to(exportid, path)

func _on_FileType_item_selected(index: int):
	update_export_type()

func _on_ExportPathSelect_pressed():
	node_folderdialog.popup_centered()
