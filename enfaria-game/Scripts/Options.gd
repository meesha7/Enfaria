extends Control

var path = "res://config.cfg"
var config = ConfigFile.new()
var fullscreen = false
var borderless = false
onready var tree = get_tree()
onready var fullscreen_box = get_node("Fullscreen").get_node("FullscreenBox")
onready var borderless_box = get_node("Borderless").get_node("BorderlessBox")

func _ready():
	get_node("ControlButtons").get_node("Back").connect("pressed", self, "_on_back_pressed")
	get_node("ControlButtons").get_node("Save").connect("pressed", self, "_on_save_pressed")
	get_node("Fullscreen").get_node("FullscreenBox").connect("pressed", self, "_on_fullscreen_pressed")
	get_node("Borderless").get_node("BorderlessBox").connect("pressed", self, "_on_borderless_pressed")
	
	config.load(path)
	
	if config.has_section_key("Display", "Fullscreen"):
		fullscreen = config.get_value("Display", "Fullscreen")
		
	if fullscreen:
		get_node("Borderless").get_node("BorderlessBox").disabled = false
		get_node("Fullscreen").get_node("FullscreenBox").pressed = true
		if borderless:
			get_node("Borderless").get_node("BorderlessBox").pressed = true
		
	if config.has_section_key("Display", "Borderless"):
		borderless = config.get_value("Display", "Borderless")

func _on_back_pressed():
	tree.change_scene("Scenes/MainMenu.tscn")

func _on_save_pressed():
	config.set_value("Display", "Fullscreen", fullscreen)
	config.set_value("Display", "Borderless", borderless)
	config.save(path)

func _on_fullscreen_pressed():
	fullscreen = !fullscreen
	if fullscreen:
		borderless_box.disabled = false
	else:
		borderless_box.disabled = true
		borderless_box.pressed = false

func _on_borderless_pressed():
	borderless = !borderless
