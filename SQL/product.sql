SELECT 
	p.id AS id,
	p.title AS title,
    p.descr AS descr,
    sc.id AS sub_category_id,
	sc.name AS sub_category_name,
    c.id AS category_id,
	c.name AS category_name,
    ps.id AS state_id,
    ps.name AS state_name,
    p.seller_id AS seller_id,
    f.id IS NOT NULL AS in_favourites,
	p.price AS price,
    p.location AS location,
    b2.id AS brand_id,
	b2.name AS brand_name,
    s2.id AS size_id,
    s2.name AS size_name,
    pt.id AS type_id,
	pt.name AS type_name,
    p.pictures AS pictures,
    p.create_datetime AS create_datetime
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
		ON f.product_id = p.id AND $2 IS NOT NULL AND f.user_id = $2 
WHERE
	p.id = $1 AND p.status != 'deleted'