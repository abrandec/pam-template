extern crate pam;

use pam::{
    constants::{PamFlag, PamResultCode, PAM_PROMPT_ECHO_OFF},
    conv::Conv,
    module::{PamHandle, PamHooks},
    pam_try,
};
use std::ffi::CStr;

struct PamPassAuth;

impl PamHooks for PamPassAuth {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let _user = pam_try!(pamh.get_user(None));

        // pam conversation
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
        //

        let password = pam_try!(conv.send(PAM_PROMPT_ECHO_OFF, "Word, yo: "));
        let password = match password {
            Some(password) => Some(pam_try!(password.to_str(), PamResultCode::PAM_AUTH_ERR)),
            None => None,
        };

        if password == Some("test_password") {
            PamResultCode::PAM_SUCCESS
        } else {
            PamResultCode::PAM_AUTH_ERR
        }
    }

    fn sm_setcred(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("set credentials");
        PamResultCode::PAM_SUCCESS
    }

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("account management");
        PamResultCode::PAM_SUCCESS
    }
}
