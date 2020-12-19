CREATE TABLE `users` (
	`id` INT(11) NOT NULL AUTO_INCREMENT,
	`email` VARCHAR(100) NOT NULL COLLATE 'utf8_general_ci',
	`username` VARCHAR(50) NOT NULL COLLATE 'utf8_general_ci',
	`password` BINARY(60) NOT NULL,
	PRIMARY KEY (`id`) USING BTREE
)
COLLATE='utf8_general_ci'
ENGINE=InnoDB
;

CREATE TABLE `sessions` (
	`user_id` INT(11) NOT NULL,
	`secret` VARCHAR(300) NOT NULL COLLATE 'utf8_general_ci',
	`expiry_date` DATETIME NOT NULL,
	PRIMARY KEY (`user_id`) USING BTREE
)
COLLATE='utf8_general_ci'
ENGINE=InnoDB
;
