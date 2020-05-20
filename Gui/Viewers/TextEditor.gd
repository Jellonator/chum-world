extends PanelContainer

onready var node_edit := $TextEdit

func set_file(file):
	if file == null:
		node_edit.text = ""
		node_edit.readonly = true
		return
	var value = ChumReader.read_text(file)
	if value["exists"]:
		node_edit.text = value["text"]
		node_edit.readonly = value["readonly"]
	else:
		node_edit.readonly = true
		node_edit.text = "Invalid"
