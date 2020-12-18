SELECT 	s.id,
		u.username,
		u.register_data,
		false AS in_subs,
		u.picture_path
FROM subscribes AS s 
	JOIN users AS u
		ON u.id = s.to_id
WHERE
	s.from_id = $1
LIMIT $2
OFFSET $3