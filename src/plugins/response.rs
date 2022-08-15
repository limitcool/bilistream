// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Welcome {
    #[serde(rename = "responseContext")]
    pub response_context: ResponseContext,

    #[serde(rename = "contents")]
    pub contents: Contents,

    #[serde(rename = "header")]
    pub header: Header,

    #[serde(rename = "metadata")]
    pub metadata: Metadata,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "topbar")]
    pub topbar: Topbar,

    #[serde(rename = "microformat")]
    pub microformat: Microformat,
}

#[derive(Serialize, Deserialize)]
pub struct Contents {
    #[serde(rename = "twoColumnBrowseResultsRenderer")]
    pub two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct TwoColumnBrowseResultsRenderer {
    #[serde(rename = "tabs")]
    pub tabs: Vec<Tab>,
}

#[derive(Serialize, Deserialize)]
pub struct Tab {
    #[serde(rename = "tabRenderer")]
    pub tab_renderer: Option<TabRenderer>,

    #[serde(rename = "expandableTabRenderer")]
    pub expandable_tab_renderer: Option<ExpandableTabRenderer>,
}

#[derive(Serialize, Deserialize)]
pub struct ExpandableTabRenderer {
    #[serde(rename = "endpoint")]
    pub endpoint: NextEndpointClass,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "selected")]
    pub selected: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NextEndpointClass {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: EndpointCommandMetadata,

    #[serde(rename = "browseEndpoint")]
    pub browse_endpoint: NextEndpointBrowseEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct NextEndpointBrowseEndpoint {
    #[serde(rename = "browseId")]
    pub browse_id: Id,

    #[serde(rename = "params")]
    pub params: Option<String>,

    #[serde(rename = "canonicalBaseUrl")]
    pub canonical_base_url: Option<CanonicalBaseUrl>,
}

#[derive(Serialize, Deserialize)]
pub struct EndpointCommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    pub web_command_metadata: PurpleWebCommandMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleWebCommandMetadata {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "webPageType")]
    pub web_page_type: WebPageType,

    #[serde(rename = "rootVe")]
    pub root_ve: i64,

    #[serde(rename = "apiUrl")]
    pub api_url: Option<PurpleApiUrl>,
}

#[derive(Serialize, Deserialize)]
pub struct TabRenderer {
    #[serde(rename = "endpoint")]
    pub endpoint: NextEndpointClass,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "selected")]
    pub selected: Option<bool>,

    #[serde(rename = "content")]
    pub content: Option<TabRendererContent>,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct TabRendererContent {
    #[serde(rename = "sectionListRenderer")]
    pub section_list_renderer: SectionListRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct SectionListRenderer {
    #[serde(rename = "contents")]
    pub contents: Vec<SectionListRendererContent>,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "targetId")]
    pub target_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SectionListRendererContent {
    #[serde(rename = "itemSectionRenderer")]
    pub item_section_renderer: ItemSectionRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct ItemSectionRenderer {
    #[serde(rename = "contents")]
    pub contents: Vec<ItemSectionRendererContent>,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct ItemSectionRendererContent {
    #[serde(rename = "channelVideoPlayerRenderer")]
    pub channel_video_player_renderer: Option<ChannelVideoPlayerRenderer>,

    #[serde(rename = "shelfRenderer")]
    pub shelf_renderer: Option<ShelfRenderer>,

    #[serde(rename = "reelShelfRenderer")]
    pub reel_shelf_renderer: Option<ReelShelfRenderer>,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelVideoPlayerRenderer {
    #[serde(rename = "videoId")]
    pub video_id: String,

    #[serde(rename = "title")]
    pub title: PurpleTitle,

    #[serde(rename = "description")]
    pub description: Description,

    #[serde(rename = "viewCountText")]
    pub view_count_text: ContentClass,

    #[serde(rename = "publishedTimeText")]
    pub published_time_text: TextClass,

    #[serde(rename = "readMoreText")]
    pub read_more_text: ReadMoreText,
}

#[derive(Serialize, Deserialize)]
pub struct Description {
    #[serde(rename = "runs")]
    pub runs: Vec<DescriptionRun>,
}

#[derive(Serialize, Deserialize)]
pub struct DescriptionRun {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: Option<PrimaryLinkNavigationEndpoint>,
}

#[derive(Serialize, Deserialize)]
pub struct PrimaryLinkNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: TrackingParams,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: EndpointCommandMetadata,

    #[serde(rename = "watchEndpoint")]
    pub watch_endpoint: Option<PurpleWatchEndpoint>,

    #[serde(rename = "urlEndpoint")]
    pub url_endpoint: Option<UrlEndpoint>,
}

#[derive(Serialize, Deserialize)]
pub struct UrlEndpoint {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "target")]
    pub target: Target,

    #[serde(rename = "nofollow")]
    pub nofollow: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleWatchEndpoint {
    #[serde(rename = "videoId")]
    pub video_id: String,

    #[serde(rename = "startTimeSeconds")]
    pub start_time_seconds: i64,

    #[serde(rename = "watchEndpointSupportedOnesieConfig")]
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize)]
pub struct WatchEndpointSupportedOnesieConfig {
    #[serde(rename = "html5PlaybackOnesieConfig")]
    pub html5_playback_onesie_config: Html5PlaybackOnesieConfig,
}

#[derive(Serialize, Deserialize)]
pub struct Html5PlaybackOnesieConfig {
    #[serde(rename = "commonConfig")]
    pub common_config: CommonConfigElement,
}

#[derive(Serialize, Deserialize)]
pub struct CommonConfigElement {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct TextClass {
    #[serde(rename = "runs")]
    pub runs: Vec<TextRun>,
}

#[derive(Serialize, Deserialize)]
pub struct TextRun {
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReadMoreText {
    #[serde(rename = "runs")]
    pub runs: Vec<ReadMoreTextRun>,
}

#[derive(Serialize, Deserialize)]
pub struct ReadMoreTextRun {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: GridVideoRendererNavigationEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct GridVideoRendererNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: EndpointCommandMetadata,

    #[serde(rename = "watchEndpoint")]
    pub watch_endpoint: FluffyWatchEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyWatchEndpoint {
    #[serde(rename = "videoId")]
    pub video_id: String,

    #[serde(rename = "watchEndpointSupportedOnesieConfig")]
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleTitle {
    #[serde(rename = "runs")]
    pub runs: Vec<ReadMoreTextRun>,

    #[serde(rename = "accessibility")]
    pub accessibility: HotkeyAccessibilityLabelClass,
}

#[derive(Serialize, Deserialize)]
pub struct HotkeyAccessibilityLabelClass {
    #[serde(rename = "accessibilityData")]
    pub accessibility_data: AccessibilityAccessibilityData,
}

#[derive(Serialize, Deserialize)]
pub struct AccessibilityAccessibilityData {
    #[serde(rename = "label")]
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContentClass {
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReelShelfRenderer {
    #[serde(rename = "title")]
    pub title: TextClass,

    #[serde(rename = "items")]
    pub items: Vec<ReelShelfRendererItem>,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "icon")]
    pub icon: IconImage,
}

#[derive(Serialize, Deserialize)]
pub struct IconImage {
    #[serde(rename = "iconType")]
    pub icon_type: IconType,
}

#[derive(Serialize, Deserialize)]
pub struct ReelShelfRendererItem {
    #[serde(rename = "reelItemRenderer")]
    pub reel_item_renderer: ReelItemRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct ReelItemRenderer {
    #[serde(rename = "videoId")]
    pub video_id: String,

    #[serde(rename = "headline")]
    pub headline: ContentClass,

    #[serde(rename = "thumbnail")]
    pub thumbnail: ReelWatchEndpointThumbnail,

    #[serde(rename = "viewCountText")]
    pub view_count_text: SubscriberCountText,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: ReelItemRendererNavigationEndpoint,

    #[serde(rename = "menu")]
    pub menu: ReelItemRendererMenu,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "accessibility")]
    pub accessibility: HotkeyAccessibilityLabelClass,

    #[serde(rename = "style")]
    pub style: ReelItemRendererStyle,

    #[serde(rename = "videoType")]
    pub video_type: VideoType,

    #[serde(rename = "loggingDirectives")]
    pub logging_directives: LoggingDirectives,
}

#[derive(Serialize, Deserialize)]
pub struct LoggingDirectives {
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "visibility")]
    pub visibility: Visibility,

    #[serde(rename = "enableDisplayloggerExperiment")]
    pub enable_displaylogger_experiment: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Visibility {
    #[serde(rename = "types")]
    pub types: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReelItemRendererMenu {
    #[serde(rename = "menuRenderer")]
    pub menu_renderer: PurpleMenuRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleMenuRenderer {
    #[serde(rename = "items")]
    pub items: Vec<PurpleItem>,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "accessibility")]
    pub accessibility: HotkeyAccessibilityLabelClass,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleItem {
    #[serde(rename = "menuNavigationItemRenderer")]
    pub menu_navigation_item_renderer: MenuNavigationItemRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct MenuNavigationItemRenderer {
    #[serde(rename = "text")]
    pub text: TextClass,

    #[serde(rename = "icon")]
    pub icon: IconImage,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: MenuNavigationItemRendererNavigationEndpoint,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "accessibility")]
    pub accessibility: HotkeyAccessibilityLabelClass,
}

#[derive(Serialize, Deserialize)]
pub struct MenuNavigationItemRendererNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: PurpleCommandMetadata,

    #[serde(rename = "userFeedbackEndpoint")]
    pub user_feedback_endpoint: UserFeedbackEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleCommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    pub web_command_metadata: FluffyWebCommandMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyWebCommandMetadata {
    #[serde(rename = "ignoreNavigation")]
    pub ignore_navigation: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UserFeedbackEndpoint {
    #[serde(rename = "additionalDatas")]
    pub additional_datas: Vec<AdditionalData>,
}

#[derive(Serialize, Deserialize)]
pub struct AdditionalData {
    #[serde(rename = "userFeedbackEndpointProductSpecificValueData")]
    pub user_feedback_endpoint_product_specific_value_data: Param,
}

#[derive(Serialize, Deserialize)]
pub struct Param {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReelItemRendererNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: EndpointCommandMetadata,

    #[serde(rename = "reelWatchEndpoint")]
    pub reel_watch_endpoint: ReelWatchEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct ReelWatchEndpoint {
    #[serde(rename = "videoId")]
    pub video_id: String,

    #[serde(rename = "playerParams")]
    pub player_params: PlayerParams,

    #[serde(rename = "thumbnail")]
    pub thumbnail: ReelWatchEndpointThumbnail,

    #[serde(rename = "overlay")]
    pub overlay: Overlay,

    #[serde(rename = "params")]
    pub params: ReelWatchEndpointParams,

    #[serde(rename = "sequenceProvider")]
    pub sequence_provider: SequenceProvider,

    #[serde(rename = "sequenceParams")]
    pub sequence_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct Overlay {
    #[serde(rename = "reelPlayerOverlayRenderer")]
    pub reel_player_overlay_renderer: ReelPlayerOverlayRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct ReelPlayerOverlayRenderer {
    #[serde(rename = "reelPlayerHeaderSupportedRenderers")]
    pub reel_player_header_supported_renderers: ReelPlayerHeaderSupportedRenderers,

    #[serde(rename = "nextItemButton")]
    pub next_item_button: ItemButton,

    #[serde(rename = "prevItemButton")]
    pub prev_item_button: ItemButton,

    #[serde(rename = "style")]
    pub style: ReelPlayerOverlayRendererStyle,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct ItemButton {
    #[serde(rename = "buttonRenderer")]
    pub button_renderer: NextItemButtonButtonRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct NextItemButtonButtonRenderer {
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReelPlayerHeaderSupportedRenderers {
    #[serde(rename = "reelPlayerHeaderRenderer")]
    pub reel_player_header_renderer: ReelPlayerHeaderRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct ReelPlayerHeaderRenderer {
    #[serde(rename = "reelTitleText")]
    pub reel_title_text: ReelTitleText,

    #[serde(rename = "timestampText")]
    pub timestamp_text: ContentClass,

    #[serde(rename = "channelNavigationEndpoint")]
    pub channel_navigation_endpoint: NavigationEndpoint,

    #[serde(rename = "channelTitleText")]
    pub channel_title_text: EText,

    #[serde(rename = "channelThumbnail")]
    pub channel_thumbnail: Avatar,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "accessibility")]
    pub accessibility: HotkeyAccessibilityLabelClass,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: EndpointCommandMetadata,

    #[serde(rename = "browseEndpoint")]
    pub browse_endpoint: ChannelNavigationEndpointBrowseEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelNavigationEndpointBrowseEndpoint {
    #[serde(rename = "browseId")]
    pub browse_id: Id,

    #[serde(rename = "canonicalBaseUrl")]
    pub canonical_base_url: CanonicalBaseUrl,
}

#[derive(Serialize, Deserialize)]
pub struct Avatar {
    #[serde(rename = "thumbnails")]
    pub thumbnails: Vec<AvatarThumbnail>,
}

#[derive(Serialize, Deserialize)]
pub struct AvatarThumbnail {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "width")]
    pub width: i64,

    #[serde(rename = "height")]
    pub height: i64,
}

#[derive(Serialize, Deserialize)]
pub struct EText {
    #[serde(rename = "runs")]
    pub runs: Vec<ShortBylineTextRun>,
}

#[derive(Serialize, Deserialize)]
pub struct ShortBylineTextRun {
    #[serde(rename = "text")]
    pub text: TitleEnum,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: NavigationEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct ReelTitleText {
    #[serde(rename = "runs")]
    pub runs: Vec<ReelTitleTextRun>,
}

#[derive(Serialize, Deserialize)]
pub struct ReelTitleTextRun {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: Option<PurpleNavigationEndpoint>,

    #[serde(rename = "loggingDirectives")]
    pub logging_directives: Option<LoggingDirectives>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReelWatchEndpointThumbnail {
    #[serde(rename = "thumbnails")]
    pub thumbnails: Vec<AvatarThumbnail>,

    #[serde(rename = "isOriginalAspectRatio")]
    pub is_original_aspect_ratio: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SubscriberCountText {
    #[serde(rename = "accessibility")]
    pub accessibility: HotkeyAccessibilityLabelClass,

    #[serde(rename = "simpleText")]
    pub simple_text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShelfRenderer {
    #[serde(rename = "title")]
    pub title: TentacledTitle,

    #[serde(rename = "endpoint")]
    pub endpoint: NextEndpointClass,

    #[serde(rename = "content")]
    pub content: ShelfRendererContent,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "playAllButton")]
    pub play_all_button: Option<PlayAllButton>,

    #[serde(rename = "subtitle")]
    pub subtitle: Option<ContentClass>,
}

#[derive(Serialize, Deserialize)]
pub struct ShelfRendererContent {
    #[serde(rename = "horizontalListRenderer")]
    pub horizontal_list_renderer: HorizontalListRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct HorizontalListRenderer {
    #[serde(rename = "items")]
    pub items: Vec<HorizontalListRendererItem>,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "visibleItemCount")]
    pub visible_item_count: i64,

    #[serde(rename = "nextButton")]
    pub next_button: VoiceSearchButtonClass,

    #[serde(rename = "previousButton")]
    pub previous_button: VoiceSearchButtonClass,
}

#[derive(Serialize, Deserialize)]
pub struct HorizontalListRendererItem {
    #[serde(rename = "gridVideoRenderer")]
    pub grid_video_renderer: Option<GridVideoRenderer>,

    #[serde(rename = "gridPlaylistRenderer")]
    pub grid_playlist_renderer: Option<GridPlaylistRenderer>,
}

#[derive(Serialize, Deserialize)]
pub struct GridPlaylistRenderer {
    #[serde(rename = "playlistId")]
    pub playlist_id: String,

    #[serde(rename = "thumbnail")]
    pub thumbnail: Avatar,

    #[serde(rename = "title")]
    pub title: FluffyTitle,

    #[serde(rename = "videoCountText")]
    pub video_count_text: TextClass,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: GridPlaylistRendererNavigationEndpoint,

    #[serde(rename = "videoCountShortText")]
    pub video_count_short_text: ContentClass,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "sidebarThumbnails")]
    pub sidebar_thumbnails: Vec<Avatar>,

    #[serde(rename = "thumbnailText")]
    pub thumbnail_text: ThumbnailText,

    #[serde(rename = "ownerBadges")]
    pub owner_badges: Vec<OwnerBadgeElement>,

    #[serde(rename = "thumbnailRenderer")]
    pub thumbnail_renderer: ThumbnailRenderer,

    #[serde(rename = "thumbnailOverlays")]
    pub thumbnail_overlays: Vec<GridPlaylistRendererThumbnailOverlay>,

    #[serde(rename = "viewPlaylistText")]
    pub view_playlist_text: ViewPlaylistText,

    #[serde(rename = "publishedTimeText")]
    pub published_time_text: Option<ContentClass>,
}

#[derive(Serialize, Deserialize)]
pub struct GridPlaylistRendererNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: EndpointCommandMetadata,

    #[serde(rename = "watchEndpoint")]
    pub watch_endpoint: TentacledWatchEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledWatchEndpoint {
    #[serde(rename = "videoId")]
    pub video_id: String,

    #[serde(rename = "playlistId")]
    pub playlist_id: String,

    #[serde(rename = "params")]
    pub params: Option<WatchEndpointParams>,

    #[serde(rename = "loggingContext")]
    pub logging_context: LoggingContext,

    #[serde(rename = "watchEndpointSupportedOnesieConfig")]
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig,
}

#[derive(Serialize, Deserialize)]
pub struct LoggingContext {
    #[serde(rename = "vssLoggingContext")]
    pub vss_logging_context: VssLoggingContext,
}

#[derive(Serialize, Deserialize)]
pub struct VssLoggingContext {
    #[serde(rename = "serializedContextData")]
    pub serialized_context_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct OwnerBadgeElement {
    #[serde(rename = "metadataBadgeRenderer")]
    pub metadata_badge_renderer: OwnerBadgeMetadataBadgeRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct OwnerBadgeMetadataBadgeRenderer {
    #[serde(rename = "icon")]
    pub icon: IconImage,

    #[serde(rename = "style")]
    pub style: MetadataBadgeRendererStyle,

    #[serde(rename = "tooltip")]
    pub tooltip: Tooltip,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "accessibilityData")]
    pub accessibility_data: AccessibilityAccessibilityData,
}

#[derive(Serialize, Deserialize)]
pub struct GridPlaylistRendererThumbnailOverlay {
    #[serde(rename = "thumbnailOverlaySidePanelRenderer")]
    pub thumbnail_overlay_side_panel_renderer: Option<ThumbnailOverlaySidePanelRenderer>,

    #[serde(rename = "thumbnailOverlayHoverTextRenderer")]
    pub thumbnail_overlay_hover_text_renderer: Option<ThumbnailOverlayHoverTextRenderer>,

    #[serde(rename = "thumbnailOverlayNowPlayingRenderer")]
    pub thumbnail_overlay_now_playing_renderer: Option<ThumbnailOverlayNowPlayingRenderer>,
}

#[derive(Serialize, Deserialize)]
pub struct ThumbnailOverlayHoverTextRenderer {
    #[serde(rename = "text")]
    pub text: TextClass,

    #[serde(rename = "icon")]
    pub icon: IconImage,
}

#[derive(Serialize, Deserialize)]
pub struct ThumbnailOverlayNowPlayingRenderer {
    #[serde(rename = "text")]
    pub text: TextClass,
}

#[derive(Serialize, Deserialize)]
pub struct ThumbnailOverlaySidePanelRenderer {
    #[serde(rename = "text")]
    pub text: ContentClass,

    #[serde(rename = "icon")]
    pub icon: IconImage,
}

#[derive(Serialize, Deserialize)]
pub struct ThumbnailRenderer {
    #[serde(rename = "playlistVideoThumbnailRenderer")]
    pub playlist_video_thumbnail_renderer: PlaylistVideoThumbnailRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct PlaylistVideoThumbnailRenderer {
    #[serde(rename = "thumbnail")]
    pub thumbnail: Avatar,
}

#[derive(Serialize, Deserialize)]
pub struct ThumbnailText {
    #[serde(rename = "runs")]
    pub runs: Vec<ThumbnailTextRun>,
}

#[derive(Serialize, Deserialize)]
pub struct ThumbnailTextRun {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "bold")]
    pub bold: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyTitle {
    #[serde(rename = "runs")]
    pub runs: Vec<PurpleRun>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleRun {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: GridPlaylistRendererNavigationEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct ViewPlaylistText {
    #[serde(rename = "runs")]
    pub runs: Vec<ViewPlaylistTextRun>,
}

#[derive(Serialize, Deserialize)]
pub struct ViewPlaylistTextRun {
    #[serde(rename = "text")]
    pub text: TextEnum,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: TopbarLogoRendererEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct TopbarLogoRendererEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: EndpointCommandMetadata,

    #[serde(rename = "browseEndpoint")]
    pub browse_endpoint: PurpleBrowseEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleBrowseEndpoint {
    #[serde(rename = "browseId")]
    pub browse_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GridVideoRenderer {
    #[serde(rename = "videoId")]
    pub video_id: String,

    #[serde(rename = "thumbnail")]
    pub thumbnail: Avatar,

    #[serde(rename = "title")]
    pub title: SubscriberCountText,

    #[serde(rename = "viewCountText")]
    pub view_count_text: Option<ViewCountText>,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: GridVideoRendererNavigationEndpoint,

    #[serde(rename = "upcomingEventData")]
    pub upcoming_event_data: Option<UpcomingEventData>,

    #[serde(rename = "ownerBadges")]
    pub owner_badges: Vec<OwnerBadgeElement>,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "shortViewCountText")]
    pub short_view_count_text: Option<ShortViewCountText>,

    #[serde(rename = "menu")]
    pub menu: GridVideoRendererMenu,

    #[serde(rename = "thumbnailOverlays")]
    pub thumbnail_overlays: Vec<GridVideoRendererThumbnailOverlay>,

    #[serde(rename = "publishedTimeText")]
    pub published_time_text: Option<ContentClass>,

    #[serde(rename = "badges")]
    pub badges: Option<Vec<PurpleBadge>>,

    #[serde(rename = "shortBylineText")]
    pub short_byline_text: Option<EText>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleBadge {
    #[serde(rename = "metadataBadgeRenderer")]
    pub metadata_badge_renderer: PurpleMetadataBadgeRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleMetadataBadgeRenderer {
    #[serde(rename = "style")]
    pub style: String,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "accessibilityData")]
    pub accessibility_data: AccessibilityAccessibilityData,
}

#[derive(Serialize, Deserialize)]
pub struct GridVideoRendererMenu {
    #[serde(rename = "menuRenderer")]
    pub menu_renderer: FluffyMenuRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyMenuRenderer {
    #[serde(rename = "items")]
    pub items: Vec<FluffyItem>,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "accessibility")]
    pub accessibility: HotkeyAccessibilityLabelClass,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyItem {
    #[serde(rename = "menuServiceItemRenderer")]
    pub menu_service_item_renderer: MenuServiceItemRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct MenuServiceItemRenderer {
    #[serde(rename = "text")]
    pub text: TextClass,

    #[serde(rename = "icon")]
    pub icon: IconImage,

    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: MenuServiceItemRendererServiceEndpoint,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct MenuServiceItemRendererServiceEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: ServiceEndpointCommandMetadata,

    #[serde(rename = "signalServiceEndpoint")]
    pub signal_service_endpoint: UntoggledServiceEndpointSignalServiceEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceEndpointCommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    pub web_command_metadata: TentacledWebCommandMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledWebCommandMetadata {
    #[serde(rename = "sendPost")]
    pub send_post: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UntoggledServiceEndpointSignalServiceEndpoint {
    #[serde(rename = "signal")]
    pub signal: Signal,

    #[serde(rename = "actions")]
    pub actions: Vec<PurpleAction>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleAction {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "addToPlaylistCommand")]
    pub add_to_playlist_command: AddToPlaylistCommand,
}

#[derive(Serialize, Deserialize)]
pub struct AddToPlaylistCommand {
    #[serde(rename = "openMiniplayer")]
    pub open_miniplayer: bool,

    #[serde(rename = "videoId")]
    pub video_id: String,

    #[serde(rename = "listType")]
    pub list_type: ListType,

    #[serde(rename = "onCreateListCommand")]
    pub on_create_list_command: OnCreateListCommand,

    #[serde(rename = "videoIds")]
    pub video_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OnCreateListCommand {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: OnCreateListCommandCommandMetadata,

    #[serde(rename = "createPlaylistServiceEndpoint")]
    pub create_playlist_service_endpoint: CreatePlaylistServiceEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct OnCreateListCommandCommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    pub web_command_metadata: StickyWebCommandMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct StickyWebCommandMetadata {
    #[serde(rename = "sendPost")]
    pub send_post: bool,

    #[serde(rename = "apiUrl")]
    pub api_url: Option<FluffyApiUrl>,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePlaylistServiceEndpoint {
    #[serde(rename = "videoIds")]
    pub video_ids: Vec<String>,

    #[serde(rename = "params")]
    pub params: CreatePlaylistServiceEndpointParams,
}

#[derive(Serialize, Deserialize)]
pub struct ShortViewCountText {
    #[serde(rename = "runs")]
    pub runs: Option<Vec<TextRun>>,

    #[serde(rename = "accessibility")]
    pub accessibility: Option<HotkeyAccessibilityLabelClass>,

    #[serde(rename = "simpleText")]
    pub simple_text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GridVideoRendererThumbnailOverlay {
    #[serde(rename = "thumbnailOverlayTimeStatusRenderer")]
    pub thumbnail_overlay_time_status_renderer: Option<ThumbnailOverlayTimeStatusRenderer>,

    #[serde(rename = "thumbnailOverlayToggleButtonRenderer")]
    pub thumbnail_overlay_toggle_button_renderer: Option<ThumbnailOverlayToggleButtonRenderer>,

    #[serde(rename = "thumbnailOverlayNowPlayingRenderer")]
    pub thumbnail_overlay_now_playing_renderer: Option<ThumbnailOverlayNowPlayingRenderer>,
}

#[derive(Serialize, Deserialize)]
pub struct ThumbnailOverlayTimeStatusRenderer {
    #[serde(rename = "text")]
    pub text: SubscriberCountText,

    #[serde(rename = "style")]
    pub style: ThumbnailOverlayTimeStatusRendererStyle,
}

#[derive(Serialize, Deserialize)]
pub struct ThumbnailOverlayToggleButtonRenderer {
    #[serde(rename = "isToggled")]
    pub is_toggled: Option<bool>,

    #[serde(rename = "untoggledIcon")]
    pub untoggled_icon: IconImage,

    #[serde(rename = "toggledIcon")]
    pub toggled_icon: IconImage,

    #[serde(rename = "untoggledTooltip")]
    pub untoggled_tooltip: UntoggledTooltip,

    #[serde(rename = "toggledTooltip")]
    pub toggled_tooltip: ToggledTooltip,

    #[serde(rename = "untoggledServiceEndpoint")]
    pub untoggled_service_endpoint: UntoggledServiceEndpoint,

    #[serde(rename = "toggledServiceEndpoint")]
    pub toggled_service_endpoint: Option<ToggledServiceEndpoint>,

    #[serde(rename = "untoggledAccessibility")]
    pub untoggled_accessibility: HotkeyAccessibilityLabelClass,

    #[serde(rename = "toggledAccessibility")]
    pub toggled_accessibility: HotkeyAccessibilityLabelClass,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct ToggledServiceEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: OnCreateListCommandCommandMetadata,

    #[serde(rename = "playlistEditEndpoint")]
    pub playlist_edit_endpoint: ToggledServiceEndpointPlaylistEditEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct ToggledServiceEndpointPlaylistEditEndpoint {
    #[serde(rename = "playlistId")]
    pub playlist_id: PlaylistId,

    #[serde(rename = "actions")]
    pub actions: Vec<FluffyAction>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyAction {
    #[serde(rename = "action")]
    pub action: HilariousAction,

    #[serde(rename = "removedVideoId")]
    pub removed_video_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UntoggledServiceEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: OnCreateListCommandCommandMetadata,

    #[serde(rename = "playlistEditEndpoint")]
    pub playlist_edit_endpoint: Option<UntoggledServiceEndpointPlaylistEditEndpoint>,

    #[serde(rename = "signalServiceEndpoint")]
    pub signal_service_endpoint: Option<UntoggledServiceEndpointSignalServiceEndpoint>,
}

#[derive(Serialize, Deserialize)]
pub struct UntoggledServiceEndpointPlaylistEditEndpoint {
    #[serde(rename = "playlistId")]
    pub playlist_id: PlaylistId,

    #[serde(rename = "actions")]
    pub actions: Vec<TentacledAction>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledAction {
    #[serde(rename = "addedVideoId")]
    pub added_video_id: String,

    #[serde(rename = "action")]
    pub action: AmbitiousAction,
}

#[derive(Serialize, Deserialize)]
pub struct UpcomingEventData {
    #[serde(rename = "startTime")]
    pub start_time: String,

    #[serde(rename = "isReminderSet")]
    pub is_reminder_set: bool,

    #[serde(rename = "upcomingEventText")]
    pub upcoming_event_text: TextClass,
}

#[derive(Serialize, Deserialize)]
pub struct ViewCountText {
    #[serde(rename = "runs")]
    pub runs: Option<Vec<TextRun>>,

    #[serde(rename = "simpleText")]
    pub simple_text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VoiceSearchDialogRenderer {
    #[serde(rename = "placeholderHeader")]
    pub placeholder_header: TextClass,

    #[serde(rename = "promptHeader")]
    pub prompt_header: TextClass,

    #[serde(rename = "exampleQuery1")]
    pub example_query1: TextClass,

    #[serde(rename = "exampleQuery2")]
    pub example_query2: TextClass,

    #[serde(rename = "promptMicrophoneLabel")]
    pub prompt_microphone_label: TextClass,

    #[serde(rename = "loadingHeader")]
    pub loading_header: TextClass,

    #[serde(rename = "connectionErrorHeader")]
    pub connection_error_header: TextClass,

    #[serde(rename = "connectionErrorMicrophoneLabel")]
    pub connection_error_microphone_label: TextClass,

    #[serde(rename = "permissionsHeader")]
    pub permissions_header: TextClass,

    #[serde(rename = "permissionsSubtext")]
    pub permissions_subtext: TextClass,

    #[serde(rename = "disabledHeader")]
    pub disabled_header: TextClass,

    #[serde(rename = "disabledSubtext")]
    pub disabled_subtext: TextClass,

    #[serde(rename = "microphoneButtonAriaLabel")]
    pub microphone_button_aria_label: TextClass,

    #[serde(rename = "exitButton")]
    pub exit_button: VoiceSearchButtonClass,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "microphoneOffPromptHeader")]
    pub microphone_off_prompt_header: TextClass,
}

#[derive(Serialize, Deserialize)]
pub struct PurplePopup {
    #[serde(rename = "voiceSearchDialogRenderer")]
    pub voice_search_dialog_renderer: VoiceSearchDialogRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleOpenPopupAction {
    #[serde(rename = "popup")]
    pub popup: PurplePopup,

    #[serde(rename = "popupType")]
    pub popup_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct StickyAction {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "openPopupAction")]
    pub open_popup_action: PurpleOpenPopupAction,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSignalServiceEndpoint {
    #[serde(rename = "signal")]
    pub signal: Signal,

    #[serde(rename = "actions")]
    pub actions: Vec<StickyAction>,
}

#[derive(Serialize, Deserialize)]
pub struct ButtonRendererServiceEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: ServiceEndpointCommandMetadata,

    #[serde(rename = "signalServiceEndpoint")]
    pub signal_service_endpoint: PurpleSignalServiceEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct VoiceSearchButtonButtonRenderer {
    #[serde(rename = "style")]
    pub style: ButtonRendererStyle,

    #[serde(rename = "size")]
    pub size: Size,

    #[serde(rename = "isDisabled")]
    pub is_disabled: bool,

    #[serde(rename = "icon")]
    pub icon: IconImage,

    #[serde(rename = "accessibility")]
    pub accessibility: Option<AccessibilityAccessibilityData>,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "accessibilityData")]
    pub accessibility_data: Option<HotkeyAccessibilityLabelClass>,

    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: Option<ButtonRendererServiceEndpoint>,

    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VoiceSearchButtonClass {
    #[serde(rename = "buttonRenderer")]
    pub button_renderer: VoiceSearchButtonButtonRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct PlayAllButton {
    #[serde(rename = "buttonRenderer")]
    pub button_renderer: PlayAllButtonButtonRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct PlayAllButtonButtonRenderer {
    #[serde(rename = "style")]
    pub style: String,

    #[serde(rename = "size")]
    pub size: Size,

    #[serde(rename = "isDisabled")]
    pub is_disabled: bool,

    #[serde(rename = "text")]
    pub text: TextClass,

    #[serde(rename = "icon")]
    pub icon: IconImage,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: GridPlaylistRendererNavigationEndpoint,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledTitle {
    #[serde(rename = "runs")]
    pub runs: Vec<FluffyRun>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyRun {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: NextEndpointClass,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "c4TabbedHeaderRenderer")]
    pub c4_tabbed_header_renderer: C4TabbedHeaderRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct C4TabbedHeaderRenderer {
    #[serde(rename = "channelId")]
    pub channel_id: Id,

    #[serde(rename = "title")]
    pub title: TitleEnum,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: NavigationEndpoint,

    #[serde(rename = "avatar")]
    pub avatar: Avatar,

    #[serde(rename = "banner")]
    pub banner: Avatar,

    #[serde(rename = "badges")]
    pub badges: Vec<OwnerBadgeElement>,

    #[serde(rename = "headerLinks")]
    pub header_links: HeaderLinks,

    #[serde(rename = "subscribeButton")]
    pub subscribe_button: SubscribeButtonClass,

    #[serde(rename = "subscriberCountText")]
    pub subscriber_count_text: SubscriberCountText,

    #[serde(rename = "tvBanner")]
    pub tv_banner: Avatar,

    #[serde(rename = "mobileBanner")]
    pub mobile_banner: Avatar,

    #[serde(rename = "trackingParams")]
    pub tracking_params: TrackingParams,
}

#[derive(Serialize, Deserialize)]
pub struct HeaderLinks {
    #[serde(rename = "channelHeaderLinksRenderer")]
    pub channel_header_links_renderer: ChannelHeaderLinksRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelHeaderLinksRenderer {
    #[serde(rename = "primaryLinks")]
    pub primary_links: Vec<AryLink>,

    #[serde(rename = "secondaryLinks")]
    pub secondary_links: Vec<AryLink>,
}

#[derive(Serialize, Deserialize)]
pub struct AryLink {
    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: PrimaryLinkNavigationEndpoint,

    #[serde(rename = "icon")]
    pub icon: Icon,

    #[serde(rename = "title")]
    pub title: ContentClass,
}

#[derive(Serialize, Deserialize)]
pub struct Icon {
    #[serde(rename = "thumbnails")]
    pub thumbnails: Vec<CommonConfigElement>,
}

#[derive(Serialize, Deserialize)]
pub struct SubscribeButtonClass {
    #[serde(rename = "buttonRenderer")]
    pub button_renderer: SubscribeButtonButtonRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct SubscribeButtonButtonRenderer {
    #[serde(rename = "style")]
    pub style: String,

    #[serde(rename = "size")]
    pub size: Size,

    #[serde(rename = "isDisabled")]
    pub is_disabled: bool,

    #[serde(rename = "text")]
    pub text: TextClass,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: Option<FluffyNavigationEndpoint>,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "command")]
    pub command: Option<Command>,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: ServiceEndpointCommandMetadata,

    #[serde(rename = "signalServiceEndpoint")]
    pub signal_service_endpoint: CommandSignalServiceEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct CommandSignalServiceEndpoint {
    #[serde(rename = "signal")]
    pub signal: Signal,

    #[serde(rename = "actions")]
    pub actions: Vec<IndigoAction>,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoAction {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "signalAction")]
    pub signal_action: SignalAction,
}

#[derive(Serialize, Deserialize)]
pub struct SignalAction {
    #[serde(rename = "signal")]
    pub signal: String,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: PurpleCommandMetadata,

    #[serde(rename = "modalEndpoint")]
    pub modal_endpoint: ModalEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct ModalEndpoint {
    #[serde(rename = "modal")]
    pub modal: Modal,
}

#[derive(Serialize, Deserialize)]
pub struct Modal {
    #[serde(rename = "modalWithTitleAndButtonRenderer")]
    pub modal_with_title_and_button_renderer: ModalWithTitleAndButtonRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct ModalWithTitleAndButtonRenderer {
    #[serde(rename = "title")]
    pub title: ContentClass,

    #[serde(rename = "content")]
    pub content: ContentClass,

    #[serde(rename = "button")]
    pub button: Button,
}

#[derive(Serialize, Deserialize)]
pub struct Button {
    #[serde(rename = "buttonRenderer")]
    pub button_renderer: ButtonButtonRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct ButtonButtonRenderer {
    #[serde(rename = "style")]
    pub style: String,

    #[serde(rename = "size")]
    pub size: Size,

    #[serde(rename = "isDisabled")]
    pub is_disabled: bool,

    #[serde(rename = "text")]
    pub text: ContentClass,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: TentacledNavigationEndpoint,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: EndpointCommandMetadata,

    #[serde(rename = "signInEndpoint")]
    pub sign_in_endpoint: PurpleSignInEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSignInEndpoint {
    #[serde(rename = "nextEndpoint")]
    pub next_endpoint: NextEndpointClass,

    #[serde(rename = "continueAction")]
    pub continue_action: String,

    #[serde(rename = "idamTag")]
    pub idam_tag: String,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "channelMetadataRenderer")]
    pub channel_metadata_renderer: ChannelMetadataRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelMetadataRenderer {
    #[serde(rename = "title")]
    pub title: TitleEnum,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "rssUrl")]
    pub rss_url: String,

    #[serde(rename = "externalId")]
    pub external_id: Id,

    #[serde(rename = "keywords")]
    pub keywords: String,

    #[serde(rename = "ownerUrls")]
    pub owner_urls: Vec<String>,

    #[serde(rename = "avatar")]
    pub avatar: Avatar,

    #[serde(rename = "channelUrl")]
    pub channel_url: String,

    #[serde(rename = "isFamilySafe")]
    pub is_family_safe: bool,

    #[serde(rename = "availableCountryCodes")]
    pub available_country_codes: Vec<String>,

    #[serde(rename = "androidDeepLink")]
    pub android_deep_link: String,

    #[serde(rename = "androidAppindexingLink")]
    pub android_appindexing_link: String,

    #[serde(rename = "iosAppindexingLink")]
    pub ios_appindexing_link: String,

    #[serde(rename = "vanityChannelUrl")]
    pub vanity_channel_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Microformat {
    #[serde(rename = "microformatDataRenderer")]
    pub microformat_data_renderer: MicroformatDataRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct MicroformatDataRenderer {
    #[serde(rename = "urlCanonical")]
    pub url_canonical: String,

    #[serde(rename = "title")]
    pub title: TitleEnum,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "thumbnail")]
    pub thumbnail: Avatar,

    #[serde(rename = "siteName")]
    pub site_name: String,

    #[serde(rename = "appName")]
    pub app_name: String,

    #[serde(rename = "androidPackage")]
    pub android_package: String,

    #[serde(rename = "iosAppStoreId")]
    pub ios_app_store_id: String,

    #[serde(rename = "iosAppArguments")]
    pub ios_app_arguments: String,

    #[serde(rename = "ogType")]
    pub og_type: String,

    #[serde(rename = "urlApplinksWeb")]
    pub url_applinks_web: String,

    #[serde(rename = "urlApplinksIos")]
    pub url_applinks_ios: String,

    #[serde(rename = "urlApplinksAndroid")]
    pub url_applinks_android: String,

    #[serde(rename = "urlTwitterIos")]
    pub url_twitter_ios: String,

    #[serde(rename = "urlTwitterAndroid")]
    pub url_twitter_android: String,

    #[serde(rename = "twitterCardType")]
    pub twitter_card_type: String,

    #[serde(rename = "twitterSiteHandle")]
    pub twitter_site_handle: String,

    #[serde(rename = "schemaDotOrgType")]
    pub schema_dot_org_type: String,

    #[serde(rename = "noindex")]
    pub noindex: bool,

    #[serde(rename = "unlisted")]
    pub unlisted: bool,

    #[serde(rename = "familySafe")]
    pub family_safe: bool,

    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "availableCountries")]
    pub available_countries: Vec<String>,

    #[serde(rename = "linkAlternates")]
    pub link_alternates: Vec<LinkAlternate>,
}

#[derive(Serialize, Deserialize)]
pub struct LinkAlternate {
    #[serde(rename = "hrefUrl")]
    pub href_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseContext {
    #[serde(rename = "serviceTrackingParams")]
    pub service_tracking_params: Vec<ServiceTrackingParam>,

    #[serde(rename = "maxAgeSeconds")]
    pub max_age_seconds: i64,

    #[serde(rename = "mainAppWebResponseContext")]
    pub main_app_web_response_context: MainAppWebResponseContext,

    #[serde(rename = "webResponseContextExtensionData")]
    pub web_response_context_extension_data: WebResponseContextExtensionData,
}

#[derive(Serialize, Deserialize)]
pub struct MainAppWebResponseContext {
    #[serde(rename = "loggedOut")]
    pub logged_out: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceTrackingParam {
    #[serde(rename = "service")]
    pub service: String,

    #[serde(rename = "params")]
    pub params: Vec<Param>,
}

#[derive(Serialize, Deserialize)]
pub struct WebResponseContextExtensionData {
    #[serde(rename = "ytConfigData")]
    pub yt_config_data: YtConfigData,

    #[serde(rename = "hasDecorated")]
    pub has_decorated: bool,
}

#[derive(Serialize, Deserialize)]
pub struct YtConfigData {
    #[serde(rename = "visitorData")]
    pub visitor_data: String,

    #[serde(rename = "rootVisualElementType")]
    pub root_visual_element_type: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Topbar {
    #[serde(rename = "desktopTopbarRenderer")]
    pub desktop_topbar_renderer: DesktopTopbarRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct DesktopTopbarRenderer {
    #[serde(rename = "logo")]
    pub logo: Logo,

    #[serde(rename = "searchbox")]
    pub searchbox: Searchbox,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "topbarButtons")]
    pub topbar_buttons: Vec<TopbarButton>,

    #[serde(rename = "hotkeyDialog")]
    pub hotkey_dialog: HotkeyDialog,

    #[serde(rename = "backButton")]
    pub back_button: BackButtonClass,

    #[serde(rename = "forwardButton")]
    pub forward_button: BackButtonClass,

    #[serde(rename = "a11ySkipNavigationButton")]
    pub a11_y_skip_navigation_button: SubscribeButtonClass,

    #[serde(rename = "voiceSearchButton")]
    pub voice_search_button: VoiceSearchButtonClass,
}

#[derive(Serialize, Deserialize)]
pub struct BackButtonClass {
    #[serde(rename = "buttonRenderer")]
    pub button_renderer: BackButtonButtonRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct BackButtonButtonRenderer {
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "command")]
    pub command: Command,
}

#[derive(Serialize, Deserialize)]
pub struct HotkeyDialog {
    #[serde(rename = "hotkeyDialogRenderer")]
    pub hotkey_dialog_renderer: HotkeyDialogRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct HotkeyDialogRenderer {
    #[serde(rename = "title")]
    pub title: TextClass,

    #[serde(rename = "sections")]
    pub sections: Vec<Section>,

    #[serde(rename = "dismissButton")]
    pub dismiss_button: SubscribeButtonClass,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Serialize, Deserialize)]
pub struct Section {
    #[serde(rename = "hotkeyDialogSectionRenderer")]
    pub hotkey_dialog_section_renderer: HotkeyDialogSectionRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct HotkeyDialogSectionRenderer {
    #[serde(rename = "title")]
    pub title: TextClass,

    #[serde(rename = "options")]
    pub options: Vec<Option>,
}

#[derive(Serialize, Deserialize)]
pub struct Option {
    #[serde(rename = "hotkeyDialogSectionOptionRenderer")]
    pub hotkey_dialog_section_option_renderer: HotkeyDialogSectionOptionRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct HotkeyDialogSectionOptionRenderer {
    #[serde(rename = "label")]
    pub label: TextClass,

    #[serde(rename = "hotkey")]
    pub hotkey: String,

    #[serde(rename = "hotkeyAccessibilityLabel")]
    pub hotkey_accessibility_label: Option<HotkeyAccessibilityLabelClass>,
}

#[derive(Serialize, Deserialize)]
pub struct Logo {
    #[serde(rename = "topbarLogoRenderer")]
    pub topbar_logo_renderer: TopbarLogoRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct TopbarLogoRenderer {
    #[serde(rename = "iconImage")]
    pub icon_image: IconImage,

    #[serde(rename = "tooltipText")]
    pub tooltip_text: TextClass,

    #[serde(rename = "endpoint")]
    pub endpoint: TopbarLogoRendererEndpoint,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "overrideEntityKey")]
    pub override_entity_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Searchbox {
    #[serde(rename = "fusionSearchboxRenderer")]
    pub fusion_searchbox_renderer: FusionSearchboxRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct FusionSearchboxRenderer {
    #[serde(rename = "icon")]
    pub icon: IconImage,

    #[serde(rename = "placeholderText")]
    pub placeholder_text: TextClass,

    #[serde(rename = "config")]
    pub config: Config,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "searchEndpoint")]
    pub search_endpoint: FusionSearchboxRendererSearchEndpoint,

    #[serde(rename = "clearButton")]
    pub clear_button: VoiceSearchButtonClass,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "webSearchboxConfig")]
    pub web_searchbox_config: WebSearchboxConfig,
}

#[derive(Serialize, Deserialize)]
pub struct WebSearchboxConfig {
    #[serde(rename = "requestLanguage")]
    pub request_language: String,

    #[serde(rename = "requestDomain")]
    pub request_domain: String,

    #[serde(rename = "hasOnscreenKeyboard")]
    pub has_onscreen_keyboard: bool,

    #[serde(rename = "focusSearchbox")]
    pub focus_searchbox: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FusionSearchboxRendererSearchEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: EndpointCommandMetadata,

    #[serde(rename = "searchEndpoint")]
    pub search_endpoint: SearchEndpointSearchEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct SearchEndpointSearchEndpoint {
    #[serde(rename = "query")]
    pub query: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopbarButton {
    #[serde(rename = "topbarMenuButtonRenderer")]
    pub topbar_menu_button_renderer: Option<TopbarMenuButtonRenderer>,

    #[serde(rename = "buttonRenderer")]
    pub button_renderer: Option<TopbarButtonButtonRenderer>,
}

#[derive(Serialize, Deserialize)]
pub struct TopbarButtonButtonRenderer {
    #[serde(rename = "style")]
    pub style: String,

    #[serde(rename = "size")]
    pub size: String,

    #[serde(rename = "text")]
    pub text: TextClass,

    #[serde(rename = "icon")]
    pub icon: IconImage,

    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: StickyNavigationEndpoint,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "targetId")]
    pub target_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct StickyNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: EndpointCommandMetadata,

    #[serde(rename = "signInEndpoint")]
    pub sign_in_endpoint: FluffySignInEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct FluffySignInEndpoint {
    #[serde(rename = "idamTag")]
    pub idam_tag: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopbarMenuButtonRenderer {
    #[serde(rename = "icon")]
    pub icon: IconImage,

    #[serde(rename = "menuRequest")]
    pub menu_request: MenuRequest,

    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "accessibility")]
    pub accessibility: HotkeyAccessibilityLabelClass,

    #[serde(rename = "tooltip")]
    pub tooltip: String,

    #[serde(rename = "style")]
    pub style: ButtonRendererStyle,
}

#[derive(Serialize, Deserialize)]
pub struct MenuRequest {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "commandMetadata")]
    pub command_metadata: OnCreateListCommandCommandMetadata,

    #[serde(rename = "signalServiceEndpoint")]
    pub signal_service_endpoint: MenuRequestSignalServiceEndpoint,
}

#[derive(Serialize, Deserialize)]
pub struct MenuRequestSignalServiceEndpoint {
    #[serde(rename = "signal")]
    pub signal: String,

    #[serde(rename = "actions")]
    pub actions: Vec<IndecentAction>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentAction {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,

    #[serde(rename = "openPopupAction")]
    pub open_popup_action: FluffyOpenPopupAction,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyOpenPopupAction {
    #[serde(rename = "popup")]
    pub popup: FluffyPopup,

    #[serde(rename = "popupType")]
    pub popup_type: String,

    #[serde(rename = "beReused")]
    pub be_reused: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyPopup {
    #[serde(rename = "multiPageMenuRenderer")]
    pub multi_page_menu_renderer: MultiPageMenuRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct MultiPageMenuRenderer {
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    #[serde(rename = "style")]
    pub style: String,

    #[serde(rename = "showLoadingSpinner")]
    pub show_loading_spinner: bool,
}

#[derive(Serialize, Deserialize)]
pub enum Id {
    #[serde(rename = "UC1zFJrfEKvCixhsjNSb1toQ")]
    Uc1ZFJrfEKvCixhsjNSb1ToQ,

    #[serde(rename = "VLPL5su7mgHJJj8n-bG40uSnU7mtdqhx5Err")]
    Vlpl5Su7MgHjJj8NBG40USnU7Mtdqhx5Err,

    #[serde(rename = "VLPL5su7mgHJJj_h6_uZ4rcw-efQQFjd3cWq")]
    Vlpl5Su7MgHjJjH6UZ4RcwEfQqFjd3CWq,

    #[serde(rename = "VLPL5su7mgHJJj_oyH9J7O5yAZeY4NsPMpUP")]
    Vlpl5Su7MgHjJjOyH9J7O5YAZeY4NsPMpUp,
}

#[derive(Serialize, Deserialize)]
pub enum CanonicalBaseUrl {
    #[serde(rename = "/channel/UC1zFJrfEKvCixhsjNSb1toQ")]
    ChannelUc1ZFJrfEKvCixhsjNSb1ToQ,
}

#[derive(Serialize, Deserialize)]
pub enum PurpleApiUrl {
    #[serde(rename = "/youtubei/v1/browse")]
    YoutubeiV1Browse,
}

#[derive(Serialize, Deserialize)]
pub enum WebPageType {
    #[serde(rename = "WEB_PAGE_TYPE_BROWSE")]
    WebPageTypeBrowse,

    #[serde(rename = "WEB_PAGE_TYPE_CHANNEL")]
    WebPageTypeChannel,

    #[serde(rename = "WEB_PAGE_TYPE_PLAYLIST")]
    WebPageTypePlaylist,

    #[serde(rename = "WEB_PAGE_TYPE_SEARCH")]
    WebPageTypeSearch,

    #[serde(rename = "WEB_PAGE_TYPE_SHORTS")]
    WebPageTypeShorts,

    #[serde(rename = "WEB_PAGE_TYPE_UNKNOWN")]
    WebPageTypeUnknown,

    #[serde(rename = "WEB_PAGE_TYPE_WATCH")]
    WebPageTypeWatch,
}

#[derive(Serialize, Deserialize)]
pub enum TrackingParams {
    #[serde(rename = "CBAQ8DsiEwiYyvSo6cf5AhUDSQ8CHe4QDoQ=")]
    Cbaq8DsiEwiYyvSo6Cf5AhUdsq8CHe4QDoQ,

    #[serde(rename = "CKUDELsvGAAiEwiYyvSo6cf5AhUDSQ8CHe4QDoQ=")]
    CkudeLsvGaAiEwiYyvSo6Cf5AhUdsq8CHe4QDoQ,
}

#[derive(Serialize, Deserialize)]
pub enum Target {
    #[serde(rename = "TARGET_NEW_WINDOW")]
    TargetNewWindow,
}

#[derive(Serialize, Deserialize)]
pub enum IconType {
    #[serde(rename = "ADD_TO_QUEUE_TAIL")]
    AddToQueueTail,

    #[serde(rename = "AVATAR_LOGGED_OUT")]
    AvatarLoggedOut,

    #[serde(rename = "CHECK")]
    Check,

    #[serde(rename = "CHECK_CIRCLE_THICK")]
    CheckCircleThick,

    #[serde(rename = "CHEVRON_LEFT")]
    ChevronLeft,

    #[serde(rename = "CHEVRON_RIGHT")]
    ChevronRight,

    #[serde(rename = "CLOSE")]
    Close,

    #[serde(rename = "FEEDBACK")]
    Feedback,

    #[serde(rename = "MICROPHONE_ON")]
    MicrophoneOn,

    #[serde(rename = "MORE_VERT")]
    MoreVert,

    #[serde(rename = "PLAY_ALL")]
    PlayAll,

    #[serde(rename = "PLAYLIST_ADD_CHECK")]
    PlaylistAddCheck,

    #[serde(rename = "PLAYLISTS")]
    Playlists,

    #[serde(rename = "SEARCH")]
    Search,

    #[serde(rename = "WATCH_LATER")]
    WatchLater,

    #[serde(rename = "YOUTUBE_LOGO")]
    YoutubeLogo,

    #[serde(rename = "YOUTUBE_SHORTS_BRAND_24")]
    YoutubeShortsBrand24,
}

#[derive(Serialize, Deserialize)]
pub enum TitleEnum {
    #[serde(rename = "シスター・クレア -SisterClaire-")]
    SisterClaire,
}

#[derive(Serialize, Deserialize)]
pub enum ReelPlayerOverlayRendererStyle {
    #[serde(rename = "REEL_PLAYER_OVERLAY_STYLE_SHORTS")]
    ReelPlayerOverlayStyleShorts,
}

#[derive(Serialize, Deserialize)]
pub enum ReelWatchEndpointParams {
    #[serde(rename = "CBEwAg%3D%3D")]
    CbEwAg3D3D,
}

#[derive(Serialize, Deserialize)]
pub enum PlayerParams {
    #[serde(rename = "8AEByAMkuAQR")]
    The8AeByAMkuAqr,
}

#[derive(Serialize, Deserialize)]
pub enum SequenceProvider {
    #[serde(rename = "REEL_WATCH_SEQUENCE_PROVIDER_RPC")]
    ReelWatchSequenceProviderRpc,
}

#[derive(Serialize, Deserialize)]
pub enum ReelItemRendererStyle {
    #[serde(rename = "REEL_ITEM_STYLE_AVATAR_CIRCLE")]
    ReelItemStyleAvatarCircle,
}

#[derive(Serialize, Deserialize)]
pub enum VideoType {
    #[serde(rename = "REEL_VIDEO_TYPE_VIDEO")]
    ReelVideoTypeVideo,
}

#[derive(Serialize, Deserialize)]
pub enum WatchEndpointParams {
    #[serde(rename = "OAI%3D")]
    Oai3D,
}

#[derive(Serialize, Deserialize)]
pub enum MetadataBadgeRendererStyle {
    #[serde(rename = "BADGE_STYLE_TYPE_VERIFIED")]
    BadgeStyleTypeVerified,
}

#[derive(Serialize, Deserialize)]
pub enum Tooltip {
    #[serde(rename = "Verified")]
    Verified,
}

#[derive(Serialize, Deserialize)]
pub enum TextEnum {
    #[serde(rename = "View full playlist")]
    ViewFullPlaylist,
}

#[derive(Serialize, Deserialize)]
pub enum ListType {
    #[serde(rename = "PLAYLIST_EDIT_LIST_TYPE_QUEUE")]
    PlaylistEditListTypeQueue,
}

#[derive(Serialize, Deserialize)]
pub enum FluffyApiUrl {
    #[serde(rename = "/youtubei/v1/account/account_menu")]
    YoutubeiV1AccountAccountMenu,

    #[serde(rename = "/youtubei/v1/browse/edit_playlist")]
    YoutubeiV1BrowseEditPlaylist,

    #[serde(rename = "/youtubei/v1/playlist/create")]
    YoutubeiV1PlaylistCreate,
}

#[derive(Serialize, Deserialize)]
pub enum CreatePlaylistServiceEndpointParams {
    #[serde(rename = "CAQ%3D")]
    Caq3D,
}

#[derive(Serialize, Deserialize)]
pub enum Signal {
    #[serde(rename = "CLIENT_SIGNAL")]
    ClientSignal,
}

#[derive(Serialize, Deserialize)]
pub enum ThumbnailOverlayTimeStatusRendererStyle {
    #[serde(rename = "DEFAULT")]
    Default,

    #[serde(rename = "UPCOMING")]
    Upcoming,
}

#[derive(Serialize, Deserialize)]
pub enum HilariousAction {
    #[serde(rename = "ACTION_REMOVE_VIDEO_BY_VIDEO_ID")]
    ActionRemoveVideoByVideoId,
}

#[derive(Serialize, Deserialize)]
pub enum PlaylistId {
    #[serde(rename = "WL")]
    Wl,
}

#[derive(Serialize, Deserialize)]
pub enum ToggledTooltip {
    #[serde(rename = "Added")]
    Added,
}

#[derive(Serialize, Deserialize)]
pub enum AmbitiousAction {
    #[serde(rename = "ACTION_ADD_VIDEO")]
    ActionAddVideo,
}

#[derive(Serialize, Deserialize)]
pub enum UntoggledTooltip {
    #[serde(rename = "Add to queue")]
    AddToQueue,

    #[serde(rename = "Watch later")]
    WatchLater,
}

#[derive(Serialize, Deserialize)]
pub enum Size {
    #[serde(rename = "SIZE_DEFAULT")]
    SizeDefault,
}

#[derive(Serialize, Deserialize)]
pub enum ButtonRendererStyle {
    #[serde(rename = "STYLE_DEFAULT")]
    StyleDefault,
}
