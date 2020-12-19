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
    news (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        picture -> Varchar,
        creation_datetime -> Timestamp,
        subtitle -> Varchar,
        banner -> Varchar,
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
        product_state -> Int4,
        brand_id -> Int4,
        seller_id -> Int4,
        pictures -> Array<Text>,
        type_id -> Int4,
        size_id -> Int4,
        total_views -> Int8,
        create_datetime -> Timestamp,
        phone_number -> Varchar,
        status -> Varchar,
        bought_with -> Nullable<Int4>,
    }
}

table! {
    product_state (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    product_type (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    promos (id) {
        id -> Int4,
        promo -> Varchar,
        sale -> Int4,
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
        in_news -> Bool,
    }
}

table! {
    rating (id) {
        id -> Int4,
        voter_id -> Int4,
        seller_id -> Int4,
        stars -> Int2,
        comment -> Varchar,
        feedback_type -> Varchar,
        create_datetime -> Timestamp,
        product_id -> Int4,
    }
}

table! {
    rescue_issues (id) {
        id -> Int4,
        contact -> Varchar,
        issue -> Varchar,
    }
}

table! {
    selled_posts (id) {
        id -> Int4,
        product_id -> Int4,
        selled_datetime -> Timestamp,
    }
}

table! {
    sizes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    social_links (id) {
        id -> Int4,
        name -> Varchar,
        link -> Varchar,
        icon -> Varchar,
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
    subscribes (id) {
        id -> Int4,
        from_id -> Int4,
        to_id -> Int4,
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
        is_banned -> Bool,
    }
}

table! {
    views (id) {
        id -> Int4,
        product_id -> Int4,
        count -> Int4,
    }
}

joinable!(activation_links -> users (user_id));
joinable!(chat_messages -> chat (chat_id));
joinable!(favourites -> products (product_id));
joinable!(favourites -> users (user_id));
joinable!(priveleges -> users (user_id));
joinable!(products -> brands (brand_id));
joinable!(products -> product_state (product_state));
joinable!(products -> product_type (type_id));
joinable!(products -> sizes (size_id));
joinable!(products -> sub_categories (sub_category_id));
joinable!(products -> users (seller_id));
joinable!(promotions -> products (product_id));
joinable!(selled_posts -> products (product_id));
joinable!(sub_categories -> categories (category_id));
joinable!(views -> products (product_id));

allow_tables_to_appear_in_same_query!(
    activation_links,
    brands,
    categories,
    chat,
    chat_messages,
    favourites,
    news,
    priveleges,
    products,
    product_state,
    product_type,
    promos,
    promotions,
    rating,
    rescue_issues,
    selled_posts,
    sizes,
    social_links,
    sub_categories,
    subscribes,
    users,
    views,
);
