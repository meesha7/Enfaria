extends Control

onready var tree = get_tree()

func _ready():
	get_node("Play").connect("pressed", self, "_on_play_pressed")
	get_node("Options").connect("pressed", self, "_on_option_pressed")
	get_node("Quit").connect("pressed", self, "_on_quit_pressed")

func _on_start_pressed():
	tree.change_scene("Scenes/Join.tscn")

func _on_option_pressed():
	tree.change_scene("Scenes/Options.tscn")

func _on_quit_pressed():
	tree.quit()
