extends Node

var connection = PacketPeerUDP.new()
var server_ip = "127.0.0.1"
var server_port = 8888

var session_id = ""

var packet_list = []
var packet_queue = []

onready var packet = preload("res://src/native/enfaria_common.gdns")

func _ready():
	connection.connect_to_host(server_ip, server_port)
