extends Tree

signal file_selected(file)

const ChumArchive := preload("res://gdchum/ChumArchive.gdns")
const ChumFile := preload("res://gdchum/ChumFile.gdns")

class HierarchyItem:
	var item: TreeItem
	var dict: Dictionary
	func _init(p_item: TreeItem):
		self.item = p_item
		self.dict = {}

var hierarchy: HierarchyItem
var tree_root: TreeItem
var archive = null
var archive_files = []

func split_fname(name: String) -> PoolStringArray:
	return name.split(">", false)

func get_item_parent(name: String) -> TreeItem:
	var split := split_fname(name)
	var current := hierarchy
	var i := 0
	while i < split.size() - 1:
		var s := split[i]
		if s in current.dict:
			current = current.dict[s]
		else:
			var item := create_item(current.item)
			item.set_selectable(0, false)
			item.set_selectable(1, false)
			item.set_selectable(2, false)
			item.set_custom_color(0, Color.darkgray)
			item.set_text(0, s)
			item.set_meta("category", true)
			var newh := HierarchyItem.new(item)
			current.dict[s] = newh
			current = newh
		i += 1
	return current.item

func _sort_file(a, b):
	if a.name != b.name:
		var acount = a.name.count(">")
		var bcount = b.name.count(">")
		if acount != bcount:
			return bcount < acount
		return a.name < b.name
	elif a.type != b.type:
		return a.type < b.type
	else:
		return a.subtype < b.subtype

var prev_search := ""
func do_search(text: String):
	prev_search = text
	prints("SEARCH", text, archive)
	text = text.to_lower()
	clear()
	tree_root = create_item()
	hierarchy = HierarchyItem.new(tree_root)
	hide_root = true
	if archive == null:
		return
	for file in archive_files:
		var name := file.name as String
		var type := file.type as String
		var subtype := file.subtype as String
		if text != "" and not text in name.to_lower()\
					  and not text in type.to_lower()\
					  and not text in subtype.to_lower():
			continue
		var item := create_item(get_item_parent(name))
		item.set_text(0, split_fname(name)[-1])
		item.set_text(1, type)
		item.set_text(2, subtype)
		item.set_custom_color(0, Color.white)
		item.set_meta("file", file)
		item.set_meta("category", false)

func set_archive(p_archive):
	self.archive = p_archive
	self.archive_files = archive.get_file_list()
	archive_files.sort_custom(self, "_sort_file")
	do_search(prev_search)

func _on_Tree_item_selected():
	emit_signal("file_selected", get_selected().get_meta("file"))

func _ready():
	columns = 3
	set_column_titles_visible(true)
	set_column_title(0, "Name")
	set_column_title(1, "Type")
	set_column_title(2, "Subtype")
	set_column_min_width(0, 8)
	set_column_min_width(1, 4)
	set_column_min_width(2, 4)

func _on_LineEdit_text_changed(new_text: String):
	do_search(new_text)
