// @generated automatically by Diesel CLI.

diesel::table! {
    attendees (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        event_date -> Timestamp,
        #[max_length = 255]
        location -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tickets (id) {
        id -> Int4,
        event_id -> Nullable<Int4>,
        attendee_id -> Nullable<Int4>,
        #[max_length = 100]
        ticket_type -> Nullable<Varchar>,
        price -> Nullable<Numeric>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    venues (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        capacity -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(tickets -> attendees (attendee_id));
diesel::joinable!(tickets -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(attendees, events, tickets, venues,);
