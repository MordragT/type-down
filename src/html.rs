use html_tag::HtmlTag;
use yaserde::YaSerialize;
use yaserde_derive::YaSerialize;

// #[derive(Default, PartialEq, Debug, YaSerialize)]
// #[yaserde(rename = "h1")]
// pub struct H1<C: YaSerialize> {
//     #[yaserde(child)]
//     child: C,
// }

pub fn to_html(body: HtmlTag) -> String {
    let body = body.to_html();
    format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>HTML5 Boilerplate</title></head><body>{body}</body></html>")
}
