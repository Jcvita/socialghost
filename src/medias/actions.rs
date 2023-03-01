//create an enum for all possible social medias
pub enum SocialMedias {
    Snapchat,
    Instagram,
    Twitter,
    Facebook,
    Youtube,
    Pinterest,
    Linkedin,
    Tiktok,
}

//create an enum for the following actions: login, post, friend, feed, search, profile, logout
pub enum Action {
    Login,
    Post,
    Friend,
    Feed,
    Search,
    Profile,
    Logout,
}

pub struct Advertisement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub link: String,
    pub image: String,
}

impl Advertisement {
    pub fn new(id: String, name: String, description: String, link: String, image: String) -> Self {
        Self {
            id,
            name,
            description,
            link,
            image,
        }
    }

    pub fn from_file() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            link: String::new(),
            image: String::new(),
        }
    }

    pub fn to_file(&self) {

    }
}

pub trait SocialMedia {
    fn login(&self);
    fn logout(&self);
}

pub trait Advertise {
    fn advertise(&self, ad: Advertisement);
    fn metrics(&self, ad: Advertisement);
}

pub trait Friend {
    fn add(&self, friend: String);
    fn remove(&self, friend: String);
}

pub trait Post {
    fn post(&self, post: String);
    fn delete(&self, post: String);
}