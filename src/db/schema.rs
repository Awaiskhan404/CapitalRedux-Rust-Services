// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        is_entrepreneur -> Bool,
        is_admin -> Bool,
        is_investor -> Bool,
        user_type -> Text,
        cash -> Int4,
        net_assets -> Int4,
        net_liabilities -> Int4,
        net_worth -> Int4,
        profile_pic -> Nullable<Text>,
        bio -> Nullable<Text>,
        location -> Nullable<Text>,
        website -> Nullable<Text>,
        phone -> Nullable<Text>,
        facebook -> Nullable<Text>,
        twitter -> Nullable<Text>,
        instagram -> Nullable<Text>,
        linkedin -> Nullable<Text>,
    }
}
