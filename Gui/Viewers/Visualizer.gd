extends Control

# based on: https://github.com/godotengine/godot-demo-projects/tree/master/audio/spectrum

export var gradient: Gradient

const VU_COUNT := 16
const FREQ_MAX := 11050.0

# const WIDTH = 400
# const HEIGHT = 100

const MIN_DB := 60.0

var spectrum: AudioEffectSpectrumAnalyzerInstance

func _draw():
	var width := get_rect().size.x
	var height := get_rect().size.y
	#warning-ignore:integer_division
	var w = width / VU_COUNT
	var prev_hz = 0
	for i in range(1, VU_COUNT+1):
		var hz = i * FREQ_MAX / VU_COUNT;
		var magnitude: float = spectrum.get_magnitude_for_frequency_range(prev_hz, hz).length()
		var energy = clamp((MIN_DB + linear2db(magnitude)) / MIN_DB, 0, 1)
		var bar_height = energy * height / 2
		var color := gradient.interpolate(1.0 - (float(i) - 1.0) / (VU_COUNT - 1.0))
		if energy < 0.5:
			color = color.darkened(0.5 - energy)
		else:
			color = color.lightened(energy - 0.5)
		draw_rect(Rect2(w * (i-1), height / 2 - bar_height, w, bar_height), color)
		prev_hz = hz

func _process(delta):
	if is_visible_in_tree():
		update()

func _ready():
	var idx = AudioServer.get_bus_index("Analyze")
	spectrum = AudioServer.get_bus_effect_instance(idx, 0)
