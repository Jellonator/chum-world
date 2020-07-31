extends Tree

signal node_selected(node)

var added_nodes = {}

func _sort_nodes(a, b):
	if len(a["children"]) != len(b["children"]):
		return len(a["children"]) > len(b["children"])
	if a["type"] != b["type"]:
		return a["type"] < b["type"]
	return a["name"] < b["name"]

func get_node_icon(node):
	if node["parent"] == 0:
		return MeshData.ICON_ROOT
	if node["type"] in MeshData.TYPE_ICONS:
		return MeshData.TYPE_ICONS[node["type"]]
	return MeshData.ICON_NODE

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
