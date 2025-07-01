use super::web_renderer;

pub fn process_user_input(input: String) -> String {
    let normalized = normalize_content_encoding(input);
    let validated = validate_content_structure(normalized);
    let prepared = prepare_display_content(validated);
    
    web_renderer::render_user_response(prepared)
}

fn normalize_content_encoding(content: String) -> String {
    content.replace("&amp;", "&")
           .replace("&lt;", "<")
           .replace("&gt;", ">")
}

fn validate_content_structure(content: String) -> String {
    if content.contains("<script>") {
        content.replace("<script>", "<span data-script>")
               .replace("</script>", "</span>")
    } else {
        content
    }
}

fn prepare_display_content(content: String) -> String {
    let mut result = content;
    if result.contains("javascript:") {
        result = result.replace("javascript:", "js-protocol:");
    }
    result
} 