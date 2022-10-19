use pam;
use std::os::unix::process::CommandExt;
use std::process::Command;
use users::get_user_by_name;

pub fn main() {
    let service = "pam-pass-auth";
    let user = "testenv";
    let passwd = "test_password";
    let mut auth = pam::Authenticator::with_password(service).unwrap();
    auth.get_handler().set_credentials(user, passwd);

    // actually try to authenticate:
    let is_authenticated = auth.authenticate();
    match is_authenticated {
        Ok(_) => println!("{:?}", is_authenticated),
        Err(_) => println!("{:?}", is_authenticated),
    }

    // Now that we are authenticated, it's possible to open a sesssion:
    auth.open_session().expect("Failed to open a session!");
    let user = get_user_by_name(&user).unwrap();
    let error = Command::new("/bin/bash")
        .uid(user.uid())
        .gid(user.primary_group_id())
        .exec();
    // if exec() returned, this means there was an error:
    println!("Error spawning bash: {:?}", error);
}
