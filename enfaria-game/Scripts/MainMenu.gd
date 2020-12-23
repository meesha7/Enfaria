extends Control

onready var tree = get_tree()
onready var url = get_node("/root/Env").get("DOMAIN")

func _ready():
	get_node("Play").connect("pressed", self, "_on_play_pressed")
	get_node("Options").connect("pressed", self, "_on_option_pressed")
	get_node("Quit").connect("pressed", self, "_on_quit_pressed")
	$HTTPRequest.connect("request_completed", self, "_on_request_completed")

func _on_play_pressed():
	var username = get_parent().get_parent().get_node("FieldContainer/Fields/Username").text
	var password = get_parent().get_parent().get_node("FieldContainer/Fields/Password").text
	var payload = "username" + "=" + username + "&" + "password" + "=" + password
	$HTTPRequest.request(url + "/api/login", [], true, HTTPClient.METHOD_POST, payload)
	
func _on_request_completed(result, response_code, headers, body):
	if response_code == 200:
		tree.change_scene("Scenes/Player.tscn")
		get_node("/root/Connection").session_id = body
	else:
		get_parent().get_parent().get_node("FieldContainer/Fields/Error").text = "Failed to connect."

func _on_option_pressed():
	tree.change_scene("Scenes/Options.tscn")

func _on_quit_pressed():
	tree.quit()
