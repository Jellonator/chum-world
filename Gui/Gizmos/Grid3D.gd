tool
extends Spatial

const NUM := 16

onready var mesh := $MeshInstance

func _ready():
	print(1)
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_LINES)
	st.add_color(Color.red)
	st.set_material(preload("res://Shader/unshaded.tres"))
	st.add_vertex(Vector3(-NUM, 0, 0))
	st.add_vertex(Vector3(NUM, 0, 0))
	st.add_color(Color.blue)
	st.add_vertex(Vector3(0, 0, -NUM))
	st.add_vertex(Vector3(0, 0, NUM))
	st.add_color(Color.green)
	st.add_vertex(Vector3(0, -NUM, 0))
	st.add_vertex(Vector3(0, NUM, 0))
	st.add_color(Color(0.2, 0.2, 0.2))
	for i in range(-NUM, NUM+1):
		if i == 0:
			continue
		# X
		st.add_vertex(Vector3(-NUM, 0, i))
		st.add_vertex(Vector3(NUM, 0, i))
		# Z
		st.add_vertex(Vector3(i, 0, -NUM))
		st.add_vertex(Vector3(i, 0, NUM))
	mesh.mesh = st.commit()
