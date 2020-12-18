SELECT 	s.id,
		u.username,
		u.register_data,
		EXISTS (
			SELECT * 
			FROM subscribes AS sub
			WHERE sub.from_id = u2.id 
				AND sub.to_id = u.id ) AS in_subs,
		u.picture_path
FROM subscribes AS s 
	JOIN users AS u
		ON u.id = s.from_id
	JOIN users AS u2 
		ON u2.id = s.to_id
WHERE
	s.to_id = $1
LIMIT $2
OFFSET $3