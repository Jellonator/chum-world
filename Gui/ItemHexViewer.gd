extends RichTextLabel

const ChumFile := preload("res://gdchum/ChumFile.gdns")

func set_hex(bytes: PoolByteArray, perline: int):
	var i = 0
	var s = ""
	for byte in bytes:
		if i % perline == 0:
			if i > 0:
				s += "\n"
		else:
			s += " "
		s += "%02X" % byte
		i += 1
	self.text = s

func set_file(file):
	clear()
	var data := file.data as PoolByteArray
	set_hex(data, 16)
