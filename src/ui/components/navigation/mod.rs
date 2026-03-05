use crate::ui::styles::themes::ThemeName;
use crate::ui::utils::html_escape::escape_html;

#[derive(Debug, Clone, PartialEq)]
pub struct NavbarProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub brand: Option<String>,
    pub brand_href: Option<String>,
    pub fixed: Option<String>,
    pub theme: Option<ThemeName>,
}

impl Default for NavbarProps {
    fn default() -> Self {
        Self {
            id: None,
            class_name: None,
            brand: None,
            brand_href: Some("#".to_string()),
            fixed: None,
            theme: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Navbar {
    props: NavbarProps,
    children: String,
}

impl Navbar {
    pub fn new(props: NavbarProps, children: String) -> Self {
        Self { props, children }
    }

    pub fn render(&self) -> String {
        let mut html = String::from("<nav");

        if let Some(id) = &self.props.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }

        let mut classes = String::from("navbar navbar-expand-lg navbar-light bg-light");
        if let Some(class_name) = &self.props.class_name {
            classes.push_str(&format!(" {}", escape_html(class_name)));
        }
        if let Some(fixed) = &self.props.fixed {
            classes.push_str(&format!(" fixed-{}", escape_html(fixed)));
        }
        html.push_str(&format!(" class=\"{}\"", classes));

        html.push('>');

        if let Some(brand) = &self.props.brand {
            html.push_str(&format!(
                "<a class=\"navbar-brand\" href=\"{}\">{}</a>",
                escape_html(self.props.brand_href.as_deref().unwrap_or("#")),
                escape_html(brand)
            ));
        }

        html.push_str("<button class=\"navbar-toggler\" type=\"button\" data-toggle=\"collapse\" data-target=\"#navbarSupportedContent\" aria-controls=\"navbarSupportedContent\" aria-expanded=\"false\" aria-label=\"Toggle navigation\">");
        html.push_str("<span class=\"navbar-toggler-icon\"></span>");
        html.push_str("</button>");

        html.push_str("<div class=\"collapse navbar-collapse\" id=\"navbarSupportedContent\">");
        html.push_str(&escape_html(&self.children));
        html.push_str("</div>");

        html.push_str("</nav>");

        html
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NavProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub variant: Option<String>,
    pub theme: Option<ThemeName>,
}

#[derive(Debug, Clone)]
pub struct Nav {
    props: NavProps,
    children: String,
}

impl Nav {
    pub fn new(props: NavProps, children: String) -> Self {
        Self { props, children }
    }

    pub fn render(&self) -> String {
        let mut html = String::from("<ul");

        if let Some(id) = &self.props.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }

        let mut classes = String::from("navbar-nav");
        if let Some(class_name) = &self.props.class_name {
            classes.push_str(&format!(" {}", escape_html(class_name)));
        }
        if let Some(variant) = &self.props.variant {
            classes.push_str(&format!(" nav-{}", escape_html(variant)));
        }
        html.push_str(&format!(" class=\"{}\"", classes));

        html.push('>');
        html.push_str(&escape_html(&self.children));
        html.push_str("</ul>");

        html
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NavItemProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub active: bool,
    pub disabled: bool,
    pub theme: Option<ThemeName>,
}

#[derive(Debug, Clone)]
pub struct NavItem {
    props: NavItemProps,
    children: String,
}

impl NavItem {
    pub fn new(props: NavItemProps, children: String) -> Self {
        Self { props, children }
    }

    pub fn render(&self) -> String {
        let mut html = String::from("<li");

        if let Some(id) = &self.props.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }

        let mut classes = String::from("nav-item");
        if self.props.active {
            classes.push_str(" active");
        }
        if self.props.disabled {
            classes.push_str(" disabled");
        }
        if let Some(class_name) = &self.props.class_name {
            classes.push_str(&format!(" {}", escape_html(class_name)));
        }
        html.push_str(&format!(" class=\"{}\"", classes));

        html.push('>');
        html.push_str(&escape_html(&self.children));
        html.push_str("</li>");

        html
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavLinkProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub href: String,
    pub disabled: bool,
    pub theme: Option<ThemeName>,
}

impl Default for NavLinkProps {
    fn default() -> Self {
        Self {
            id: None,
            class_name: None,
            href: String::from("#"),
            disabled: false,
            theme: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NavLink {
    props: NavLinkProps,
    children: String,
}

impl NavLink {
    pub fn new(props: NavLinkProps, children: String) -> Self {
        Self { props, children }
    }

    pub fn render(&self) -> String {
        let mut html = String::from("<a");

        if let Some(id) = &self.props.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }

        let mut classes = String::from("nav-link");
        if self.props.disabled {
            classes.push_str(" disabled");
        }
        if let Some(class_name) = &self.props.class_name {
            classes.push_str(&format!(" {}", escape_html(class_name)));
        }
        html.push_str(&format!(" class=\"{}\"", classes));

        html.push_str(&format!(" href=\"{}\"", escape_html(&self.props.href)));
        if self.props.disabled {
            html.push_str(" tabindex=\"-1\" aria-disabled=\"true\"");
        }

        html.push('>');
        html.push_str(&escape_html(&self.children));
        html.push_str("</a>");

        html
    }
}
