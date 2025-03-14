use uuid::Uuid;

pub fn create_uuid_v4() -> String {
    Uuid::new_v4().to_string()
}
