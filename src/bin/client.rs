use sandbloodwater::common::EventId;
use sandbloodwater::network::HttpClient;

fn main() {
    let client_res = HttpClient::new("127.0.0.1:7878");
    if let Ok(mut client) = client_res {
        loop {
            client.send_event(EventId::Test(String::from("Test string content")));
            client.receive_event();
        }
    }
}
