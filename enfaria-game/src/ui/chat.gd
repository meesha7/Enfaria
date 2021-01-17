extends MarginContainer

var message = ""
onready var line = get_node("HBoxContainer/VBoxContainer/Panel/VBoxContainer/LineEdit")
onready var box = get_node("HBoxContainer/VBoxContainer/Panel/VBoxContainer/RichTextLabel")

func hide_chat():
    visible = false
    clear()

func show_chat():
    visible = true
    line.call_deferred("grab_focus")


func is_visible():
    return visible


func clear():
    line.clear()


func add_message(msg):
    box.append_bbcode(msg)
    box.newline()


func send_message(msg):
    get_node("/root/connection").generate_packet(Dictionary({"ChatSend":msg}))


func _input(event):
    if !(event is InputEventKey):
        return
    if !event.is_pressed():
        return

    if event.scancode != KEY_ENTER:
        return

    send_message(line.text)
    clear()
