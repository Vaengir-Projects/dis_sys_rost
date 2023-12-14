#[cfg(test)]
mod db_connection_test {
  use dotenv::dotenv;
  use sqlx::MySqlPool;
  use std::env;

  #[tokio::test]
  async fn connection_successfull() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Can't find database url");
    let pool = MySqlPool::connect(&database_url).await;

    match pool {
      Ok(_) => (),
      Err(err) => panic!("Connection failed: {:?}", err),
    };
  }
}
