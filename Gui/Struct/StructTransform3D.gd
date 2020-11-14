extends GridContainer

signal modified(data)
var data: Dictionary

func set_data(p_data: Dictionary):
	self.data = p_data
	$X1.value = data["value"].basis.x.x
	$X2.value = data["value"].basis.x.y
	$X3.value = data["value"].basis.x.z
	$Y1.value = data["value"].basis.y.x
	$Y2.value = data["value"].basis.y.y
	$Y3.value = data["value"].basis.y.z
	$Z1.value = data["value"].basis.z.x
	$Z2.value = data["value"].basis.z.y
	$Z3.value = data["value"].basis.z.z
	$O1.value = data["value"].origin.x
	$O2.value = data["value"].origin.y
	$O3.value = data["value"].origin.z
	$X1.editable = true
	$X2.editable = true
	$X3.editable = true
	$Y1.editable = true
	$Y2.editable = true
	$Y3.editable = true
	$Z1.editable = true
	$Z2.editable = true
	$Z3.editable = true
	$O1.editable = true
	$O2.editable = true
	$O3.editable = true

func _on_X1_value_changed(value):
	data["value"].basis.x.x = value
	emit_signal("modified", self.data)

func _on_X2_value_changed(value):
	data["value"].basis.x.y = value
	emit_signal("modified", self.data)

func _on_X3_value_changed(value):
	data["value"].basis.x.z = value
	emit_signal("modified", self.data)

func _on_Y1_value_changed(value):
	data["value"].basis.y.x = value
	emit_signal("modified", self.data)

func _on_Y2_value_changed(value):
	data["value"].basis.y.y = value
	emit_signal("modified", self.data)

func _on_Y3_value_changed(value):
	data["value"].basis.y.z = value
	emit_signal("modified", self.data)

func _on_Z1_value_changed(value):
	data["value"].basis.z.x = value
	emit_signal("modified", self.data)

func _on_Z2_value_changed(value):
	data["value"].basis.z.y = value
	emit_signal("modified", self.data)

func _on_Z3_value_changed(value):
	data["value"].basis.z.z = value
	emit_signal("modified", self.data)

func _on_O1_value_changed(value):
	data["value"].origin.x = value
	emit_signal("modified", self.data)

func _on_O2_value_changed(value):
	data["value"].origin.y = value
	emit_signal("modified", self.data)

func _on_O3_value_changed(value):
	data["value"].origin.z = value
	emit_signal("modified", self.data)
