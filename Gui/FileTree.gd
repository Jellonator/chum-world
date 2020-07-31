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

const TYPE_ICONS = {
	"ANIMATION": preload("res://Gui/Icon/animation.png"),
	"BITMAP": preload("res://Gui/Icon/bitmap.png"),
	"CAMERA": preload("res://Gui/Icon/camera.png"),
	"CAMERAZONE": preload("res://Gui/Icon/camerazone.png"),
	"COLLISIONVOL": preload("res://Gui/Icon/collisionvol.png"),
	"GAMEOBJ": preload("res://Gui/Icon/gameobj.png"),
	"HFOG": preload("res://Gui/Icon/hfog.png"),
	"LIGHT": preload("res://Gui/Icon/light.png"),
	"LOD": preload("res://Gui/Icon/lod.png"),
	"MATERIAL": preload("res://Gui/Icon/material.png"),
	"MATERIALANIM": preload("res://Gui/Icon/materialanim.png"),
	"MESH": preload("res://Gui/Icon/mesh.png"),
	"NODE": preload("res://Gui/Icon/node.png"),
	"OCCLUDER": preload("res://Gui/Icon/occluder.png"),
	"OMNI": preload("res://Gui/Icon/omni.png"),
	"PARTICLES": preload("res://Gui/Icon/particles.png"),
	"ROTSHAPE": preload("res://Gui/Icon/rotshape.png"),
	# RTC
	"SKIN": preload("res://Gui/Icon/skin.png"),
	"SOUND": preload("res://Gui/Icon/sound.png"),
	"SPLINE": preload("res://Gui/Icon/spline.png"),
	"SURFACE": preload("res://Gui/Icon/surface.png"),
	"TXT": preload("res://Gui/Icon/txt.png"),
	"USERDEFINE": preload("res://Gui/Icon/userdefine.png"),
	"WARP": preload("res://Gui/Icon/warp.png"),
	"WORLD": preload("res://Gui/Icon/world.png"),
}

const ICON_UNKNOWN := preload("res://Gui/Icon/unknown.png")

func get_type_icon(typename):
	if typename in TYPE_ICONS:
		return TYPE_ICONS[typename]
	return ICON_UNKNOWN

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
		if a.type != b.type:
			return a.type < b.type
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
		item.set_icon(0, get_type_icon(type))
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
