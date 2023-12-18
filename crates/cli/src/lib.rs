use adapters::api::Api;
use adapters::db::Db;

pub fn run<D>(db: D)
    where
        D: Db,
{
    let app_api = Api::new(db);
    let res = app_api.create_user("333", "333");
    println!("{:?}", res)
}