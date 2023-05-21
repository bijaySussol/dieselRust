// @generated automatically by Diesel CLI.

diesel::table! {
    grades (student, subject, grade, time) {
        student -> Uuid,
        subject -> Int4,
        grade -> Float4,
        time -> Time,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    students (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        address -> Varchar,
        age -> Nullable<Int8>,
        date_of_birth -> Nullable<Date>,
    }
}

diesel::table! {
    subjects (index) {
        index -> Int4,
        subject -> Varchar,
        teacher -> Nullable<Uuid>,
    }
}

diesel::table! {
    teachers (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        degree -> Varchar,
        fully_employed -> Nullable<Bool>,
        contract_timestamp -> Timestamptz,
    }
}

diesel::joinable!(grades -> students (student));
diesel::joinable!(grades -> subjects (subject));
diesel::joinable!(subjects -> teachers (teacher));

diesel::allow_tables_to_appear_in_same_query!(
    grades,
    posts,
    students,
    subjects,
    teachers,
);
