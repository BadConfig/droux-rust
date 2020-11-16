SELECT 
 	pr.id AS id,
 	pr.title AS title,
 	pr.pictures AS pictures,
 	u.id AS seller_id,
 	u.username AS seller_username,
 	u.picture_path AS seller_picture,
 	pr.status = 'published' AS is_published
FROM products AS pr
	JOIN users AS u
		ON pr.seller_id  = u.id
ORDER BY pr.create_datetime DESC
OFFSET $1
LIMIT $2
