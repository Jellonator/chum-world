extends Tree

var added_nodes = {}

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
