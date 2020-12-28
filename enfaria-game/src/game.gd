extends Node2D

func _ready():
	pass

func _input(event):
	if !(event is InputEventKey):
		return
	if !event.is_pressed():
		return
	if event.scancode != KEY_ESCAPE:
		return
	var popup = get_node("Pause/Popup")
	if popup.visible:
		popup.hide()
	else:
		popup.show()
