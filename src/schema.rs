table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        firstname -> Varchar,
        lastname -> Nullable<Varchar>,
        password -> Varchar,
        salt -> Varchar,
        email -> Varchar,
        wk_key -> Nullable<Varchar>,
        settings -> Nullable<Varchar>,
    }
}

