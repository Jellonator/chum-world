extends GridContainer

signal modified(data)
var data: Dictionary

func set_data(p_data: Dictionary):
	self.data = p_data
	$X1.value = data["value"].x.x
	$X2.value = data["value"].x.y
	$Y1.value = data["value"].y.x
	$Y2.value = data["value"].y.y
	$O1.value = data["value"].origin.x
	$O2.value = data["value"].origin.y
	$X1.editable = true
	$X2.editable = true
	$Y1.editable = true
	$Y2.editable = true
	$O1.editable = true
	$O2.editable = true

func _on_X1_value_changed(value):
	data["value"].x.x = value
	emit_signal("modified", self.data)

func _on_X2_value_changed(value):
	data["value"].x.y = value
	emit_signal("modified", self.data)

func _on_Y1_value_changed(value):
	data["value"].y.x = value
	emit_signal("modified", self.data)

func _on_Y2_value_changed(value):
	data["value"].y.y = value
	emit_signal("modified", self.data)

func _on_O1_value_changed(value):
	data["value"].origin.x = value
	emit_signal("modified", self.data)

func _on_O2_value_changed(value):
	data["value"].origin.y = value
	emit_signal("modified", self.data)
