table! {
    activation_links (id) {
        id -> Int4,
        user_id -> Int4,
        reference -> Varchar,
    }
}

table! {
    brands (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    chat (id) {
        id -> Int4,
        from_user_id -> Int4,
        to_user_id -> Int4,
        product_id -> Int4,
    }
}

table! {
    chat_messages (id) {
        id -> Int4,
        chat_id -> Int4,
        from_user_id -> Int4,
        to_user_id -> Int4,
        msg_data -> Text,
        is_read -> Bool,
        send_datetime -> Timestamp,
    }
}

table! {
    favourites (id) {
        id -> Int4,
        user_id -> Int4,
        product_id -> Int4,
    }
}

table! {
    priveleges (id) {
        id -> Int4,
        user_id -> Int4,
        privelege_type -> Varchar,
    }
}

table! {
    products (id) {
        id -> Int4,
        sub_category_id -> Int4,
        title -> Varchar,
        descr -> Text,
        price -> Int4,
        location -> Varchar,
        state -> Varchar,
        brand_id -> Int4,
        seller_id -> Int4,
        pictures -> Array<Text>,
        create_datetime -> Timestamp,
    }
}

table! {
    promotions (id) {
        id -> Int4,
        product_id -> Int4,
        is_marked -> Bool,
        top_by_cat -> Bool,
        top_by_name -> Bool,
        is_pre_prder -> Bool,
        prod_bought_date -> Timestamp,
    }
}

table! {
    rating (id) {
        id -> Int4,
        voter_id -> Int4,
        seller_id -> Int4,
        stars -> Int2,
    }
}

table! {
    sub_categories (id) {
        id -> Int4,
        category_id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        username -> Varchar,
        pass_hash -> Varchar,
        picture_path -> Varchar,
        verifyed -> Bool,
        rate_summ -> Int8,
        rate_count -> Int8,
        register_data -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    activation_links,
    brands,
    categories,
    chat,
    chat_messages,
    favourites,
    priveleges,
    products,
    promotions,
    rating,
    sub_categories,
    users,
);
