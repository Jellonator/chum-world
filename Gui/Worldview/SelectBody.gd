extends PhysicsBody

var node_data = null setget set_node_data, get_node_data

func set_node_data(p_node):
	node_data = p_node

func get_node_data():
	return node_data

func add_shape(shape: Shape, tx: Transform = Transform.IDENTITY):
	var col := CollisionShape.new()
	col.shape = shape
	col.transform = tx
	add_child(col)
