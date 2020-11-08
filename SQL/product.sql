SELECT 
	p.id,
	p.title,
    p.descr,
    sc.id,
	sc.name,
    c.id,
	c.name,
    ps.id,
    ps.name,
    p.seller_id,
    f.id != NULL,
	p.price,
    p.location,
    b2.id,
	b2.name,
    s2.id,
    s2.name,
    pt.id,
	pt.name,
    p.pictures,
    p.create_datetime
FROM 
	products AS p 
	LEFT JOIN promotions AS p2 
		ON p.id = p2.product_id
	JOIN brands AS b2
		ON b2.id = p.brand_id
	JOIN sub_categories AS sc 
		ON sc.id = p.sub_category_id
	JOIN categories AS c 
		ON c.id = sc.category_id
	JOIN sizes AS s2 
		ON s2.id = p.size_id
	JOIN product_state AS ps
		ON ps.id = p.product_state
	JOIN product_type AS pt 
		ON pt.id = p.type_id
	JOIN users AS u 
		ON u.id = p.seller_id
	LEFT JOIN favourites AS f 
		ON f.user_id = $2 AND f.product_id = p.id
	LEFT JOIN deleted_posts AS dp 
		ON dp.post_id = p.id
WHERE
	p.id = $1 AND dp.id = NULL