class_name GodotHelloWorld
extends Node3D

signal  on_ready_hello_world_message(text:String)

@export var message_to_dispaly_at_hello_world:String ="Hey mon ami... Tu aimes ca les patates ?"


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	print("Hello World:", message_to_dispaly_at_hello_world )
	on_ready_hello_world_message.emit(message_to_dispaly_at_hello_world)
	
	
