# Enfaria

Enfaria is a work-in-progress multiplayer game inspired by Stardew Valley. It is currently in a very early phase of development and as such, any information put here would quickly become obsolete.

### Running the game

For now, there is no script to run everything.

You will need `enfaria-web` and `enfaria-server` running at the same time. Then you can run the game by opening the project in Godot and pressing F5.

## Contributing

### Native files

Run `native.sh` or open it and run the command from inside of the file.

Alternatively, compile `enfaria-common` and put the resulting .dll/.so file in `enfaria-game/src/native`.

## DB

You need a MariaDB/MySQL instance running with credentials described at `.env`

You'll have to create the user and database and make sure the names match.

Use `schema.sql` for the current schema to create your tables.

There will be a setup script in the future.
