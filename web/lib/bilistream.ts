export type NotificationChannel = "gotify" | "ntfy" | "both"

export type BilistreamConfig = {
  BiliLive: {
    SESSDATA: string
    bili_jct: string
    DedeUserID: string
    DedeUserID__ckMd5: string
    Room: number
    BiliRtmpUrl: string
    BiliRtmpKey: string
  }
  Twitch: {
    Room: string
  }
  Interval: number
  Youtube: {
    Room: string
    AccessToken: string
  }
  Platform: string
  YoutubePreviewLive: {
    ChannelId: string
  }
  FfmpegProxy?: string | null
  Gotify?: {
    Url: string
    Token: string
  } | null
  Ntfy?: {
    Url: string
    Topic: string
    Token?: string | null
    Username?: string | null
    Password?: string | null
    Priority?: string | null
    Tags?: string | null
  } | null
  Notification: {
    Enabled: boolean
    Channel: NotificationChannel
    GotifyEnabled: boolean
    NtfyEnabled: boolean
  }
  Cookies?: string | null
  WebConsole?: {
    Addr: string
    Username: string
    Password: string
  } | null
}

export type StatusSnapshot = {
  app_version: string
  github_url: string
  auth_enabled: boolean
  web_addr: string
  config_path: string
  config_loaded: boolean
  config_error: string | null
  enabled: boolean
  worker_state: string
  source_platform: string
  source_room: string
  source_live: boolean | null
  bilibili_live: boolean | null
  ffmpeg_running: boolean
  last_checked_at: number | null
  last_event: string | null
  last_error: string | null
  config_summary: {
    interval: number
    platform: string
    bilibili_room: number
    twitch_room: string
    youtube_room: string
    youtube_preview_channel: string
    notification_enabled: boolean
    notification_channel: string
    gotify_enabled: boolean
    ntfy_enabled: boolean
    has_proxy: boolean
    has_cookies: boolean
  }
  logs: Array<{
    timestamp: number
    level: string
    message: string
  }>
}

export function createDefaultConfig(): BilistreamConfig {
  return {
    BiliLive: {
      SESSDATA: "",
      bili_jct: "",
      DedeUserID: "",
      DedeUserID__ckMd5: "",
      Room: 0,
      BiliRtmpUrl: "",
      BiliRtmpKey: "",
    },
    Twitch: {
      Room: "",
    },
    Interval: 60,
    Youtube: {
      Room: "",
      AccessToken: "",
    },
    Platform: "Twitch",
    YoutubePreviewLive: {
      ChannelId: "",
    },
    FfmpegProxy: "",
    Gotify: {
      Url: "",
      Token: "",
    },
    Ntfy: {
      Url: "https://ntfy.sh",
      Topic: "",
      Token: "",
      Username: "",
      Password: "",
      Priority: "",
      Tags: "",
    },
    Notification: {
      Enabled: true,
      Channel: "gotify",
      GotifyEnabled: true,
      NtfyEnabled: false,
    },
    Cookies: "",
    WebConsole: {
      Addr: "0.0.0.0:9090",
      Username: "admin",
      Password: "",
    },
  }
}

export function normalizeConfig(
  input?: Partial<BilistreamConfig> | null,
): BilistreamConfig {
  const defaults = createDefaultConfig()
  const gotify: Partial<NonNullable<BilistreamConfig["Gotify"]>> =
    input?.Gotify ?? {}
  const ntfy: Partial<NonNullable<BilistreamConfig["Ntfy"]>> =
    input?.Ntfy ?? {}
  const webConsole: Partial<NonNullable<BilistreamConfig["WebConsole"]>> =
    input?.WebConsole ?? {}

  return {
    ...defaults,
    ...input,
    BiliLive: {
      ...defaults.BiliLive,
      ...(input?.BiliLive ?? {}),
    },
    Twitch: {
      ...defaults.Twitch,
      ...(input?.Twitch ?? {}),
    },
    Youtube: {
      ...defaults.Youtube,
      ...(input?.Youtube ?? {}),
    },
    YoutubePreviewLive: {
      ...defaults.YoutubePreviewLive,
      ...(input?.YoutubePreviewLive ?? {}),
    },
    Gotify: {
      Url: gotify.Url ?? defaults.Gotify?.Url ?? "",
      Token: gotify.Token ?? defaults.Gotify?.Token ?? "",
    },
    Ntfy: {
      Url: ntfy.Url ?? defaults.Ntfy?.Url ?? "https://ntfy.sh",
      Topic: ntfy.Topic ?? defaults.Ntfy?.Topic ?? "",
      Token: ntfy.Token ?? defaults.Ntfy?.Token ?? "",
      Username: ntfy.Username ?? defaults.Ntfy?.Username ?? "",
      Password: ntfy.Password ?? defaults.Ntfy?.Password ?? "",
      Priority: ntfy.Priority ?? defaults.Ntfy?.Priority ?? "",
      Tags: ntfy.Tags ?? defaults.Ntfy?.Tags ?? "",
    },
    Notification: {
      ...defaults.Notification,
      ...(input?.Notification ?? {}),
    },
    WebConsole: {
      Addr: webConsole.Addr ?? defaults.WebConsole?.Addr ?? "0.0.0.0:9090",
      Username: webConsole.Username ?? defaults.WebConsole?.Username ?? "admin",
      Password: webConsole.Password ?? defaults.WebConsole?.Password ?? "",
    },
  }
}
