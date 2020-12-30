extends Node

var connection = PacketPeerUDP.new()
var connected = false

var server_ip = "127.0.0.1"
var server_port = 8888
var session_id = ""

var last_timestamp = 0
var send_queue = []
var receive_queue = []

onready var packet = preload("res://src/native/enfaria_common.gdns")

func generate_packet(data):
	var p = packet.new()
	p.set_destination(server_ip + ":" + str(server_port))
	p.set_session_id(session_id)
	p.set_command(data)
	send_queue.append(p)


func _process(_delta):
	if !connected:
		return
	
	var now = OS.get_ticks_msec()
	if now > last_timestamp + 10000:
		leave()
		var _x = get_tree().change_scene("res://src/menu/mainmenu.tscn")

	receive_packets()
	send_packets()


func receive_packets():
	var received = connection.get_available_packet_count()
	if received == 0:
		return
		
	for _x in range(received):
		var raw = Array(connection.get_packet())
		var p = packet.new()
		p.from_bytes(raw)
		last_timestamp = OS.get_ticks_msec()
		if p.get_command() == "ping":
			p.queue_free()
		else:
			receive_queue.append(p)
		
	if len(receive_queue) > 10000:
		print("Packet overflow!")
		receive_queue.clear()


func send_packets():
	for p in send_queue:
		connection.put_packet(p.to_bytes())
		p.queue_free()
		
	send_queue.clear()


func join():
	connection.connect_to_host(server_ip, server_port)
	var p = packet.new()
	p.set_destination(server_ip + ":" + str(server_port))
	p.set_session_id(session_id)
	p.set_command("connect")
	connection.put_packet(p.to_bytes())
	p.queue_free()
	connected = true


func leave():
	var p = packet.new()
	p.set_destination(server_ip + ":" + str(server_port))
	p.set_session_id(session_id)
	p.set_command("quit")
	connection.put_packet(p.to_bytes())
	p.queue_free()
	connection.close()
	connected = false
