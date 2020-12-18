SELECT 	s.id,
		u.username,
		u.register_data,
		EXISTS (
			SELECT * 
			FROM subscribes AS sub
			WHERE sub.to_id = u2.id 
				AND sub.from_id = u.id ) AS in_subs,
		u.picture_path
FROM subscribes AS s 
	JOIN users AS u
		ON u.id = s.to_id
	JOIN users AS u2 
		ON u2.id = s.from_id
WHERE
	s.to_id = $1
LIMIT $2
OFFSET $3