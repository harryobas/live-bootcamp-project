pub mod user;
pub mod error;
pub mod user_stores;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Email(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Password(pub String);

impl Email {
    pub fn parse(s: &str) -> Result<Self, & 'static str> {
        if s.contains('@') && !s.is_empty() {
            Ok(Email(s.to_string()))
        }else{
            Err("Invalid email format")
        }
    }

}

impl AsRef<str> for Email {
     fn as_ref(&self) -> &str {
        self.0.as_str()
        
    }
}

impl Password {
    pub fn parse(s: &str) -> Result<Self, & 'static str> {
        if s.len() >= 8 {
            Ok(Password(s.to_string()))
        }else{
            Err("Invalid password")
        }
    }
    
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
    
}

mod tests{
    use super::*;

    #[test]
    fn valid_email_should_parse(){
        let input = "user@example.com";
        let email = Email::parse(input);

        assert!(email.is_ok());
        assert_eq!(email.unwrap().as_ref(), input);
    }

    #[test]
    fn invalid_email_should_fail() {
        let input = "userexample.com";
        let email = Email::parse(input);

        assert!(email.is_err())
    }

    #[test]
    fn valid_password_should_parse() {
        let input = "$password";
        let password = Password::parse(input);

        assert!(password.is_ok());
        assert_eq!(password.unwrap().as_ref(), input);
    }

    #[test]
    fn invalid_password_should_fail() {
        let input = "pass";
        let password = Password::parse(input);

        assert!(password.is_err())
    }






}



