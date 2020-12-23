extends Control

var path = "res://config.cfg"
var config = ConfigFile.new()
var fullscreen = false

func _ready():
	var resolution = get_node("Resolution/ResolutionButton")
	resolution.add_item("1024x720")
	resolution.add_item("1280x1024")
	resolution.add_item("1920x1080")
	
	get_node("Resolution/ResolutionButton").connect("item_selected", self, "_on_resolution_selected")
	get_node("ControlButtons/Back").connect("pressed", self, "_on_back_pressed")
	get_node("ControlButtons/Save").connect("pressed", self, "_on_save_pressed")
	get_node("Fullscreen/FullscreenBox").connect("pressed", self, "_on_fullscreen_pressed")
	
	var fullscreen_box = get_node("Fullscreen/FullscreenBox")
	
	config.load(path)
	
	if config.has_section_key("Display", "Resolution"):
		resolution = config.get_value("Display", "Fullscreen")
	
	if config.has_section_key("Display", "Fullscreen"):
		fullscreen = config.get_value("Display", "Fullscreen")
		
	if fullscreen:
		fullscreen_box.pressed = true

func _on_back_pressed():
	get_tree().change_scene("Scenes/MainMenu.tscn")

func _on_save_pressed():
	var resolution_button = get_node("Resolution/ResolutionButton")
	var resolution = resolution_button.get_item_text(resolution_button.get_selected_id())
	
	config.set_value("Display", "Fullscreen", fullscreen)
	config.set_value("Display", "Resolution", resolution)
	config.save(path)
	
	OS.window_fullscreen = fullscreen
	if fullscreen:
		OS.set_window_size(OS.get_screen_size())
	else:
		var split = resolution.split("x", false, 1)
		OS.set_window_size(Vector2(split[0], split[1]))
	

func _on_fullscreen_pressed():
	fullscreen = !fullscreen

