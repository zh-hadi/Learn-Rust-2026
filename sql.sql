CREATE TABLE `todo` (
	`id` int AUTO_INCREMENT,
    `data` text,
    
    PRIMARY KEY (`id`)
);

INSERT INTO `todo`( `data`) VALUES ('today start rust hard-way'), ('today is bad day');

SELECT id, data FROM `todo`;
DELETE FROM todo WHERE id = 5;
UPDATE todo SET data = "today is good day" WHERE id = 2;