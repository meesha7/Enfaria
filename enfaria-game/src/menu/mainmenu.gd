extends Control

onready var tree = get_tree()
onready var url = get_node("/root/env").get("DOMAIN")
onready var networking = get_node("/root/env").get("NETWORKING")
onready var path = get_node("/root/constants").config_path
var config = ConfigFile.new()

func _ready():
	load_config()

	var _x = get_node("Container/ButtonContainer/Buttons/Play").connect("pressed", self, "_on_play_pressed")
	var _y = get_node("Container/ButtonContainer/Buttons/Options").connect("pressed", self, "_on_option_pressed")
	var _z = get_node("Container/ButtonContainer/Buttons/Quit").connect("pressed", self, "_on_quit_pressed")
	var _a = get_node("Container/ButtonContainer/Buttons/Login").connect("request_completed", self, "_on_login_completed")
	var _b = get_node("Container/ButtonContainer/Buttons/GetServer").connect("request_completed", self, "_on_getserver_completed")


func _on_play_pressed():
	get_node("Container/FieldContainer/Fields/Error").text = "Connecting..."
	get_node("Container/ButtonContainer/Buttons/GetServer").timeout = 4
	get_node("Container/ButtonContainer/Buttons/GetServer").request(url + "/api/getserver", [], true, HTTPClient.METHOD_GET, "")


func _on_getserver_completed(_result, response_code, _headers, body):
	if response_code == 200:
		var response = body.get_string_from_utf8().replace("\"", "").split(":")
		get_node("/root/connection").server_ip = response[0]
		get_node("/root/connection").server_port = int(response[1])
		
		var username = get_node("Container/FieldContainer/Fields/Username").text
		var password = get_node("Container/FieldContainer/Fields/Password").text
		
		if username == "" or password == "":
			return
		
		var payload = "username" + "=" + username + "&" + "password" + "=" + password
		
		get_node("Container/ButtonContainer/Buttons/Login").timeout = 4
		get_node("Container/ButtonContainer/Buttons/Login").request(url + "/api/login", [], true, HTTPClient.METHOD_POST, payload)
	else:
		get_parent().get_parent().get_node("Container/FieldContainer/Fields/Error").text = "Failed to connect."


func _on_login_completed(_result, response_code, _headers, body):
	if response_code == 200:
		get_node("/root/connection").session_id = body.get_string_from_utf8().replace("\"", "")
		get_node("/root/connection").join()
		tree.change_scene("res://src/game.tscn")
	else:
		get_parent().get_parent().get_node("Container/FieldContainer/Fields/Error").text = "Failed to connect."


func _on_option_pressed():
	tree.change_scene("res://src/menu/options.tscn")


func _on_quit_pressed():
	tree.quit()


func load_config():
	config.load(path)
	
	var fullscreen = false
	var resolution = "1024x720"
	
	if config.has_section_key("Display", "Resolution"):
		resolution = config.get_value("Display", "Fullscreen")
	
	if config.has_section_key("Display", "Fullscreen"):
		fullscreen = config.get_value("Display", "Fullscreen")
		
	OS.window_fullscreen = fullscreen
	if fullscreen:
		OS.set_window_size(OS.get_screen_size())
	else:
		var split = resolution.split("x", false, 1)
		OS.set_window_size(Vector2(split[0], split[1]))
		
	if config.has_section("Controls"):
		for x in config.get_section_keys("Controls"):
			InputMap.action_erase_events(x)
			var event = InputEventKey.new()
			var value = config.get_value("Controls", x)
			var scancode = OS.find_scancode_from_string(value)
			event.set_scancode(scancode)
			InputMap.action_add_event(x, event)
