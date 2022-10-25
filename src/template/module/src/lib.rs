use pam::{
    constants::{PamFlag, PamResultCode, PAM_PROMPT_ECHO_OFF},
    conv::Conv,
    module::{PamHandle, PamHooks},
    pam_try,
};

use std::{ffi::CStr, os::unix::process::CommandExt, process::Command};
use users::get_user_by_name;

struct PamTemplate;
pam::pam_hooks!(PamTemplate);

impl PamHooks for PamTemplate {
    // This function performs the task of authenticating the user.
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("Let's get this convo started...");
        // attempt to start a pam conversation
        let conv = match pamh.get_item::<Conv>() {
            Ok(Some(conv)) => conv,
            Ok(None) => {
                unreachable!("No conv available");
            }
            Err(err) => {
                println!("Couldn't get pam_conv");
                return err;
            }
        };

        let username = pam_try!(pamh.get_user(None));
        // attempt to check if username exists
        let user = match get_user_by_name(&username) {
            Some(user) => user,
            None => return PamResultCode::PAM_USER_UNKNOWN,
        };

        println!("username: {:?}", username);

        // get password from stack
        let password = pam_try!(conv.send(PAM_PROMPT_ECHO_OFF, "password: "));

        let password = match password {
            Some(password) => password.to_str().unwrap(),
            None => return PamResultCode::PAM_ABORT,
        };

        println!("password: {:?}", password);
        // password validation zone
        if password == "test_password" {
            println!("PAM_SUCCESS");
            // we now try to spawn `/bin/bash` as this user
            // note that setting the uid/gid is likely to fail if this program is not already run as the
            // proper user or as root
            /* let error = Command::new("/bin/bash")
                .uid(user.uid())
                .gid(user.primary_group_id())
                .exec();
            // if exec() returned, this means there was an error:
            println!("Error spawning bash: {:?}", error);
            */
            PamResultCode::PAM_SUCCESS
        } else {
            // incorrect password
            PamResultCode::PAM_AUTH_ERR
        }
    }

    fn sm_setcred(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        //println!("set credentials");
        PamResultCode::PAM_SUCCESS
    }

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        //println!("account management");
        PamResultCode::PAM_SUCCESS
    }
}
