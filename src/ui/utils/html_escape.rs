//! HTML escape utilities
//! Provides functions to escape HTML special characters to prevent XSS attacks

/// Escape HTML special characters to prevent XSS attacks
///
/// # Examples
/// ```
/// use ymaxum::ui::utils::html_escape::escape_html;
///
/// let unsafe_html = "<script>alert('XSS')</script>";
/// let safe_html = escape_html(unsafe_html);
/// assert_eq!(safe_html, "&lt;script&gt;alert('XSS')&lt;/script&gt;");
/// ```
pub fn escape_html(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        match c {
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '&' => result.push_str("&amp;"),
            '"' => result.push_str("&quot;"),
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        // Test basic HTML escaping
        let input = "<script>alert('XSS')</script>";
        let expected = "&lt;script&gt;alert('XSS')&lt;/script&gt;";
        assert_eq!(escape_html(input), expected);

        // Test all special characters
        let input = "<>&\"'";
        let expected = "&lt;&gt;&amp;&quot;'";
        assert_eq!(escape_html(input), expected);

        // Test normal text (no escaping needed)
        let input = "Hello, World!";
        let expected = "Hello, World!";
        assert_eq!(escape_html(input), expected);

        // Test empty string
        let input = "";
        let expected = "";
        assert_eq!(escape_html(input), expected);
    }
}
