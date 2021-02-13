extends Control

onready var resume = get_node("Popup/ButtonContainer/ControlButtons/Resume")
onready var quit = get_node("Popup/ButtonContainer/ControlButtons/Quit")
onready var pause = get_node("Popup")

func toggle_pause():
    pause.visible = !pause.visible


func is_paused():
    return pause.visible


func _ready():
    resume.connect("pressed", self, "_on_resume_pressed")
    quit.connect("pressed", self, "_on_quit_pressed")


func _on_resume_pressed():
    pause.visible = false


func _on_quit_pressed():
    get_node("/root/connection").leave()
    get_tree().change_scene("res://src/menu/mainmenu.tscn")
