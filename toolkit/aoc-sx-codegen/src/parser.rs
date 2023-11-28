use scraper::{ElementRef, Html, Selector};
use url::Url;

#[derive(Debug, Default)]
pub struct MarkdownContent(String);

#[derive(Debug)]
pub struct DayPageParser {
    page_url: Url,
    input_url: Url,
}

impl DayPageParser {
    pub fn new(page_url: Url, input_url: Url) -> Self {
        Self {
            page_url,
            input_url,
        }
    }

    pub fn parse(&self, html_content: &str) -> MarkdownContent {
        let mut md = MarkdownContent::new();

        let document = Html::parse_document(html_content);
        let selector = Selector::parse("article[class='day-desc']").unwrap();
        let nodes = document.root_element().select(&selector);
        for node in nodes {
            md.push_content(self.convert_html_to_markdown(&node));
        }

        // Insert links at second line
        md.insert_at_line(
            2,
            MarkdownContent(format!(
                "> _Exercise page: <{}>_\n\n> _Input page: <{}>_\n",
                self.page_url, self.input_url
            )),
        );

        md
    }

    fn convert_html_to_markdown(&self, node: &ElementRef) -> MarkdownContent {
        let mut output = String::new();

        for child in node.children() {
            if let Some(tag) = child.value().as_element() {
                if tag.name() == "h2" {
                    output.push_str(&format!(
                        "# {}\n",
                        ElementRef::wrap(child).unwrap().text().collect::<String>()
                    ));
                } else if tag.name() == "p" {
                    output.push_str(&ElementRef::wrap(child).unwrap().inner_html());
                } else if tag.name() == "pre" {
                    output.push_str("```text\n");
                    output.push_str(&ElementRef::wrap(child).unwrap().text().collect::<String>());
                    output.push_str("```");
                } else if tag.name() == "ul" {
                    for child in child.children() {
                        if child.value().is_element() {
                            output.push_str(&format!(
                                "- {}\n",
                                ElementRef::wrap(child).unwrap().text().collect::<String>()
                            ));
                        }
                    }
                }
            }

            output.push('\n');
        }

        MarkdownContent(output)
    }
}

impl MarkdownContent {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn push_content(&mut self, content: MarkdownContent) {
        self.0.push_str(&content.0);
    }

    pub fn insert_at_line(&mut self, line: usize, content: MarkdownContent) {
        let mut lines = self.0.lines().map(|x| x.to_owned()).collect::<Vec<_>>();
        lines.insert(line, content.0);
        self.0 = lines.join("\n");
    }
}
