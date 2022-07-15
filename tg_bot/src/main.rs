use std::net::Ipv4Addr;
use ip_network::Ipv4Network;

let ip_network = Ipv4Network::new(Ipv4Addr::new(192,168,88,21), 24)?;
assert_eq!(Ipv4Addr::new(192,168,88,21), ip_network.network_address());
assert_eq!(24, ip_network.netmask());
assert_eq!(254, ip_network.hosts().len());
assert_eq!("192.168.88.21/24", ip_network.to_string());


fn main()
{
    match open::that(r"https://api.telegram.org/bot5407477830:AAHRzUQ1_Yt-TQmhgYfqrTfEe3HWgXMNSJM/sendMessage?chat_id=856546075\&parse_mode=HTML&text=hello") {
        Ok(()) => println!("Opened link successfully."),
        Err(err) => eprintln!("An error occurred when opening link: '{}'",err),
    }

}