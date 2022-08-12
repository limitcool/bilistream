use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub response_context: ResponseContext,
    pub contents: Contents,
    pub header: Header,
    pub metadata: Metadata,
    pub tracking_params: String,
    pub topbar: Topbar,
    pub microformat: Microformat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContext {
    pub service_tracking_params: Vec<ServiceTrackingParam>,
    pub max_age_seconds: i64,
    pub main_app_web_response_context: MainAppWebResponseContext,
    pub web_response_context_extension_data: WebResponseContextExtensionData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceTrackingParam {
    pub service: String,
    pub params: Vec<Param>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainAppWebResponseContext {
    pub logged_out: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebResponseContextExtensionData {
    pub yt_config_data: YtConfigData,
    pub has_decorated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YtConfigData {
    pub visitor_data: String,
    pub root_visual_element_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnBrowseResultsRenderer {
    pub tabs: Vec<Tab>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub tab_renderer: TabRenderer,
    pub expandable_tab_renderer: ExpandableTabRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabRenderer {
    pub endpoint: Endpoint,
    pub title: String,
    pub selected: bool,
    pub content: Content,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata,
    pub browse_endpoint: BrowseEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata {
    pub web_command_metadata: WebCommandMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: String,
    pub params: String,
    pub canonical_base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub section_list_renderer: SectionListRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionListRenderer {
    pub contents: Vec<Content2>,
    pub tracking_params: String,
    pub target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content2 {
    pub item_section_renderer: ItemSectionRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionRenderer {
    pub contents: Vec<Content3>,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content3 {
    pub channel_video_player_renderer: ChannelVideoPlayerRenderer,
    pub shelf_renderer: ShelfRenderer,
    pub reel_shelf_renderer: ReelShelfRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideoPlayerRenderer {
    pub video_id: String,
    pub title: Title,
    pub description: Description,
    pub view_count_text: ViewCountText,
    pub published_time_text: PublishedTimeText,
    pub read_more_text: ReadMoreText,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub runs: Vec<Run>,
    pub accessibility: Accessibility,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata2,
    pub watch_endpoint: WatchEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata2 {
    pub web_command_metadata: WebCommandMetadata2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata2 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint {
    pub video_id: String,
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpointSupportedOnesieConfig {
    #[serde(rename = "html5PlaybackOnesieConfig")]
    pub html5playback_onesie_config: Html5PlaybackOnesieConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Html5PlaybackOnesieConfig {
    pub common_config: CommonConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonConfig {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility {
    pub accessibility_data: AccessibilityData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub runs: Vec<Run2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run2 {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint2 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata3,
    pub url_endpoint: UrlEndpoint,
    pub watch_endpoint: WatchEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata3 {
    pub web_command_metadata: WebCommandMetadata3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata3 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlEndpoint {
    pub url: String,
    pub target: String,
    pub nofollow: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint2 {
    pub video_id: String,
    pub start_time_seconds: i64,
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpointSupportedOnesieConfig2 {
    #[serde(rename = "html5PlaybackOnesieConfig")]
    pub html5playback_onesie_config: Html5PlaybackOnesieConfig2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Html5PlaybackOnesieConfig2 {
    pub common_config: CommonConfig2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonConfig2 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewCountText {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishedTimeText {
    pub runs: Vec<Run3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run3 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadMoreText {
    pub runs: Vec<Run4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run4 {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint3 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata4,
    pub watch_endpoint: WatchEndpoint3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata4 {
    pub web_command_metadata: WebCommandMetadata4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata4 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint3 {
    pub video_id: String,
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpointSupportedOnesieConfig3 {
    #[serde(rename = "html5PlaybackOnesieConfig")]
    pub html5playback_onesie_config: Html5PlaybackOnesieConfig3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Html5PlaybackOnesieConfig3 {
    pub common_config: CommonConfig3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonConfig3 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShelfRenderer {
    pub title: Title2,
    pub endpoint: Endpoint2,
    pub content: Content4,
    pub subtitle: Option<Subtitle>,
    pub tracking_params: String,
    pub play_all_button: Option<PlayAllButton>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title2 {
    pub runs: Vec<Run5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run5 {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint4 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata5,
    pub browse_endpoint: BrowseEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata5 {
    pub web_command_metadata: WebCommandMetadata5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata5 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint2 {
    pub browse_id: String,
    pub params: Option<String>,
    pub canonical_base_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint2 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata6,
    pub browse_endpoint: BrowseEndpoint3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata6 {
    pub web_command_metadata: WebCommandMetadata6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata6 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint3 {
    pub browse_id: String,
    pub params: Option<String>,
    pub canonical_base_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content4 {
    pub horizontal_list_renderer: HorizontalListRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalListRenderer {
    pub items: Vec<Item>,
    pub tracking_params: String,
    pub visible_item_count: i64,
    pub next_button: NextButton,
    pub previous_button: PreviousButton,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub grid_video_renderer: Option<GridVideoRenderer>,
    pub grid_playlist_renderer: Option<GridPlaylistRenderer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridVideoRenderer {
    pub video_id: String,
    pub thumbnail: Thumbnail,
    pub title: Title3,
    pub published_time_text: Option<PublishedTimeText2>,
    pub view_count_text: Option<ViewCountText2>,
    pub navigation_endpoint: NavigationEndpoint5,
    pub short_byline_text: Option<ShortBylineText>,
    pub owner_badges: Vec<OwnerBadge>,
    pub tracking_params: String,
    pub short_view_count_text: Option<ShortViewCountText>,
    pub menu: Menu,
    pub thumbnail_overlays: Vec<ThumbnailOverlay>,
    #[serde(default)]
    pub badges: Vec<Badge>,
    pub upcoming_event_data: Option<UpcomingEventData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub thumbnails: Vec<Thumbnail2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail2 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title3 {
    pub accessibility: Accessibility2,
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility2 {
    pub accessibility_data: AccessibilityData2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData2 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishedTimeText2 {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewCountText2 {
    pub simple_text: Option<String>,
    pub runs: Option<Vec<Run6>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run6 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint5 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata7,
    pub watch_endpoint: WatchEndpoint4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata7 {
    pub web_command_metadata: WebCommandMetadata7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata7 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint4 {
    pub video_id: String,
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpointSupportedOnesieConfig4 {
    #[serde(rename = "html5PlaybackOnesieConfig")]
    pub html5playback_onesie_config: Html5PlaybackOnesieConfig4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Html5PlaybackOnesieConfig4 {
    pub common_config: CommonConfig4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonConfig4 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortBylineText {
    pub runs: Vec<Run7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run7 {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint6 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata8,
    pub browse_endpoint: BrowseEndpoint4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata8 {
    pub web_command_metadata: WebCommandMetadata8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata8 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint4 {
    pub browse_id: String,
    pub canonical_base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerBadge {
    pub metadata_badge_renderer: MetadataBadgeRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataBadgeRenderer {
    pub icon: Icon,
    pub style: String,
    pub tooltip: String,
    pub tracking_params: String,
    pub accessibility_data: AccessibilityData3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData3 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortViewCountText {
    pub accessibility: Option<Accessibility3>,
    pub simple_text: Option<String>,
    pub runs: Option<Vec<Run8>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility3 {
    pub accessibility_data: AccessibilityData4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData4 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run8 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub menu_renderer: MenuRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuRenderer {
    pub items: Vec<Item2>,
    pub tracking_params: String,
    pub accessibility: Accessibility4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item2 {
    pub menu_service_item_renderer: MenuServiceItemRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuServiceItemRenderer {
    pub text: Text,
    pub icon: Icon2,
    pub service_endpoint: ServiceEndpoint,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub runs: Vec<Run9>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run9 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon2 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata9,
    pub signal_service_endpoint: SignalServiceEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata9 {
    pub web_command_metadata: WebCommandMetadata9,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata9 {
    pub send_post: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalServiceEndpoint {
    pub signal: String,
    pub actions: Vec<Action>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub click_tracking_params: String,
    pub add_to_playlist_command: AddToPlaylistCommand,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToPlaylistCommand {
    pub open_miniplayer: bool,
    pub video_id: String,
    pub list_type: String,
    pub on_create_list_command: OnCreateListCommand,
    pub video_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnCreateListCommand {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata10,
    pub create_playlist_service_endpoint: CreatePlaylistServiceEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata10 {
    pub web_command_metadata: WebCommandMetadata10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata10 {
    pub send_post: bool,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlaylistServiceEndpoint {
    pub video_ids: Vec<String>,
    pub params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility4 {
    pub accessibility_data: AccessibilityData5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData5 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlay {
    pub thumbnail_overlay_time_status_renderer: Option<ThumbnailOverlayTimeStatusRenderer>,
    pub thumbnail_overlay_toggle_button_renderer: Option<ThumbnailOverlayToggleButtonRenderer>,
    pub thumbnail_overlay_now_playing_renderer: Option<ThumbnailOverlayNowPlayingRenderer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayTimeStatusRenderer {
    pub text: Text2,
    pub style: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text2 {
    pub accessibility: Accessibility5,
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility5 {
    pub accessibility_data: AccessibilityData6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData6 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayToggleButtonRenderer {
    pub untoggled_icon: UntoggledIcon,
    pub toggled_icon: ToggledIcon,
    pub untoggled_tooltip: String,
    pub toggled_tooltip: String,
    pub untoggled_service_endpoint: UntoggledServiceEndpoint,
    pub untoggled_accessibility: UntoggledAccessibility,
    pub toggled_accessibility: ToggledAccessibility,
    pub tracking_params: String,
    pub is_toggled: Option<bool>,
    pub toggled_service_endpoint: Option<ToggledServiceEndpoint>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UntoggledIcon {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggledIcon {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UntoggledServiceEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata11,
    pub signal_service_endpoint: Option<SignalServiceEndpoint2>,
    pub playlist_edit_endpoint: Option<PlaylistEditEndpoint>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata11 {
    pub web_command_metadata: WebCommandMetadata11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata11 {
    pub send_post: bool,
    pub api_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalServiceEndpoint2 {
    pub signal: String,
    pub actions: Vec<Action2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action2 {
    pub click_tracking_params: String,
    pub add_to_playlist_command: AddToPlaylistCommand2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToPlaylistCommand2 {
    pub open_miniplayer: bool,
    pub video_id: String,
    pub list_type: String,
    pub on_create_list_command: OnCreateListCommand2,
    pub video_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnCreateListCommand2 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata12,
    pub create_playlist_service_endpoint: CreatePlaylistServiceEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata12 {
    pub web_command_metadata: WebCommandMetadata12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata12 {
    pub send_post: bool,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlaylistServiceEndpoint2 {
    pub video_ids: Vec<String>,
    pub params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEditEndpoint {
    pub playlist_id: String,
    pub actions: Vec<Action3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action3 {
    pub added_video_id: String,
    pub action: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UntoggledAccessibility {
    pub accessibility_data: AccessibilityData7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData7 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggledAccessibility {
    pub accessibility_data: AccessibilityData8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData8 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggledServiceEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata13,
    pub playlist_edit_endpoint: PlaylistEditEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata13 {
    pub web_command_metadata: WebCommandMetadata13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata13 {
    pub send_post: bool,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEditEndpoint2 {
    pub playlist_id: String,
    pub actions: Vec<Action4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action4 {
    pub action: String,
    pub removed_video_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayNowPlayingRenderer {
    pub text: Text3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text3 {
    pub runs: Vec<Run10>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run10 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub metadata_badge_renderer: MetadataBadgeRenderer2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataBadgeRenderer2 {
    pub style: String,
    pub label: String,
    pub tracking_params: String,
    pub accessibility_data: AccessibilityData9,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData9 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpcomingEventData {
    pub start_time: String,
    pub is_reminder_set: bool,
    pub upcoming_event_text: UpcomingEventText,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpcomingEventText {
    pub runs: Vec<Run11>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run11 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridPlaylistRenderer {
    pub playlist_id: String,
    pub thumbnail: Thumbnail3,
    pub title: Title4,
    pub video_count_text: VideoCountText,
    pub navigation_endpoint: NavigationEndpoint8,
    pub video_count_short_text: VideoCountShortText,
    pub tracking_params: String,
    pub sidebar_thumbnails: Vec<SidebarThumbnail>,
    pub thumbnail_text: ThumbnailText,
    pub owner_badges: Vec<OwnerBadge2>,
    pub thumbnail_renderer: ThumbnailRenderer,
    pub thumbnail_overlays: Vec<ThumbnailOverlay2>,
    pub view_playlist_text: ViewPlaylistText,
    pub published_time_text: Option<PublishedTimeText3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail3 {
    pub thumbnails: Vec<Thumbnail4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail4 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title4 {
    pub runs: Vec<Run12>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run12 {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint7 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata14,
    pub watch_endpoint: WatchEndpoint5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata14 {
    pub web_command_metadata: WebCommandMetadata14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata14 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint5 {
    pub video_id: String,
    pub playlist_id: String,
    pub params: String,
    pub logging_context: LoggingContext,
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingContext {
    pub vss_logging_context: VssLoggingContext,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VssLoggingContext {
    pub serialized_context_data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpointSupportedOnesieConfig5 {
    #[serde(rename = "html5PlaybackOnesieConfig")]
    pub html5playback_onesie_config: Html5PlaybackOnesieConfig5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Html5PlaybackOnesieConfig5 {
    pub common_config: CommonConfig5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonConfig5 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoCountText {
    pub runs: Vec<Run13>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run13 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint8 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata15,
    pub watch_endpoint: WatchEndpoint6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata15 {
    pub web_command_metadata: WebCommandMetadata15,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata15 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint6 {
    pub video_id: String,
    pub playlist_id: String,
    pub params: String,
    pub logging_context: LoggingContext2,
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingContext2 {
    pub vss_logging_context: VssLoggingContext2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VssLoggingContext2 {
    pub serialized_context_data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpointSupportedOnesieConfig6 {
    #[serde(rename = "html5PlaybackOnesieConfig")]
    pub html5playback_onesie_config: Html5PlaybackOnesieConfig6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Html5PlaybackOnesieConfig6 {
    pub common_config: CommonConfig6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonConfig6 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoCountShortText {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarThumbnail {
    pub thumbnails: Vec<Thumbnail5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail5 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailText {
    pub runs: Vec<Run14>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run14 {
    pub text: String,
    pub bold: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerBadge2 {
    pub metadata_badge_renderer: MetadataBadgeRenderer3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataBadgeRenderer3 {
    pub icon: Icon3,
    pub style: String,
    pub tooltip: String,
    pub tracking_params: String,
    pub accessibility_data: AccessibilityData10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon3 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData10 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailRenderer {
    pub playlist_video_thumbnail_renderer: PlaylistVideoThumbnailRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideoThumbnailRenderer {
    pub thumbnail: Thumbnail6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail6 {
    pub thumbnails: Vec<Thumbnail7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail7 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlay2 {
    pub thumbnail_overlay_side_panel_renderer: Option<ThumbnailOverlaySidePanelRenderer>,
    pub thumbnail_overlay_hover_text_renderer: Option<ThumbnailOverlayHoverTextRenderer>,
    pub thumbnail_overlay_now_playing_renderer: Option<ThumbnailOverlayNowPlayingRenderer2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlaySidePanelRenderer {
    pub text: Text4,
    pub icon: Icon4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text4 {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon4 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayHoverTextRenderer {
    pub text: Text5,
    pub icon: Icon5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text5 {
    pub runs: Vec<Run15>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run15 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon5 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayNowPlayingRenderer2 {
    pub text: Text6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text6 {
    pub runs: Vec<Run16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run16 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewPlaylistText {
    pub runs: Vec<Run17>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run17 {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint9,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint9 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata16,
    pub browse_endpoint: BrowseEndpoint5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata16 {
    pub web_command_metadata: WebCommandMetadata16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata16 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint5 {
    pub browse_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishedTimeText3 {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextButton {
    pub button_renderer: ButtonRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub icon: Icon6,
    pub accessibility: Accessibility6,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon6 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility6 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousButton {
    pub button_renderer: ButtonRenderer2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer2 {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub icon: Icon7,
    pub accessibility: Accessibility7,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon7 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility7 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtitle {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayAllButton {
    pub button_renderer: ButtonRenderer3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer3 {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub text: Text7,
    pub icon: Icon8,
    pub navigation_endpoint: NavigationEndpoint10,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text7 {
    pub runs: Vec<Run18>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run18 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon8 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint10 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata17,
    pub watch_endpoint: WatchEndpoint7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata17 {
    pub web_command_metadata: WebCommandMetadata17,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata17 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint7 {
    pub video_id: String,
    pub playlist_id: String,
    pub logging_context: LoggingContext3,
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingContext3 {
    pub vss_logging_context: VssLoggingContext3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VssLoggingContext3 {
    pub serialized_context_data: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpointSupportedOnesieConfig7 {
    #[serde(rename = "html5PlaybackOnesieConfig")]
    pub html5playback_onesie_config: Html5PlaybackOnesieConfig7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Html5PlaybackOnesieConfig7 {
    pub common_config: CommonConfig7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonConfig7 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelShelfRenderer {
    pub title: Title5,
    pub items: Vec<Item3>,
    pub tracking_params: String,
    pub icon: Icon10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title5 {
    pub runs: Vec<Run19>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run19 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item3 {
    pub reel_item_renderer: ReelItemRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelItemRenderer {
    pub video_id: String,
    pub headline: Headline,
    pub thumbnail: Thumbnail8,
    pub view_count_text: ViewCountText3,
    pub navigation_endpoint: NavigationEndpoint11,
    pub menu: Menu2,
    pub tracking_params: String,
    pub accessibility: Accessibility12,
    pub style: String,
    pub video_type: String,
    pub logging_directives: LoggingDirectives2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail8 {
    pub thumbnails: Vec<Thumbnail9>,
    pub is_original_aspect_ratio: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail9 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewCountText3 {
    pub accessibility: Accessibility8,
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility8 {
    pub accessibility_data: AccessibilityData11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData11 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint11 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata18,
    pub reel_watch_endpoint: ReelWatchEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata18 {
    pub web_command_metadata: WebCommandMetadata18,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata18 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelWatchEndpoint {
    pub video_id: String,
    pub player_params: String,
    pub thumbnail: Thumbnail10,
    pub overlay: Overlay,
    pub params: String,
    pub sequence_provider: String,
    pub sequence_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail10 {
    pub thumbnails: Vec<Thumbnail11>,
    pub is_original_aspect_ratio: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail11 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overlay {
    pub reel_player_overlay_renderer: ReelPlayerOverlayRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelPlayerOverlayRenderer {
    pub reel_player_header_supported_renderers: ReelPlayerHeaderSupportedRenderers,
    pub next_item_button: NextItemButton,
    pub prev_item_button: PrevItemButton,
    pub style: String,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelPlayerHeaderSupportedRenderers {
    pub reel_player_header_renderer: ReelPlayerHeaderRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelPlayerHeaderRenderer {
    pub reel_title_text: ReelTitleText,
    pub timestamp_text: TimestampText,
    pub channel_navigation_endpoint: ChannelNavigationEndpoint,
    pub channel_title_text: ChannelTitleText,
    pub channel_thumbnail: ChannelThumbnail,
    pub tracking_params: String,
    pub accessibility: Accessibility9,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelTitleText {
    pub runs: Vec<Run20>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run20 {
    pub text: String,
    pub navigation_endpoint: Option<NavigationEndpoint12>,
    pub logging_directives: Option<LoggingDirectives>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint12 {
    pub click_tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingDirectives {
    pub tracking_params: String,
    pub visibility: Visibility,
    pub enable_displaylogger_experiment: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visibility {
    pub types: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimestampText {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelNavigationEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata19,
    pub browse_endpoint: BrowseEndpoint6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata19 {
    pub web_command_metadata: WebCommandMetadata19,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata19 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint6 {
    pub browse_id: String,
    pub canonical_base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelTitleText {
    pub runs: Vec<Run21>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run21 {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint13 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata20,
    pub browse_endpoint: BrowseEndpoint7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata20 {
    pub web_command_metadata: WebCommandMetadata20,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata20 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint7 {
    pub browse_id: String,
    pub canonical_base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelThumbnail {
    pub thumbnails: Vec<Thumbnail12>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail12 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility9 {
    pub accessibility_data: AccessibilityData12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData12 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextItemButton {
    pub button_renderer: ButtonRenderer4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer4 {
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrevItemButton {
    pub button_renderer: ButtonRenderer5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer5 {
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Menu2 {
    pub menu_renderer: MenuRenderer2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuRenderer2 {
    pub items: Vec<Item4>,
    pub tracking_params: String,
    pub accessibility: Accessibility11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item4 {
    pub menu_navigation_item_renderer: MenuNavigationItemRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuNavigationItemRenderer {
    pub text: Text8,
    pub icon: Icon9,
    pub navigation_endpoint: NavigationEndpoint14,
    pub tracking_params: String,
    pub accessibility: Accessibility10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text8 {
    pub runs: Vec<Run22>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run22 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon9 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint14 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata21,
    pub user_feedback_endpoint: UserFeedbackEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata21 {
    pub web_command_metadata: WebCommandMetadata21,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata21 {
    pub ignore_navigation: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFeedbackEndpoint {
    pub additional_datas: Vec<AdditionalData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalData {
    pub user_feedback_endpoint_product_specific_value_data:
        UserFeedbackEndpointProductSpecificValueData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFeedbackEndpointProductSpecificValueData {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility10 {
    pub accessibility_data: AccessibilityData13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData13 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility11 {
    pub accessibility_data: AccessibilityData14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData14 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility12 {
    pub accessibility_data: AccessibilityData15,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData15 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingDirectives2 {
    pub tracking_params: String,
    pub visibility: Visibility2,
    pub enable_displaylogger_experiment: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visibility2 {
    pub types: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon10 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandableTabRenderer {
    pub endpoint: Endpoint3,
    pub title: String,
    pub selected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint3 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata22,
    pub browse_endpoint: BrowseEndpoint8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata22 {
    pub web_command_metadata: WebCommandMetadata22,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata22 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint8 {
    pub browse_id: String,
    pub params: String,
    pub canonical_base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    #[serde(rename = "c4TabbedHeaderRenderer")]
    pub c4tabbed_header_renderer: C4TabbedHeaderRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct C4TabbedHeaderRenderer {
    pub channel_id: String,
    pub title: String,
    pub navigation_endpoint: NavigationEndpoint15,
    pub avatar: Avatar,
    pub banner: Banner,
    pub badges: Vec<Badge2>,
    pub header_links: HeaderLinks,
    pub subscribe_button: SubscribeButton,
    pub subscriber_count_text: SubscriberCountText,
    pub tv_banner: TvBanner,
    pub mobile_banner: MobileBanner,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint15 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata23,
    pub browse_endpoint: BrowseEndpoint9,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata23 {
    pub web_command_metadata: WebCommandMetadata23,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata23 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint9 {
    pub browse_id: String,
    pub canonical_base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub thumbnails: Vec<Thumbnail13>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail13 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub thumbnails: Vec<Thumbnail14>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail14 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Badge2 {
    pub metadata_badge_renderer: MetadataBadgeRenderer4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataBadgeRenderer4 {
    pub icon: Icon11,
    pub style: String,
    pub tooltip: String,
    pub tracking_params: String,
    pub accessibility_data: AccessibilityData16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon11 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData16 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderLinks {
    pub channel_header_links_renderer: ChannelHeaderLinksRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelHeaderLinksRenderer {
    pub primary_links: Vec<PrimaryLink>,
    pub secondary_links: Vec<SecondaryLink>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryLink {
    pub navigation_endpoint: NavigationEndpoint16,
    pub icon: Icon12,
    pub title: Title6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint16 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata24,
    pub url_endpoint: UrlEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata24 {
    pub web_command_metadata: WebCommandMetadata24,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata24 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlEndpoint2 {
    pub url: String,
    pub target: String,
    pub nofollow: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon12 {
    pub thumbnails: Vec<Thumbnail15>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail15 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title6 {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryLink {
    pub navigation_endpoint: NavigationEndpoint17,
    pub icon: Icon13,
    pub title: Title7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint17 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata25,
    pub url_endpoint: UrlEndpoint3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata25 {
    pub web_command_metadata: WebCommandMetadata25,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata25 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlEndpoint3 {
    pub url: String,
    pub target: String,
    pub nofollow: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon13 {
    pub thumbnails: Vec<Thumbnail16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail16 {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title7 {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeButton {
    pub button_renderer: ButtonRenderer6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer6 {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub text: Text9,
    pub navigation_endpoint: NavigationEndpoint18,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text9 {
    pub runs: Vec<Run23>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run23 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint18 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata26,
    pub modal_endpoint: ModalEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata26 {
    pub web_command_metadata: WebCommandMetadata26,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata26 {
    pub ignore_navigation: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModalEndpoint {
    pub modal: Modal,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modal {
    pub modal_with_title_and_button_renderer: ModalWithTitleAndButtonRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModalWithTitleAndButtonRenderer {
    pub title: Title8,
    pub content: Content5,
    pub button: Button,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title8 {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content5 {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Button {
    pub button_renderer: ButtonRenderer7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer7 {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub text: Text10,
    pub navigation_endpoint: NavigationEndpoint19,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text10 {
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint19 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata27,
    pub sign_in_endpoint: SignInEndpoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata27 {
    pub web_command_metadata: WebCommandMetadata27,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata27 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInEndpoint {
    pub next_endpoint: NextEndpoint,
    pub continue_action: String,
    pub idam_tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata28,
    pub browse_endpoint: BrowseEndpoint10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata28 {
    pub web_command_metadata: WebCommandMetadata28,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata28 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint10 {
    pub browse_id: String,
    pub params: String,
    pub canonical_base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriberCountText {
    pub accessibility: Accessibility13,
    pub simple_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility13 {
    pub accessibility_data: AccessibilityData17,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData17 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvBanner {
    pub thumbnails: Vec<Thumbnail17>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail17 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileBanner {
    pub thumbnails: Vec<Thumbnail18>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail18 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub channel_metadata_renderer: ChannelMetadataRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelMetadataRenderer {
    pub title: String,
    pub description: String,
    pub rss_url: String,
    pub external_id: String,
    pub keywords: String,
    pub owner_urls: Vec<String>,
    pub avatar: Avatar2,
    pub channel_url: String,
    pub is_family_safe: bool,
    pub available_country_codes: Vec<String>,
    pub android_deep_link: String,
    pub android_appindexing_link: String,
    pub ios_appindexing_link: String,
    pub vanity_channel_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar2 {
    pub thumbnails: Vec<Thumbnail19>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail19 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topbar {
    pub desktop_topbar_renderer: DesktopTopbarRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DesktopTopbarRenderer {
    pub logo: Logo,
    pub searchbox: Searchbox,
    pub tracking_params: String,
    pub topbar_buttons: Vec<TopbarButton>,
    pub hotkey_dialog: HotkeyDialog,
    pub back_button: BackButton,
    pub forward_button: ForwardButton,
    pub a11y_skip_navigation_button: A11ySkipNavigationButton,
    pub voice_search_button: VoiceSearchButton,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logo {
    pub topbar_logo_renderer: TopbarLogoRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopbarLogoRenderer {
    pub icon_image: IconImage,
    pub tooltip_text: TooltipText,
    pub endpoint: Endpoint4,
    pub tracking_params: String,
    pub override_entity_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconImage {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TooltipText {
    pub runs: Vec<Run24>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run24 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint4 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata29,
    pub browse_endpoint: BrowseEndpoint11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata29 {
    pub web_command_metadata: WebCommandMetadata29,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata29 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint11 {
    pub browse_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Searchbox {
    pub fusion_searchbox_renderer: FusionSearchboxRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FusionSearchboxRenderer {
    pub icon: Icon14,
    pub placeholder_text: PlaceholderText,
    pub config: Config,
    pub tracking_params: String,
    pub search_endpoint: SearchEndpoint,
    pub clear_button: ClearButton,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon14 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderText {
    pub runs: Vec<Run25>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run25 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub web_searchbox_config: WebSearchboxConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSearchboxConfig {
    pub request_language: String,
    pub request_domain: String,
    pub has_onscreen_keyboard: bool,
    pub focus_searchbox: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchEndpoint {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata30,
    pub search_endpoint: SearchEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata30 {
    pub web_command_metadata: WebCommandMetadata30,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata30 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchEndpoint2 {
    pub query: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearButton {
    pub button_renderer: ButtonRenderer8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer8 {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub icon: Icon15,
    pub tracking_params: String,
    pub accessibility_data: AccessibilityData18,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon15 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData18 {
    pub accessibility_data: AccessibilityData19,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData19 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopbarButton {
    pub topbar_menu_button_renderer: Option<TopbarMenuButtonRenderer>,
    pub button_renderer: Option<ButtonRenderer9>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopbarMenuButtonRenderer {
    pub icon: Icon16,
    pub menu_request: MenuRequest,
    pub tracking_params: String,
    pub accessibility: Accessibility14,
    pub tooltip: String,
    pub style: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon16 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuRequest {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata31,
    pub signal_service_endpoint: SignalServiceEndpoint3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata31 {
    pub web_command_metadata: WebCommandMetadata31,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata31 {
    pub send_post: bool,
    pub api_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalServiceEndpoint3 {
    pub signal: String,
    pub actions: Vec<Action5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action5 {
    pub click_tracking_params: String,
    pub open_popup_action: OpenPopupAction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPopupAction {
    pub popup: Popup,
    pub popup_type: String,
    pub be_reused: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Popup {
    pub multi_page_menu_renderer: MultiPageMenuRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiPageMenuRenderer {
    pub tracking_params: String,
    pub style: String,
    pub show_loading_spinner: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility14 {
    pub accessibility_data: AccessibilityData20,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData20 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer9 {
    pub style: String,
    pub size: String,
    pub text: Text11,
    pub icon: Icon17,
    pub navigation_endpoint: NavigationEndpoint20,
    pub tracking_params: String,
    pub target_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text11 {
    pub runs: Vec<Run26>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run26 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon17 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint20 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata32,
    pub sign_in_endpoint: SignInEndpoint2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata32 {
    pub web_command_metadata: WebCommandMetadata32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata32 {
    pub url: String,
    pub web_page_type: String,
    pub root_ve: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInEndpoint2 {
    pub idam_tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyDialog {
    pub hotkey_dialog_renderer: HotkeyDialogRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyDialogRenderer {
    pub title: Title9,
    pub sections: Vec<Section>,
    pub dismiss_button: DismissButton,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title9 {
    pub runs: Vec<Run27>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run27 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub hotkey_dialog_section_renderer: HotkeyDialogSectionRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyDialogSectionRenderer {
    pub title: Title10,
    pub options: Vec<Option>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title10 {
    pub runs: Vec<Run28>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run28 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Option {
    pub hotkey_dialog_section_option_renderer: HotkeyDialogSectionOptionRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyDialogSectionOptionRenderer {
    pub label: Label,
    pub hotkey: String,
    pub hotkey_accessibility_label: Option<HotkeyAccessibilityLabel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub runs: Vec<Run29>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run29 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyAccessibilityLabel {
    pub accessibility_data: AccessibilityData21,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData21 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DismissButton {
    pub button_renderer: ButtonRenderer10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer10 {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub text: Text12,
    pub tracking_params: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text12 {
    pub runs: Vec<Run30>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run30 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackButton {
    pub button_renderer: ButtonRenderer11,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer11 {
    pub tracking_params: String,
    pub command: Command,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata33,
    pub signal_service_endpoint: SignalServiceEndpoint4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata33 {
    pub web_command_metadata: WebCommandMetadata33,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata33 {
    pub send_post: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalServiceEndpoint4 {
    pub signal: String,
    pub actions: Vec<Action6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action6 {
    pub click_tracking_params: String,
    pub signal_action: SignalAction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalAction {
    pub signal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardButton {
    pub button_renderer: ButtonRenderer12,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer12 {
    pub tracking_params: String,
    pub command: Command2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command2 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata34,
    pub signal_service_endpoint: SignalServiceEndpoint5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata34 {
    pub web_command_metadata: WebCommandMetadata34,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata34 {
    pub send_post: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalServiceEndpoint5 {
    pub signal: String,
    pub actions: Vec<Action7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action7 {
    pub click_tracking_params: String,
    pub signal_action: SignalAction2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalAction2 {
    pub signal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A11ySkipNavigationButton {
    pub button_renderer: ButtonRenderer13,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer13 {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub text: Text13,
    pub tracking_params: String,
    pub command: Command3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text13 {
    pub runs: Vec<Run31>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run31 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command3 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata35,
    pub signal_service_endpoint: SignalServiceEndpoint6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata35 {
    pub web_command_metadata: WebCommandMetadata35,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata35 {
    pub send_post: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalServiceEndpoint6 {
    pub signal: String,
    pub actions: Vec<Action8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action8 {
    pub click_tracking_params: String,
    pub signal_action: SignalAction3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalAction3 {
    pub signal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceSearchButton {
    pub button_renderer: ButtonRenderer14,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer14 {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub service_endpoint: ServiceEndpoint2,
    pub icon: Icon19,
    pub tooltip: String,
    pub tracking_params: String,
    pub accessibility_data: AccessibilityData24,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceEndpoint2 {
    pub click_tracking_params: String,
    pub command_metadata: CommandMetadata36,
    pub signal_service_endpoint: SignalServiceEndpoint7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata36 {
    pub web_command_metadata: WebCommandMetadata36,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata36 {
    pub send_post: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalServiceEndpoint7 {
    pub signal: String,
    pub actions: Vec<Action9>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action9 {
    pub click_tracking_params: String,
    pub open_popup_action: OpenPopupAction2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPopupAction2 {
    pub popup: Popup2,
    pub popup_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Popup2 {
    pub voice_search_dialog_renderer: VoiceSearchDialogRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceSearchDialogRenderer {
    pub placeholder_header: PlaceholderHeader,
    pub prompt_header: PromptHeader,
    pub example_query1: ExampleQuery1,
    pub example_query2: ExampleQuery2,
    pub prompt_microphone_label: PromptMicrophoneLabel,
    pub loading_header: LoadingHeader,
    pub connection_error_header: ConnectionErrorHeader,
    pub connection_error_microphone_label: ConnectionErrorMicrophoneLabel,
    pub permissions_header: PermissionsHeader,
    pub permissions_subtext: PermissionsSubtext,
    pub disabled_header: DisabledHeader,
    pub disabled_subtext: DisabledSubtext,
    pub microphone_button_aria_label: MicrophoneButtonAriaLabel,
    pub exit_button: ExitButton,
    pub tracking_params: String,
    pub microphone_off_prompt_header: MicrophoneOffPromptHeader,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderHeader {
    pub runs: Vec<Run32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run32 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptHeader {
    pub runs: Vec<Run33>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run33 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleQuery1 {
    pub runs: Vec<Run34>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run34 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleQuery2 {
    pub runs: Vec<Run35>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run35 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptMicrophoneLabel {
    pub runs: Vec<Run36>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run36 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadingHeader {
    pub runs: Vec<Run37>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run37 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionErrorHeader {
    pub runs: Vec<Run38>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run38 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionErrorMicrophoneLabel {
    pub runs: Vec<Run39>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run39 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsHeader {
    pub runs: Vec<Run40>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run40 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsSubtext {
    pub runs: Vec<Run41>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run41 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisabledHeader {
    pub runs: Vec<Run42>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run42 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisabledSubtext {
    pub runs: Vec<Run43>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run43 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicrophoneButtonAriaLabel {
    pub runs: Vec<Run44>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run44 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExitButton {
    pub button_renderer: ButtonRenderer15,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer15 {
    pub style: String,
    pub size: String,
    pub is_disabled: bool,
    pub icon: Icon18,
    pub tracking_params: String,
    pub accessibility_data: AccessibilityData22,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon18 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData22 {
    pub accessibility_data: AccessibilityData23,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData23 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicrophoneOffPromptHeader {
    pub runs: Vec<Run45>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run45 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon19 {
    pub icon_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData24 {
    pub accessibility_data: AccessibilityData25,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData25 {
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Microformat {
    pub microformat_data_renderer: MicroformatDataRenderer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroformatDataRenderer {
    pub url_canonical: String,
    pub title: String,
    pub description: String,
    pub thumbnail: Thumbnail20,
    pub site_name: String,
    pub app_name: String,
    pub android_package: String,
    pub ios_app_store_id: String,
    pub ios_app_arguments: String,
    pub og_type: String,
    pub url_applinks_web: String,
    pub url_applinks_ios: String,
    pub url_applinks_android: String,
    pub url_twitter_ios: String,
    pub url_twitter_android: String,
    pub twitter_card_type: String,
    pub twitter_site_handle: String,
    pub schema_dot_org_type: String,
    pub noindex: bool,
    pub unlisted: bool,
    pub family_safe: bool,
    pub tags: Vec<String>,
    pub available_countries: Vec<String>,
    pub link_alternates: Vec<LinkAlternate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail20 {
    pub thumbnails: Vec<Thumbnail21>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail21 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkAlternate {
    pub href_url: String,
}
