# Enfaria

Enfaria is a work-in-progress multiplayer game inspired by Stardew Valley. It is currently in a very early phase of development and as such, any information put here would quickly become obsolete.

### Running the game

For now, there is no script to set up and run everything.

You will need `enfaria-web` and `enfaria-server` running at the same time. Then you can run the game by opening the project in Godot and pressing F5.

## Contributing

### Native files

Run `scripts/native.sh` or open it and run the command from inside of the file.

Alternatively, compile `enfaria-common` and put the resulting .dll/.so file in `enfaria-game/src/native`.

## DB

You need a MariaDB/MySQL database.

Set it up by running `scripts/database.sh`.
