SELECT 	pr.id, 
		pr.title,
		pr.descr, 
		pr.pictures[1] AS pictures
FROM products AS pr 
	LEFT JOIN promotions AS prom 
		ON prom.product_id = pr.id 
WHERE
	pr.status = 'published' AND (prom.is_pre_prder = true OR prom.in_news = true)
ORDER BY 
    pr.create_datetime