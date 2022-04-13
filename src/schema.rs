table! {
    pending_msgs (id) {
        id -> Int8,
        msg_tag -> Bytea,
        epoch_tag -> Int2,
        parent_recovered_msg_id -> Nullable<Int8>,
        message -> Bytea,
    }
}

table! {
    recovered_msgs (id) {
        id -> Int8,
        msg_tag -> Bytea,
        epoch_tag -> Int2,
        metric_name -> Varchar,
        metric_value -> Varchar,
        key -> Bytea,
        parent_recovered_msg_id -> Nullable<Int8>,
        count -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(pending_msgs, recovered_msgs,);
