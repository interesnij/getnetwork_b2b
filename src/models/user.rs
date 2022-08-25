use crate::schema;
use crate::schema::{
    users,
};
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use actix_web::web::Json;


#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct User {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub phone:      String,
    pub email:      String,
    pub password:   String,
    pub perm:       i16,
}

impl User {
    pub fn is_superuser(&self) -> bool {
        return self.perm > 59;
    }
    pub fn create_superuser(&self) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::users::perm.eq(60))
            .get_result::<User>(&_connection)
            .expect("E");
    }
}
