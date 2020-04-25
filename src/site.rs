use serde::Serialize;

use pulldown_cmark::{html, Options, Parser};

use std::fs;
use std::io;
use std::path;
use std::time;

const HOME_DIR: &str = "/home/bender/Documents/Projects/Rust/blog";

#[derive(Serialize, Debug)]
pub struct SitePage<T> {
    options: [String; 4],
    content: T,
}

impl<T> SitePage<T>
where
    T: Serialize,
{
    pub fn new(content: T) -> Self {
        Self {
            content,
            options: [
                String::from("Home"),
                String::from("Blog"),
                String::from("About me"),
                String::from("Projects"),
            ],
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Home {
    front_img: String,
    main_text: String,
    more_text: String,
}
impl Home {
    pub fn new() -> Self {
        Home {
            front_img: String::from("Home_img.webp"),
            main_text: String::from("I've never been good at writing."),
            more_text: fs::read_to_string(path::PathBuf::from(format!(
                "{}{}",
                HOME_DIR, "/static/content/introduction_text.txt"
            )))
            .unwrap(),
        }
    }
}

#[derive(Serialize)]
pub struct BlogHome {
    posts: Vec<Post>,
}
impl BlogHome {
    //TODO: Double check that all these unwraps are safe
    //working with IO sucks.
    pub fn new() -> Self {
        let mut posts = vec![];
        //this unwrap is safe as long as directory can be read, which
        //is a valid assumption for this server 100% of the time.
        for post in fs::read_dir("./static/blog_posts").unwrap() {
            //this unwrap is unknown.
            let post = post.unwrap();
            if post.metadata().unwrap().is_file() {
                //this unwrap is valid due to my system being UTF-8
                //Not trivial to convert to utf-8 if unknown encoding.
                posts.push(Post::from_file_path(&post.path()).unwrap());
            }
        }
        posts.sort_by(|post, post2| post2.timestamp.cmp(&post.timestamp));
        Self { posts }
    }
}
#[derive(Serialize)]
struct Post {
    name: String,
    content: String,
    timestamp: time::SystemTime,
}
impl Post {
    fn new(name: String, content: String, time: time::SystemTime) -> Self {
        Self {
            name,
            content,
            timestamp: time,
        }
    }
    fn from_file_path(path: &path::Path) -> Result<Self, io::Error> {
        let timestamp = fs::metadata(path)?.created()?;
        let markdown = fs::read_to_string(path.to_owned().into_os_string().into_string().unwrap())?;
        let parsed_markdown = Parser::new_ext(&markdown, Options::all());
        let mut html = String::new();
        html::push_html(&mut html, parsed_markdown);
        Ok(Self {
            name: path.file_name().unwrap().to_owned().into_string().unwrap(),
            content: html,
            timestamp,
        })
    }
}
