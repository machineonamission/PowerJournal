use ammonia::Builder;
use anyhow::Result;
use std::collections::HashSet;

pub fn sanitize_html(html: &str) -> Result<String> {
    // 1. Sanitize: allow only the tags/attrs we actually render.
    //    Anything else (div, span, style, script, table, img, etc.) is stripped,
    //    but its inner text is kept.
    let allowed_tags: HashSet<&str> = [
        "b",
        "br",
        "i",
        "u",
        "ul",
        "li",
        "ol",
        "p",
        "a",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "strike",
        "em",
        "strong",
        "code",
        "pre",
        "blockquote",
        "hr",
        "div",
        "span",
    ]
    .into_iter()
    .collect();

    let mut builder = Builder::default();
    builder
        .tags(allowed_tags)
        .link_rel(None) // don't inject rel="noopener" etc, keep it plain
        .generic_attributes(HashSet::from(["style", "class", "id", "href"])); // strip style/class/id/etc everywhere

    let clean_html = builder.clean(html).to_string();

    Ok(clean_html)
}

