use std::net::Ipv4Addr;
use ip_network::Ipv4Network;

fn main()
{
    match open::that(r"https://api.telegram.org/bot5407477830:AAHRzUQ1_Yt-TQmhgYfqrTfEe3HWgXMNSJM/sendMessage?chat_id=856546075\&parse_mode=HTML&text=hello") {
        Ok(()) => println!("Opened link successfully."),
        Err(err) => eprintln!("An error occurred when opening link: '{}'",err),
    }

}