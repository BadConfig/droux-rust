 SELECT 
 	rating.id,
 	rating.voter_id,
    u.picture_path AS voter_photo,
 	u.username AS voter_username,
 	rating.seller_id,
 	rating.stars,
 	rating.comment,
 	rating.feedback_type,
 	rating.create_datetime
 FROM 
 	rating JOIN users AS u
 		ON u.id = rating.voter_id
WHERE rating.seller_id = $1;