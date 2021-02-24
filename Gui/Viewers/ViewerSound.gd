extends Control

onready var node_player := $AudioStreamPlayer as AudioStreamPlayer
onready var node_slider := $VBox/Controls/HSlider as HSlider
onready var node_play := $VBox/Controls/Play as CheckButton

func set_file(file):
	# disable values
	_play_disable = true
	node_play.pressed = false
	_play_disable = false
	_slider_disable = true
	node_slider.value = 0
	_slider_disable = false
	# stop playing sound
	if node_player.playing:
		_player_disable = true
		node_player.stop()
		_player_disable = false
	if file != null:
		node_play.disabled = false
		node_slider.editable = true
		var sndview = ChumReader.get_sound_view(file)
		var stream := AudioStreamSample.new()
		stream.data = sndview.get_stream()
		stream.stereo = sndview.is_stereo()
		stream.format = sndview.get_format()
		stream.mix_rate = sndview.get_mix_rate()
		node_player.stream = stream
		_slider_disable = true # just in case
		node_slider.max_value = stream.get_length()
		_slider_disable = false
	else:
		node_play.disabled = true
		node_slider.editable = false

var _player_disable := false
func _on_AudioStreamPlayer_finished():
	if _player_disable or node_player.get_playback_position() < node_player.stream.get_length():
		return
	print("RESET")
	_play_disable = true
	node_play.pressed = false
	_play_disable = false
	_slider_disable = true
	node_slider.value = 0
	_slider_disable = false

var _play_disable := false
func _on_Play_toggled(button_pressed: bool):
	if _play_disable:
		return
	_player_disable = true
	if button_pressed:
		node_player.play(node_slider.value)
		print("TPLAY: ", node_slider.value)
	else:
		node_player.stop()
	_player_disable = false

func _physics_process(delta):
	if node_player.playing:
		_slider_disable = true
		node_slider.value = node_player.get_playback_position()
		_slider_disable = false

var _slider_disable := false
func _on_HSlider_value_changed(value: float):
	if _slider_disable:
		return
	if node_player.playing:
		_player_disable = true
		node_player.stop()
		node_player.play(value)
		print("SPLAY: ", node_slider.value)
		_player_disable = false
