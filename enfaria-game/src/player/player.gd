extends KinematicBody2D

const WALK_SPEED = 200

var velocity = Vector2()

func _physics_process(delta):
	if Input.is_key_pressed(KEY_A):
		velocity.x = -WALK_SPEED
	elif Input.is_key_pressed(KEY_D):
		velocity.x =  WALK_SPEED
	else:
		velocity.x = 0
		
	if Input.is_key_pressed(KEY_W):
		velocity.y = -WALK_SPEED
	elif Input.is_key_pressed(KEY_S):
		velocity.y = WALK_SPEED
	else:
		velocity.y = 0

	move_and_slide(velocity)
