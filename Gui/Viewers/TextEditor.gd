extends PanelContainer

onready var node_edit := $TextEdit
var filehandle = null

func set_file(file):
	filehandle = file
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

func _on_TextEdit_text_changed():
	ChumReader.invalidate(filehandle.get_hash_id())
	filehandle.replace_txt_with_string($TextEdit.text)
#	var t = $TextEdit.text
#	var i = 0
#	var s = ""
##	print(t)
#	for c in t:
#		if i > 0 and i % 32 == 0:
#			s += "\n"
#		var value = ord(c)
#		s += "%02X " % value
#		i += 1
#	print(s)
