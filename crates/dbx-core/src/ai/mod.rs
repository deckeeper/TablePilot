// TablePilot modification: data-only legacy storage compatibility; no AI providers or execution.
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub mod prompt_template;
use std::collections::{BTreeMap, HashMap};
pub const DEFAULT_MAX_RETRIES: u32 = 2;
pub const MAX_MAX_RETRIES: u32 = 10;
pub fn clamp_max_retries(value: u32) -> u32 { value.clamp(0, MAX_MAX_RETRIES) }
pub const DEFAULT_MAX_AGENT_TURNS: u32 = 30;
pub const MIN_MAX_AGENT_TURNS: u32 = 5;
pub const MAX_MAX_AGENT_TURNS: u32 = 500;
pub fn clamp_max_agent_turns(value: u32) -> u32 { value.clamp(MIN_MAX_AGENT_TURNS, MAX_MAX_AGENT_TURNS) }
pub mod agent_explain;
pub mod agent_kv;
pub mod mcp_policy;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AiProvider {
    #[serde(alias = "anthropic")]
    Claude,
    #[serde(rename = "anthropic-compatible")]
    AnthropicCompatible,
    Openai,
    Gemini,
    Deepseek,
    Kimi,
    Qwen,
    Zhipu,
    MiniMax,
    Ollama,
    #[serde(rename = "openai-compatible")]
    OpenaiCompatible,
    #[serde(rename = "codex-cli")]
    CodexCli,
    #[serde(rename = "claude-code-cli")]
    ClaudeCodeCli,
    #[serde(rename = "pi-agent-cli")]
    PiAgentCli,
    #[serde(rename = "opencode-cli")]
    OpenCodeCli,
    #[serde(rename = "cursor-cli")]
    CursorCli,
    #[serde(rename = "grok-cli")]
    GrokCli,
    #[serde(rename = "codebuddy-cli")]
    CodeBuddyCli,
    #[serde(rename = "qoder-cli")]
    QoderCli,
    Custom,
}

impl AiProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            AiProvider::Claude => "claude",
            AiProvider::AnthropicCompatible => "anthropic-compatible",
            AiProvider::Openai => "openai",
            AiProvider::Gemini => "gemini",
            AiProvider::Deepseek => "deepseek",
            AiProvider::Kimi => "kimi",
            AiProvider::Qwen => "qwen",
            AiProvider::Zhipu => "zhipu",
            AiProvider::MiniMax => "minimax",
            AiProvider::Ollama => "ollama",
            AiProvider::OpenaiCompatible => "openai-compatible",
            AiProvider::ClaudeCodeCli => "claude-code-cli",
            AiProvider::PiAgentCli => "pi-agent-cli",
            AiProvider::OpenCodeCli => "opencode-cli",
            AiProvider::CursorCli => "cursor-cli",
            AiProvider::GrokCli => "grok-cli",
            AiProvider::CodeBuddyCli => "codebuddy-cli",
            AiProvider::QoderCli => "qoder-cli",
            AiProvider::CodexCli => "codex-cli",
            AiProvider::Custom => "custom",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AiApiStyle {
    #[default]
    Completions,
    Responses,
    #[serde(rename = "anthropic-messages")]
    AnthropicMessages,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AiAuthMethod {
    #[default]
    ApiKey,
    Bearer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AiReasoningLevel {
    #[default]
    Default,
    Minimal,
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

impl AiReasoningLevel {
    pub fn as_codex_effort(&self) -> Option<&'static str> {
        match self {
            AiReasoningLevel::Default => None,
            AiReasoningLevel::Minimal => Some("minimal"),
            AiReasoningLevel::Low => Some("low"),
            AiReasoningLevel::Medium => Some("medium"),
            AiReasoningLevel::High => Some("high"),
            AiReasoningLevel::Xhigh | AiReasoningLevel::Max => None,
        }
    }

    pub fn as_claude_code_effort(&self) -> Option<&'static str> {
        match self {
            AiReasoningLevel::Default | AiReasoningLevel::Minimal => None,
            AiReasoningLevel::Low => Some("low"),
            AiReasoningLevel::Medium => Some("medium"),
            AiReasoningLevel::High => Some("high"),
            AiReasoningLevel::Xhigh => Some("xhigh"),
            AiReasoningLevel::Max => Some("max"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum AiEffortLevel {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

impl std::str::FromStr for AiEffortLevel {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "xhigh" => Ok(Self::Xhigh),
            "max" => Ok(Self::Max),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AiCapabilitySource {
    ProviderApi,
    LocalCli,
    OfficialRegistry,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum AiEffortSelection {
    ProviderDefault,
    Disabled,
    Enum(String),
    Integer(i64),
    Boolean(bool),
    Text(String),
}

impl AiEffortSelection {
    pub fn explicit_string(&self) -> Option<&str> {
        match self {
            Self::Enum(value) | Self::Text(value) => Some(value),
            Self::ProviderDefault | Self::Disabled | Self::Integer(_) | Self::Boolean(_) => None,
        }
    }

    pub fn cli_value(&self) -> Option<String> {
        match self {
            Self::ProviderDefault => None,
            Self::Disabled => Some("none".to_string()),
            Self::Enum(value) | Self::Text(value) => {
                let value = value.trim();
                (!value.is_empty()).then(|| value.to_string())
            }
            Self::Integer(value) => Some(value.to_string()),
            Self::Boolean(value) => Some(if *value { "high" } else { "none" }.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AiEffortOption {
    pub id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub selection: AiEffortSelection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AiEffortCapability {
    Enum {
        options: Vec<AiEffortOption>,
        default: AiEffortSelection,
        source: AiCapabilitySource,
    },
    Integer {
        min: i64,
        max: i64,
        step: i64,
        default: AiEffortSelection,
        #[serde(default, rename = "specialValues", skip_serializing_if = "Vec::is_empty")]
        special_values: Vec<AiEffortOption>,
        source: AiCapabilitySource,
    },
    Boolean {
        default: AiEffortSelection,
        source: AiCapabilitySource,
    },
    FreeText {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        placeholder: Option<String>,
        source: AiCapabilitySource,
    },
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AiActiveModelSelection {
    pub config_id: String,
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AiModelEffortPreference {
    pub config_id: String,
    pub model_id: String,
    pub selection: AiEffortSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AiAssistantMode {
    Ask,
    Agent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AiChatSelectionState {
    #[serde(default = "default_ai_chat_selection_version")]
    pub version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<AiActiveModelSelection>,
    #[serde(default)]
    pub effort_preferences: Vec<AiModelEffortPreference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<AiAssistantMode>,
    /// Whether new AI conversations start with the action picker on `auto`
    /// (intent routing). Defaults to false: a concrete action stays the norm.
    #[serde(default)]
    pub default_auto_routing: bool,
    /// Whether opening the AI panel should restore the most recently updated conversation.
    #[serde(default)]
    pub restore_last_conversation: bool,
    /// Prompt template ids auto-applied when the AI panel opens, keyed by
    /// connection db_type. BTreeMap keeps serialized key order stable.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub default_templates_by_db_type: BTreeMap<String, Vec<String>>,
    /// Prompt template ids from the most recent send, keyed by connection
    /// db_type; used as fallback when no defaults are configured.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub last_used_templates_by_db_type: BTreeMap<String, Vec<String>>,
}

impl Default for AiChatSelectionState {
    fn default() -> Self {
        Self {
            version: default_ai_chat_selection_version(),
            active: None,
            effort_preferences: Vec::new(),
            default_mode: None,
            default_auto_routing: false,
            restore_last_conversation: false,
            default_templates_by_db_type: BTreeMap::new(),
            last_used_templates_by_db_type: BTreeMap::new(),
        }
    }
}

fn default_ai_chat_selection_version() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiConfigItem {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub is_default: bool,
    #[serde(flatten)]
    pub config: AiConfig,
}

impl AiConfigItem {
    pub fn new_id() -> String {
        Uuid::new_v4().to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiModelListItem {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_effort_levels: Vec<AiEffortLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiConfig {
    pub provider: AiProvider,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub auth_method: AiAuthMethod,
    #[serde(default)]
    pub endpoint: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub models: Vec<AiModelListItem>,
    #[serde(default)]
    pub api_style: AiApiStyle,
    /// Additional headers attached to every API-backed AI request. Values can
    /// contain gateway credentials, so callers must never log this map.
    #[serde(default)]
    pub custom_headers: HashMap<String, String>,
    #[serde(default)]
    pub proxy_enabled: bool,
    #[serde(default)]
    pub proxy_url: String,
    /// Disable TLS certificate verification for the AI endpoint. Intended for
    /// trusted self-signed / private-CA gateways only.
    #[serde(default)]
    pub skip_tls_verify: bool,
    #[serde(default = "default_enable_thinking")]
    pub enable_thinking: bool,
    #[serde(default)]
    pub reasoning_level: AiReasoningLevel,
    /// Per-request effort selected in the assistant. Provider settings do not
    /// persist this field; it is attached to a runtime config clone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_effort: Option<AiEffortSelection>,
    /// Optional output token budget configured for this provider.
    #[serde(default)]
    pub max_output_tokens: Option<u32>,
    #[serde(default)]
    pub context_window: Option<u32>,
    /// Maximum number of automatic retries on transient API errors
    /// (rate limits, timeouts, network blips). `None` uses the default (2).
    /// Set to `Some(0)` to disable automatic retries entirely.
    #[serde(default)]
    pub max_retries: Option<u32>,
    #[serde(default)]
    pub codex_cli_path: Option<String>,
    #[serde(default)]
    pub codex_cli_env: HashMap<String, String>,
    #[serde(default)]
    pub claude_code_cli_path: Option<String>,
    #[serde(default)]
    pub claude_code_cli_env: HashMap<String, String>,
    #[serde(default)]
    pub pi_agent_cli_path: Option<String>,
    #[serde(default)]
    pub pi_agent_cli_env: HashMap<String, String>,
    #[serde(default)]
    pub opencode_cli_path: Option<String>,
    #[serde(default)]
    pub opencode_cli_env: HashMap<String, String>,
    #[serde(default)]
    pub cursor_cli_path: Option<String>,
    #[serde(default)]
    pub cursor_cli_env: HashMap<String, String>,
    #[serde(default)]
    pub grok_cli_path: Option<String>,
    #[serde(default)]
    pub grok_cli_env: HashMap<String, String>,
    #[serde(default)]
    pub codebuddy_cli_path: Option<String>,
    #[serde(default)]
    pub codebuddy_cli_env: HashMap<String, String>,
    #[serde(default)]
    pub qoder_cli_path: Option<String>,
    #[serde(default)]
    pub qoder_cli_env: HashMap<String, String>,
}

fn default_enable_thinking() -> bool {
    true
}

/// Whether the provider is a CLI-based provider that goes through its own
/// executable (claude-code, codex, cursor, opencode, pi) rather than through `with_retry` /
/// `with_stream_retry`.
pub fn is_cli_provider(provider: &AiProvider) -> bool {
    matches!(
        provider,
        AiProvider::CodexCli
            | AiProvider::ClaudeCodeCli
            | AiProvider::PiAgentCli
            | AiProvider::OpenCodeCli
            | AiProvider::CursorCli
            | AiProvider::GrokCli
            | AiProvider::CodeBuddyCli
            | AiProvider::QoderCli
    )
}

/// Merge the global `max_retries` setting into an `AiConfig`.
///
/// Applied by all API-backed entry points at request time.  CLI providers are
/// skipped because they use their own retry logic and never reach the
/// `with_retry` / `with_stream_retry` paths.
///
/// Some API-backed entry points (model listing, effort resolution) do not
/// currently call `with_retry` and so the merged value is a no-op for them.
/// The merge is still applied uniformly so that every API-backed command
/// follows the same entry-point pattern — if retry logic is added to those
/// functions later, the global setting takes effect automatically.
pub fn merge_global_max_retries(config: &mut AiConfig, max_retries: u32) {
    if !is_cli_provider(&config.provider) {
        config.max_retries = Some(max_retries);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiMessage {
    pub role: String,
    pub content: String,
    /// Transient multimodal input. Conversation persistence intentionally
    /// omits this field so image payloads are not replayed on later turns.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<AiInlineImage>,
    /// Tool call ID for tool results (role="tool"). Used to associate
    /// a tool result with its originating tool call in multi-turn loops.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool calls made by the assistant (role="assistant"). Used to
    /// reconstruct tool_use content blocks for providers like Anthropic
    /// that require them in the conversation history.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCallRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiInlineImage {
    pub media_type: String,
    pub data: String,
}

/// A lightweight reference to a tool call within an assistant message.
/// Stores the id, name, and arguments needed to reconstruct provider-specific
/// tool_use content blocks (e.g. Anthropic's `{"type":"tool_use", ...}`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallRef {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
    /// Opaque provider response data that must be replayed unchanged in
    /// follow-up requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AiTaskContract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_request: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiCompletionRequest {
    pub config: AiConfig,
    pub system_prompt: String,
    pub messages: Vec<AiMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_contract: Option<AiTaskContract>,
    pub max_tokens: Option<u32>,
    /// Stable key for reusing the provider's prompt cache within one conversation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiStreamChunk {
    pub session_id: String,
    pub delta: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_delta: Option<String>,
    /// Provider termination reason observed on the final stream event.
    /// `length` means output was cut by the provider limit, not that the model
    /// completed a final answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiChatSourceBinding {
    pub connection_id: String,
    pub database: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiChatMessage {
    pub role: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Set on the assistant message whose generation failed; persisted so the
    /// export failure marker survives reloads and locale switches (#6467 PR3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub covered_messages: Option<usize>,
    /// Frozen target of the assistant turn. A Web confirmation card can remain
    /// actionable after the conversation itself is rebound (#9902).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_binding: Option<AiChatSourceBinding>,
    /// Footprint of a turn that carried a context selection (#10058): the
    /// selection text is session-only (up to 12 000 chars, and conversation
    /// records are synced), so only the fact that one existed is persisted, and
    /// a reloaded transcript can still tell the model the content is gone
    /// instead of showing an empty turn. Absent on records written before the
    /// field existed.
    ///
    /// NOTE (2026-10-01): the sync surface carries no conversations member today
    /// — `cloud_sync::SyncSnapshot` has no such field, and there is no projection
    /// to exclude one from — so the reason this field holds today is write
    /// amplification rather than sync exposure. If conversations ever do join the
    /// snapshot, the original reason applies again as written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selections_omitted: Option<bool>,
    /// Which skills this conversation has had loaded (prd 09-30 Req 13), written
    /// onto the newest assistant message of every snapshot.
    ///
    /// Only the FACT is persisted, never the body: a body can reach 1 MiB per
    /// skill, `ai_conversations.messages_json` is an unbounded TEXT column with no
    /// per-message or per-record cap, and the whole message array is re-serialised
    /// on every save (send, streamed snapshot, chip click). A body here would be
    /// written in full each time, and would ride along with anything that ships a
    /// conversation record. Same shape as `selections_omitted` above, which keeps a
    /// footprint instead of its payload; the panel holds the body in memory for the
    /// current session and re-reads it from disk when a request needs it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loaded_skill_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiConversation {
    /// Immutable plugin data retained independently of sent and pending messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_context: Option<serde_json::Value>,
    pub id: String,
    pub title: String,
    pub connection_name: String,
    pub database: String,
    /// Connection this conversation is bound to (#9902). The binding belongs to
    /// the conversation — not to whatever editor tab happens to be active — so
    /// several conversations can run against different connections at once.
    ///
    /// Empty for conversations persisted before session-scoped binding existed,
    /// and for legacy records whose `connection_name` matched zero or several
    /// saved connections (a name is not unique). Callers must treat empty as
    /// "unbound" and ask the user, never fall back to the active tab.
    #[serde(default)]
    pub connection_id: String,
    /// Schema for schema-scoped engines (Postgres, Dameng). `None` when the
    /// engine has no schema layer or the user has not picked one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub messages: Vec<AiChatMessage>,
    /// One editable "send later" input saved while an active run occupies the
    /// conversation (parent PRD §5). Persisted with the conversation so it
    /// survives a restart and is restored when the conversation reopens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queued_input: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AiRunStatus {
    Preparing,
    Queued,
    Running,
    AwaitingWriteConfirmation,
    Completed,
    Failed,
    Cancelled,
    Interrupted,
    /// A normal-send FIFO item recovered after a process restart. The user's
    /// input was never actually submitted, so the run is surfaced as an
    /// editable, unsent pending draft — never as a failure/interruption.
    /// (Parent PRD §7: "恢复为其会话中可编辑、未发送的待发输入".)
    PendingRecoverable,
}

impl AiRunStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preparing => "preparing",
            Self::Queued => "queued",
            Self::Running => "running",
            Self::AwaitingWriteConfirmation => "awaiting_write_confirmation",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Interrupted => "interrupted",
            Self::PendingRecoverable => "pending_recoverable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "preparing" => Ok(Self::Preparing),
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "awaiting_write_confirmation" => Ok(Self::AwaitingWriteConfirmation),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "interrupted" => Ok(Self::Interrupted),
            "pending_recoverable" => Ok(Self::PendingRecoverable),
            other => Err(format!("Unknown AI run status: {other}")),
        }
    }

    pub fn protects_conversation(self) -> bool {
        matches!(
            self,
            Self::Preparing | Self::Queued | Self::Running | Self::AwaitingWriteConfirmation | Self::PendingRecoverable
        )
    }
}

/// Why a run occupies the global FIFO queue. Normal sends and accepted
/// write-confirmation resumes recover differently after a restart (parent
/// PRD §3 line 58), so the category must be persisted.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AiRunFifoCategory {
    /// A user-sent message waiting for a global concurrency slot.
    NormalSend,
    /// An accepted write confirmation whose resume segment is queued.
    WriteConfirmationResume,
}

impl AiRunFifoCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NormalSend => "normal_send",
            Self::WriteConfirmationResume => "write_confirmation_resume",
        }
    }

    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "normal_send" => Ok(Self::NormalSend),
            "write_confirmation_resume" => Ok(Self::WriteConfirmationResume),
            other => Err(format!("Unknown AI run FIFO category: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRun {
    pub run_id: String,
    pub conversation_id: String,
    #[serde(default)]
    pub session_ids: Vec<String>,
    pub status: AiRunStatus,
    #[serde(default)]
    pub connection_id: String,
    #[serde(default)]
    pub database: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_confirmation: Option<serde_json::Value>,
    /// Which queue category this run occupied. Persisted so restart recovery
    /// can distinguish a normal send (→ editable pending input) from an
    /// accepted write-confirmation resume (→ back to awaiting confirmation).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fifo_category: Option<AiRunFifoCategory>,
    /// The user's input text for a normal-send FIFO item, recovered after
    /// restart as an editable, unsent pending draft.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_input: Option<String>,
    /// Highest event `seq` ever assigned to this run, across all its sessions
    /// (parent PRD §8). Strictly increasing from 1; the run manager persists it
    /// to drive event dedup, the unread baseline, and the "updates while you
    /// were away" separator anchor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_seq: Option<u64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AiModelInfo {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_effort_levels: Vec<AiEffortLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort_capability: Option<AiEffortCapability>,
}

impl AiModelInfo {
    pub fn new(id: impl Into<String>, display_name: Option<String>) -> Self {
        Self { id: id.into(), display_name, supported_effort_levels: Vec::new(), effort_capability: None }
    }
}

/// Result of an AI connection test (mirrors CC-Switch's StreamCheckResult).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiTestConnectionResult {
    pub success: bool,
    pub message: String,
    /// First-chunk latency in milliseconds, if successful.
    pub latency_ms: Option<u64>,
    pub model_used: String,
    /// Error category for the frontend to render specific guidance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_category: Option<String>,
}
