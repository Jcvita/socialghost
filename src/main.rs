/*
    Usage:
    sg [<social media> | <letters>] <action> (<arguments>)

    Example:
    sg login snapchat -u <username> -p <password>
    sg advertise isyk -n <name> -d <description> -l <link> -i <image>
    sg post y -t [short | video] -v <video> -d <description>

    s -> snapchat
    i -> instagram
    t -> twitter
    f -> facebook
    y -> youtube shorts
    p -> pinterest
    l -> linkedin
    k -> tiktok
 */
pub mod sg;
pub mod medias;

use sg::SocialGhost;

fn main() {
    //for each argument in the command line, print the corresponding social media
    let mut args = std::env::args();
    args.next();

    let mut sg: SocialGhost = SocialGhost::new();

    let social_media = args.next().unwrap();
    sg.init_social_media(social_media.as_str());

    let action = args.next().unwrap();
    match action.as_str() {
        "login" => println!("login"),
        "post" => println!("post"),
        "friend" => println!("friend"),
        "feed" => println!("feed"),
        "search" => println!("search"),
        "profile" => println!("profile"),
        "logout" => println!("logout"),
        _ => println!("{} is not a valid argument", action),
    }
    
}