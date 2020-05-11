extends Control

const HelloWorld := preload("res://gdchum/ChumArchive.gdns")

var thing

onready var node_NGC := $GridContainer/NGC
onready var node_DGC := $GridContainer/DGC

func _ready():
	thing = HelloWorld.new()

func _input(event):
	if event.is_action_pressed("ui_home"):
		print("START")
		for file in thing.get_file_list():
			print(file)
		print("END")

func _on_Button_pressed():
	thing.load(node_NGC.text, node_DGC.text)
