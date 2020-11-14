extends HSplitContainer

const MONOFONT := preload("res://Font/Mono.tres")
const ByteData := preload("res://gdchum/ByteData.gdns")

var select_from := 0
var select_to := 0
var is_selecting := false
var data = ByteData.new()
var bytes_per_line := 16
var x = 0

onready var node_chars := $Right/Chars
onready var node_hex := $Left/HBox/Hex
onready var node_scroll := $Left/HBox/Scroll

func refresh_view():
	node_chars.update()
	node_hex.update()

func _ready():
	node_scroll.max_value = 1
	var err = connect("item_rect_changed", self, "_on_HexEditor_item_rect_changed")
	if err != OK:
		push_warning("Connect failed")

func get_font_height() -> int:
	return int(MONOFONT.get_string_size("w").y)

func get_font_width() -> int:
	return int(MONOFONT.get_string_size("w").x)

func get_scroll_line() -> int:
	return int(node_scroll.value)

func get_last_line() -> int:
	return int(ceil(float(get_last_byte()) / bytes_per_line))

func get_first_byte() -> int:
	return get_scroll_line() * bytes_per_line

func get_last_byte() -> int:
	var first := get_first_byte()
	var viewlen := int(node_hex.rect_size.y / 16) * bytes_per_line
	return int(min(first + viewlen, data.size()))

func get_char(value: int) -> String:
	if value >= 32 and value <= 126:
		return char(value)
	else:
		return "."

func set_data(p_data):
	self.data = p_data
	refresh_view()
	node_scroll.value = 0
	node_scroll.max_value = max(1, int(ceil(float(data.size() / bytes_per_line))))

func set_file(file):
	if file == null:
		set_data(ByteData.new())
	else:
		set_data(file.data)

func _on_Hex_draw():
	var iy := -1
	var offset := 4 + get_font_width() * 11
	for i in range(get_first_byte(), get_last_byte()):
		if i % bytes_per_line == 0:
			iy += 1
		var s = "%02X" % data.get(i)
		var pos := Vector2()
		pos.x = get_font_width() * (i % bytes_per_line) * 3 + offset
		pos.y = get_font_height() * (iy + 1)
		node_hex.draw_string(MONOFONT, pos, s, Color.white)
	for i in range(get_scroll_line(), get_last_line()):
		var s := "%08X" % (i * bytes_per_line)
		var pos := Vector2()
		pos.x = 4
		pos.y = get_font_height() * (i + 1 - get_scroll_line())
		node_hex.draw_string(MONOFONT, pos, s, Color.white)

func _on_Chars_draw():
	var iy := 0
	for i in range(get_first_byte(), get_last_byte()):
		if i % bytes_per_line == 0:
			iy += 1
		var c := get_char(data.get(i))
		var pos := Vector2()
		pos.x = get_font_width() * (i % bytes_per_line) + 4
		pos.y = get_font_height() * iy
		node_chars.draw_string(MONOFONT, pos, c, Color.white)

func _on_HexEditor_item_rect_changed():
	refresh_view()

func _on_Scroll_scrolling():
	refresh_view()

func _input(event):
	if not has_focus():
		return
	if event.is_action("ui_scroll_up") and event.is_pressed():
		node_scroll.value -= 1
		refresh_view()
	elif event.is_action("ui_scroll_down") and event.is_pressed():
		node_scroll.value += 1
		refresh_view()
	elif event.is_action("ui_page_up") and event.is_pressed():
		node_scroll.value -= 16
		refresh_view()
	elif event.is_action("ui_page_down") and event.is_pressed():
		node_scroll.value += 16
		refresh_view()
