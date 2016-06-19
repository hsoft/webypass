use kuchiki;
use kuchiki::traits::*;
use liquid;
use liquid::{Renderable, Context, Value};

pub fn fix(html: &str) -> String {
    let doc = kuchiki::parse_html().one(html);
    let mut titles: Vec<Value> = Vec::new();
    let matches = doc.select("div.thing a.title").unwrap();

    for css_match in matches {
        let node = css_match.as_node();
        let text_node = node.first_child().unwrap();
        let text = Value::Str(text_node.as_text().unwrap().borrow().clone());
        titles.push(text);
    }

    let template = liquid::parse("<html><body><ul>{% for title in titles %}<li>{{ title }}</li>{% endfor %}</ul></body></html>", Default::default()).unwrap();
    let mut context = Context::new();
    context.set_val("titles", Value::Array(titles));
    template.render(&mut context).unwrap().unwrap()
}
