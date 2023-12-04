use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

use rand::Rng;
use std::env;

mod domain;

use domain::user::user::User;

pub mod schema;
use schema::users;

use self::schema::users::dsl::*;

pub fn establish_connection() ->  PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
   PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn show_users(connection : &mut PgConnection){

    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for post in results {
        println!("{}", post.id);
        println!("{}", post.username);
    }
}


//pub fn create_user(connection: &mut PgConnection, user : &CreateUser) -> User {
//
//    diesel::insert_into(users::table)
//        .values(user)
//        .returning(User::as_returning())
//        .get_result(connection)
//        .expect("Error saving new post")
//}



#[tokio::main]
async fn main() {


    let connection = &mut establish_connection();

    //create_user(connection, &CreateUser { username: "root".to_string(), password: "1234".to_string() });
    show_users(connection);

}

