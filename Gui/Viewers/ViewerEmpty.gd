extends CenterContainer

func set_file(file):
	if file == null:
		$Label.text = "Nothing to view."
	else:
		$Label.text = "No viewer set up for file of type %s" % file.type
