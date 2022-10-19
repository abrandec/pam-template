use pam::Authenticator;

pub fn main() {
    let service = "template-pamrs";
    let user = "testenv";
    let passwd = "test_password";
    let mut auth = Authenticator::with_password(service).unwrap();
    auth.get_handler().set_credentials(user, passwd);

    // actually try to authenticate:
    let is_authenticated = auth.authenticate();
    println!("{:?}", is_authenticated);
}
