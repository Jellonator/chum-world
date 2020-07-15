extends Node

const STRUCT_TYPES = {
	"enum": preload("res://Gui/Struct/StructEnum.tscn"),
	"flags": preload("res://Gui/Struct/StructFlags.tscn"),
	"integer": preload("res://Gui/Struct/StructInteger.tscn"),
#	"float"
#	"vec2"
#	"vec3"
#	"transform"
#	"array"
	"struct": preload("res://Gui/Struct/StructStruct.tscn"),
}

func instance(data: Dictionary):
	print("INSTANCE: ", data)
	var t = data["type"]
	if t in STRUCT_TYPES:
		var value = STRUCT_TYPES[t].instance()
		value.set_data(data)
		return value
	else:
		return null
