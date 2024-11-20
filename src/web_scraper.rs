use reqwest::blocking::get;
use scraper::{Html, Selector};

pub(crate) fn init() {
    let url = "https://malopolskalokalnie.pl/z-tata-zdobywam-szczyty/";
    let response = get(url);
    let page_content = response.unwrap().text().unwrap();

    let document = Html::parse_document(&page_content);
    let selector = Selector::parse("div.single-contest__file").unwrap();

    let images = document
        .select(&selector)
        .map(|o| {
            o.value()
                .attr("style")
                .unwrap()
                .replace("background-image:url(", "")
                .replace(")", "")
        })
        .collect::<Vec<_>>();
    print!("{:?}", images);
}
