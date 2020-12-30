extends Node

var env = {};

func _ready():
    env = parse("res://.env");
    
func get(name):
    if(OS.has_environment(name)):
        return OS.get_environment(name);
        
    if(env.has(name)):
        return env[name];
    return "";

func parse(filename):
    var file = File.new()
    if(!file.file_exists(filename)):
        return {};

    file.open(filename, File.READ)

    var variables = {};
    var line = "";

    while !file.eof_reached():
        line = file.get_line();
        if line == "":
            continue
        var split = line.split("=");
        variables[split[0]] = split[1].lstrip("\"").rstrip("\"");
    return variables;
