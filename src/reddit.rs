use std::collections::HashMap;

use kuchiki::NodeRef;
use liquid;
use liquid::{Renderable, Context, Value};

pub fn fix(document: &NodeRef) -> String {
    let mut links: Vec<Value> = Vec::new();
    let matches = document.select("div.thing a.title").unwrap();

    for css_match in matches {
        let mut link: HashMap<String, Value> = HashMap::new();
        let node = css_match.as_node();
        let text_node = node.first_child().unwrap();
        let title = text_node.as_text().unwrap().borrow().clone();
        link.insert("title".to_owned(), Value::Str(title));
        let node_attrs = node.as_element().unwrap().attributes.borrow();
        let href = node_attrs.get("href").unwrap();
        link.insert("href".to_owned(), Value::Str(href.to_owned()));
        links.push(Value::Object(link));
    }

    let template = liquid::parse(
        "
<html>
    <body>
        <ul>
        {% for link in links %}
            <li><a href=\"{{ link.href }}\">{{ link.title }}</a></li>
        {% endfor %}
        </ul>
    </body>
</html>
",
        Default::default()
    ).unwrap();
    let mut context = Context::new();
    context.set_val("links", Value::Array(links));
    template.render(&mut context).unwrap().unwrap()
}
