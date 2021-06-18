use once_cell::sync::Lazy;
use tera::Tera;

pub static TEMPLETES: Lazy<Tera> = Lazy::new(init_template);
pub fn init_template() -> Tera {
    let mut tera = Tera::new("templates/*").unwrap();
    tera.autoescape_on(vec!["tera"]);
    tera
}

pub fn card<T: serde::Serialize>(tera: &Tera, context: &T) -> String {
    let context = tera::Context::from_serialize(context).unwrap();
    tera.render("card.html.tera", &context).unwrap()
}

pub fn index(tera: &Tera) -> String {
    tera.render("index.html.tera", &tera::Context::new())
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn init_test() {
        let tera = init_template();

        dbg!(tera);
    }

    #[test]
    fn card_test() {
        let a = card(
            &TEMPLETES,
            &crate::crawl::TargetPage {
                url: "aiueo".to_string(),
                title: "kakikukeko".to_string(),
                description: "sashisuseso".to_string(),
                image_url: "tatituteto".to_string(),
            },
        );
        dbg!(a);
    }
}
