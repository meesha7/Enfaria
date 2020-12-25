extends Control

onready var tree = get_tree()
onready var url = get_node("/root/env").get("DOMAIN")
onready var networking = get_node("/root/env").get("NETWORKING")

func _ready():
	if networking == "true":
		get_node("Play").connect("pressed", self, "_on_play_pressed")
	else:
		get_node("Play").connect("pressed", self, "start")
	get_node("Options").connect("pressed", self, "_on_option_pressed")
	get_node("Quit").connect("pressed", self, "_on_quit_pressed")
	$HTTPRequest.connect("request_completed", self, "_on_request_completed")


func _on_play_pressed():
	var username = get_parent().get_parent().get_node("FieldContainer/Fields/Username").text
	var password = get_parent().get_parent().get_node("FieldContainer/Fields/Password").text
	var payload = "username" + "=" + username + "&" + "password" + "=" + password
	$HTTPRequest.request(url + "/api/login", [], true, HTTPClient.METHOD_POST, payload)
	
func _on_request_completed(_result, response_code, _headers, body):
	if response_code == 200:
		get_node("/root/connection").session_id = body
		start()
	else:
		get_parent().get_parent().get_node("FieldContainer/Fields/Error").text = "Failed to connect."

func start():
	tree.change_scene("res://src/game.tscn")

func _on_option_pressed():
	tree.change_scene("res://src/menu/options.tscn")

func _on_quit_pressed():
	tree.quit()
