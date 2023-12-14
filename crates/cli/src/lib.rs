use adapter::api::Api;

pub fn run() {
    let app_api = Api;

    let res = app_api.create_user("ddd");
    println!("{:?}", res.user)
}
