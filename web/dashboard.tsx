"use client"

import { startTransition, useEffect, useRef, useState } from "react"
import {
  Activity,
  BellRing,
  CheckCircle2,
  CircleOff,
  Cpu,
  ExternalLink,
  Github,
  HardDrive,
  Hexagon,
  KeyRound,
  LoaderCircle,
  Radio,
  RefreshCw,
  Save,
  Server,
  Settings2,
  Shield,
  Siren,
  SquareTerminal,
  Wifi,
} from "lucide-react"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Separator } from "@/components/ui/separator"
import { Switch } from "@/components/ui/switch"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Textarea } from "@/components/ui/textarea"
import {
  createDefaultConfig,
  normalizeConfig,
  type BilistreamConfig,
  type NotificationChannel,
  type StatusSnapshot,
} from "@/lib/bilistream"

const REFRESH_INTERVAL_MS = 5000

type PendingAction = "start" | "stop" | "reload" | "save" | null

type FlashState = {
  tone: "success" | "error" | "info"
  message: string
} | null

type DashboardTab = "overview" | "config" | "logs"

type DashboardSection =
  | "overview"
  | "source"
  | "notification"
  | "config"
  | "logs"
  | "deployment"

const SECTION_TO_TAB: Record<DashboardSection, DashboardTab> = {
  overview: "overview",
  source: "overview",
  notification: "config",
  config: "config",
  logs: "logs",
  deployment: "overview",
}

const SECTION_META: Record<
  DashboardSection,
  {
    title: string
    description: string
    badge: string
  }
> = {
  overview: {
    title: "运行总览",
    description: "集中查看 bilistream 当前状态、推流进度与关键运行信号。",
    badge: "总览",
  },
  source: {
    title: "直播源监控",
    description: "查看源站在线状态、房间目标与最近一次监测结果。",
    badge: "源站",
  },
  notification: {
    title: "通知策略",
    description: "管理总开关、主通道策略，以及 gotify / ntfy 的独立启停。",
    badge: "通知",
  },
  config: {
    title: "配置管理",
    description: "直接编辑 config.yaml 对应字段，保存后立即同步到后端运行配置。",
    badge: "配置",
  },
  logs: {
    title: "运行日志",
    description: "检查最近状态变更、错误记录与自动化动作输出。",
    badge: "日志",
  },
  deployment: {
    title: "部署信息",
    description: "面向 self-host 场景查看远程访问、鉴权状态与部署入口信息。",
    badge: "部署",
  },
}

export default function Dashboard() {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [currentTime, setCurrentTime] = useState(new Date())
  const [isBooting, setIsBooting] = useState(true)
  const [snapshot, setSnapshot] = useState<StatusSnapshot | null>(null)
  const [config, setConfig] = useState<BilistreamConfig>(createDefaultConfig())
  const [errorMessage, setErrorMessage] = useState<string | null>(null)
  const [pendingAction, setPendingAction] = useState<PendingAction>(null)
  const [flash, setFlash] = useState<FlashState>(null)
  const [activeSection, setActiveSection] = useState<DashboardSection>("overview")
  const [activeTab, setActiveTab] = useState<DashboardTab>("overview")

  useEffect(() => {
    const timer = window.setTimeout(() => {
      setIsBooting(false)
    }, 700)

    return () => window.clearTimeout(timer)
  }, [])

  useEffect(() => {
    const interval = window.setInterval(() => {
      setCurrentTime(new Date())
    }, 1000)

    return () => window.clearInterval(interval)
  }, [])

  useEffect(() => {
    let cancelled = false

    const load = async () => {
      try {
        const [statusResponse, configResponse] = await Promise.all([
          fetch("/api/status", { cache: "no-store" }),
          fetch("/api/config", { cache: "no-store" }),
        ])

        if (!statusResponse.ok) {
          throw new Error(`状态接口返回 ${statusResponse.status}`)
        }

        if (!configResponse.ok) {
          throw new Error(`配置接口返回 ${configResponse.status}`)
        }

        const [statusData, configData] = (await Promise.all([
          statusResponse.json(),
          configResponse.json(),
        ])) as [StatusSnapshot, BilistreamConfig]

        if (cancelled) {
          return
        }

        startTransition(() => {
          setSnapshot(statusData)
          setConfig(normalizeConfig(configData))
          setErrorMessage(null)
        })
      } catch (error) {
        if (cancelled) {
          return
        }

        const message =
          error instanceof Error ? error.message : "控制台暂时无法连接到后端接口"
        setErrorMessage(message)
      }
    }

    void load()
    const interval = window.setInterval(() => {
      void load()
    }, REFRESH_INTERVAL_MS)

    return () => {
      cancelled = true
      window.clearInterval(interval)
    }
  }, [])

  useEffect(() => {
    if (!flash) {
      return
    }

    const timer = window.setTimeout(() => {
      setFlash(null)
    }, 3200)

    return () => window.clearTimeout(timer)
  }, [flash])

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) {
      return
    }

    const context = canvas.getContext("2d")
    if (!context) {
      return
    }

    let animationFrameId = 0

    const resize = () => {
      canvas.width = canvas.offsetWidth
      canvas.height = canvas.offsetHeight
    }

    resize()

    const particles = Array.from({ length: 72 }, () => ({
      x: Math.random() * canvas.width,
      y: Math.random() * canvas.height,
      radius: Math.random() * 2.4 + 0.6,
      velocityX: (Math.random() - 0.5) * 0.24,
      velocityY: (Math.random() - 0.5) * 0.24,
      hue: 190 + Math.random() * 35,
      alpha: Math.random() * 0.35 + 0.08,
    }))

    const render = () => {
      context.clearRect(0, 0, canvas.width, canvas.height)

      for (const particle of particles) {
        particle.x += particle.velocityX
        particle.y += particle.velocityY

        if (particle.x < 0) particle.x = canvas.width
        if (particle.x > canvas.width) particle.x = 0
        if (particle.y < 0) particle.y = canvas.height
        if (particle.y > canvas.height) particle.y = 0

        context.beginPath()
        context.fillStyle = `hsla(${particle.hue}, 88%, 68%, ${particle.alpha})`
        context.arc(particle.x, particle.y, particle.radius, 0, Math.PI * 2)
        context.fill()
      }

      animationFrameId = window.requestAnimationFrame(render)
    }

    render()
    window.addEventListener("resize", resize)

    return () => {
      window.cancelAnimationFrame(animationFrameId)
      window.removeEventListener("resize", resize)
    }
  }, [])

  const runAction = async (action: Exclude<PendingAction, "save" | null>) => {
    setPendingAction(action)
    try {
      const response = await fetch(`/api/control/${action}`, {
        method: "POST",
      })

      if (!response.ok) {
        throw new Error(`${action} 请求失败，状态码 ${response.status}`)
      }

      setFlash({
        tone: "success",
        message:
          action === "start"
            ? "监控已启动"
            : action === "stop"
              ? "监控已停止"
              : "配置已从磁盘重载",
      })
      await refreshData()
    } catch (error) {
      setFlash({
        tone: "error",
        message:
          error instanceof Error ? error.message : "操作失败，请稍后重试",
      })
    } finally {
      setPendingAction(null)
    }
  }

  const refreshData = async () => {
    try {
      const [statusResponse, configResponse] = await Promise.all([
        fetch("/api/status", { cache: "no-store" }),
        fetch("/api/config", { cache: "no-store" }),
      ])

      if (!statusResponse.ok || !configResponse.ok) {
        throw new Error("刷新失败，请检查控制台服务是否正常运行")
      }

      const [statusData, configData] = (await Promise.all([
        statusResponse.json(),
        configResponse.json(),
      ])) as [StatusSnapshot, BilistreamConfig]

      startTransition(() => {
        setSnapshot(statusData)
        setConfig(normalizeConfig(configData))
        setErrorMessage(null)
      })
    } catch (error) {
      setFlash({
        tone: "error",
        message:
          error instanceof Error ? error.message : "刷新失败，请稍后重试",
      })
    }
  }

  const saveConfig = async () => {
    setPendingAction("save")
    try {
      const response = await fetch("/api/config", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(config),
      })

      if (!response.ok) {
        const message = await response.text()
        throw new Error(message || `保存失败，状态码 ${response.status}`)
      }

      setFlash({
        tone: "success",
        message: "配置已写入 config.yaml",
      })
      await refreshData()
    } catch (error) {
      setFlash({
        tone: "error",
        message:
          error instanceof Error ? error.message : "保存失败，请稍后重试",
      })
    } finally {
      setPendingAction(null)
    }
  }

  const setConfigField = <K extends keyof BilistreamConfig>(
    key: K,
    value: BilistreamConfig[K],
  ) => {
    setConfig((current) => ({
      ...current,
      [key]: value,
    }))
  }

  const setNestedField = <
    K extends keyof BilistreamConfig,
    N extends keyof NonNullable<BilistreamConfig[K]>,
  >(
    key: K,
    nestedKey: N,
    value: NonNullable<BilistreamConfig[K]>[N],
  ) => {
    setConfig((current) => ({
      ...current,
      [key]: {
        ...(current[key] as Record<string, unknown>),
        [nestedKey]: value,
      } as BilistreamConfig[K],
    }))
  }

  const handleNotificationChannel = (value: string) => {
    setNestedField(
      "Notification",
      "Channel",
      value as NotificationChannel,
    )
  }

  const statusMetrics = buildStatusMetrics(snapshot)
  const logs = snapshot?.logs ?? []
  const latestLog = logs[0]
  const sectionMeta = SECTION_META[activeSection]

  const openSection = (section: DashboardSection) => {
    setActiveSection(section)
    setActiveTab(SECTION_TO_TAB[section])
  }

  const handleTabChange = (value: string) => {
    const nextTab = value as DashboardTab
    setActiveTab(nextTab)

    if (nextTab === "overview") {
      setActiveSection("overview")
      return
    }

    if (nextTab === "config") {
      setActiveSection("config")
      return
    }

    setActiveSection("logs")
  }

  return (
    <div className="min-h-screen overflow-hidden bg-[radial-gradient(circle_at_top,_rgba(34,211,238,0.14),_transparent_34%),linear-gradient(135deg,_#020617_0%,_#081122_40%,_#050816_100%)] text-slate-100">
      <canvas ref={canvasRef} className="pointer-events-none fixed inset-0 opacity-40" />

      {isBooting && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/90 backdrop-blur-sm">
          <div className="flex flex-col items-center">
            <div className="relative h-24 w-24">
              <div className="absolute inset-0 rounded-full border border-cyan-400/20" />
              <div className="absolute inset-2 rounded-full border-2 border-cyan-400/40 border-t-cyan-300 animate-spin" />
              <div className="absolute inset-6 rounded-full border border-sky-300/25 animate-pulse" />
            </div>
            <div className="mt-5 text-sm tracking-[0.28em] text-cyan-300/90">
              BILISTREAM CONTROL ONLINE
            </div>
          </div>
        </div>
      )}

      <div className="relative z-10 mx-auto max-w-[1680px] px-4 py-5 sm:px-6 lg:px-8">
        <header className="mb-6 flex flex-col gap-4 border-b border-slate-800/70 pb-5 lg:flex-row lg:items-center lg:justify-between">
          <div className="flex items-start gap-4">
            <div className="flex h-14 w-14 items-center justify-center rounded-2xl border border-cyan-400/25 bg-cyan-400/10 shadow-[0_0_40px_rgba(34,211,238,0.14)]">
              <Hexagon className="h-7 w-7 text-cyan-300" />
            </div>
            <div className="space-y-2">
              <div className="flex flex-wrap items-center gap-2">
                <h1 className="text-2xl font-semibold tracking-tight text-white">
                  bilistream 控制台
                </h1>
                <Badge className="border-cyan-400/30 bg-cyan-400/10 px-2.5 py-1 text-cyan-200 hover:bg-cyan-400/10">
                  v{snapshot?.app_version ?? "0.2.0"}
                </Badge>
              </div>
              <p className="max-w-3xl text-sm leading-6 text-slate-400">
                面向 GitHub self-host 用户的远程值守界面，直接查看监控状态、通知策略与配置落盘情况。
              </p>
            </div>
          </div>

          <div className="flex flex-wrap items-center gap-3">
            <HeaderPill
              icon={snapshot?.enabled ? Radio : CircleOff}
              label={snapshot?.enabled ? "监控运行中" : "监控已暂停"}
              tone={snapshot?.enabled ? "success" : "muted"}
            />
            <HeaderPill
              icon={snapshot?.auth_enabled ? Shield : KeyRound}
              label={snapshot?.auth_enabled ? "已启用访问鉴权" : "未启用访问鉴权"}
              tone={snapshot?.auth_enabled ? "info" : "warning"}
            />
            <Button
              variant="outline"
              onClick={() => void refreshData()}
              className="h-11 border-slate-700 bg-slate-900/70 px-4 text-slate-100 hover:bg-slate-800"
            >
              <RefreshCw className="mr-2 h-4 w-4" />
              立即刷新
            </Button>
          </div>
        </header>

        {flash && (
          <div
            className={`mb-5 flex items-center gap-3 rounded-2xl border px-4 py-3 text-sm ${
              flash.tone === "success"
                ? "border-emerald-400/30 bg-emerald-400/10 text-emerald-100"
                : flash.tone === "error"
                  ? "border-rose-400/30 bg-rose-400/10 text-rose-100"
                  : "border-cyan-400/30 bg-cyan-400/10 text-cyan-100"
            }`}
          >
            {flash.tone === "error" ? (
              <Siren className="h-4 w-4" />
            ) : (
              <CheckCircle2 className="h-4 w-4" />
            )}
            <span>{flash.message}</span>
          </div>
        )}

        {errorMessage && (
          <div className="mb-5 rounded-2xl border border-rose-400/25 bg-rose-500/10 px-4 py-3 text-sm text-rose-100">
            控制台尚未拿到最新状态，原因：{errorMessage}
          </div>
        )}

        <div className="grid grid-cols-12 gap-6">
          <aside className="col-span-12 lg:col-span-2">
            <Card className="h-full border-slate-800/70 bg-slate-950/60 shadow-[0_24px_80px_rgba(2,6,23,0.45)] backdrop-blur">
              <CardContent className="p-4">
                <div className="space-y-2">
                  <NavItem
                    icon={Activity}
                    label="总览面板"
                    active={activeSection === "overview"}
                    onClick={() => openSection("overview")}
                  />
                  <NavItem
                    icon={Radio}
                    label="直播源监控"
                    active={activeSection === "source"}
                    onClick={() => openSection("source")}
                  />
                  <NavItem
                    icon={BellRing}
                    label="通知策略"
                    active={activeSection === "notification"}
                    onClick={() => openSection("notification")}
                  />
                  <NavItem
                    icon={Settings2}
                    label="配置管理"
                    active={activeSection === "config"}
                    onClick={() => openSection("config")}
                  />
                  <NavItem
                    icon={SquareTerminal}
                    label="运行日志"
                    active={activeSection === "logs"}
                    onClick={() => openSection("logs")}
                  />
                  <NavItem
                    icon={Server}
                    label="部署信息"
                    active={activeSection === "deployment"}
                    onClick={() => openSection("deployment")}
                  />
                </div>

                <nav className="hidden space-y-2">
                  <NavItem icon={Activity} label="总览面板" active />
                  <NavItem icon={Radio} label="直播源监控" />
                  <NavItem icon={BellRing} label="通知策略" />
                  <NavItem icon={Settings2} label="配置管理" />
                  <NavItem icon={SquareTerminal} label="运行日志" />
                  <NavItem icon={Server} label="部署信息" />
                </nav>

                <div className="mt-8 border-t border-slate-800/80 pt-6">
                  <div className="mb-3 text-xs font-medium uppercase tracking-[0.22em] text-slate-500">
                    Health Snapshot
                  </div>
                  <div className="space-y-4">
                    {statusMetrics.map((metric) => (
                      <StatusBar
                        key={metric.label}
                        label={metric.label}
                        value={metric.value}
                        tone={metric.tone}
                      />
                    ))}
                  </div>
                </div>
              </CardContent>
            </Card>
          </aside>

          <main className="col-span-12 lg:col-span-7">
            <div className="space-y-6">
              <Card className="overflow-hidden border-slate-800/70 bg-slate-950/60 shadow-[0_24px_80px_rgba(2,6,23,0.45)] backdrop-blur">
                <CardHeader className="border-b border-slate-800/70 pb-4">
                  <div className="flex flex-col gap-4 xl:flex-row xl:items-center xl:justify-between">
                    <div>
                      <CardTitle className="flex items-center gap-2 text-xl text-white">
                        <Activity className="h-5 w-5 text-cyan-300" />
                        {sectionMeta.title}
                      </CardTitle>
                      <p className="mt-2 text-sm text-slate-400">
                        这是模板主视图，已经切到 bilistream 的真实运行状态与配置接口。
                      </p>
                    </div>
                    <div className="flex flex-wrap items-center gap-3">
                      <ActionButton
                        label="启动监控"
                        icon={Radio}
                        variant="success"
                        loading={pendingAction === "start"}
                        onClick={() => void runAction("start")}
                      />
                      <ActionButton
                        label="停止监控"
                        icon={CircleOff}
                        variant="danger"
                        loading={pendingAction === "stop"}
                        onClick={() => void runAction("stop")}
                      />
                      <ActionButton
                        label="重载配置"
                        icon={RefreshCw}
                        variant="neutral"
                        loading={pendingAction === "reload"}
                        onClick={() => void runAction("reload")}
                      />
                    </div>
                  </div>
                </CardHeader>

                <CardContent className="p-6">
                  <div className="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-4">
                    <MetricCard
                      title="监控状态"
                      value={snapshot?.enabled ? "运行中" : "已暂停"}
                      detail={snapshot?.worker_state ?? "等待后端状态"}
                      icon={Cpu}
                      tone={snapshot?.enabled ? "cyan" : "slate"}
                    />
                    <MetricCard
                      title="源站状态"
                      value={formatLiveState(snapshot?.source_live)}
                      detail={`${snapshot?.source_platform ?? "-"} / ${snapshot?.source_room ?? "-"}`}
                      icon={Wifi}
                      tone={snapshot?.source_live ? "emerald" : "amber"}
                    />
                    <MetricCard
                      title="B 站直播"
                      value={formatLiveState(snapshot?.bilibili_live)}
                      detail="自动开播与关播同步状态"
                      icon={Radio}
                      tone={snapshot?.bilibili_live ? "cyan" : "slate"}
                    />
                    <MetricCard
                      title="FFmpeg"
                      value={snapshot?.ffmpeg_running ? "推流中" : "待机"}
                      detail="当前 CLI 推流会话状态"
                      icon={HardDrive}
                      tone={snapshot?.ffmpeg_running ? "emerald" : "slate"}
                    />
                  </div>

                  <div className="mt-8">
                    <Tabs value={activeTab} onValueChange={handleTabChange} className="w-full">
                      <div className="mb-5 flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
                        <TabsList className="grid h-auto w-full grid-cols-3 rounded-2xl border border-slate-800 bg-slate-900/80 p-1 lg:w-[520px]">
                          <TabsTrigger
                            value="overview"
                            className="rounded-xl py-2.5 text-sm data-[state=active]:bg-cyan-400/10 data-[state=active]:text-cyan-200"
                          >
                            运行总览
                          </TabsTrigger>
                          <TabsTrigger
                            value="config"
                            className="rounded-xl py-2.5 text-sm data-[state=active]:bg-cyan-400/10 data-[state=active]:text-cyan-200"
                          >
                            配置编辑
                          </TabsTrigger>
                          <TabsTrigger
                            value="logs"
                            className="rounded-xl py-2.5 text-sm data-[state=active]:bg-cyan-400/10 data-[state=active]:text-cyan-200"
                          >
                            运行日志
                          </TabsTrigger>
                        </TabsList>

                        <div className="flex flex-wrap items-center gap-2 text-xs text-slate-400">
                          <Badge className="border-slate-700 bg-slate-900 px-2.5 py-1 text-slate-300 hover:bg-slate-900">
                            配置路径：{snapshot?.config_path ?? "./config.yaml"}
                          </Badge>
                          <Badge className="border-slate-700 bg-slate-900 px-2.5 py-1 text-slate-300 hover:bg-slate-900">
                            Web：{snapshot?.web_addr ?? "0.0.0.0:9090"}
                          </Badge>
                        </div>
                      </div>

                      <div className="mb-5 flex items-center gap-3 rounded-2xl border border-cyan-400/15 bg-cyan-400/5 px-4 py-3">
                        <Badge className="border-cyan-400/30 bg-cyan-400/10 px-2 py-1 text-cyan-200 hover:bg-cyan-400/10">
                          {sectionMeta.badge}
                        </Badge>
                        <p className="text-sm text-slate-300">{sectionMeta.description}</p>
                      </div>

                      <TabsContent value="overview" className="mt-0">
                        <div className="grid gap-5 xl:grid-cols-[1.2fr_0.8fr]">
                          <div className="rounded-3xl border border-slate-800/80 bg-slate-900/45 p-5">
                            <SectionTitle
                              icon={Activity}
                              title="运行摘要"
                              description="模板中央大卡片改成 bilistream 的业务状态，不再使用模拟系统数据。"
                            />
                            <div className="mt-5 grid gap-4 md:grid-cols-2">
                              <InfoBlock
                                label="最近检查"
                                value={formatTimestamp(snapshot?.last_checked_at)}
                              />
                              <InfoBlock
                                label="最近事件"
                                value={snapshot?.last_event ?? "暂无事件"}
                              />
                              <InfoBlock
                                label="通知模式"
                                value={describeChannel(
                                  config.Notification.Channel,
                                  config.Notification.Enabled,
                                )}
                              />
                              <InfoBlock
                                label="故障状态"
                                value={snapshot?.last_error ?? "当前未记录错误"}
                                danger={Boolean(snapshot?.last_error)}
                              />
                            </div>

                            <div className="mt-6 space-y-3">
                              <TimelineItem
                                title="源站平台"
                                description={`${snapshot?.source_platform ?? "-"} · ${snapshot?.source_room ?? "-"}`}
                              />
                              <TimelineItem
                                title="B 站房间"
                                description={
                                  config.BiliLive.Room > 0
                                    ? `房间号 ${config.BiliLive.Room}`
                                    : "尚未填写 B 站房间号"
                                }
                              />
                              <TimelineItem
                                title="推流目标"
                                description={
                                  config.BiliLive.BiliRtmpUrl
                                    ? "已配置 RTMP 地址与推流 Key"
                                    : "尚未填写 B 站推流地址"
                                }
                              />
                            </div>
                          </div>

                          <div className="rounded-3xl border border-slate-800/80 bg-slate-900/45 p-5">
                            <SectionTitle
                              icon={Shield}
                              title="运维建议"
                              description="围绕 self-host 场景，保留模板右侧辅助信息区的结构。"
                            />
                            <div className="mt-5 space-y-3">
                              <AdviceItem
                                ok={snapshot?.auth_enabled ?? false}
                                title="远程访问鉴权"
                                description={
                                  snapshot?.auth_enabled
                                    ? "当前已启用 Basic Auth，适合直接部署在服务器上。"
                                    : "建议为 WebConsole 配置密码，避免默认裸露在公网。"
                                }
                              />
                              <AdviceItem
                                ok={config.Notification.Enabled}
                                title="通知策略"
                                description={
                                  config.Notification.Enabled
                                    ? `当前使用 ${describeChannel(config.Notification.Channel, true)}，通道状态可在下方直接调整。`
                                    : "通知总开关已关闭，直播切换时不会发送提醒。"
                                }
                              />
                              <AdviceItem
                                ok={Boolean(config.FfmpegProxy || config.Cookies)}
                                title="源站兼容性"
                                description={
                                  config.Cookies || config.FfmpegProxy
                                    ? "已配置访问增强参数，适合更复杂的直播源环境。"
                                    : "如果遇到受限源站，可补充 Cookies 或 FFmpeg 代理。"
                                }
                              />
                            </div>
                          </div>
                        </div>
                      </TabsContent>

                      <TabsContent value="config" className="mt-0">
                        <div className="space-y-5">
                          <div className="grid gap-5 xl:grid-cols-2">
                            <FormSection
                              title="基础控制"
                              description="监控平台、轮询间隔与全局兼容参数。"
                            >
                              <div className="grid gap-4 md:grid-cols-2">
                                <Field>
                                  <Label htmlFor="platform">监控平台</Label>
                                  <Select
                                    value={config.Platform}
                                    onValueChange={(value) =>
                                      setConfigField("Platform", value)
                                    }
                                  >
                                    <SelectTrigger id="platform" className="border-slate-700 bg-slate-950/70">
                                      <SelectValue placeholder="选择平台" />
                                    </SelectTrigger>
                                    <SelectContent>
                                      <SelectItem value="Twitch">Twitch</SelectItem>
                                      <SelectItem value="Youtube">YouTube</SelectItem>
                                      <SelectItem value="YoutubePreviewLive">
                                        YouTube 预告直播
                                      </SelectItem>
                                    </SelectContent>
                                  </Select>
                                </Field>
                                <Field>
                                  <Label htmlFor="interval">轮询间隔（秒）</Label>
                                  <Input
                                    id="interval"
                                    type="number"
                                    min={5}
                                    value={config.Interval}
                                    onChange={(event) =>
                                      setConfigField(
                                        "Interval",
                                        Number(event.target.value || 0),
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                              </div>
                              <Field>
                                <Label htmlFor="ffmpeg-proxy">FFmpeg 代理</Label>
                                <Input
                                  id="ffmpeg-proxy"
                                  value={config.FfmpegProxy ?? ""}
                                  onChange={(event) =>
                                    setConfigField(
                                      "FfmpegProxy",
                                      event.target.value,
                                    )
                                  }
                                  placeholder="例如 http://127.0.0.1:7890"
                                  className="border-slate-700 bg-slate-950/70"
                                />
                              </Field>
                              <Field>
                                <Label htmlFor="cookies">Cookies</Label>
                                <Textarea
                                  id="cookies"
                                  value={config.Cookies ?? ""}
                                  onChange={(event) =>
                                    setConfigField("Cookies", event.target.value)
                                  }
                                  placeholder="仅在源站需要额外身份信息时填写"
                                  className="min-h-24 border-slate-700 bg-slate-950/70"
                                />
                              </Field>
                            </FormSection>

                            <FormSection
                              title="直播源与 B 站"
                              description="源站房间、B 站房间号、推流地址与登录参数。"
                            >
                              <div className="grid gap-4 md:grid-cols-2">
                                <Field>
                                  <Label htmlFor="twitch-room">Twitch 房间</Label>
                                  <Input
                                    id="twitch-room"
                                    value={config.Twitch.Room}
                                    onChange={(event) =>
                                      setNestedField("Twitch", "Room", event.target.value)
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="youtube-room">YouTube 直播地址 / ID</Label>
                                  <Input
                                    id="youtube-room"
                                    value={config.Youtube.Room}
                                    onChange={(event) =>
                                      setNestedField("Youtube", "Room", event.target.value)
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="youtube-preview-channel">YouTube 预告频道 ID</Label>
                                  <Input
                                    id="youtube-preview-channel"
                                    value={config.YoutubePreviewLive.ChannelId}
                                    onChange={(event) =>
                                      setNestedField(
                                        "YoutubePreviewLive",
                                        "ChannelId",
                                        event.target.value,
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="youtube-token">YouTube Access Token</Label>
                                  <Input
                                    id="youtube-token"
                                    type="password"
                                    value={config.Youtube.AccessToken}
                                    onChange={(event) =>
                                      setNestedField(
                                        "Youtube",
                                        "AccessToken",
                                        event.target.value,
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="bili-room">B 站房间号</Label>
                                  <Input
                                    id="bili-room"
                                    type="number"
                                    value={config.BiliLive.Room}
                                    onChange={(event) =>
                                      setNestedField(
                                        "BiliLive",
                                        "Room",
                                        Number(event.target.value || 0),
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="bili-rtmp-url">B 站 RTMP 地址</Label>
                                  <Input
                                    id="bili-rtmp-url"
                                    value={config.BiliLive.BiliRtmpUrl}
                                    onChange={(event) =>
                                      setNestedField(
                                        "BiliLive",
                                        "BiliRtmpUrl",
                                        event.target.value,
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="bili-rtmp-key">B 站推流 Key</Label>
                                  <Input
                                    id="bili-rtmp-key"
                                    type="password"
                                    value={config.BiliLive.BiliRtmpKey}
                                    onChange={(event) =>
                                      setNestedField(
                                        "BiliLive",
                                        "BiliRtmpKey",
                                        event.target.value,
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                              </div>

                              <Separator className="bg-slate-800/80" />

                              <div className="grid gap-4 md:grid-cols-2">
                                <Field>
                                  <Label htmlFor="sessdata">SESSDATA</Label>
                                  <Input
                                    id="sessdata"
                                    type="password"
                                    value={config.BiliLive.SESSDATA}
                                    onChange={(event) =>
                                      setNestedField(
                                        "BiliLive",
                                        "SESSDATA",
                                        event.target.value,
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="bili-jct">bili_jct</Label>
                                  <Input
                                    id="bili-jct"
                                    type="password"
                                    value={config.BiliLive.bili_jct}
                                    onChange={(event) =>
                                      setNestedField(
                                        "BiliLive",
                                        "bili_jct",
                                        event.target.value,
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="dede-user-id">DedeUserID</Label>
                                  <Input
                                    id="dede-user-id"
                                    value={config.BiliLive.DedeUserID}
                                    onChange={(event) =>
                                      setNestedField(
                                        "BiliLive",
                                        "DedeUserID",
                                        event.target.value,
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="dede-user-id-ckmd5">DedeUserID__ckMd5</Label>
                                  <Input
                                    id="dede-user-id-ckmd5"
                                    type="password"
                                    value={config.BiliLive.DedeUserID__ckMd5}
                                    onChange={(event) =>
                                      setNestedField(
                                        "BiliLive",
                                        "DedeUserID__ckMd5",
                                        event.target.value,
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                              </div>
                            </FormSection>
                          </div>

                          <div className="grid gap-5 xl:grid-cols-2">
                            <FormSection
                              title="通知策略"
                              description="支持总开关、主通道选择，以及 gotify / ntfy 单独启停。"
                            >
                              <div className="grid gap-4 md:grid-cols-2">
                                <ToggleRow
                                  label="通知总开关"
                                  description="关闭后不会发送任何开播提醒。"
                                  checked={config.Notification.Enabled}
                                  onCheckedChange={(checked) =>
                                    setNestedField("Notification", "Enabled", checked)
                                  }
                                />
                                <ToggleRow
                                  label="Gotify 通道"
                                  description="即使主模式是 both，也可以单独关闭。"
                                  checked={config.Notification.GotifyEnabled}
                                  onCheckedChange={(checked) =>
                                    setNestedField(
                                      "Notification",
                                      "GotifyEnabled",
                                      checked,
                                    )
                                  }
                                />
                                <ToggleRow
                                  label="ntfy 通道"
                                  description="支持 ntfy.sh 或自托管 ntfy 服务。"
                                  checked={config.Notification.NtfyEnabled}
                                  onCheckedChange={(checked) =>
                                    setNestedField("Notification", "NtfyEnabled", checked)
                                  }
                                />
                                <Field>
                                  <Label htmlFor="notification-channel">主通道策略</Label>
                                  <Select
                                    value={config.Notification.Channel}
                                    onValueChange={handleNotificationChannel}
                                  >
                                    <SelectTrigger
                                      id="notification-channel"
                                      className="border-slate-700 bg-slate-950/70"
                                    >
                                      <SelectValue placeholder="选择策略" />
                                    </SelectTrigger>
                                    <SelectContent>
                                      <SelectItem value="gotify">仅 gotify</SelectItem>
                                      <SelectItem value="ntfy">仅 ntfy</SelectItem>
                                      <SelectItem value="both">同时发送</SelectItem>
                                    </SelectContent>
                                  </Select>
                                </Field>
                              </div>

                              <Separator className="bg-slate-800/80" />

                              <div className="grid gap-4 md:grid-cols-2">
                                <Field>
                                  <Label htmlFor="gotify-url">Gotify 地址</Label>
                                  <Input
                                    id="gotify-url"
                                    value={config.Gotify?.Url ?? ""}
                                    onChange={(event) =>
                                      setNestedField("Gotify", "Url", event.target.value)
                                    }
                                    placeholder="https://push.example.com"
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="gotify-token">Gotify Token</Label>
                                  <Input
                                    id="gotify-token"
                                    type="password"
                                    value={config.Gotify?.Token ?? ""}
                                    onChange={(event) =>
                                      setNestedField("Gotify", "Token", event.target.value)
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="ntfy-url">ntfy 地址</Label>
                                  <Input
                                    id="ntfy-url"
                                    value={config.Ntfy?.Url ?? ""}
                                    onChange={(event) =>
                                      setNestedField("Ntfy", "Url", event.target.value)
                                    }
                                    placeholder="https://ntfy.sh"
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="ntfy-topic">ntfy Topic</Label>
                                  <Input
                                    id="ntfy-topic"
                                    value={config.Ntfy?.Topic ?? ""}
                                    onChange={(event) =>
                                      setNestedField("Ntfy", "Topic", event.target.value)
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="ntfy-token">ntfy Token</Label>
                                  <Input
                                    id="ntfy-token"
                                    type="password"
                                    value={config.Ntfy?.Token ?? ""}
                                    onChange={(event) =>
                                      setNestedField("Ntfy", "Token", event.target.value)
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="ntfy-priority">ntfy Priority</Label>
                                  <Input
                                    id="ntfy-priority"
                                    value={config.Ntfy?.Priority ?? ""}
                                    onChange={(event) =>
                                      setNestedField("Ntfy", "Priority", event.target.value)
                                    }
                                    placeholder="min / low / default / high"
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="ntfy-username">ntfy Username</Label>
                                  <Input
                                    id="ntfy-username"
                                    value={config.Ntfy?.Username ?? ""}
                                    onChange={(event) =>
                                      setNestedField("Ntfy", "Username", event.target.value)
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="ntfy-password">ntfy Password</Label>
                                  <Input
                                    id="ntfy-password"
                                    type="password"
                                    value={config.Ntfy?.Password ?? ""}
                                    onChange={(event) =>
                                      setNestedField("Ntfy", "Password", event.target.value)
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                              </div>

                              <Field>
                                <Label htmlFor="ntfy-tags">ntfy Tags</Label>
                                <Input
                                  id="ntfy-tags"
                                  value={config.Ntfy?.Tags ?? ""}
                                  onChange={(event) =>
                                    setNestedField("Ntfy", "Tags", event.target.value)
                                  }
                                  placeholder="live,bilistream,self-host"
                                  className="border-slate-700 bg-slate-950/70"
                                />
                              </Field>
                            </FormSection>

                            <FormSection
                              title="WebConsole 访问"
                              description="优先为服务器部署准备，建议保持账号密码与公网监听配置。"
                            >
                              <div className="grid gap-4 md:grid-cols-2">
                                <Field>
                                  <Label htmlFor="web-addr">监听地址</Label>
                                  <Input
                                    id="web-addr"
                                    value={config.WebConsole?.Addr ?? ""}
                                    onChange={(event) =>
                                      setNestedField("WebConsole", "Addr", event.target.value)
                                    }
                                    placeholder="0.0.0.0:9090"
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                                <Field>
                                  <Label htmlFor="web-username">用户名</Label>
                                  <Input
                                    id="web-username"
                                    value={config.WebConsole?.Username ?? ""}
                                    onChange={(event) =>
                                      setNestedField(
                                        "WebConsole",
                                        "Username",
                                        event.target.value,
                                      )
                                    }
                                    className="border-slate-700 bg-slate-950/70"
                                  />
                                </Field>
                              </div>
                              <Field>
                                <Label htmlFor="web-password">访问密码</Label>
                                <Input
                                  id="web-password"
                                  type="password"
                                  value={config.WebConsole?.Password ?? ""}
                                  onChange={(event) =>
                                    setNestedField(
                                      "WebConsole",
                                      "Password",
                                      event.target.value,
                                    )
                                  }
                                  className="border-slate-700 bg-slate-950/70"
                                />
                              </Field>
                              

                              <div className="flex flex-wrap items-center gap-3 pt-2">
                                <Button
                                  onClick={() => void saveConfig()}
                                  disabled={pendingAction === "save"}
                                  className="h-11 rounded-xl bg-amber-400 px-5 text-slate-950 hover:bg-amber-300"
                                >
                                  {pendingAction === "save" ? (
                                    <LoaderCircle className="mr-2 h-4 w-4 animate-spin" />
                                  ) : (
                                    <Save className="mr-2 h-4 w-4" />
                                  )}
                                  保存到 config.yaml
                                </Button>
                                <span className="text-sm text-slate-500">
                                  保存后后端会自动更新内存配置，并继续按新的 YAML 运行。
                                </span>
                              </div>
                            </FormSection>
                          </div>
                        </div>
                      </TabsContent>

                      <TabsContent value="logs" className="mt-0">
                        <div className="rounded-3xl border border-slate-800/80 bg-slate-950/70 p-3">
                          <div className="max-h-[620px] overflow-y-auto rounded-2xl border border-slate-800/80 bg-black/20">
                            {logs.length === 0 ? (
                              <div className="px-5 py-8 text-center text-sm text-slate-500">
                                还没有日志输出，等下一轮状态检查后会自动出现。
                              </div>
                            ) : (
                              <div className="divide-y divide-slate-800/70">
                                {logs.map((entry) => (
                                  <LogRow key={`${entry.timestamp}-${entry.message}`} entry={entry} />
                                ))}
                              </div>
                            )}
                          </div>
                        </div>
                      </TabsContent>
                    </Tabs>
                  </div>
                </CardContent>
              </Card>
            </div>
          </main>

          <aside className="col-span-12 lg:col-span-3">
            <div className="space-y-6">
              <Card className="overflow-hidden border-slate-800/70 bg-slate-950/60 shadow-[0_24px_80px_rgba(2,6,23,0.45)] backdrop-blur">
                <CardContent className="p-0">
                  <div className="border-b border-slate-800/70 bg-[linear-gradient(180deg,_rgba(15,23,42,0.84),_rgba(8,47,73,0.45))] px-6 py-7">
                    <div className="text-center">
                      <div className="text-xs font-medium uppercase tracking-[0.22em] text-slate-500">
                        System Time
                      </div>
                      <div className="mt-2 font-mono text-4xl font-semibold text-cyan-200">
                        {formatTime(currentTime)}
                      </div>
                      <div className="mt-2 text-sm text-slate-400">
                        {formatDate(currentTime)}
                      </div>
                    </div>
                  </div>
                  <div className="grid grid-cols-2 gap-3 p-4">
                    <MiniInfo label="最近事件" value={latestLog?.level ?? "idle"} />
                    <MiniInfo
                      label="最后时间"
                      value={formatTimestamp(latestLog?.timestamp)}
                    />
                  </div>
                </CardContent>
              </Card>

              <Card className="border-slate-800/70 bg-slate-950/60 shadow-[0_24px_80px_rgba(2,6,23,0.45)] backdrop-blur">
                <CardHeader className="pb-3">
                  <CardTitle className="text-base text-white">通知策略</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  <StrategyRow
                    label="总开关"
                    value={config.Notification.Enabled ? "已启用" : "已关闭"}
                    tone={config.Notification.Enabled ? "success" : "muted"}
                  />
                  <StrategyRow
                    label="主模式"
                    value={describeChannel(
                      config.Notification.Channel,
                      config.Notification.Enabled,
                    )}
                    tone="info"
                  />
                  <StrategyRow
                    label="Gotify"
                    value={config.Notification.GotifyEnabled ? "开启" : "关闭"}
                    tone={config.Notification.GotifyEnabled ? "success" : "muted"}
                  />
                  <StrategyRow
                    label="ntfy"
                    value={config.Notification.NtfyEnabled ? "开启" : "关闭"}
                    tone={config.Notification.NtfyEnabled ? "success" : "muted"}
                  />
                </CardContent>
              </Card>

              <Card className="border-slate-800/70 bg-slate-950/60 shadow-[0_24px_80px_rgba(2,6,23,0.45)] backdrop-blur">
                <CardHeader className="pb-3">
                  <CardTitle className="text-base text-white">Self-host 信息</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  <LinkRow
                    icon={Github}
                    label="GitHub 仓库"
                    value={snapshot?.github_url ?? "https://github.com/limitcool/bilistream"}
                    href={snapshot?.github_url ?? "https://github.com/limitcool/bilistream"}
                  />
                  <DetailRow label="监听地址" value={snapshot?.web_addr ?? "0.0.0.0:9090"} />
                  <DetailRow label="配置文件" value={snapshot?.config_path ?? "./config.yaml"} />
                  <DetailRow
                    label="鉴权状态"
                    value={snapshot?.auth_enabled ? "已启用" : "未启用"}
                  />
                </CardContent>
              </Card>

              <Card className="border-slate-800/70 bg-slate-950/60 shadow-[0_24px_80px_rgba(2,6,23,0.45)] backdrop-blur">
                <CardHeader className="pb-3">
                  <CardTitle className="text-base text-white">快速动作</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="mb-3 space-y-3">
                    <SectionShortcut
                      label="打开通知策略"
                      description="跳到 gotify / ntfy、总开关与主通道策略设置。"
                      onClick={() => openSection("notification")}
                    />
                    <SectionShortcut
                      label="打开配置管理"
                      description="进入 config.yaml 编辑区，调整直播源与 WebConsole 参数。"
                      onClick={() => openSection("config")}
                    />
                    <SectionShortcut
                      label="打开直播源监控"
                      description="回到总览区检查源站在线状态、房间目标与监控结果。"
                      onClick={() => openSection("source")}
                    />
                  </div>
                  <div className="grid grid-cols-2 gap-3">
                    <QuickTile
                      icon={Radio}
                      label="启动监控"
                      tone="success"
                      onClick={() => void runAction("start")}
                    />
                    <QuickTile
                      icon={CircleOff}
                      label="停止监控"
                      tone="danger"
                      onClick={() => void runAction("stop")}
                    />
                    <QuickTile
                      icon={RefreshCw}
                      label="重载配置"
                      tone="neutral"
                      onClick={() => void runAction("reload")}
                    />
                    <QuickTile
                      icon={Save}
                      label="保存 YAML"
                      tone="accent"
                      onClick={() => void saveConfig()}
                    />
                  </div>
                </CardContent>
              </Card>
            </div>
          </aside>
        </div>
      </div>
    </div>
  )
}

function buildStatusMetrics(snapshot: StatusSnapshot | null) {
  if (!snapshot) {
    return [
      { label: "控制台可用性", value: 32, tone: "cyan" as const },
      { label: "源站连通性", value: 18, tone: "amber" as const },
      { label: "推流状态", value: 12, tone: "slate" as const },
    ]
  }

  const availability = snapshot.config_loaded ? 100 : 42
  const source = snapshot.source_live === true ? 100 : snapshot.source_live === false ? 24 : 46
  const pushing = snapshot.ffmpeg_running ? 100 : 14

  return [
    { label: "控制台可用性", value: availability, tone: "cyan" as const },
    { label: "源站连通性", value: source, tone: source > 80 ? ("emerald" as const) : ("amber" as const) },
    { label: "推流状态", value: pushing, tone: pushing > 80 ? ("cyan" as const) : ("slate" as const) },
  ]
}

function formatLiveState(value: boolean | null | undefined) {
  if (value === true) return "在线"
  if (value === false) return "离线"
  return "待检测"
}

function describeChannel(channel: string, enabled: boolean) {
  if (!enabled) {
    return "通知已关闭"
  }

  switch (channel) {
    case "both":
      return "Gotify + ntfy"
    case "ntfy":
      return "仅 ntfy"
    default:
      return "仅 Gotify"
  }
}

function formatTime(date: Date) {
  return date.toLocaleTimeString("zh-CN", {
    hour12: false,
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  })
}

function formatDate(date: Date) {
  return date.toLocaleDateString("zh-CN", {
    year: "numeric",
    month: "long",
    day: "numeric",
    weekday: "long",
  })
}

function formatTimestamp(timestamp?: number | null) {
  if (!timestamp) {
    return "尚无记录"
  }

  return new Date(timestamp * 1000).toLocaleString("zh-CN", {
    hour12: false,
  })
}

function HeaderPill({
  icon: Icon,
  label,
  tone,
}: {
  icon: typeof Radio
  label: string
  tone: "success" | "warning" | "muted" | "info"
}) {
  const className =
    tone === "success"
      ? "border-emerald-400/30 bg-emerald-400/10 text-emerald-100"
      : tone === "warning"
        ? "border-amber-400/30 bg-amber-400/10 text-amber-100"
        : tone === "info"
          ? "border-cyan-400/30 bg-cyan-400/10 text-cyan-100"
          : "border-slate-700 bg-slate-900/80 text-slate-300"

  return (
    <div className={`flex items-center gap-2 rounded-full border px-3 py-2 text-sm ${className}`}>
      <Icon className="h-4 w-4" />
      <span>{label}</span>
    </div>
  )
}

function NavItem({
  icon: Icon,
  label,
  active,
  onClick,
}: {
  icon: typeof Activity
  label: string
  active?: boolean
  onClick?: () => void
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={`flex w-full items-center gap-3 rounded-2xl px-3 py-3 text-left text-sm transition ${
        active
          ? "border border-cyan-400/30 bg-cyan-400/12 text-cyan-100 shadow-[0_0_0_1px_rgba(34,211,238,0.08)]"
          : "border border-transparent text-slate-400 hover:border-slate-800 hover:bg-slate-900/80 hover:text-slate-200"
      }`}
    >
      <Icon className="h-4 w-4" />
      <span>{label}</span>
    </button>
  )
}

function StatusBar({
  label,
  value,
  tone,
}: {
  label: string
  value: number
  tone: "cyan" | "emerald" | "amber" | "slate"
}) {
  const gradient =
    tone === "emerald"
      ? "from-emerald-400 to-green-500"
      : tone === "amber"
        ? "from-amber-300 to-orange-500"
        : tone === "slate"
          ? "from-slate-500 to-slate-400"
          : "from-cyan-400 to-sky-500"

  return (
    <div>
      <div className="mb-1.5 flex items-center justify-between text-xs text-slate-400">
        <span>{label}</span>
        <span>{value}%</span>
      </div>
      <div className="h-1.5 overflow-hidden rounded-full bg-slate-900">
        <div
          className={`h-full rounded-full bg-gradient-to-r ${gradient}`}
          style={{ width: `${value}%` }}
        />
      </div>
    </div>
  )
}

function ActionButton({
  label,
  icon: Icon,
  variant,
  loading,
  onClick,
}: {
  label: string
  icon: typeof Activity
  variant: "success" | "danger" | "neutral"
  loading?: boolean
  onClick: () => void
}) {
  const className =
    variant === "success"
      ? "bg-emerald-400 text-slate-950 hover:bg-emerald-300"
      : variant === "danger"
        ? "bg-rose-500 text-white hover:bg-rose-400"
        : "border-slate-700 bg-slate-900 text-slate-100 hover:bg-slate-800"

  return (
    <Button
      onClick={onClick}
      className={`h-11 rounded-xl px-4 ${className}`}
      variant={variant === "neutral" ? "outline" : "default"}
      disabled={loading}
    >
      {loading ? (
        <LoaderCircle className="mr-2 h-4 w-4 animate-spin" />
      ) : (
        <Icon className="mr-2 h-4 w-4" />
      )}
      {label}
    </Button>
  )
}

function MetricCard({
  title,
  value,
  detail,
  icon: Icon,
  tone,
}: {
  title: string
  value: string
  detail: string
  icon: typeof Activity
  tone: "cyan" | "emerald" | "amber" | "slate"
}) {
  const toneClass =
    tone === "emerald"
      ? "border-emerald-400/25 bg-emerald-400/8 text-emerald-200"
      : tone === "amber"
        ? "border-amber-300/25 bg-amber-300/10 text-amber-100"
        : tone === "slate"
          ? "border-slate-700 bg-slate-900/80 text-slate-200"
          : "border-cyan-400/25 bg-cyan-400/10 text-cyan-100"

  return (
    <div className={`relative overflow-hidden rounded-3xl border p-4 ${toneClass}`}>
      <div className="mb-4 flex items-center justify-between">
        <span className="text-sm text-slate-300">{title}</span>
        <Icon className="h-5 w-5" />
      </div>
      <div className="text-2xl font-semibold tracking-tight">{value}</div>
      <div className="mt-2 text-sm leading-6 text-slate-400">{detail}</div>
      <div className="pointer-events-none absolute -bottom-8 -right-8 h-24 w-24 rounded-full bg-white/5 blur-2xl" />
    </div>
  )
}

function SectionTitle({
  icon: Icon,
  title,
  description,
}: {
  icon: typeof Activity
  title: string
  description: string
}) {
  return (
    <div>
      <div className="flex items-center gap-2 text-base font-semibold text-white">
        <Icon className="h-4 w-4 text-cyan-300" />
        <span>{title}</span>
      </div>
      <p className="mt-2 text-sm leading-6 text-slate-400">{description}</p>
    </div>
  )
}

function InfoBlock({
  label,
  value,
  danger,
}: {
  label: string
  value: string
  danger?: boolean
}) {
  return (
    <div className="rounded-2xl border border-slate-800/80 bg-slate-950/80 p-4">
      <div className="text-xs uppercase tracking-[0.18em] text-slate-500">{label}</div>
      <div className={`mt-3 text-sm leading-6 ${danger ? "text-rose-200" : "text-slate-200"}`}>
        {value}
      </div>
    </div>
  )
}

function TimelineItem({
  title,
  description,
}: {
  title: string
  description: string
}) {
  return (
    <div className="flex gap-3 rounded-2xl border border-slate-800/80 bg-slate-950/50 px-4 py-3">
      <div className="mt-1 h-2.5 w-2.5 rounded-full bg-cyan-300 shadow-[0_0_14px_rgba(103,232,249,0.75)]" />
      <div>
        <div className="text-sm font-medium text-slate-200">{title}</div>
        <div className="mt-1 text-sm leading-6 text-slate-400">{description}</div>
      </div>
    </div>
  )
}

function AdviceItem({
  ok,
  title,
  description,
}: {
  ok: boolean
  title: string
  description: string
}) {
  return (
    <div className="rounded-2xl border border-slate-800/80 bg-slate-950/60 px-4 py-4">
      <div className="flex items-center gap-2 text-sm font-medium text-slate-200">
        {ok ? (
          <CheckCircle2 className="h-4 w-4 text-emerald-300" />
        ) : (
          <Siren className="h-4 w-4 text-amber-300" />
        )}
        <span>{title}</span>
      </div>
      <p className="mt-2 text-sm leading-6 text-slate-400">{description}</p>
    </div>
  )
}

function FormSection({
  title,
  description,
  children,
}: {
  title: string
  description: string
  children: React.ReactNode
}) {
  return (
    <div className="rounded-3xl border border-slate-800/80 bg-slate-900/45 p-5">
      <div className="mb-5">
        <h3 className="text-base font-semibold text-white">{title}</h3>
        <p className="mt-2 text-sm leading-6 text-slate-400">{description}</p>
      </div>
      <div className="space-y-4">{children}</div>
    </div>
  )
}

function Field({ children }: { children: React.ReactNode }) {
  return <div className="space-y-2">{children}</div>
}

function ToggleRow({
  label,
  description,
  checked,
  onCheckedChange,
}: {
  label: string
  description: string
  checked: boolean
  onCheckedChange: (checked: boolean) => void
}) {
  return (
    <div className="flex items-start justify-between rounded-2xl border border-slate-800/80 bg-slate-950/60 px-4 py-4">
      <div className="pr-4">
        <div className="text-sm font-medium text-slate-200">{label}</div>
        <div className="mt-1 text-sm leading-6 text-slate-400">{description}</div>
      </div>
      <Switch checked={checked} onCheckedChange={onCheckedChange} />
    </div>
  )
}

function LogRow({ entry }: { entry: StatusSnapshot["logs"][number] }) {
  const toneClass =
    entry.level === "error"
      ? "border-rose-400/20 bg-rose-400/10 text-rose-100"
      : entry.level === "warn"
        ? "border-amber-300/20 bg-amber-300/10 text-amber-100"
        : "border-cyan-400/20 bg-cyan-400/10 text-cyan-100"

  return (
    <div className="grid gap-3 px-4 py-4 md:grid-cols-[170px_80px_minmax(0,1fr)]">
      <div className="font-mono text-xs text-slate-500">{formatTimestamp(entry.timestamp)}</div>
      <div>
        <span className={`inline-flex rounded-full border px-2.5 py-1 text-xs ${toneClass}`}>
          {entry.level}
        </span>
      </div>
      <div className="text-sm leading-6 text-slate-200">{entry.message}</div>
    </div>
  )
}

function MiniInfo({
  label,
  value,
}: {
  label: string
  value: string
}) {
  return (
    <div className="rounded-2xl border border-slate-800/70 bg-slate-950/70 p-3">
      <div className="text-xs uppercase tracking-[0.18em] text-slate-500">{label}</div>
      <div className="mt-2 text-sm text-slate-200">{value}</div>
    </div>
  )
}

function SectionShortcut({
  label,
  description,
  onClick,
}: {
  label: string
  description: string
  onClick: () => void
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className="flex w-full items-start justify-between rounded-2xl border border-slate-800/80 bg-slate-950/70 px-4 py-3 text-left transition hover:border-cyan-400/30 hover:bg-slate-900"
    >
      <div>
        <div className="text-sm font-medium text-slate-100">{label}</div>
        <div className="mt-1 text-sm leading-6 text-slate-400">{description}</div>
      </div>
      <ExternalLink className="mt-0.5 h-4 w-4 text-cyan-300" />
    </button>
  )
}

function StrategyRow({
  label,
  value,
  tone,
}: {
  label: string
  value: string
  tone: "success" | "muted" | "info"
}) {
  const toneClass =
    tone === "success"
      ? "text-emerald-200"
      : tone === "info"
        ? "text-cyan-200"
        : "text-slate-300"

  return (
    <div className="flex items-center justify-between rounded-2xl border border-slate-800/80 bg-slate-950/70 px-4 py-3">
      <span className="text-sm text-slate-400">{label}</span>
      <span className={`text-sm font-medium ${toneClass}`}>{value}</span>
    </div>
  )
}

function LinkRow({
  icon: Icon,
  label,
  value,
  href,
}: {
  icon: typeof Github
  label: string
  value: string
  href: string
}) {
  return (
    <a
      href={href}
      target="_blank"
      rel="noreferrer"
      className="flex items-center justify-between rounded-2xl border border-slate-800/80 bg-slate-950/70 px-4 py-3 transition hover:border-cyan-400/30 hover:bg-slate-900"
    >
      <div className="flex items-center gap-3">
        <Icon className="h-4 w-4 text-cyan-300" />
        <div>
          <div className="text-sm text-slate-400">{label}</div>
          <div className="mt-1 text-sm text-slate-200">{value}</div>
        </div>
      </div>
      <ExternalLink className="h-4 w-4 text-slate-500" />
    </a>
  )
}

function DetailRow({
  label,
  value,
}: {
  label: string
  value: string
}) {
  return (
    <div className="rounded-2xl border border-slate-800/80 bg-slate-950/70 px-4 py-3">
      <div className="text-sm text-slate-400">{label}</div>
      <div className="mt-1 break-all text-sm text-slate-200">{value}</div>
    </div>
  )
}

function QuickTile({
  icon: Icon,
  label,
  tone,
  onClick,
}: {
  icon: typeof Radio
  label: string
  tone: "success" | "danger" | "neutral" | "accent"
  onClick: () => void
}) {
  const toneClass =
    tone === "success"
      ? "border-emerald-400/25 bg-emerald-400/10 text-emerald-50 hover:bg-emerald-400/20"
      : tone === "danger"
        ? "border-rose-400/25 bg-rose-500/12 text-rose-50 hover:bg-rose-500/20"
        : tone === "accent"
          ? "border-amber-300/25 bg-amber-300/12 text-amber-50 hover:bg-amber-300/20"
          : "border-slate-700 bg-slate-950/70 text-slate-100 hover:bg-slate-900"

  return (
    <button
      type="button"
      onClick={onClick}
      className={`flex min-h-24 flex-col items-center justify-center gap-2 rounded-2xl border px-3 py-4 text-center text-sm transition ${toneClass}`}
    >
      <Icon className="h-5 w-5" />
      <span>{label}</span>
    </button>
  )
}
