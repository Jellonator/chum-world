extends Spatial

onready var original_transform := global_transform
onready var node_camera := $PivotY/PivotX/Camera
onready var node_pivotx := $PivotY/PivotX
onready var node_pivoty := $PivotY

const MAX_PITCH := PI * 0.475
const SENSITIVITY := 0.0075

var pitch := 0.0
var yaw := 0.0

func reset_transform():
	global_transform = original_transform
	node_pivotx.transform = Transform()
	node_pivoty.transform = Transform()
	pitch = 0
	yaw = 0

func move_mouse(amount: Vector2):
	node_pivotx.rotate_x(-amount.y * SENSITIVITY)
	node_pivoty.rotate_y(-amount.x * SENSITIVITY)
	node_pivotx.rotation.x = clamp(node_pivotx.rotation.x, -MAX_PITCH, MAX_PITCH)

func get_camera_transform() -> Transform:
	return node_camera.global_transform

func move_strafe(dir: Vector3):
	transform.origin += dir

func move_to(position: Vector3):
	transform.origin = position

func move_look(transform: Transform, distance: float):
	move_to(transform.origin + Vector3(0, 1, 1).normalized() * distance)
	node_pivotx.rotation_degrees.x = -45
	node_pivoty.rotation.y = 0
