use serde::Deserialize;

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
pub struct PinterestRelayPinQueryData {
    pub title: String,
    #[serde(rename = "imageSpec_orig")]
    pub image_spec_orig: PinterestRelayImageSpec,
    #[serde(rename = "storyPinData")]
    pub story_pin_data: PinterestRelayStoryData,
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
