extends Control

onready var tree = get_tree()
onready var url = get_node("/root/env").get("DOMAIN")
onready var path = get_node("/root/constants").config_path
var config = ConfigFile.new()

onready var play = get_node("Container/ButtonContainer/Buttons/Play")
onready var options = get_node("Container/ButtonContainer/Buttons/Options")
onready var quit = get_node("Container/ButtonContainer/Buttons/Quit")
onready var login = get_node("Container/ButtonContainer/Buttons/Login")
onready var username = get_node("Container/FieldContainer/Fields/Username")
onready var password = get_node("Container/FieldContainer/Fields/Password")
onready var error = get_node("Container/FieldContainer/Fields/Error")
onready var getserver = get_node("Container/ButtonContainer/Buttons/GetServer")

func _ready():
    load_config()

    play.connect("pressed", self, "_on_play_pressed")
    options.connect("pressed", self, "_on_option_pressed")
    quit.connect("pressed", self, "_on_quit_pressed")
    login.connect("request_completed", self, "_on_login_completed")
    getserver.connect("request_completed", self, "_on_getserver_completed")


func _on_play_pressed():
    error.text = "Connecting..."
    getserver.timeout = 4
    getserver.request(url + "/api/server", [], true, HTTPClient.METHOD_GET, "")


func _on_getserver_completed(_result, response_code, _headers, body):
    if response_code == 200:
        var response = body.get_string_from_utf8().replace("\"", "").split(":")
        get_node("/root/connection").server_ip = response[0]
        get_node("/root/connection").server_port = int(response[1])

        if username.text == "" or password.text == "":
            return

        var payload = "username" + "=" + username.text + "&" + "password" + "=" + password.text

        login.timeout = 4
        login.request(url + "/api/login", [], true, HTTPClient.METHOD_POST, payload)
    else:
        error.text = "Server is down."


func _on_login_completed(_result, response_code, _headers, body):
    if response_code == 200:
        get_node("/root/connection").session_id = body.get_string_from_utf8().replace("\"", "")
        if get_node("/root/connection").join():
            tree.change_scene("res://src/game.tscn")
    error.text = "Login failed."


func _on_option_pressed():
    tree.change_scene("res://src/menu/options.tscn")


func _on_quit_pressed():
    tree.quit()


func load_config():
    config.load(path)

    var fullscreen = false
    var resolution = "1024x720"

    resolution = config.get_value("Display", "Resolution", "1024x768")
    fullscreen = config.get_value("Display", "Fullscreen", false)

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
