extends Node

var connection = PacketPeerUDP.new()
var server_ip = "127.0.0.1"
var server_port = 8888

var packet_list = []
var packet_queue = []

onready var common = preload("res://Scripts/enfaria_common.gdns").new()

func _ready():
	connection.connect_to_host(server_ip, server_port)

func _process(delta):
	print("processing")
	var packet = common.to_bytes()
	connection.put_packet(packet)
