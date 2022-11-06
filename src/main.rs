
use std::error::Error;
use rss::{Channel};
use tokio;
use iced::{Settings, Length, Scrollable, scrollable};
use iced::widget::{Button, Column, Container, Text};
use iced::Sandbox;


#[derive(Debug)]
struct Post {
    title : String,
    description : String,
    url : String
}

#[derive(Debug)]
struct Reader {
    url : String,
    posts : Vec<Post>,

    scrollable_state: scrollable::State,
}

#[derive(Debug, Clone, Copy)]
enum Messages {
    Refresh
}


fn main() {
    println!("Hello, world!");
    let mut rss = Reader::new("https://www.bundesgesundheitsministerium.de/meldungen.xml".to_string()).expect("Error");
    rss.fetch();
    rss.log();
    println!("{:?}",Reader::run(Settings::default()));
}

impl Reader {
    pub fn new(url : String) -> Result<Reader, Box<dyn Error>> {
        Ok(Reader{
            url,
            posts : Vec::new(),
            scrollable_state: scrollable::State::new()
        })
    }
    pub fn fetch(&mut self){
        let rt = tokio::runtime::Runtime::new().unwrap();
        let feed = rt.block_on(example_feed(self.url.to_string())).unwrap();
        //let feed = futures::executor::block_on(example_feed(self.url.to_string())).unwrap();
        self.posts = feed.items().iter().take(20).map(|p| 
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
            Messages::Refresh => self.fetch(),
        }
    }

    fn new() -> Self {
        Reader{
            url : "https://www.bundesgesundheitsministerium.de/meldungen.xml".to_string(),
            posts : Vec::new(),
            scrollable_state : scrollable::State::new()
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        //let post = Post{title : "Title".to_string(), description: "Description".to_string(), url : "www.url.com".to_string()};

        let mut news = Scrollable::new(&mut self.scrollable_state)
            .spacing(20)
            .padding(25);

        for post in &mut self.posts {
            news = news.push(post.view());
        }

        Container::new(news)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .into()
    }
}

impl Post {
    pub fn view(&self) -> iced::Element<Messages> {
        Column::new()
        .push(Text::new(&self.title).size(20))
        .push(Text::new(&self.description).size(12))
        .push(Text::new(&self.url))
        .spacing(8)
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