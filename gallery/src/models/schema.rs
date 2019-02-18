table! {
    albums (id) {
        id -> Text,
        name -> Text,
        parent_album_id -> Nullable<Text>,
    }
}

table! {
    photos (id) {
        id -> Text,
        name -> Text,
        album_id -> Text,
        hash -> Text,
        creation_date -> Nullable<Timestamp>,
        camera -> Nullable<Text>,
        exposure_time -> Nullable<Text>,
        aperture -> Nullable<Text>,
        focal_length -> Nullable<Text>,
        focal_length_in_35mm -> Nullable<Text>,
        flash -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    albums,
    photos,
);
