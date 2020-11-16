SELECT DISTINCT ON (chat.id) 
	chat.id AS chat_id,
	CASE 
		WHEN (users_to.id != $1) THEN
	 		users_to.id
	 	WHEN (users_from.id != $1) THEN
	 		users_from.id
	END AS talker_id,
	CASE 
	 WHEN users_to.id != $1 THEN 
	 	users_to.username
	 WHEN users_from.id != $1 THEN
	 	users_from.username
	END AS talker_username,
    CASE 
	    WHEN users_to.id != $1 THEN 
	 	    users_to.picture_path
	    WHEN users_from.id != $1 THEN
	 	    users_from.picture_path
	END AS talker_picture,
	chat.id IN
		(SELECT chat_messages.chat_id
		FROM 	chat_messages 
		WHERE 	chat_messages.chat_id = chat.id AND 
				chat_messages.is_read = false) AS is_read,
	chat.product_id AS pr_id,
	pr.title AS pr_title,
	pr.pictures AS pr_pictures,
	pr.price AS pr_price,
	(SELECT
		chat_messages.send_datetime
		FROM chat_messages WHERE 
			chat_messages.chat_id = chat.id
		ORDER BY chat_messages.send_datetime DESC
		LIMIT 1) AS last_msg_time,
	(SELECT
		chat_messages.msg_data
		FROM chat_messages WHERE 
			chat_messages.chat_id = chat.id
		ORDER BY chat_messages.send_datetime DESC
		LIMIT 1) AS last_msg
	FROM chat AS chat 
	JOIN users AS users_from
    	ON chat.from_user_id = users_from.id 
    JOIN users AS users_to
    	ON users_to.id = chat.to_user_id 
    JOIN products AS pr 
    	ON pr.id = chat.product_id
    LEFT JOIN chat_messages AS cm 
    	ON cm.chat_id = chat.id
    WHERE (chat.to_user_id = $1 OR chat.from_user_id = $1)