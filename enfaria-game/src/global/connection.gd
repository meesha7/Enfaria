extends Node

var connection = PacketPeerUDP.new()
var connected = false

var server_ip = "127.0.0.1"
var server_port = 8888
var session_id = ""

var beat = 0
var last_timestamp = 0
var send_queue = []
var receive_queue = []

onready var packet = preload("res://src/native/enfaria_common.gdns")


func _process(_delta):
    if !connected:
        return

    var now = OS.get_ticks_msec()
    if now > last_timestamp + 10000:
        print("Timed out, leaving.")
        leave()
        var _x = get_tree().change_scene("res://src/menu/mainmenu.tscn")
        return

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
        if p.get_command() != Dictionary({"Ping":[]}):
            receive_queue.append(p)

    if len(receive_queue) > 10000:
        print("Packet overflow!")
        receive_queue.clear()


func send_packets():
    for p in send_queue:
        connection.put_packet(p.to_bytes())

    send_queue.clear()


func join():
    var result = connection.connect_to_host(server_ip, server_port)
    if result != OK:
        return false

    last_timestamp = OS.get_ticks_msec()
    var p = packet.new()
    p.set_destination(server_ip + ":" + str(server_port))
    p.session_id = session_id
    p.set_command(Dictionary({"Connect": []}))

    result = connection.put_packet(p.to_bytes())
    if result != OK:
        return false

    connected = true
    return true


func leave():
    var p = packet.new()
    p.session_id = session_id
    p.set_destination(server_ip + ":" + str(server_port))
    p.set_command(Dictionary({"Quit": []}))
    connection.put_packet(p.to_bytes())
    connection.close()
    connected = false


func generate_packet(data):
    var p = packet.new()
    p.session_id = session_id
    p.beat = beat
    beat += 1

    p.set_destination(server_ip + ":" + str(server_port))
    p.set_command(data)

    send_queue.append(p)


func c_ping():
    generate_packet(Dictionary({"Ping":[]}))


func c_move(pos):
    generate_packet(Dictionary({"Move": Dictionary({"x": int(pos.x), "y": int(pos.y), "z": int(pos.z)})}))


func c_move_item(from, to):
    generate_packet(Dictionary({"MoveItem":[from, to]}))
