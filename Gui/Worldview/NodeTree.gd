extends Tree

signal node_selected(node)

var added_nodes = {}
const NODE_TYPE_ICONS = {
	"SURFACE": preload("res://Gui/Icon/surface.png"),
	"SPLINE": preload("res://Gui/Icon/spline.png"),
	"SKIN": preload("res://Gui/Icon/skin.png"),
	"ROTSHAPE": preload("res://Gui/Icon/rotshape.png"),
	"LOD": preload("res://Gui/Icon/lod.png"),
	"MESH": preload("res://Gui/Icon/mesh.png"),
	"CAMERA": preload("res://Gui/Icon/camera.png"),
	# 8
	# 9
	"OCCLUDER": preload("res://Gui/Icon/occluder.png"),
	"CAMERAZONE": preload("res://Gui/Icon/camerazone.png"),
	"LIGHT": preload("res://Gui/Icon/light.png"),
	"HFOG": preload("res://Gui/Icon/hfog.png"),
	"COLLISIONVOL": preload("res://Gui/Icon/collisionvol.png"),
	# 15
	"OMNI": preload("res://Gui/Icon/omni.png"),
	# 17
	"PARTICLES": preload("res://Gui/Icon/particles.png"),
}

const ICON_ROOT := preload("res://Gui/Icon/root.png")
const ICON_NODE := preload("res://Gui/Icon/node.png")

func _sort_nodes(a, b):
	if len(a["children"]) != len(b["children"]):
		return len(a["children"]) > len(b["children"])
	if a["type"] != b["type"]:
		return a["type"] < b["type"]
	return a["name"] < b["name"]

func get_node_icon(node):
	if node["parent"] == 0:
		return ICON_ROOT
	if node["type"] in NODE_TYPE_ICONS:
		return NODE_TYPE_ICONS[node["type"]]
	return ICON_NODE

func add_node(node, item_parent):
	if node in added_nodes:
		MessageOverlay.push_warn("Node already in tree!")
		return
	else:
		added_nodes[node] = item_parent
	var item = create_item(item_parent)
	item.set_text(0, node["name"])
	item.set_custom_color(0, Color.white)
	item.set_meta("node", node)
	item.set_icon(0, get_node_icon(node))
	node["children"].sort_custom(self, "_sort_nodes")
	for child in node["children"]:
		add_node(child, item)

func assemble_tree(node_root):
	clear()
	added_nodes.clear()
	var tree_root = create_item()
	if node_root == null:
		return
	add_node(node_root, tree_root)

func _ready():
	columns = 1
	set_column_titles_visible(true)
	set_column_title(0, "Name")

func _on_Items_item_selected():
	emit_signal("node_selected", get_selected().get_meta("node"))
