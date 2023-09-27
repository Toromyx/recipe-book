//! This module implements logic for tauri events.

use std::{fmt::Debug, sync::Mutex};

use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{dom_content_loaded::await_dom_content_loaded, window::get_window};

pub mod answer_channel;
pub mod channel;
pub mod question_channel;

#[derive(Debug, Clone, Serialize)]
struct IdQuestionPayload<T> {
    pub id: String,
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
struct IdAnswerPayload<T> {
    pub id: String,
    pub data: T,
}

/// Ask the frontend a [question](QuestionData) via a [question channel](question_channel) and await the [answer](AnswerData) via an [answer channel](answer_channel).
///
/// This function uses a channel to send the answer from the event listener back to the calling function.
/// It uses a [`Uuid`] to make sure only the answer to the specific question is returned.
/// The listener is removed after getting an answer.
///
/// # Panics
///
/// This function panics when the frontend does not return a payload or a payload that is no deserializable to the expected [`AnswerData`].
///
/// # Errors
///
/// This function errors when the question event can not be emitted to the frontend.
pub async fn ask<QuestionData, AnswerData>(
    question_channel: &'static str,
    answer_channel: &'static str,
    question_data: QuestionData,
) -> anyhow::Result<AnswerData>
where
    QuestionData: Debug + Clone + Serialize,
    AnswerData: Debug + Clone + Send + for<'de> Deserialize<'de> + 'static,
{
    await_dom_content_loaded();
    let id = Uuid::new_v4().to_string();
    let (tx, rx) = oneshot::channel();
    let id_2 = id.clone();
    let tx_mutex = Mutex::new(Some(tx));
    let event_handler = get_window().listen(answer_channel, move |event| {
        let id_payload: IdAnswerPayload<AnswerData> =
            serde_json::from_str(event.payload().unwrap()).unwrap();
        if id_payload.id != id_2 {
            return;
        }
        if let Some(tx) = tx_mutex.lock().unwrap().take() {
            tx.send(id_payload.data).unwrap();
        }
    });
    get_window().emit(
        question_channel,
        IdQuestionPayload {
            id: id.clone(),
            data: question_data,
        },
    )?;
    let answer_data = rx.await.unwrap();
    get_window().unlisten(event_handler);
    Ok(answer_data)
}
