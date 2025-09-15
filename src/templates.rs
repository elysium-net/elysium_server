use askama::Template;

#[derive(Template)]
#[template(path = "./verify_email.html")]
pub struct VerifyEmailTemplate<'a> {
    pub token: &'a str,
}
