use serde::Serialize;

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
            more_text: String::from(include_str!("../static/content/introduction_text.txt")),
        }
    }
}
