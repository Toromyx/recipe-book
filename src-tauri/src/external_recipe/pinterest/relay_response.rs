use std::{fmt, fmt::Formatter};

use serde::{
    de::Visitor,
    Deserialize, Deserializer,
    __private::de::{ContentDeserializer, TaggedContentVisitor},
};

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayRequestParameters {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayImageSpec {
    pub width: u64,
    pub height: u64,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayStoryListBlockItem {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayStoryListBlock {
    pub heading: String,
    pub blocks: Vec<PinterestRelayStoryListBlockItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayStoryBasics {
    #[serde(rename = "listBlocks")]
    pub list_blocks: Vec<PinterestRelayStoryListBlock>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayStoryMetadata {
    #[serde(rename = "pinTitle")]
    pub pin_title: String,
    pub basics: PinterestRelayStoryBasics,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayVideo {
    pub thumbnail: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayVideoList {
    #[serde(rename = "vEXP3")]
    pub video: PinterestRelayVideo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayVideoData {
    #[serde(rename = "videoListEXP3")]
    pub video_list: PinterestRelayVideoList,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayStoryPageBlock {
    #[serde(rename = "videoData")]
    pub video_data: PinterestRelayVideoData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayStoryPage {
    pub blocks: Vec<PinterestRelayStoryPageBlock>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayStoryData {
    pub metadata: PinterestRelayStoryMetadata,
    pub pages: Vec<PinterestRelayStoryPage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayPinQueryDataUploaded {
    pub title: String,
    #[serde(rename = "storyPinData")]
    pub story_pin_data: PinterestRelayStoryData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayPinQueryDataExternal {
    pub link: String,
}

#[derive(Debug, Clone)]
pub enum PinterestRelayPinQueryData {
    Uploaded(PinterestRelayPinQueryDataUploaded),
    External(PinterestRelayPinQueryDataExternal),
}

impl<'de> Deserialize<'de> for PinterestRelayPinQueryData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[doc(hidden)]
        enum Field {
            Uploaded,
            External,
        }
        #[doc(hidden)]
        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field;
            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                Formatter::write_str(formatter, "a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "Uploaded by user" => Ok(Field::Uploaded),
                    _ => Ok(Field::External),
                }
            }
        }
        impl<'de> Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserializer::deserialize_identifier(deserializer, FieldVisitor)
            }
        }

        let (tag, content) = Deserializer::deserialize_any(
            deserializer,
            TaggedContentVisitor::<Field>::new(
                "domain",
                "enum PinterestRelayPinQueryData specified by field \"domain\"",
            ),
        )?;
        let deserializer = ContentDeserializer::<D::Error>::new(content);
        match tag {
            Field::Uploaded => Result::map(
                <PinterestRelayPinQueryDataUploaded as Deserialize>::deserialize(deserializer),
                PinterestRelayPinQueryData::Uploaded,
            ),
            Field::External => Result::map(
                <PinterestRelayPinQueryDataExternal as Deserialize>::deserialize(deserializer),
                PinterestRelayPinQueryData::External,
            ),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayPinQuery {
    pub data: PinterestRelayPinQueryData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayResponseData {
    #[serde(rename = "v3GetPinQuery")]
    pub pin_query: PinterestRelayPinQuery,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelayResponse {
    pub data: PinterestRelayResponseData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinterestRelay {
    #[serde(rename = "requestParameters")]
    pub request_parameters: PinterestRelayRequestParameters,
    pub response: PinterestRelayResponse,
}
