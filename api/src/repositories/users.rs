use crate::dto::users_dto::CreateUserDTO;
use crate::models::users::User;
use diesel::result::QueryResult;

// user traits/interface
pub trait UserRepository {
    fn create_user(&self, user_dto: &CreateUserDTO) -> QueryResult<User>;
}
