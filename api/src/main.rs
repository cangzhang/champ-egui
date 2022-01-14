pub mod web;

#[tokio::main]
pub async fn main() {
  match web::fetch_npm_info(String::from("@champ-r/op.gg")).await {
    Ok(data) => {
      println!("{:?}", data)
    }
    Err(e) => {
      println!("{:?}", e)
    }
  }
  
  match web::fetch_source_list().await {
    Ok(list) => {
      println!("{:?}", list)
    }
    Err(e) => {
      println!("{:?}", e)
    }
  }

  match web::fetch_champ_list(String::from("12.1.1")).await {
    Ok(list) => {
      println!("{:?}", list)
    }
    Err(e) => {
      println!("{:?}", e)
    }
  }
}
