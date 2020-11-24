-- $1 - a string to search in title
-- $2 - size table id
-- $3 - product state id
-- $4 - limit
-- $5 - offset
-- $6 - subcategory id
-- $7 - category id
-- $8 - brand id
-- $9 - type id
-- $10 - order by: one of 'Date' 'PriceDESC'
-- 'PriceASC' 'Views' 'none'
-- $12 user_id 
--
SELECT 	pr.id, 
		pr.title,
		pr.price, 
		fv.id IS NOT NULL AS is_in_favourites,
		c2.name AS category_name,
		sz.name AS size_name,
		pr.pictures,
		u2.id AS seller_id,
		u2.picture_path AS profile_picture,
		u2.username AS seller_uname,
		u2.rate_summ / 
			CASE 
				WHEN (u2.rate_count = 0) THEN 1
				ELSE u2.rate_count 
			END
			AS seller_rating
FROM products AS pr 
	LEFT JOIN promotions AS prom 
		ON prom.product_id = pr.id 
	JOIN sub_categories AS sc 
		ON sc.id = pr.sub_category_id 
	JOIN categories AS c2 
		ON c2.id = sc.category_id
	LEFT JOIN favourites AS fv 
		ON fv.product_id = pr.id AND fv.user_id = $11
	JOIN sizes AS sz 
		ON sz.id = pr.size_id
	JOIN users AS u2 
    	ON u2.id = pr.seller_id 
WHERE
	pr.status = 'published' AND
	(CASE WHEN ($1 IS NOT NULL) THEN POSITION($1 IN pr.title) != 0 		END OR
	CASE WHEN ($7 IS NOT NULL) THEN sc.category_id 		= $7 			END OR
	CASE WHEN ($6 IS NOT NULL) THEN pr.sub_category_id 	= $6			END OR
	CASE WHEN ($2 IS NOT NULL) THEN pr.size_id 			= $2 			END OR
	CASE WHEN ($3 IS NOT NULL) THEN pr.product_state 	= $6 			END OR
	CASE WHEN ($8 IS NOT NULL) THEN pr.brand_id 		= $8 			END OR
	CASE WHEN ($9 IS NOT NULL) THEN pr.type_id 			= $9 			END)
ORDER BY 
	CASE WHEN ($10 = 'Date')		THEN CAST(EXTRACT(epoch FROM pr.create_datetime) AS INTEGER) END ASC,
	CASE WHEN ($10 = 'PriceDESC') 	THEN pr.price END DESC,
	CASE WHEN ($10 = 'PriceASC') 	THEN pr.price END ASC,
	CASE WHEN ($10 = 'Views')		THEN pr.total_views END ASC,
	CASE 
	 		WHEN ($6 IS NOT NULL)	THEN prom.top_by_cat::INT
	 		WHEN ($7 IS NOT NULL)	THEN prom.top_by_cat::INT
	 		WHEN ($1 IS NOT NULL)	THEN prom.top_by_name::INT
	END
	NULLS LAST
LIMIT $4
OFFSET $5;
