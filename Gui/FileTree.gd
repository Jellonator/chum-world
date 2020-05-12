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
			item.set_custom_color(0, Color.darkgray)
			item.set_text(0, s)
			var newh := HierarchyItem.new(item)
			current.dict[s] = newh
			current = newh
		i += 1
	return current.item

func set_archive(archive):
	clear()
	tree_root = create_item()
	hierarchy = HierarchyItem.new(tree_root)
	hide_root = true
	for file in archive.get_file_list():
		var name := file.name as String
		var item := create_item(get_item_parent(name))
		item.set_text(0, split_fname(name)[-1])
		item.set_custom_color(0, Color.white)
		item.set_meta("file", file)

func _on_Tree_item_selected():
	emit_signal("file_selected", get_selected().get_meta("file"))
