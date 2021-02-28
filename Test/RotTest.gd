extends Spatial

func printtx(tx1):
	print(tx1.basis.x)
	print(tx1.basis.y)
	print(tx1.basis.z)
	print(tx1.origin)
	print(".")

func _ready():
	print(get_path())
	var sp := $Spatial as Spatial
	

	sp.rotation = Quat(0.763008, 0.00000, 0.00000, 0.645989).get_euler()
	print(sp.rotation_degrees)
	#var parent_tx = self.global_transform
	#var child_tx = $Spatial.global_transform
	#printtx(parent_tx)
	#printtx(child_tx)
	#printtx($Spatial.transform)
	#var child_local_tx = parent_tx.affine_inverse() * child_tx
	#printtx(child_local_tx)
#	print(self.rotation)
#	print(self.rotation_degrees)
#	print(self.transform.basis.get_rotation_quat())
#	print(self.transform.affine_inverse())
#	print($Spatial.transform.affine_inverse())
#	var parent_tx = Transform(
#		Vector3(0.9220588,    0.3869429,  0.001894809),
#		Vector3(-0.3445713,    0.8188990,    0.4587930),
#		Vector3(0.1760654,   -0.4236692,    0.8884774),
#		Vector3(  0.000,      34.68,      13.84))
#	var child_tx = Transform(
#		Vector3(1.243769,   -0.4665298,    0.2403003),
#		Vector3( 0.5247802,     1.105809 ,  -0.5691731),
#		Vector3(-1.879893e-05,    0.6178206,     1.200247),
#		Vector3( -19.43370 ,   -41.38246,    -17.75902))
#	var tx1 = parent_tx * child_tx
#	var tx2 = child_tx * parent_tx
#	print(tx1.basis.x)
#	print(tx1.basis.y)
#	print(tx1.basis.z)
#	print(tx1.origin)
#	print(".")
#	print(tx2.basis.x)
#	print(tx2.basis.y)
#	print(tx2.basis.z)
#	print(tx2.origin)
#	print(".")
