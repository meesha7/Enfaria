extends VBoxContainer

func _ready():
	get_node("Resume").connect("pressed", self, "_on_resume_pressed")
	get_node("Quit").connect("pressed", self, "_on_quit_pressed")
	
func _on_resume_pressed():
	get_parent().get_parent().hide()

func _on_quit_pressed():
	get_tree().change_scene("res://src/menu/mainmenu.tscn")
