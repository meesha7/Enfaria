extends VBoxContainer

onready var path = get_node("/root/constants").config_path
var config = ConfigFile.new()
var actions = ["Up", "Down", "Left", "Right"]

var listening = false

func _ready():
	get_node("ControlButtons/Back").connect("pressed", self, "_on_back_pressed")
	get_node("ControlButtons/Save").connect("pressed", self, "_on_save_pressed")
	for x in get_children():
		if !(x.name in actions):
			continue
		x.get_node(x.name + "Button").connect("toggled", self, "_on_control_pressed")
		
	config.load(path)
	
	if config.has_section("Controls"):
		for x in config.get_section_keys("Controls"):
			for y in get_children():
				if x == y.name:
					y.get_node(y.name + "Button").text = config.get_value("Controls", x)
		
func _input(event):
	if !listening:
		return
	if !(event is InputEventKey):
		return
		
	for x in get_children():
		if !(x.name in actions):
			continue
		
		var button = x.get_node(x.name + "Button")
		
		if !button.pressed:
			continue
		
		if event.as_text() != "Escape":
			button.text = event.as_text()

		toggle_rest()

		button.pressed = false
		listening = false
		
		break
	
func _on_back_pressed():
	get_tree().change_scene("res://src/menu/options.tscn")

func _on_save_pressed():
	for x in get_children():
		if !(x.name in actions):
			continue
			
		var value = x.get_node(x.name + "Button").text
		config.set_value("Controls", x.name, value)
	
	config.save(path)

func _on_control_pressed(pressed):
	if listening:
		return
	listening = true
	toggle_rest()

func toggle_rest():
	for x in get_children():
		if !(x.name in actions):
			continue
		var button = x.get_node(x.name + "Button")
		if !button.pressed:
			button.disabled = !button.disabled
