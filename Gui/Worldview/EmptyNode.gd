extends Spatial

func _ready():
	$MeshInstance.mesh = MeshData.get_emptynode_mesh()
