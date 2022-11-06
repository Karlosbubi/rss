
use std::error::Error;
use rss::{Channel};
use tokio;
use iced::{Settings, Length};
use iced::pure::widget::{Button, Column, Container, Text};
use iced::pure::Sandbox;


#[derive(Debug)]
struct Post {
    title : String,
    description : String,
    url : String
}

#[derive(Debug)]
struct Reader {
    url : String,
    posts : Vec<Post>
}

#[derive(Debug)]
enum Messages {
    Refresh
}


fn main() {
    println!("Hello, world!");
    let mut rss = Reader::new("https://www.bundesgesundheitsministerium.de/meldungen.xml".to_string()).expect("Error");
    rss.update();
    rss.log();
}

impl Reader {
    pub fn new(url : String) -> Result<Reader, Box<dyn Error>> {
        Ok(Reader{
            url,
            posts : Vec::new(),
        })
    }
    pub fn update(&mut self){
        let rt = tokio::runtime::Runtime::new().unwrap();
        let feed = rt.block_on(example_feed(self.url.to_string())).unwrap();
        //let feed = futures::executor::block_on(example_feed(self.url.to_string())).unwrap();
        self.posts = feed.items().iter().map(|p| 
            Post{
                title : p.title().unwrap_or("No Title Provided").to_string(),
                description : p.description().unwrap_or("No Description Provided").to_string(),
                url : p.link().unwrap_or("No Link Provided").to_string()
            }).collect();
    }
    pub fn log(&self){
        for p in self.posts.iter(){
            println!("{}", p.title);
            println!("{}", p.description);
            println!("{}", p.url);
            println!("------------------------------------------------");
        }
    }


}

impl Sandbox for Reader {
    type Message = Messages;

    fn title(&self) -> String {
        String::from("BMG Feed")
    }


    fn update(&mut self, message: Self::Message) {
        match message {
            Messages::Refresh => self.update(),
        }
    }

    fn new() -> Self {
        Reader{
            url : "https://www.bundesgesundheitsministerium.de/meldungen.xml".to_string(),
            posts : Vec::new(),
        }
    }

    fn view(&self) -> iced::pure::Element<'_, Self::Message> {
        Container::new(Text::new("Hello Iced"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

async fn example_feed(url : String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
    .await?
    .bytes()
    .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}