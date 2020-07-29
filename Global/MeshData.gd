extends Node

var EMPTYNODE_MESH = null

func get_emptynode_mesh():
	if EMPTYNODE_MESH != null:
		return EMPTYNODE_MESH
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_LINES)
	st.add_color(Color.red)
	st.set_material(preload("res://Shader/unshaded.tres"))
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(1, 0, 0))
	st.add_color(Color.blue)
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(0, 0, 1))
	st.add_color(Color.green)
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(0, 1, 0))
	st.add_color(Color(0.2, 0.2, 0.2))
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(-1, 0, 0))
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(0, 0, -1))
	st.add_vertex(Vector3(0, 0, 0))
	st.add_vertex(Vector3(0, -1, 0))
	EMPTYNODE_MESH = st.commit()
	return EMPTYNODE_MESH
