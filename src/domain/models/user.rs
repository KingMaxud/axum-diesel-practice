use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
}
