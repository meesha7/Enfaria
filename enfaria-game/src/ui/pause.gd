extends Control

func _ready():
    var _x = get_node("Popup/ButtonContainer/ControlButtons/Resume").connect("pressed", self, "_on_resume_pressed")
    var _y = get_node("Popup/ButtonContainer/ControlButtons/Quit").connect("pressed", self, "_on_quit_pressed")


func _on_resume_pressed():
    get_parent().get_parent().hide()


func _on_quit_pressed():
    get_node("/root/connection").leave()
    var _z = get_tree().change_scene("res://src/menu/mainmenu.tscn")
