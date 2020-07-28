extends Spatial

const font := preload("res://Font/Base.tres")

var text := ""

onready var draw := $Viewport/Node2D
onready var viewport := $Viewport
onready var sprite := $Sprite3D

func set_text(txt: String):
	self.text = txt
	viewport.size = font.get_string_size(txt)
	sprite.centered = false
	sprite.centered = true
	draw.update()

func _on_Node2D_draw():
	draw.draw_string(font, Vector2(), text)
