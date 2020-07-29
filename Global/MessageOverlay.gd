extends Control

onready var node_logrect := $FullView/Color
onready var button_showlog := $Overlay/HBox/ShowLog
onready var node_log := $FullView/Color/Scroll/Log
onready var node_log_scroll := $FullView/Color/Scroll
onready var node_preview := $Overlay/Preview

const MAX_LINES := 1000
const MAX_PREVIEW_QUEUE := 500
const PREVIEW_QUEUE_UPPER := 20
const MAX_PREVIEW_NODES := 20
const DEFAULT_PREVIEW_SPEED := 1/5.0 # When queue is empty
const MIN_PREVIEW_SPEED := 1/2.0 # When queue >= 1
const MAX_PREVIEW_SPEED := 1/0.1 # When queue >= 100

# Labels in the message log
var _log_labels := []
# Preview node list. These give information even when the log isn't being shown.
# List of dictionaries with the following keys:
#     "timer": float - The amount of time that the node has been alive. The
#                      node will be killed when this value reaches 1.
#     "label": Label - The label node. Contains the text.
var _preview_nodes := []
var _preview_queue := []

enum MessageLevel {
	INFORMATION = 0,
	WARNING = 1,
	ERROR = 2
}

const TEXT_COLOR = {
	MessageLevel.INFORMATION: Color.white,
	MessageLevel.ERROR: Color.red,
	MessageLevel.WARNING: Color.yellow
}

# Get the speed at which preview messages will disappear.
func get_preview_speed() -> float:
	var value = range_lerp(_preview_queue.size(), 0, PREVIEW_QUEUE_UPPER,
		MIN_PREVIEW_SPEED, MAX_PREVIEW_SPEED)
	return clamp(value, MIN_PREVIEW_SPEED, MAX_PREVIEW_SPEED)

# Log the given message
func push(text: String, level: int):
	var label := Label.new()
	label.text = text
	label.add_color_override("font_color", TEXT_COLOR[level])
	node_log.add_child(label)
	_log_labels.append(label)
	while len(_log_labels) > MAX_LINES:
		_log_labels[0].queue_free()
		_log_labels.remove(0)
	if len(_preview_nodes) < MAX_PREVIEW_NODES:
		_add_preview_node(text, level)
	else:
		_preview_queue.append({
			"text": text,
			"level": level
		})

# Add a preview node to the preview list.
func _add_preview_node(text: String, level: int):
	var label := Label.new()
	label.text = text
	label.add_color_override("font_color", TEXT_COLOR[level])
	label.add_stylebox_override("normal", preload("res://Global/preview.tres"))
	label.focus_mode = Control.FOCUS_NONE
	label.mouse_filter = Control.MOUSE_FILTER_IGNORE
	node_preview.add_child(label)
	_preview_nodes.append({
		"label": label,
		"timer": 0.0
	})

func _physics_process(delta):
	if _preview_nodes.size() > 0:
		# Handle first node, remove it if its alive too long
		var speed := get_preview_speed()
		_preview_nodes[0]["timer"] += delta * speed
		if _preview_nodes[0]["timer"] >= 1:
			# Move the margin as part of the disappear animation
			var amt = _preview_nodes[0]["label"].rect_size.y 
			amt += node_preview.get_constant("separation")
			node_preview.margin_top += amt
			# Remove the node
			_preview_nodes[0]["label"].queue_free()
			_preview_nodes.remove(0)
		# "Fading" effect on disappearing nodes
		elif _preview_nodes[0]["timer"] >= 0.85:
			var alpha = range_lerp(
				_preview_nodes[0]["timer"], 0.85, 1.0, 1.0, 0.0)
			_preview_nodes[0]["label"].modulate.a = alpha
		# "cook" all existing nodes so that they don't stay on screen as long
		for i in range(1, _preview_nodes.size()):
			_preview_nodes[i]["timer"] = clamp(
				_preview_nodes[i]["timer"] + delta * speed, 0, 0.4)
	# Add items from the queue to the preview
	while _preview_nodes.size() < MAX_PREVIEW_NODES and _preview_queue.size() > 0:
		_add_preview_node(_preview_queue[0]["text"], _preview_queue[0]["level"])
		_preview_queue.remove(0)
	# Smoothly animate the margin back to its normal position
	if node_preview.margin_top > 32:
		var speed = 5 + abs(32 - node_preview.margin_top) * 6
		node_preview.margin_top = clamp(node_preview.margin_top - delta * speed, 32, 256)

func _on_HideLog_pressed():
	button_showlog.show()
	node_logrect.hide()

func _on_ShowLog_pressed():
	button_showlog.hide()
	node_logrect.show()
	yield(get_tree(), "idle_frame")
	var vbar = node_log_scroll.get_v_scrollbar()
	vbar.value = vbar.max_value

func _on_EmptyPreview_pressed():
	for item in _preview_nodes:
		item["label"].queue_free()
	_preview_nodes.clear()
	_preview_queue.clear()
