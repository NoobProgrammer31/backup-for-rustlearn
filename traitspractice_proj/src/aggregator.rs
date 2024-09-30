pub trait Playable {
    fn summarize_song(&self) -> String;
}

pub trait Display1 {
    fn num_details(&self) -> u32;
}
pub struct Song {
    pub title: String,
    pub artist: String,
    pub length: u32,
}

pub struct Video {
    pub title: String,
    pub lyrics: String,
    pub length: u32,
}

impl Display1 for Song {
    fn num_details(&self) -> u32 {
        // error: i got the error that , expected u32 found (), so solved it with that return
        // self.length , because we need a u32 type return
        println!("Duration : {}", self.length);
        self.length
    }
}

impl Playable for Song {
    fn summarize_song(&self) -> String {
        format!("{}, by {}", self.title, self.artist)
    }
}
impl Playable for Video {
    fn summarize_song(&self) -> String {
        format!("{}, lyrics by {}", self.title, self.lyrics)
    }
}

// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. This parameter accepts any type that implements the specified trait.
// In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize. We can call notify and pass in any instance of NewsArticle or Tweet.
// Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.
pub fn notify(item: &impl Playable) {
    println!("New Release !,  {}", item.summarize_song())
}

// Similar appraoch with trait bound syntax
pub fn notify2<T: Playable>(item: &T) {
    println!("You are playing, {}", item.summarize_song());
}

// Specifying Multiple Trait Bounds with the + Syntax
pub fn mix_traits<T: Playable + Display1>(item: &T) {
    println!("You are playing, {}", item.summarize_song());
}
