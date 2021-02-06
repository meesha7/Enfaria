# Enfaria

Enfaria is a work-in-progress multiplayer game inspired by Stardew Valley. It is currently in a very early phase of development and as such, any information put here would quickly become obsolete.

### Running the project

Run `scripts/release.sh`.
Run `release/web/enfaria-web(.exe)`
Run `release/server/enfaria-server(.exe)`
Open the game directly (`release/enfaria-game(.exe)`) or via updater (`release/enfaria-updater.exe`)

## Contributing

### Native files

Run `scripts/native.sh` or open it and run the command from inside of the file.

Alternatively, compile `enfaria-common` and put the resulting .dll/.so file in `enfaria-game/src/native`.

## DB

You need a MariaDB/MySQL database.

Set it up by running `scripts/database.sh`.
