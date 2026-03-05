use crate::ui::styles::themes::ThemeName;
use crate::ui::utils::html_escape::escape_html;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ContainerProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub fluid: bool,
    pub theme: Option<ThemeName>,
}

#[derive(Debug, Clone)]
pub struct Container {
    props: ContainerProps,
    children: String,
}

impl Container {
    pub fn new(props: ContainerProps, children: String) -> Self {
        Self { props, children }
    }

    pub fn render(&self) -> String {
        let mut html = String::from("<div");

        if let Some(id) = &self.props.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }

        let mut classes = String::from("container");
        if self.props.fluid {
            classes.push_str("-fluid");
        }
        if let Some(class_name) = &self.props.class_name {
            classes.push_str(&format!(" {}", escape_html(class_name)));
        }
        html.push_str(&format!(" class=\"{}\"", classes));

        html.push('>');
        html.push_str(&escape_html(&self.children));
        html.push_str("</div>");

        html
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RowProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub theme: Option<ThemeName>,
}

#[derive(Debug, Clone)]
pub struct Row {
    props: RowProps,
    children: String,
}

impl Row {
    pub fn new(props: RowProps, children: String) -> Self {
        Self { props, children }
    }

    pub fn render(&self) -> String {
        let mut html = String::from("<div");

        if let Some(id) = &self.props.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }

        let mut classes = String::from("row");
        if let Some(class_name) = &self.props.class_name {
            classes.push_str(&format!(" {}", escape_html(class_name)));
        }
        html.push_str(&format!(" class=\"{}\"", classes));

        html.push('>');
        html.push_str(&escape_html(&self.children));
        html.push_str("</div>");

        html
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub span: Option<usize>,
    pub offset: Option<usize>,
    pub theme: Option<ThemeName>,
}

#[derive(Debug, Clone)]
pub struct Col {
    props: ColProps,
    children: String,
}

impl Col {
    pub fn new(props: ColProps, children: String) -> Self {
        Self { props, children }
    }

    pub fn render(&self) -> String {
        let mut html = String::from("<div");

        if let Some(id) = &self.props.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }

        let mut classes = String::from("col");
        if let Some(span) = self.props.span {
            classes.push_str(&format!(" col-{}", span));
        }
        if let Some(offset) = self.props.offset {
            classes.push_str(&format!(" offset-{}", offset));
        }
        if let Some(class_name) = &self.props.class_name {
            classes.push_str(&format!(" {}", escape_html(class_name)));
        }
        html.push_str(&format!(" class=\"{}\"", classes));

        html.push('>');
        html.push_str(&escape_html(&self.children));
        html.push_str("</div>");

        html
    }
}
