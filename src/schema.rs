// @generated automatically by Diesel CLI.

diesel::table! {
    admin (id) {
        id -> Int4,
    }
}

diesel::table! {
    answer (id) {
        id -> Int4,
        body -> Text,
        date -> Timestamp,
        question_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    ingredient (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
    }
}

diesel::table! {
    ingredient_request (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
    }
}

diesel::table! {
    question (id) {
        id -> Int4,
        #[max_length = 200]
        title -> Varchar,
        body -> Text,
        date -> Timestamp,
        user_id -> Int4,
        recipe_id -> Int4,
    }
}

diesel::table! {
    rating (recipe_id, user_id) {
        score -> Int4,
        recipe_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    recipe (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Varchar,
        description -> Text,
        creation_date -> Timestamp,
        directions -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    recipe_ingredient (ingredient_id, recipe_id) {
        recipe_id -> Int4,
        ingredient_id -> Int4,
        #[max_length = 50]
        quantity -> Varchar,
    }
}

diesel::table! {
    regular_user (id) {
        id -> Int4,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        creation_time -> Timestamp,
        #[max_length = 100]
        password -> Varchar,
    }
}

diesel::joinable!(admin -> user (id));
diesel::joinable!(answer -> question (question_id));
diesel::joinable!(answer -> regular_user (user_id));
diesel::joinable!(question -> recipe (recipe_id));
diesel::joinable!(question -> regular_user (user_id));
diesel::joinable!(rating -> recipe (recipe_id));
diesel::joinable!(rating -> regular_user (user_id));
diesel::joinable!(recipe -> regular_user (user_id));
diesel::joinable!(recipe_ingredient -> ingredient (ingredient_id));
diesel::joinable!(recipe_ingredient -> recipe (recipe_id));
diesel::joinable!(regular_user -> user (id));

diesel::allow_tables_to_appear_in_same_query!(
    admin,
    answer,
    ingredient,
    ingredient_request,
    question,
    rating,
    recipe,
    recipe_ingredient,
    regular_user,
    user,
);
