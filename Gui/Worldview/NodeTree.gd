extends Tree

signal node_selected(node)

var added_nodes = {}
var item_root = null

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
	if node["id"] in added_nodes:
		MessageOverlay.push_warn("Node already in tree!")
		return
	var item = create_item(item_parent)
	item.set_text(0, node["name"])
	item.set_custom_color(0, Color.white)
	item.set_meta("node", node)
	item.set_icon(0, get_node_icon(node))
	added_nodes[node["id"]] = item
	node["visible_children"].sort_custom(self, "_sort_nodes")
	for child in node["visible_children"]:
		add_node(child, item)

func assemble_tree(node_root):
	item_root = node_root
	do_search(prev_search)

func _ready():
	columns = 1
	set_column_titles_visible(true)
	set_column_title(0, "Name")

func _on_Items_item_selected():
	var prev = get_owner().selected_node
	emit_signal("node_selected", get_selected().get_meta("node"))
	# Remove the previously selected node (and potentially its parents)
	# if it no longer matches the user's filter.
	while prev != null and not _should_include(prev, prev_search) and added_nodes.has(prev["id"]):
		var item = added_nodes.get(prev["id"])
		var parent = item.get_parent()
		parent.remove_child(item)
		parent.get_meta("node")["visible_children"].erase(prev)
		added_nodes.erase(prev["id"])
		prev = parent.get_meta("node")

func _should_include(item, text: String) -> bool:
	var name = item["name"].to_lower()
	return text == ""\
		or text in name\
		or item["visible_children"].size() > 0\
		or item == get_owner().selected_node

func filter_tree(item, text: String) -> bool:
	if not "visible_children" in item:
		item["visible_children"] = []
	else:
		item["visible_children"].clear()
	for child in item["children"]:
		if filter_tree(child, text):
			item["visible_children"].append(child)
	return _should_include(item, text)

var prev_search := ""
func do_search(text: String):
	clear()
	added_nodes.clear()
	var tree_root = create_item()
	if item_root == null:
		return
	if filter_tree(item_root, text.to_lower()):
		add_node(item_root, tree_root)
	var item = get_owner().selected_node
	if item != null and item["id"] in added_nodes:
		added_nodes[item["id"]].select(0)
		ensure_cursor_is_visible()
	prev_search = text.to_lower()

func _on_Search_text_changed(new_text: String):
	do_search(new_text)

func try_select(_item):
	do_search(prev_search)
