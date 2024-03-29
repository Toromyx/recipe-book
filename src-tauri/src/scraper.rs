//! This module implements HTML scraping via the webview frontend.

use std::fmt::Debug;

use anyhow::Result;
use async_trait::async_trait;

use crate::{
    event,
    event::{
        answer_channel::{
            SCRAPER_DOM_CREATE_ANSWER, SCRAPER_DOM_SELECT_ALL_ANSWER, SCRAPER_DOM_SELECT_ANSWER,
            SCRAPER_ELEMENT_GET_ATTRIBUTE_ANSWER, SCRAPER_ELEMENT_INNER_TEXT_ANSWER,
            SCRAPER_ELEMENT_SELECT_ALL_ANSWER, SCRAPER_ELEMENT_SELECT_ANSWER,
            SCRAPER_ELEMENT_TEXT_CONTENT_ANSWER,
        },
        channel::{SCRAPER_DOM_DROP, SCRAPER_ELEMENT_DROP},
        question_channel::{
            SCRAPER_DOM_CREATE_QUESTION, SCRAPER_DOM_SELECT_ALL_QUESTION,
            SCRAPER_DOM_SELECT_QUESTION, SCRAPER_ELEMENT_GET_ATTRIBUTE_QUESTION,
            SCRAPER_ELEMENT_INNER_TEXT_QUESTION, SCRAPER_ELEMENT_SELECT_ALL_QUESTION,
            SCRAPER_ELEMENT_SELECT_QUESTION, SCRAPER_ELEMENT_TEXT_CONTENT_QUESTION,
        },
    },
    window::get_window,
};

#[async_trait]
pub trait ParentNode {
    async fn select(&self, selector: &str) -> Result<Option<Element>>;

    async fn select_all(&self, selector: &str) -> Result<Vec<Element>>;
}

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

impl Dom {
    /// Create a DOM in the webview frontend.
    pub async fn create(html: String) -> Result<Self> {
        let dom_id =
            event::ask(SCRAPER_DOM_CREATE_QUESTION, SCRAPER_DOM_CREATE_ANSWER, html).await?;
        Ok(Dom { id: dom_id })
    }
}

#[async_trait]
impl ParentNode for Dom {
    async fn select(&self, selector: &str) -> Result<Option<Element>> {
        let element_id_option: Option<String> = event::ask(
            SCRAPER_DOM_SELECT_QUESTION,
            SCRAPER_DOM_SELECT_ANSWER,
            (&self.id, selector),
        )
        .await?;
        Ok(element_id_option.map(|element_id| Element { id: element_id }))
    }

    async fn select_all(&self, selector: &str) -> Result<Vec<Element>> {
        let element_ids: Vec<String> = event::ask(
            SCRAPER_DOM_SELECT_ALL_QUESTION,
            SCRAPER_DOM_SELECT_ALL_ANSWER,
            (&self.id, selector),
        )
        .await?;
        Ok(element_ids
            .into_iter()
            .map(|element_id| Element { id: element_id })
            .collect())
    }
}

/// This struct represents an element created in the webview frontend.
#[derive(Debug)]
pub struct Element {
    pub id: String,
}

impl Drop for Element {
    /// Delete the element in the webview frontend once it isn't needed anymore.
    fn drop(&mut self) {
        get_window().emit(SCRAPER_ELEMENT_DROP, &self.id).ok();
    }
}

impl Element {
    pub async fn text_content(&self) -> Result<String> {
        let text_content = event::ask(
            SCRAPER_ELEMENT_TEXT_CONTENT_QUESTION,
            SCRAPER_ELEMENT_TEXT_CONTENT_ANSWER,
            &self.id,
        )
        .await?;
        Ok(text_content)
    }
    pub async fn inner_text(&self) -> Result<String> {
        let inner_text = event::ask(
            SCRAPER_ELEMENT_INNER_TEXT_QUESTION,
            SCRAPER_ELEMENT_INNER_TEXT_ANSWER,
            &self.id,
        )
        .await?;
        Ok(inner_text)
    }

    pub async fn get_attribute(&self, qualified_name: &str) -> Result<String> {
        let attribute = event::ask(
            SCRAPER_ELEMENT_GET_ATTRIBUTE_QUESTION,
            SCRAPER_ELEMENT_GET_ATTRIBUTE_ANSWER,
            (&self.id, qualified_name),
        )
        .await?;
        Ok(attribute)
    }
}

#[async_trait]
impl ParentNode for Element {
    async fn select(&self, selector: &str) -> Result<Option<Element>> {
        let element_id_option: Option<String> = event::ask(
            SCRAPER_ELEMENT_SELECT_QUESTION,
            SCRAPER_ELEMENT_SELECT_ANSWER,
            (&self.id, selector),
        )
        .await?;
        Ok(element_id_option.map(|element_id| Element { id: element_id }))
    }

    async fn select_all(&self, selector: &str) -> Result<Vec<Element>> {
        let element_ids: Vec<String> = event::ask(
            SCRAPER_ELEMENT_SELECT_ALL_QUESTION,
            SCRAPER_ELEMENT_SELECT_ALL_ANSWER,
            (&self.id, selector),
        )
        .await?;
        Ok(element_ids
            .into_iter()
            .map(|element_id| Element { id: element_id })
            .collect())
    }
}
