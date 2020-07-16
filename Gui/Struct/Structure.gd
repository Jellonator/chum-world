extends Node

const STRUCT_TYPES = {
	"enum": preload("res://Gui/Struct/StructEnum.tscn"),
	"flags": preload("res://Gui/Struct/StructFlags.tscn"),
	"integer": preload("res://Gui/Struct/StructInteger.tscn"),
	"float": preload("res://Gui/Struct/StructFloat.tscn"),
#	"vec2"
	"vec3": preload("res://Gui/Struct/StructVec3.tscn"),
#	"transform"
	"transform2d": preload("res://Gui/Struct/StructTransform2D.tscn"),
	"color": preload("res://Gui/Struct/StructColor.tscn"),
	"reference": preload("res://Gui/Struct/StructReference.tscn"),
	"array": preload("res://Gui/Struct/StructArray.tscn"),
	"struct": preload("res://Gui/Struct/StructStruct.tscn"),
}

func instance(data: Dictionary):
	var t = data["type"]
	if t in STRUCT_TYPES:
		var value = STRUCT_TYPES[t].instance()
		return value
	else:
		return null
