//! Contains helper functions for constructing URLs to documentation pages.

/// Returns the URL to the Agent sandboxing documentation.
///
/// Pass `section` to deep-link to a specific section anchor on the page (for
/// example, `Some("installing-bubblewrap")`); pass `None` to link to the top of
/// the page.
pub fn sandboxing_docs(section: Option<&str>) -> String {
    match section {
        Some(section) => release_channel::docs_url(&format!("ai/sandboxing#{section}")),
        None => release_channel::docs_url("ai/sandboxing"),
    }
}

pub fn llm_provider_docs() -> String {
    release_channel::docs_url("ai/llm-providers")
}
