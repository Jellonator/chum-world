extends Node

const FILE_TYPES = [
	"ANIMATION",
	"BITMAP",
	"CAMERA",
	"CAMERAZONE",
	"COLLISIONVOL",
	"GAMEOBJ",
	"HFOG",
	"LIGHT",
	"LOD",
	"MATERIAL",
	"MATERIALANIM",
	"MATERIALOBJ",
	"MESH",
	"NODE",
	"OCCLUDER",
	"OMNI",
	"PARTICLES",
	"ROTSHAPE",
	"RTC",
	"SKIN",
	"SOUND",
	"SPLINE",
	"SURFACE",
	"TXT",
	"USERDEFINE",
	"WARP",
	"WORLD",
]

const EXPORT_ID_BIN := 0
const EXPORT_ID_TEXT := 1
const EXPORT_ID_MODEL := 2
const EXPORT_ID_TEXTURE := 3
const EXPORT_ID_SCENE := 4
const EXPORT_ID_WAV := 5

const EXPORT_NAMES := {
	EXPORT_ID_BIN: "Raw Binary (.bin)",
	EXPORT_ID_TEXT: "Text (.txt)",
	EXPORT_ID_MODEL: "Model (.obj)",
	EXPORT_ID_TEXTURE: "Texture (.png)",
	EXPORT_ID_SCENE: "Scene (.glb/.gltf)",
	EXPORT_ID_WAV: "Sound (.wav)"
}

const EXPORT_EXTENSIONS := {
	EXPORT_ID_BIN: ["bin"],
	EXPORT_ID_TEXT: ["txt"],
	EXPORT_ID_MODEL: ["obj"],
	EXPORT_ID_TEXTURE: ["png"],
	EXPORT_ID_SCENE: ["glb", "gltf"],
	EXPORT_ID_WAV: ["wav"]
}

const VALID_EXPORTS := {
	EXPORT_ID_TEXT: ["TXT"],
	EXPORT_ID_MODEL: ["MESH", "SURFACE"],
	EXPORT_ID_TEXTURE: ["BITMAP"],
	EXPORT_ID_SCENE: ["MESH", "SKIN"],#, "SURFACE", "SKIN"]
	EXPORT_ID_WAV: ["SOUND"]
}

const EXPORT_FILE_FILTERS := {
	EXPORT_ID_BIN: ["*.bin ; Binary Files"],
	EXPORT_ID_TEXT: ["*.txt ; Text Files"],
	EXPORT_ID_MODEL: ["*.obj ; Wavefront OBJ"],
	EXPORT_ID_TEXTURE: ["*.png ; PNG Images"],
	EXPORT_ID_SCENE: ["*.glb ; GLTF binary scene", "*.gltf ; GLTF text Scene"],
	EXPORT_ID_WAV: ["*.wav ; WAV sound file"]
}

var EXPORTS_BY_TYPE := {}

func _ready():
	for key in FILE_TYPES:
		EXPORTS_BY_TYPE[key] = [EXPORT_ID_BIN]
	for key in VALID_EXPORTS:
		for tname in VALID_EXPORTS[key]:
			if not tname in EXPORTS_BY_TYPE:
				EXPORTS_BY_TYPE[tname] = [EXPORT_ID_BIN]
			EXPORTS_BY_TYPE[tname].push_back(key)
