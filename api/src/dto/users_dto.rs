pub struct CreateUserDTO {
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}

pub struct RegisterUserDTO {
    pub industry: i64,
    pub email: String,
    pub lastname: String,
    pub firstname: String,
    pub company_name: Option<String>,
    pub company_address: Option<String>,
    pub sending_domain: Option<String>,
    pub defualt_from_name: Option<String>,
    pub default_from_email: Option<String>,
}
