-- Your SQL goes here
create or replace function filters (
    arg_search_string varchar,
    arg_size_ids int[],
    arg_product_state_ids int[],
    arg_limit_num int,
    arg_offset_num int,
    arg_subcategory_ids int[],
    arg_category_ids int[],
    arg_brand_ids int[],
    arg_type_ids int[],
    arg_order_by varchar,
    arg_user_id int
)
	returns table (
		id int,
		title varchar,
		price int,
		is_in_favourites bool,
		category_name varchar,
        size_name varchar,
        pictures text[],
        seller_id int,
        profile_picture varchar,
        seller_uname varchar,
        seller_rating bigint
	)
language plpgsql
as $$
begin
	return query
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
                ON fv.product_id = pr.id AND fv.user_id = arg_user_id
            JOIN sizes AS sz
                ON sz.id = pr.size_id
            JOIN users AS u2
                ON u2.id = pr.seller_id
        WHERE
            pr.status = 'published' AND
            (CASE
                WHEN (arg_search_string IS NOT NULL) THEN POSITION(UPPER(arg_search_string) IN UPPER(pr.title)) != 0
                ELSE TRUE
            END AND
            CASE
                WHEN (array_length(arg_category_ids, 1) > 0) THEN sc.category_id = ANY (arg_category_ids)
                ELSE TRUE
            END AND
            CASE
                WHEN (array_length(arg_subcategory_ids, 1) > 0) THEN pr.sub_category_id = ANY (arg_subcategory_ids)
                ELSE TRUE
            END AND
            CASE
                WHEN (array_length(arg_size_ids, 1) > 0) THEN pr.size_id = ANY (arg_size_ids)
                ELSE TRUE
            END AND
            CASE
                WHEN (array_length(arg_product_state_ids, 1) > 0) THEN pr.product_state = ANY (arg_product_state_ids)
                ELSE TRUE
            END AND
            CASE
                WHEN (array_length(arg_brand_ids, 1) > 0) THEN pr.brand_id = ANY (arg_brand_ids)
                ELSE TRUE
            END AND
            CASE
                WHEN (array_length(arg_type_ids, 1) > 0) THEN pr.type_id = ANY (arg_type_ids)
                ELSE TRUE
            END)
        ORDER BY
            CASE WHEN (arg_order_by = 'Date')		THEN CAST(EXTRACT(epoch FROM pr.create_datetime) AS INTEGER) END ASC,
            CASE WHEN (arg_order_by = 'PriceDESC') 	THEN pr.price END DESC,
            CASE WHEN (arg_order_by = 'PriceASC') 	THEN pr.price END ASC,
            CASE WHEN (arg_order_by = 'Views')		THEN pr.total_views END ASC,
            CASE
                    WHEN (array_length(arg_subcategory_ids, 1) > 0)	THEN prom.top_by_cat::INT
                    WHEN array_length(arg_category_ids, 1) > 0 THEN prom.top_by_cat::INT
                    WHEN (arg_search_string IS NOT NULL)	THEN prom.top_by_name::INT
            END
            NULLS LAST
        LIMIT arg_limit_num
        OFFSET arg_offset_num;
end;$$;
