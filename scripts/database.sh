#!/bin/bash

mysql -u root -p -e "CREATE USER 'enfaria'@'localhost' IDENTIFIED BY 'enfaria'"
mysql -u enfaria -penfaria -e "CREATE DATABASE enfaria CHARACTER SET = 'utf8mb4' COLLATE 'utf8mb4_general_ci"
mysql -u enfaria -penfaria -e "GRANT ALL PRIVILEGES ON enfaria.* TO 'enfaria'@'localhost'"
mysql -u enfaria -penfaria enfaria < schema.sql
