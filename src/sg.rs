/*
Usage:
-s -> snapchat
-i -> instagram
-t -> twitter
-f -> facebook
-y -> youtube shorts
-p -> pinterest
-l -> linkedin
-k -> tiktok
 */

use crate::medias;

use medias::actions::SocialMedias;
use medias::actions::Advertisement;

pub struct SocialGhost {
    identity: String,
    campaigns: Vec<Box<Advertisement>>, //box of a trait object allows us to store different types structs with the same trait
    snapchat: Option<Snapchat>,
    instagram: Option<Instagram>,
    twitter: Option<Twitter>,
    facebook: Option<Facebook>,
    youtube: Option<Youtube>,
    pinterest: Option<Pinterest>,
    linkedin: Option<Linkedin>,
    tiktok: Option<Tiktok>
}

impl SocialGhost {
    pub fn new() -> Self {
        Self {
            identity: "new".to_string(),
            campaigns: Vec::new(),
            snapchat: None,
            instagram: None,
            twitter: None,
            facebook: None,
            youtube: None,
            pinterest: None,
            linkedin: None,
            tiktok: None,
        }
    }

    // pub fn from_file() -> Self {
    //     Self {

    //     }
    // }

    // pub fn to_file(&self) {

    // }

    pub fn init_social_media(&mut self, arg_str: &str) {
        match arg_str {
            "snapchat" => println!("snapchat"),
            "instagram" => println!("instagram"),
            "twitter" => println!("twitter"),
            "facebook" => println!("facebook"),
            "youtube" => println!("youtube shorts"),
            "pinterest" => println!("pinterest"),
            "linkedin" => println!("linkedin"),
            "tiktok" => println!("tiktok"),
            _ => arg_str.chars().for_each(|character| {
                self.init_social_media_by_char(character)
            }),
        }
    }

    pub fn init_social_media_by_enum(&mut self, social_media: SocialMedias) {
        match social_media {
            SocialMedias::Snapchat => println!("snapchat"),
            SocialMedias::Instagram => println!("instagram"),
            SocialMedias::Twitter => println!("twitter"),
            SocialMedias::Facebook => println!("facebook"),
            SocialMedias::Youtube => println!("youtube shorts"),
            SocialMedias::Pinterest => println!("pinterest"),
            SocialMedias::Linkedin => println!("linkedin"),
            SocialMedias::Tiktok => println!("tiktok"),
        }
    }

    pub fn init_social_media_by_name(&mut self, name: &str) {
        match name {
            "snapchat" => println!("snapchat"),
            "instagram" => println!("instagram"),
            "twitter" => println!("twitter"),
            "facebook" => println!("facebook"),
            "youtube" => println!("youtube shorts"),
            "pinterest" => println!("pinterest"),
            "linkedin" => println!("linkedin"),
            "tiktok" => println!("tiktok"),
            _ => println!("{} is not a valid social media", name),
        }
    }

    pub fn init_social_media_by_char(&mut self, letter: char) {
        match letter {
            's' => println!("snapchat"),
            'i' => println!("instagram"),
            't' => println!("twitter"),
            'f' => println!("facebook"),
            'y' => println!("youtube shorts"),
            'p' => println!("pinterest"),
            'l' => println!("linkedin"),
            'k' => println!("tiktok"),
            _ => println!("{} is not a valid social media letter", letter),
        }
    }
}