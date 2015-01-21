pub enum UserStatus {
    OK,
    TROUBLE,
    OFFLINE
}

pub struct User {
    name:   String,
    status: UserStatus,
    msg:    String
}

#[test]
fn it_works() {
}
