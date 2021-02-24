extends Control

onready var node_player := $AudioStreamPlayer as AudioStreamPlayer

func set_file(file):
	print(node_player)
	if node_player != null and node_player.playing:
		node_player.stop()
	if file != null:
		var sndview = ChumReader.get_sound_view(file)
		var stream := AudioStreamSample.new()
		stream.data = sndview.get_stream()
		stream.stereo = sndview.is_stereo()
		stream.format = sndview.get_format()
		stream.mix_rate = sndview.get_mix_rate()
		node_player.stream = stream
		node_player.play()
