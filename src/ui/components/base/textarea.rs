use crate::ui::utils::html_escape::escape_html;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextareaProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub rows: Option<usize>,
    pub cols: Option<usize>,
    pub disabled: bool,
    pub readonly: bool,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct Textarea {
    props: TextareaProps,
}

impl Textarea {
    pub fn new(props: TextareaProps) -> Self {
        Self { props }
    }

    pub fn render(&self) -> String {
        let mut html = String::from("<textarea");

        if let Some(id) = &self.props.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }

        let mut classes = String::from("form-control");
        if let Some(class_name) = &self.props.class_name {
            classes.push_str(&format!(" {}", escape_html(class_name)));
        }
        html.push_str(&format!(" class=\"{}\"", classes));

        if let Some(rows) = self.props.rows {
            html.push_str(&format!(" rows=\"{}\"", rows));
        }

        if let Some(cols) = self.props.cols {
            html.push_str(&format!(" cols=\"{}\"", cols));
        }

        if let Some(placeholder) = &self.props.placeholder {
            html.push_str(&format!(" placeholder=\"{}\"", escape_html(placeholder)));
        }

        if self.props.disabled {
            html.push_str(" disabled");
        }

        if self.props.readonly {
            html.push_str(" readonly");
        }

        if self.props.required {
            html.push_str(" required");
        }

        html.push('>');

        if let Some(value) = &self.props.value {
            html.push_str(&escape_html(value));
        }

        html.push_str("</textarea>");

        html
    }
}

/// 初始化文本域组件
pub async fn initialize() -> Result<(), crate::error::Error> {
    Ok(())
}
