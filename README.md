# Enfaria
You need a MariaDB/MySql instance running with credentials described at `.env`


## Native files

Run `native.sh` or open it and run the command from inside of the file (Windows users mostly).


## DB

Create a user for the local DB:

`CREATE USER 'enfaria'@'localhost' IDENTIFIED BY 'enfaria';`

Create a new DB:

`CREATE DATABASE enfaria CHARACTER SET = 'utf8mb4' COLLATE 'utf8mb4_general_ci';`

Give privileges to the new DB for the new user:

`GRANT ALL PRIVILEGES ON enfaria.* TO 'enfaria'@'localhost';`

Create DB structure:

`mysql -u enfaria -p'enfaria' enfaria < schema.sql`
