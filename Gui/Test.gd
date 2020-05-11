extends Control

const HelloWorld := preload("res://gdchum/ChumArchive.gdns")

var thing

func _ready():
	thing = HelloWorld.new()

func _input(event):
	if event.is_action_pressed("ui_home"):
		thing.do_thing()
