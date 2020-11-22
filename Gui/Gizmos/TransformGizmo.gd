extends Spatial

onready var node_x := $Node/Spatial/X
onready var node_y := $Node/Spatial/Y
onready var node_z := $Node/Spatial/Z
onready var mat_x := node_x.get_node("MeshInstance").get_surface_material(0) as SpatialMaterial
onready var mat_y := node_y.get_node("MeshInstance").get_surface_material(0) as SpatialMaterial
onready var mat_z := node_z.get_node("MeshInstance").get_surface_material(0) as SpatialMaterial
onready var node_base := $Node/Spatial

enum Axis {
	NONE,
	X,
	Y,
	Z,
}

var active_axis: int = Axis.NONE
var is_dragging := false

signal on_change_transform(tx)
signal on_finalize_transform(tx)

func funny_project(plane: Plane, point: Vector3, normal: Vector3) -> Vector3:
	var value := plane.intersects_ray(point, normal)
	if value != null:
		return value
	value = plane.intersects_ray(point, -normal)
	if value != null:
		return value
	MessageOverlay.push_warn("Bad Axis Project")
	return plane.project(point)

func project_onto_axis_global(point: Vector3, normal: Vector3) -> Vector3:
	point = point - translation
#	normal = transform.basis.xform_inv(normal)
	match active_axis:
		Axis.X:
			point = funny_project(Plane.PLANE_XZ, point, normal)
			normal.y = 0
			normal = normal.normalized()
			point = funny_project(Plane.PLANE_XY, point, normal)
		Axis.Y:
			point = funny_project(Plane.PLANE_XY, point, normal)
			normal.z = 0
			normal = normal.normalized()
			point = funny_project(Plane.PLANE_YZ, point, normal)
		Axis.Z:
			point = funny_project(Plane.PLANE_XZ, point, normal)
			normal.y = 0
			normal = normal.normalized()
			point = funny_project(Plane.PLANE_YZ, point, normal)
	point = point + translation
	return point

var mouse_start_position := Vector3.ZERO
var original_position := Vector3.ZERO

func set_active(value: bool):
	set_process(value)
	set_physics_process(value)
	set_process_input(value)
	active_axis = Axis.NONE
	is_dragging = false
	visible = value
	node_base.visible = value

func _process(delta):
	var p1 := get_viewport().get_camera().global_transform.origin
	var p2 = global_transform.origin
	var dis := p1.distance_to(p2)
	var new_scale = Vector3.ONE * dis / 20.0
#	$X.scale = new_scale
#	$Y.scale = new_scale
#	$Z.scale = new_scale
	node_base.scale = new_scale
	node_base.translation = self.translation

func get_mouse_cast_to() -> Vector3:
	var camera = owner.node_viewport.get_camera()
	var mouse_pos = owner.node_rect.get_local_mouse_position() * GlobalConfig.viewport_scale
	var from = camera.project_ray_origin(mouse_pos)
	var to = from - camera.project_ray_normal(mouse_pos)
	return to

func do_cast():
	var camera = owner.node_viewport.get_camera()
	var space_state = camera.get_world().direct_space_state
	var param := PhysicsShapeQueryParameters.new()
	param.collision_mask = 2
	var shape := RayShape.new()
	shape.length = 500
	var mouse_pos = owner.node_rect.get_local_mouse_position() * GlobalConfig.viewport_scale
	var from = camera.project_ray_origin(mouse_pos)
	var to = from - camera.project_ray_normal(mouse_pos)
	param.transform = Transform()\
		.translated(from)\
		.looking_at(to, Vector3.UP)
	param.set_shape(shape)
	var result = space_state.intersect_shape(param, 1)
	if result.size() > 0:
		if result[0]["collider"] == node_x:
			mat_x.albedo_color = Color(1.0, 0.0, 0.0)
			active_axis = Axis.X
		elif result[0]["collider"] == node_y:
			mat_y.albedo_color = Color(0.0, 1.0, 0.0)
			active_axis = Axis.Y
		elif result[0]["collider"] == node_z:
			mat_z.albedo_color = Color(0.0, 0.0, 1.0)
			active_axis = Axis.Z
		owner.can_move_mouse = false
#		mouse_start_position = result[0]["position"]
	else:
		active_axis = Axis.NONE
		owner.can_move_mouse = true
		mat_x.albedo_color = Color(0.4, 0.2, 0.2)
		mat_y.albedo_color = Color(0.2, 0.4, 0.2)
		mat_z.albedo_color = Color(0.2, 0.2, 0.4)

func nearest_point(pt1: Vector3, pt2: Vector3, testPt: Vector3) -> Vector3:
	var d = (pt2 - pt1) / pt1.distance_to(pt2);
	var v = testPt - pt1;
	var t = v.dot(d);
	var P = pt1 + t * d;
	return P
#	return distance(P, testPt);

func _input(event):
	if event.is_action_pressed("view_look") and active_axis != Axis.NONE:
		is_dragging = true
		var camera = owner.node_viewport.get_camera()
		var mouse_pos = owner.node_rect.get_local_mouse_position() * GlobalConfig.viewport_scale
		var from = camera.project_ray_origin(mouse_pos)
		mouse_start_position = project_onto_axis_global(from, camera.project_ray_normal(mouse_pos))
		original_position = translation
	elif event.is_action_released("view_look") and is_dragging:
		is_dragging = false
		emit_signal("on_finalize_transform", transform)
	if is_dragging:
		if event is InputEventMouseMotion:
			var camera = owner.node_viewport.get_camera()
			var mouse_pos = owner.node_rect.get_local_mouse_position() * GlobalConfig.viewport_scale
			var from = camera.project_ray_origin(mouse_pos)
			var new_position := project_onto_axis_global(from, camera.project_ray_normal(mouse_pos))
			new_position -= mouse_start_position
			if Input.is_action_pressed("edit_snap"):
				new_position = new_position.snapped(Vector3.ONE)
			translation = new_position + original_position
			emit_signal("on_change_transform", transform)

func _physics_process(delta):
	if not is_dragging and not Input.is_action_pressed("view_look"):
		do_cast()
