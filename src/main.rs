fn main()
{
    match open::that(r"https://api.telegram.org/botYOUR_TOKEN/sendMessage?chat_id=YOUR_CHAT_ID\&parse_mode=HTML&text=YOUR_TEXT") {
        Ok(()) => println!("Opened link successfully."),
        Err(err) => eprintln!("An error occurred when opening link: '{}'",err),
    }

}