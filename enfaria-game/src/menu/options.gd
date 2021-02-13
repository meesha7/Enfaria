extends Control

onready var fullscreen_box = get_node("Fullscreen/FullscreenBox")
onready var resolution = get_node("Resolution/ResolutionButton")
onready var controls = get_node("Controls/ControlsButton")
onready var back = get_node("ControlButtons/Back")
onready var save = get_node("ControlButtons/Save")

onready var path = get_node("/root/constants").config_path
var config = ConfigFile.new()
var fullscreen = false

func _ready():
    resolution.add_item("1024x720")
    resolution.add_item("1280x1024")
    resolution.add_item("1920x1080")

    resolution.connect("item_selected", self, "_on_resolution_selected")
    controls.connect("pressed", self, "_on_controls_pressed")
    back.connect("pressed", self, "_on_back_pressed")
    save.connect("pressed", self, "_on_save_pressed")
    fullscreen_box.connect("pressed", self, "_on_fullscreen_pressed")

    config.load(path)

    if config.has_section_key("Display", "Resolution"):
        resolution = config.get_value("Display", "Fullscreen")

    if config.has_section_key("Display", "Fullscreen"):
        fullscreen.pressed = config.get_value("Display", "Fullscreen")


func _on_controls_pressed():
    get_tree().change_scene("res://src/menu/controls.tscn")


func _on_back_pressed():
    get_tree().change_scene("res://src/menu/mainmenu.tscn")


func _on_save_pressed():
    var res = resolution.get_item_text(resolution.get_selected_id())

    config.set_value("Display", "Fullscreen", fullscreen)
    config.set_value("Display", "Resolution", res)
    config.save(path)

    OS.window_fullscreen = fullscreen
    if fullscreen:
        OS.set_window_size(OS.get_screen_size())
    else:
        var split = resolution.split("x", false, 1)
        OS.set_window_size(Vector2(split[0], split[1]))


func _on_fullscreen_pressed():
    fullscreen = !fullscreen

