//! This module implements HTML scraping via the webview frontend.

use std::fmt::Debug;

use anyhow::Result;

use crate::{
    event,
    event::{
        answer_channel::{SCRAPER_DOM_CANONICAL_LINK_ANSWER, SCRAPER_DOM_CREATE_ANSWER},
        channel::SCRAPER_DOM_DROP,
        question_channel::{SCRAPER_DOM_CANONICAL_LINK_QUESTION, SCRAPER_DOM_CREATE_QUESTION},
    },
    window::get_window,
};

/// This struct represents a DOM created in the webview frontend.
#[derive(Debug)]
pub struct Dom {
    pub id: String,
}

impl Drop for Dom {
    /// Delete the DOM in the webview frontend once it isn't needed anymore.
    fn drop(&mut self) {
        get_window().emit(SCRAPER_DOM_DROP, &self.id).ok();
    }
}

/// Create a DOM in the webview frontend.
pub async fn dom_create(html: String) -> Result<Dom> {
    let dom_id = event::ask(SCRAPER_DOM_CREATE_QUESTION, SCRAPER_DOM_CREATE_ANSWER, html).await?;
    Ok(Dom { id: dom_id })
}

/// Get the canonical link of a DOM.
pub async fn dom_canonical_link(dom_id: String) -> Result<String> {
    let canonical_link = event::ask(
        SCRAPER_DOM_CANONICAL_LINK_QUESTION,
        SCRAPER_DOM_CANONICAL_LINK_ANSWER,
        dom_id,
    )
    .await?;
    Ok(canonical_link)
}
