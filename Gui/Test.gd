extends Control

const HelloWorld := preload("res://gdchum/ChumArchive.gdns")

var thing

onready var node_NGC := $GridContainer/NGC
onready var node_DGC := $GridContainer/DGC

func _ready():
	thing = HelloWorld.new()

func print_hex(bytes: PoolByteArray, perline: int):
	var i = 0
	var s = ""
	for byte in bytes:
		if i % perline == 0:
			if i > 0:
				s += "\n"
		else:
			s += " "
		s += "%02X" % byte
		i += 1
	print(s)

func _input(event):
	if event.is_action_pressed("ui_home"):
		var ls = thing.get_file_list()
		prints("START", ls)
		for file in thing.get_file_list():
			print(file.name)
			print(file.type)
			print(file.subtype)
			print_hex(file.data, 32)
		print("END")

func _on_Button_pressed():
	prints("LOAD", thing.load(node_NGC.text, node_DGC.text))
