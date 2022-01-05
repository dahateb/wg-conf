use base64::encode;
use common::AuthType;
use tonic::{
    metadata::{Ascii, MetadataValue},
    Request, Status,
};

pub fn interceptor(
    auth: AuthBuilder,
) -> Box<dyn Fn(Request<()>) -> Result<Request<()>, Status> + Send + Sync + 'static> {
    let auth_header_value: &'static str = Box::leak(auth.get_auth().unwrap().into_boxed_str());
    let intercept = move |mut req: Request<()>| {
        let token: MetadataValue<Ascii> = MetadataValue::from_str(auth_header_value).unwrap();
        info!(
            "adding auth header {}, Intercepting request: {:?}",
            auth_header_value, req
        );
        req.metadata_mut().insert("authorization", token);
        Ok(req)
    };
    return Box::new(intercept);
}

pub struct AuthBuilder {
    auth_type: Option<AuthType>,
    auth_value: Option<String>,
}

impl AuthBuilder {
    pub fn new(
        auth_token: Option<&str>,
        auth_username: Option<&str>,
        auth_password: Option<&str>,
    ) -> AuthBuilder {
        let (auth_type, auth_value) = if auth_token.is_some() {
            (Some(AuthType::Bearer), Some(auth_token.unwrap().to_owned()))
        } else if auth_username.is_some() && auth_password.is_some(){
            let mut auth = auth_username.unwrap().to_string();
            auth.push_str(":");
            auth.push_str(auth_password.unwrap().as_ref());
            (Some(AuthType::Basic), Some(encode(auth)))
        } else {
            (None, None)
        };
        AuthBuilder {
            auth_type: auth_type,
            auth_value: auth_value,
        }
    }

    pub fn has_authentication(&self) -> bool {
        self.auth_type.is_some()    
    }

    pub fn get_auth(&self) -> Result<String, String> {
        if self.auth_type.is_some() && self.auth_value.is_some() {
            return Ok(self.auth_type.clone().unwrap().to_string() + " " + &self.auth_value.clone().unwrap())
        }
        return Err("no authentication defined".to_string());
    }
}

#[cfg(test)]
mod tests {
    use base64::encode;

    use super::AuthBuilder;

    #[test]
    fn test_get_auth_token() {
        let token = "asdfghjkllzuiztizu";
        let builder = AuthBuilder::new(Some(token), None, None);
        let auth_token = builder.get_auth();
        assert_eq!(format!("Bearer {}", token), auth_token.unwrap());
    }

    #[test]
    fn test_get_auth_user_password() {
        let user = "test";
        let pass = "12345678";
        let builder = AuthBuilder::new(None, Some(user), Some(pass));
        let auth_token = builder.get_auth();
        let check = format!("Basic {}", encode("test:12345678"));
        assert_eq!(check, auth_token.unwrap());
    }
}
