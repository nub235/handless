use log::{debug, warn};
use serde::de::{self, DeserializeOwned, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};
use specta::Type;
use std::collections::{HashMap, HashSet};
use std::fs;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Default, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum ActivationMode {
    Toggle,
    Hold,
    #[default]
    HoldOrToggle,
}

impl<'de> Deserialize<'de> for ActivationMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActivationModeVisitor;

        impl<'de> Visitor<'de> for ActivationModeVisitor {
            type Value = ActivationMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or boolean representing activation mode")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<ActivationMode, E> {
                match value.to_lowercase().as_str() {
                    "toggle" => Ok(ActivationMode::Toggle),
                    "hold" => Ok(ActivationMode::Hold),
                    "hold_or_toggle" => Ok(ActivationMode::HoldOrToggle),
                    _ => Err(E::unknown_variant(
                        value,
                        &["toggle", "hold", "hold_or_toggle"],
                    )),
                }
            }

            fn visit_bool<E: de::Error>(self, value: bool) -> Result<ActivationMode, E> {
                Ok(if value {
                    ActivationMode::Hold
                } else {
                    ActivationMode::Toggle
                })
            }
        }

        deserializer.deserialize_any(ActivationModeVisitor)
    }
}

pub use crate::post_process::prompts::LLMPrompt;
pub use crate::post_process::providers::PostProcessProvider;

pub const APPLE_INTELLIGENCE_PROVIDER_ID: &str = "apple_intelligence";
pub const APPLE_INTELLIGENCE_DEFAULT_MODEL_ID: &str = "Apple Intelligence";

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

// Custom deserializer to handle both old numeric format (1-5) and new string format ("trace", "debug", etc.)
impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogLevelVisitor;

        impl<'de> Visitor<'de> for LogLevelVisitor {
            type Value = LogLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or integer representing log level")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<LogLevel, E> {
                match value.to_lowercase().as_str() {
                    "trace" => Ok(LogLevel::Trace),
                    "debug" => Ok(LogLevel::Debug),
                    "info" => Ok(LogLevel::Info),
                    "warn" => Ok(LogLevel::Warn),
                    "error" => Ok(LogLevel::Error),
                    _ => Err(E::unknown_variant(
                        value,
                        &["trace", "debug", "info", "warn", "error"],
                    )),
                }
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<LogLevel, E> {
                match value {
                    1 => Ok(LogLevel::Trace),
                    2 => Ok(LogLevel::Debug),
                    3 => Ok(LogLevel::Info),
                    4 => Ok(LogLevel::Warn),
                    5 => Ok(LogLevel::Error),
                    _ => Err(E::invalid_value(de::Unexpected::Unsigned(value), &"1-5")),
                }
            }
        }

        deserializer.deserialize_any(LogLevelVisitor)
    }
}

impl From<LogLevel> for tauri_plugin_log::LogLevel {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tauri_plugin_log::LogLevel::Trace,
            LogLevel::Debug => tauri_plugin_log::LogLevel::Debug,
            LogLevel::Info => tauri_plugin_log::LogLevel::Info,
            LogLevel::Warn => tauri_plugin_log::LogLevel::Warn,
            LogLevel::Error => tauri_plugin_log::LogLevel::Error,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Type)]
pub struct ShortcutBinding {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub default_binding: String,
    #[serde(default)]
    pub current_binding: String,
    #[serde(default)]
    pub post_process_prompt_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Type)]
pub struct SttProvider {
    pub id: String,
    pub label: String,
    pub provider_type: SttProviderType,
    pub base_url: String,
    pub default_model: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum SttProviderType {
    Local,
    Cloud,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "lowercase")]
pub enum OverlayPosition {
    None,
    Top,
    Bottom,
    Notch,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum ModelUnloadTimeout {
    #[default]
    Never,
    Immediately,
    Min2,
    Min5,
    Min10,
    Min15,
    Hour1,
    Sec5, // Debug mode only
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum PasteMethod {
    CtrlV,
    Direct,
    None,
    ShiftInsert,
    CtrlShiftV,
    ExternalScript,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardHandling {
    #[default]
    DontModify,
    CopyToClipboard,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum AutoSubmitKey {
    #[default]
    Enter,
    CtrlEnter,
    CmdEnter,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum RecordingRetentionPeriod {
    Never,
    PreserveLimit,
    Days3,
    Weeks2,
    Months3,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardImplementation {
    Tauri,
    HandyKeys,
}

impl Default for KeyboardImplementation {
    fn default() -> Self {
        // Default to HandyKeys only on macOS where it's well-tested.
        // Windows and Linux use Tauri by default (handy-keys not sufficiently tested yet).
        #[cfg(target_os = "macos")]
        return KeyboardImplementation::HandyKeys;
        #[cfg(not(target_os = "macos"))]
        return KeyboardImplementation::Tauri;
    }
}

impl Default for PasteMethod {
    fn default() -> Self {
        // Default to CtrlV for macOS and Windows, Direct for Linux
        #[cfg(target_os = "linux")]
        return PasteMethod::Direct;
        #[cfg(not(target_os = "linux"))]
        return PasteMethod::CtrlV;
    }
}

impl ModelUnloadTimeout {
    pub fn to_minutes(self) -> Option<u64> {
        match self {
            ModelUnloadTimeout::Never => None,
            ModelUnloadTimeout::Immediately => Some(0), // Special case for immediate unloading
            ModelUnloadTimeout::Min2 => Some(2),
            ModelUnloadTimeout::Min5 => Some(5),
            ModelUnloadTimeout::Min10 => Some(10),
            ModelUnloadTimeout::Min15 => Some(15),
            ModelUnloadTimeout::Hour1 => Some(60),
            ModelUnloadTimeout::Sec5 => Some(0), // Special case for debug - handled separately
        }
    }

    pub fn to_seconds(self) -> Option<u64> {
        match self {
            ModelUnloadTimeout::Never => None,
            ModelUnloadTimeout::Immediately => Some(0), // Special case for immediate unloading
            ModelUnloadTimeout::Sec5 => Some(5),
            _ => self.to_minutes().map(|m| m * 60),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum SoundTheme {
    Marimba,
    Pop,
    Custom,
}

impl SoundTheme {
    fn as_str(&self) -> &'static str {
        match self {
            SoundTheme::Marimba => "marimba",
            SoundTheme::Pop => "pop",
            SoundTheme::Custom => "custom",
        }
    }

    pub fn to_start_path(self) -> String {
        format!("resources/{}_start.wav", self.as_str())
    }

    pub fn to_stop_path(self) -> String {
        format!("resources/{}_stop.wav", self.as_str())
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum AppTheme {
    Dark,
    Light,
    #[default]
    System,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum TypingTool {
    #[default]
    Auto,
    Wtype,
    Kwtype,
    Dotool,
    Ydotool,
    Xdotool,
}

/* still useful for composing the initial JSON in the store ------------- */
#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct AppSettings {
    #[serde(default = "default_bindings")]
    pub bindings: HashMap<String, ShortcutBinding>,
    #[serde(default, alias = "push_to_talk")]
    pub activation_mode: ActivationMode,
    #[serde(default = "default_audio_feedback_volume")]
    pub audio_feedback_volume: f32,
    #[serde(default = "default_sound_theme")]
    pub sound_theme: SoundTheme,
    #[serde(default = "default_start_hidden")]
    pub start_hidden: bool,
    #[serde(default = "default_autostart_enabled")]
    pub autostart_enabled: bool,
    #[serde(default = "default_update_checks_enabled")]
    pub update_checks_enabled: bool,
    #[serde(default = "default_model")]
    pub selected_model: String,
    #[serde(default = "default_always_on_microphone")]
    pub always_on_microphone: bool,
    #[serde(default)]
    pub selected_microphone: Option<String>,
    #[serde(default)]
    pub microphone_priority: Vec<String>,
    #[serde(default)]
    pub clamshell_microphone: Option<String>,
    #[serde(default)]
    pub selected_output_device: Option<String>,
    #[serde(default = "default_translate_to_english")]
    pub translate_to_english: bool,
    #[serde(default = "default_selected_language")]
    pub selected_language: String,
    #[serde(default = "default_overlay_enabled")]
    pub overlay_enabled: bool,
    #[serde(default = "default_overlay_position")]
    pub overlay_position: OverlayPosition,
    #[serde(default = "default_debug_mode")]
    pub debug_mode: bool,
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,
    #[serde(default)]
    pub custom_words: Vec<String>,
    #[serde(default)]
    pub model_unload_timeout: ModelUnloadTimeout,
    #[serde(default = "default_word_correction_threshold")]
    pub word_correction_threshold: f64,
    #[serde(default = "default_history_limit")]
    pub history_limit: usize,
    #[serde(default = "default_recording_retention_period")]
    pub recording_retention_period: RecordingRetentionPeriod,
    #[serde(default)]
    pub paste_method: PasteMethod,
    #[serde(default)]
    pub clipboard_handling: ClipboardHandling,
    #[serde(default = "default_auto_submit")]
    pub auto_submit: bool,
    #[serde(default)]
    pub auto_submit_key: AutoSubmitKey,
    #[serde(default = "default_stt_provider_id")]
    pub stt_provider_id: String,
    #[serde(default = "default_stt_providers")]
    pub stt_providers: Vec<SttProvider>,
    #[serde(default = "default_stt_api_keys")]
    pub stt_api_keys: HashMap<String, String>,
    #[serde(default = "default_stt_cloud_models")]
    pub stt_cloud_models: HashMap<String, String>,
    #[serde(default = "default_post_process_enabled")]
    pub post_process_enabled: bool,
    #[serde(default = "default_post_process_provider_id")]
    pub post_process_provider_id: String,
    #[serde(default = "default_post_process_providers")]
    pub post_process_providers: Vec<PostProcessProvider>,
    #[serde(default = "default_post_process_api_keys")]
    pub post_process_api_keys: HashMap<String, String>,
    #[serde(default = "default_post_process_models")]
    pub post_process_models: HashMap<String, String>,
    #[serde(default = "default_post_process_prompts")]
    pub post_process_prompts: Vec<LLMPrompt>,
    #[serde(default = "default_post_process_selected_prompt_id")]
    pub post_process_selected_prompt_id: Option<String>,
    #[serde(default)]
    pub mute_while_recording: bool,
    #[serde(default)]
    pub append_trailing_space: bool,
    #[serde(default = "default_app_language")]
    pub app_language: String,
    #[serde(default)]
    pub keyboard_implementation: KeyboardImplementation,
    #[serde(default = "default_show_tray_icon")]
    pub show_tray_icon: bool,
    #[serde(default = "default_paste_delay_ms")]
    pub paste_delay_ms: u64,
    #[serde(default = "default_typing_tool")]
    pub typing_tool: TypingTool,
    pub external_script_path: Option<String>,
    #[serde(default)]
    pub app_theme: AppTheme,
    #[serde(default)]
    pub stt_verified_providers: HashSet<String>,
    #[serde(default)]
    pub post_process_verified_providers: HashSet<String>,
    #[serde(default)]
    pub post_process_input_prices: HashMap<String, f64>,
    #[serde(default)]
    pub post_process_output_prices: HashMap<String, f64>,
    #[serde(default = "default_stt_cloud_options")]
    pub stt_cloud_options: HashMap<String, String>,
    #[serde(default)]
    pub stt_realtime_enabled: HashMap<String, bool>,
    #[serde(default = "default_stt_realtime_chunk_ms")]
    pub stt_realtime_chunk_ms: HashMap<String, u64>,
    #[serde(default)]
    pub stats_date_range: StatsDateRange,
    #[serde(default)]
    pub dictionary_terms: Vec<String>,
    #[serde(default)]
    pub dictionary_context: String,
}

fn default_model() -> String {
    "".to_string()
}

fn default_always_on_microphone() -> bool {
    false
}

fn default_translate_to_english() -> bool {
    false
}

fn default_start_hidden() -> bool {
    false
}

fn default_autostart_enabled() -> bool {
    false
}

fn default_update_checks_enabled() -> bool {
    true
}

fn default_selected_language() -> String {
    "auto".to_string()
}

fn default_overlay_enabled() -> bool {
    true
}

fn default_overlay_enabled_for_new_install() -> bool {
    #[cfg(target_os = "linux")]
    return false;
    #[cfg(not(target_os = "linux"))]
    return true;
}

fn default_overlay_position() -> OverlayPosition {
    #[cfg(target_os = "linux")]
    return OverlayPosition::None;
    #[cfg(not(target_os = "linux"))]
    return OverlayPosition::Bottom;
}

fn default_debug_mode() -> bool {
    false
}

fn default_log_level() -> LogLevel {
    LogLevel::Debug
}

fn default_word_correction_threshold() -> f64 {
    0.18
}

fn default_paste_delay_ms() -> u64 {
    60
}

fn default_auto_submit() -> bool {
    false
}

fn default_history_limit() -> usize {
    5
}

fn default_recording_retention_period() -> RecordingRetentionPeriod {
    RecordingRetentionPeriod::Never
}

fn default_audio_feedback_volume() -> f32 {
    1.0
}

fn default_sound_theme() -> SoundTheme {
    SoundTheme::Marimba
}

fn default_post_process_enabled() -> bool {
    false
}

fn default_app_language() -> String {
    tauri_plugin_os::locale()
        .map(|l| l.replace('_', "-"))
        .unwrap_or_else(|| "en".to_string())
}

fn default_show_tray_icon() -> bool {
    true
}

fn default_post_process_provider_id() -> String {
    "openai".to_string()
}

fn default_post_process_providers() -> Vec<PostProcessProvider> {
    crate::post_process::providers::default_providers()
}

fn default_post_process_api_keys() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for provider in default_post_process_providers() {
        map.insert(provider.id, String::new());
    }
    map
}

fn default_post_process_models() -> HashMap<String, String> {
    use crate::post_process::providers::default_model_for_provider;
    let mut map = HashMap::new();
    for provider in default_post_process_providers() {
        map.insert(
            provider.id.clone(),
            default_model_for_provider(&provider.id),
        );
    }
    map
}

fn default_post_process_prompts() -> Vec<LLMPrompt> {
    crate::post_process::prompts::default_prompts()
}

fn default_post_process_selected_prompt_id() -> Option<String> {
    crate::post_process::prompts::default_selected_prompt_id()
}

fn default_bindings() -> HashMap<String, ShortcutBinding> {
    #[cfg(target_os = "windows")]
    let default_shortcut = "ctrl+space";
    #[cfg(target_os = "macos")]
    let default_shortcut = "fn";
    #[cfg(target_os = "linux")]
    let default_shortcut = "ctrl+space";
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let default_shortcut = "alt+space";

    let mut bindings = HashMap::new();
    bindings.insert(
        "transcribe".to_string(),
        ShortcutBinding {
            id: "transcribe".to_string(),
            name: "Transcribe".to_string(),
            description: "Converts your speech into text.".to_string(),
            default_binding: default_shortcut.to_string(),
            current_binding: default_shortcut.to_string(),
            post_process_prompt_id: None,
        },
    );
    #[cfg(target_os = "windows")]
    let default_post_process_shortcut = "ctrl+shift+space";
    #[cfg(target_os = "macos")]
    let default_post_process_shortcut = "option+shift+space";
    #[cfg(target_os = "linux")]
    let default_post_process_shortcut = "ctrl+shift+space";
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let default_post_process_shortcut = "alt+shift+space";

    bindings.insert(
        "transcribe_with_post_process".to_string(),
        ShortcutBinding {
            id: "transcribe_with_post_process".to_string(),
            name: "Transcribe with Post-Processing".to_string(),
            description: "Converts your speech into text and applies AI post-processing."
                .to_string(),
            default_binding: default_post_process_shortcut.to_string(),
            current_binding: default_post_process_shortcut.to_string(),
            post_process_prompt_id: None,
        },
    );
    bindings.insert(
        "cancel".to_string(),
        ShortcutBinding {
            id: "cancel".to_string(),
            name: "Cancel".to_string(),
            description: "Cancels the current recording.".to_string(),
            default_binding: "escape".to_string(),
            current_binding: "escape".to_string(),
            post_process_prompt_id: None,
        },
    );

    bindings
}

fn default_typing_tool() -> TypingTool {
    TypingTool::Auto
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum StatsDateRange {
    Today,
    #[serde(rename = "3days")]
    ThreeDays,
    Week,
    #[default]
    Month,
    All,
    Custom,
}

fn default_stt_provider_id() -> String {
    "local".to_string()
}

fn default_stt_realtime_chunk_ms() -> HashMap<String, u64> {
    let mut map = HashMap::new();
    map.insert("parakeet-unified-en-0.6b-int8".to_string(), 560);
    map
}

fn default_stt_providers() -> Vec<SttProvider> {
    vec![
        SttProvider {
            id: "local".to_string(),
            label: "Local (on-device)".to_string(),
            provider_type: SttProviderType::Local,
            base_url: String::new(),
            default_model: String::new(),
        },
        SttProvider {
            id: "openai_stt".to_string(),
            label: "OpenAI".to_string(),
            provider_type: SttProviderType::Cloud,
            base_url: "https://api.openai.com/v1".to_string(),
            default_model: crate::cloud_stt::OPENAI_REALTIME_TRANSCRIPTION_MODEL.to_string(),
        },
        SttProvider {
            id: "cartesia".to_string(),
            label: "Cartesia".to_string(),
            provider_type: SttProviderType::Cloud,
            base_url: "https://api.cartesia.ai".to_string(),
            default_model: "ink-whisper".to_string(),
        },
        SttProvider {
            id: "mistral".to_string(),
            label: "Mistral AI".to_string(),
            provider_type: SttProviderType::Cloud,
            base_url: "https://api.mistral.ai".to_string(),
            default_model: "voxtral-mini-latest".to_string(),
        },
        SttProvider {
            id: "elevenlabs".to_string(),
            label: "ElevenLabs".to_string(),
            provider_type: SttProviderType::Cloud,
            base_url: "https://api.elevenlabs.io/v1".to_string(),
            default_model: "scribe_v2".to_string(),
        },
        SttProvider {
            id: "groq".to_string(),
            label: "Groq".to_string(),
            provider_type: SttProviderType::Cloud,
            base_url: "https://api.groq.com/openai/v1".to_string(),
            default_model: "whisper-large-v3-turbo".to_string(),
        },
        SttProvider {
            id: "soniox".to_string(),
            label: "Soniox".to_string(),
            provider_type: SttProviderType::Cloud,
            base_url: "https://api.soniox.com/v1".to_string(),
            default_model: "stt-rt-v4".to_string(),
        },
        SttProvider {
            id: "deepgram".to_string(),
            label: "Deepgram".to_string(),
            provider_type: SttProviderType::Cloud,
            base_url: "https://api.deepgram.com/v1".to_string(),
            default_model: "nova-3".to_string(),
        },
        SttProvider {
            id: "assemblyai".to_string(),
            label: "AssemblyAI".to_string(),
            provider_type: SttProviderType::Cloud,
            base_url: "https://api.assemblyai.com".to_string(),
            default_model: "universal-3-pro".to_string(),
        },
        SttProvider {
            id: "fireworks".to_string(),
            label: "Fireworks AI".to_string(),
            provider_type: SttProviderType::Cloud,
            base_url: "https://audio-prod.api.fireworks.ai/v1".to_string(),
            default_model: "whisper-v3".to_string(),
        },
        SttProvider {
            id: "doubao".to_string(),
            label: "Doubao".to_string(),
            provider_type: SttProviderType::Cloud,
            base_url: "wss://openspeech.bytedance.com/api/v3/sauc/bigmodel_async".to_string(),
            default_model: "bigmodel".to_string(),
        },
    ]
}

fn default_stt_cloud_options() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for provider in default_stt_providers() {
        if provider.provider_type == SttProviderType::Cloud {
            map.insert(provider.id, "{}".to_string());
        }
    }
    map
}

fn default_stt_api_keys() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for provider in default_stt_providers() {
        if provider.provider_type == SttProviderType::Cloud {
            map.insert(provider.id, String::new());
        }
    }
    map
}

fn default_stt_cloud_models() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for provider in default_stt_providers() {
        if provider.provider_type == SttProviderType::Cloud {
            map.insert(provider.id, provider.default_model);
        }
    }
    map
}

fn ensure_stt_defaults(settings: &mut AppSettings) -> bool {
    let mut changed = false;
    for provider in default_stt_providers() {
        match settings
            .stt_providers
            .iter_mut()
            .find(|p| p.id == provider.id)
        {
            Some(existing) => {
                // Sync default_model for existing providers (migration)
                if existing.default_model != provider.default_model {
                    existing.default_model = provider.default_model.clone();
                    changed = true;
                }
            }
            None => {
                settings.stt_providers.push(provider.clone());
                changed = true;
            }
        }

        if provider.provider_type == SttProviderType::Cloud {
            if !settings.stt_api_keys.contains_key(&provider.id) {
                settings
                    .stt_api_keys
                    .insert(provider.id.clone(), String::new());
                changed = true;
            }

            if !settings.stt_cloud_models.contains_key(&provider.id) {
                settings
                    .stt_cloud_models
                    .insert(provider.id.clone(), provider.default_model.clone());
                changed = true;
            }

            if !settings.stt_cloud_options.contains_key(&provider.id) {
                settings
                    .stt_cloud_options
                    .insert(provider.id.clone(), "{}".to_string());
                changed = true;
            }
        }
    }

    // Default realtime to true for providers that support it
    for info in crate::stt_provider::cloud_provider_registry() {
        if info.supports_realtime && !settings.stt_realtime_enabled.contains_key(&info.id) {
            settings.stt_realtime_enabled.insert(info.id, true);
            changed = true;
        }
    }
    if !settings
        .stt_realtime_enabled
        .contains_key("parakeet-unified-en-0.6b-int8")
    {
        settings
            .stt_realtime_enabled
            .insert("parakeet-unified-en-0.6b-int8".to_string(), true);
        changed = true;
    }
    if !settings
        .stt_realtime_chunk_ms
        .contains_key("parakeet-unified-en-0.6b-int8")
    {
        settings
            .stt_realtime_chunk_ms
            .insert("parakeet-unified-en-0.6b-int8".to_string(), 560);
        changed = true;
    }

    changed
}

fn ensure_post_process_defaults(settings: &mut AppSettings) -> bool {
    let prompt_changed = crate::post_process::prompts::ensure_prompt_defaults(settings);
    let provider_changed = crate::post_process::providers::ensure_provider_defaults(settings);
    prompt_changed || provider_changed
}

pub const SETTINGS_STORE_PATH: &str = "settings_store.json";

fn configured_provider_ids(values: &HashMap<String, String>) -> Vec<String> {
    let mut providers = values
        .iter()
        .filter_map(|(provider_id, value)| {
            if value.trim().is_empty() {
                None
            } else {
                Some(provider_id.clone())
            }
        })
        .collect::<Vec<_>>();
    providers.sort();
    providers
}

fn is_builtin_binding_id(binding_id: &str) -> bool {
    default_bindings().contains_key(binding_id)
}

fn normalized_bindings(
    stored_bindings: &HashMap<String, ShortcutBinding>,
) -> HashMap<String, ShortcutBinding> {
    let mut bindings = default_bindings();

    for (binding_id, stored_binding) in stored_bindings {
        if let Some(default_binding) = bindings.get_mut(binding_id) {
            if !stored_binding.current_binding.is_empty() {
                default_binding.current_binding = stored_binding.current_binding.clone();
            }
            default_binding.post_process_prompt_id = stored_binding.post_process_prompt_id.clone();
            continue;
        }

        let mut custom_binding = stored_binding.clone();
        if custom_binding.current_binding.is_empty() {
            custom_binding.current_binding = custom_binding.default_binding.clone();
        }
        if custom_binding.default_binding.is_empty() {
            custom_binding.default_binding = custom_binding.current_binding.clone();
        }

        bindings.insert(binding_id.clone(), custom_binding);
    }

    bindings
}

fn read_json_field<T: DeserializeOwned>(
    object: &JsonMap<String, JsonValue>,
    key: &str,
) -> Option<T> {
    object
        .get(key)
        .and_then(|value| serde_json::from_value(value.clone()).ok())
}

fn recover_custom_post_process_base_url(value: &JsonValue) -> Option<String> {
    value.as_array().and_then(|providers| {
        providers.iter().find_map(|provider| {
            let object = provider.as_object()?;
            let provider_id = object.get("id")?.as_str()?;
            if provider_id != "custom" {
                return None;
            }
            object
                .get("base_url")
                .and_then(|base_url| base_url.as_str())
                .map(str::to_string)
        })
    })
}

fn configured_custom_post_process_base_url(object: &JsonMap<String, JsonValue>) -> Option<String> {
    read_json_field::<String>(object, "post_process_custom_base_url").or_else(|| {
        object
            .get("post_process_providers")
            .and_then(recover_custom_post_process_base_url)
    })
}

fn apply_custom_post_process_base_url(
    settings: &mut AppSettings,
    custom_base_url: Option<String>,
) -> bool {
    let Some(custom_base_url) = custom_base_url else {
        return false;
    };

    let Some(custom_provider) = settings
        .post_process_providers
        .iter_mut()
        .find(|provider| provider.id == "custom")
    else {
        return false;
    };

    if custom_provider.base_url == custom_base_url {
        return false;
    }

    custom_provider.base_url = custom_base_url;
    true
}

fn normalized_post_process_providers(
    stored_providers: &[PostProcessProvider],
) -> Vec<PostProcessProvider> {
    let custom_base_url = stored_providers
        .iter()
        .find(|provider| provider.id == "custom")
        .map(|provider| provider.base_url.clone());

    let mut providers = default_post_process_providers();
    if let Some(custom_base_url) = custom_base_url {
        if let Some(custom_provider) = providers
            .iter_mut()
            .find(|provider| provider.id == "custom")
        {
            custom_provider.base_url = custom_base_url;
        }
    }

    providers
}

fn custom_post_process_base_url(settings: &AppSettings) -> Option<String> {
    settings
        .post_process_provider("custom")
        .map(|provider| provider.base_url.clone())
}

fn persisted_settings_requires_rewrite(settings_value: &JsonValue) -> bool {
    let Some(object) = settings_value.as_object() else {
        return false;
    };

    object.contains_key("stt_providers")
        || object.contains_key("post_process_providers")
        || object
            .get("post_process_prompts")
            .and_then(JsonValue::as_array)
            .is_some_and(|prompts| {
                prompts.iter().any(|prompt| {
                    prompt
                        .as_object()
                        .and_then(|prompt| prompt.get("id"))
                        .and_then(JsonValue::as_str)
                        .is_some_and(crate::post_process::is_builtin_prompt)
                })
            })
        || object
            .get("bindings")
            .and_then(JsonValue::as_object)
            .is_some_and(|bindings| {
                bindings.iter().any(|(binding_id, binding)| {
                    is_builtin_binding_id(binding_id)
                        && binding.as_object().is_some_and(|binding| {
                            binding.contains_key("name")
                                || binding.contains_key("description")
                                || binding.contains_key("default_binding")
                        })
                })
            })
}

fn persisted_bindings_value(settings: &AppSettings) -> JsonValue {
    let default_bindings = default_bindings();
    let bindings = settings
        .bindings
        .iter()
        .filter_map(|(binding_id, binding)| {
            if let Some(default_binding) = default_bindings.get(binding_id) {
                let has_override = binding.current_binding != default_binding.current_binding
                    || binding.post_process_prompt_id != default_binding.post_process_prompt_id;
                if !has_override {
                    return None;
                }

                return Some((
                    binding_id.clone(),
                    serde_json::json!({
                        "id": binding.id,
                        "current_binding": binding.current_binding,
                        "post_process_prompt_id": binding.post_process_prompt_id,
                    }),
                ));
            }

            Some((
                binding_id.clone(),
                serde_json::to_value(binding).expect("Failed to serialize shortcut binding"),
            ))
        })
        .collect::<JsonMap<String, JsonValue>>();

    JsonValue::Object(bindings)
}

fn persisted_post_process_prompts_value(settings: &AppSettings) -> JsonValue {
    JsonValue::Array(
        settings
            .post_process_prompts
            .iter()
            .filter(|prompt| !crate::post_process::is_builtin_prompt(&prompt.id))
            .map(|prompt| serde_json::to_value(prompt).expect("Failed to serialize prompt"))
            .collect(),
    )
}

pub(crate) fn persisted_settings_value(settings: &AppSettings) -> JsonValue {
    let mut value = serde_json::to_value(settings).expect("Failed to serialize settings");
    let JsonValue::Object(object) = &mut value else {
        unreachable!("AppSettings must serialize to a JSON object");
    };

    object.remove("stt_providers");
    object.remove("post_process_providers");
    object.insert("bindings".to_string(), persisted_bindings_value(settings));
    object.insert(
        "post_process_prompts".to_string(),
        persisted_post_process_prompts_value(settings),
    );

    if let Some(custom_base_url) = custom_post_process_base_url(settings) {
        object.insert(
            "post_process_custom_base_url".to_string(),
            JsonValue::String(custom_base_url),
        );
    }

    value
}

fn normalize_provider_catalogs(settings: &mut AppSettings) -> bool {
    let mut changed = false;

    let current_stt_providers = default_stt_providers();
    if settings.stt_providers != current_stt_providers {
        settings.stt_providers = current_stt_providers;
        changed = true;
    }

    let current_post_process_providers =
        normalized_post_process_providers(&settings.post_process_providers);
    if settings.post_process_providers != current_post_process_providers {
        settings.post_process_providers = current_post_process_providers;
        changed = true;
    }

    changed
}

fn normalize_settings_definitions(settings: &mut AppSettings) -> bool {
    let mut changed = false;

    let current_bindings = normalized_bindings(&settings.bindings);
    if settings.bindings != current_bindings {
        settings.bindings = current_bindings;
        changed = true;
    }

    let current_prompts =
        crate::post_process::prompts::normalized_prompts(&settings.post_process_prompts);
    if settings.post_process_prompts != current_prompts {
        settings.post_process_prompts = current_prompts;
        changed = true;
    }

    changed
}

fn recover_settings_from_value(settings_value: JsonValue) -> AppSettings {
    let mut settings = get_default_settings();
    let Some(object) = settings_value.as_object() else {
        return settings;
    };

    if let Some(value) = read_json_field(object, "activation_mode")
        .or_else(|| read_json_field(object, "push_to_talk"))
    {
        settings.activation_mode = value;
    }

    macro_rules! recover_field {
        ($field:ident) => {
            if let Some(value) = read_json_field(object, stringify!($field)) {
                settings.$field = value;
            }
        };
    }

    recover_field!(bindings);
    recover_field!(audio_feedback_volume);
    recover_field!(sound_theme);
    recover_field!(start_hidden);
    recover_field!(autostart_enabled);
    recover_field!(update_checks_enabled);
    recover_field!(selected_model);
    recover_field!(always_on_microphone);
    recover_field!(selected_microphone);
    recover_field!(microphone_priority);
    recover_field!(clamshell_microphone);
    recover_field!(selected_output_device);
    recover_field!(translate_to_english);
    recover_field!(selected_language);
    recover_field!(overlay_enabled);
    recover_field!(overlay_position);
    if !object.contains_key("overlay_enabled") {
        settings.overlay_enabled = settings.overlay_position != OverlayPosition::None;
    }
    recover_field!(debug_mode);
    recover_field!(log_level);
    recover_field!(custom_words);
    recover_field!(model_unload_timeout);
    recover_field!(word_correction_threshold);
    recover_field!(history_limit);
    recover_field!(recording_retention_period);
    recover_field!(paste_method);
    recover_field!(clipboard_handling);
    recover_field!(auto_submit);
    recover_field!(auto_submit_key);
    recover_field!(stt_provider_id);
    recover_field!(stt_api_keys);
    recover_field!(stt_cloud_models);
    recover_field!(post_process_enabled);
    recover_field!(post_process_provider_id);
    recover_field!(post_process_api_keys);
    recover_field!(post_process_models);
    recover_field!(post_process_prompts);
    recover_field!(post_process_selected_prompt_id);
    recover_field!(mute_while_recording);
    recover_field!(append_trailing_space);
    recover_field!(app_language);
    recover_field!(keyboard_implementation);
    recover_field!(show_tray_icon);
    recover_field!(paste_delay_ms);
    recover_field!(typing_tool);
    recover_field!(external_script_path);
    recover_field!(app_theme);
    recover_field!(stt_verified_providers);
    recover_field!(post_process_verified_providers);
    recover_field!(post_process_input_prices);
    recover_field!(post_process_output_prices);
    recover_field!(stt_cloud_options);
    recover_field!(stt_realtime_enabled);
    recover_field!(stt_realtime_chunk_ms);
    recover_field!(stats_date_range);
    recover_field!(dictionary_terms);
    recover_field!(dictionary_context);

    if let Some(stored_providers) =
        read_json_field::<Vec<PostProcessProvider>>(object, "post_process_providers")
    {
        settings.post_process_providers = normalized_post_process_providers(&stored_providers);
    }

    apply_custom_post_process_base_url(
        &mut settings,
        configured_custom_post_process_base_url(object),
    );

    settings
}

fn backup_invalid_settings_store(app: &AppHandle) {
    let Ok(app_data_dir) = app.path().app_data_dir() else {
        return;
    };

    let store_path = app_data_dir.join(SETTINGS_STORE_PATH);
    if !store_path.exists() {
        return;
    }

    let backup_path = app_data_dir.join(format!(
        "settings_store.invalid-{}.json",
        chrono::Local::now().format("%Y%m%d-%H%M%S")
    ));

    match fs::copy(&store_path, &backup_path) {
        Ok(_) => warn!(
            "Backed up invalid settings store to {}",
            backup_path.display()
        ),
        Err(error) => warn!(
            "Failed to back up invalid settings store to {}: {}",
            backup_path.display(),
            error
        ),
    }
}

fn apply_settings_migrations(settings: &mut AppSettings) -> bool {
    let mut updated = normalize_provider_catalogs(settings);
    updated |= normalize_settings_definitions(settings);

    if settings.microphone_priority.is_empty() {
        if let Some(ref mic) = settings.selected_microphone {
            debug!(
                "Migrating selected_microphone '{}' to microphone_priority",
                mic
            );
            settings.microphone_priority = vec![mic.clone()];
            updated = true;
        }
    }

    if let Some(binding) = settings.bindings.get_mut("transcribe_with_post_process") {
        if binding.post_process_prompt_id.is_none() {
            let prompt_id = settings
                .post_process_selected_prompt_id
                .clone()
                .unwrap_or_else(|| crate::post_process::BUILTIN_PROMPT_CORRECT.to_string());
            debug!(
                "Migrating transcribe_with_post_process prompt_id to '{}'",
                prompt_id
            );
            binding.post_process_prompt_id = Some(prompt_id);
            updated = true;
        }
    }

    if settings.stt_provider(&settings.stt_provider_id).is_none() {
        settings.stt_provider_id = default_stt_provider_id();
        updated = true;
    }

    if settings
        .post_process_provider(&settings.post_process_provider_id)
        .is_none()
    {
        settings.post_process_provider_id = default_post_process_provider_id();
        updated = true;
    }

    updated |= ensure_stt_defaults(settings);
    updated |= ensure_post_process_defaults(settings);

    updated
}

fn read_or_create_app_settings(app: &AppHandle, log_existing: bool) -> AppSettings {
    let store = app
        .store(SETTINGS_STORE_PATH)
        .expect("Failed to initialize store");

    let (mut settings, mut updated) = if let Some(settings_value) = store.get("settings") {
        match serde_json::from_value::<AppSettings>(settings_value.clone()) {
            Ok(mut settings) => {
                let settings_updated = settings_value.as_object().is_some_and(|object| {
                    let mut updated = apply_custom_post_process_base_url(
                        &mut settings,
                        configured_custom_post_process_base_url(object),
                    );

                    if !object.contains_key("overlay_enabled") {
                        settings.overlay_enabled =
                            settings.overlay_position != OverlayPosition::None;
                        updated = true;
                    }

                    updated
                });
                if log_existing {
                    debug!(
                        "Found existing settings: selected_model={}, stt_provider_id={}, post_process_provider_id={}, bindings={:?}, microphone_priority={:?}, clipboard_handling={:?}, mute_while_recording={}, app_language={}, stats_date_range={:?}, configured_stt_api_keys={:?}, configured_post_process_api_keys={:?}",
                        settings.selected_model,
                        settings.stt_provider_id,
                        settings.post_process_provider_id,
                        settings.bindings,
                        settings.microphone_priority,
                        settings.clipboard_handling,
                        settings.mute_while_recording,
                        settings.app_language,
                        settings.stats_date_range,
                        configured_provider_ids(&settings.stt_api_keys),
                        configured_provider_ids(&settings.post_process_api_keys),
                    );
                }
                (
                    settings,
                    persisted_settings_requires_rewrite(&settings_value) || settings_updated,
                )
            }
            Err(error) => {
                warn!("Failed to parse settings: {}", error);
                backup_invalid_settings_store(app);
                let recovered = recover_settings_from_value(settings_value);
                warn!("Recovered settings from partially invalid configuration data");
                (recovered, true)
            }
        }
    } else {
        (get_default_settings(), true)
    };

    updated |= apply_settings_migrations(&mut settings);

    if updated {
        store.set("settings", persisted_settings_value(&settings));
    }

    settings
}

pub fn get_default_settings() -> AppSettings {
    AppSettings {
        bindings: default_bindings(),
        activation_mode: ActivationMode::HoldOrToggle,
        audio_feedback_volume: default_audio_feedback_volume(),
        sound_theme: default_sound_theme(),
        start_hidden: default_start_hidden(),
        autostart_enabled: default_autostart_enabled(),
        update_checks_enabled: default_update_checks_enabled(),
        selected_model: "".to_string(),
        always_on_microphone: false,
        selected_microphone: None,
        microphone_priority: Vec::new(),
        clamshell_microphone: None,
        selected_output_device: None,
        translate_to_english: false,
        selected_language: "auto".to_string(),
        overlay_enabled: default_overlay_enabled_for_new_install(),
        overlay_position: default_overlay_position(),
        debug_mode: false,
        log_level: default_log_level(),
        custom_words: Vec::new(),
        model_unload_timeout: ModelUnloadTimeout::Never,
        word_correction_threshold: default_word_correction_threshold(),
        history_limit: default_history_limit(),
        recording_retention_period: default_recording_retention_period(),
        paste_method: PasteMethod::default(),
        clipboard_handling: ClipboardHandling::default(),
        auto_submit: default_auto_submit(),
        auto_submit_key: AutoSubmitKey::default(),
        stt_provider_id: default_stt_provider_id(),
        stt_providers: default_stt_providers(),
        stt_api_keys: default_stt_api_keys(),
        stt_cloud_models: default_stt_cloud_models(),
        post_process_enabled: default_post_process_enabled(),
        post_process_provider_id: default_post_process_provider_id(),
        post_process_providers: default_post_process_providers(),
        post_process_api_keys: default_post_process_api_keys(),
        post_process_models: default_post_process_models(),
        post_process_prompts: default_post_process_prompts(),
        post_process_selected_prompt_id: default_post_process_selected_prompt_id(),
        mute_while_recording: true,
        append_trailing_space: false,
        app_language: default_app_language(),
        keyboard_implementation: KeyboardImplementation::default(),
        show_tray_icon: default_show_tray_icon(),
        paste_delay_ms: default_paste_delay_ms(),
        typing_tool: default_typing_tool(),
        external_script_path: None,
        app_theme: AppTheme::default(),
        stt_verified_providers: HashSet::new(),
        post_process_verified_providers: HashSet::new(),
        post_process_input_prices: HashMap::new(),
        post_process_output_prices: HashMap::new(),
        stt_cloud_options: default_stt_cloud_options(),
        stt_realtime_enabled: HashMap::new(),
        stt_realtime_chunk_ms: default_stt_realtime_chunk_ms(),
        stats_date_range: StatsDateRange::default(),
        dictionary_terms: Vec::new(),
        dictionary_context: String::new(),
    }
}

impl AppSettings {
    pub fn stt_provider(&self, provider_id: &str) -> Option<&SttProvider> {
        self.stt_providers
            .iter()
            .find(|provider| provider.id == provider_id)
    }

    pub fn active_post_process_provider(&self) -> Option<&PostProcessProvider> {
        self.post_process_providers
            .iter()
            .find(|provider| provider.id == self.post_process_provider_id)
    }

    pub fn post_process_provider(&self, provider_id: &str) -> Option<&PostProcessProvider> {
        self.post_process_providers
            .iter()
            .find(|provider| provider.id == provider_id)
    }

    pub fn post_process_provider_mut(
        &mut self,
        provider_id: &str,
    ) -> Option<&mut PostProcessProvider> {
        self.post_process_providers
            .iter_mut()
            .find(|provider| provider.id == provider_id)
    }
}

pub fn load_or_create_app_settings(app: &AppHandle) -> AppSettings {
    read_or_create_app_settings(app, true)
}

pub fn reload_from_disk(app: &AppHandle) -> Result<AppSettings, String> {
    let store = app
        .store(SETTINGS_STORE_PATH)
        .map_err(|e| format!("Failed to access settings store: {}", e))?;

    store
        .reload()
        .map_err(|e| format!("Failed to reload settings from disk: {}", e))?;

    Ok(load_or_create_app_settings(app))
}

pub fn get_settings(app: &AppHandle) -> AppSettings {
    read_or_create_app_settings(app, false)
}

pub fn write_settings(app: &AppHandle, settings: AppSettings) {
    let store = app
        .store(SETTINGS_STORE_PATH)
        .expect("Failed to initialize store");

    store.set("settings", persisted_settings_value(&settings));
}

pub fn get_bindings(app: &AppHandle) -> HashMap<String, ShortcutBinding> {
    let settings = get_settings(app);

    settings.bindings
}

pub fn get_stored_binding(app: &AppHandle, id: &str) -> Option<ShortcutBinding> {
    let bindings = get_bindings(app);
    bindings.get(id).cloned()
}

pub fn get_history_limit(app: &AppHandle) -> usize {
    let settings = get_settings(app);
    settings.history_limit
}

pub fn get_recording_retention_period(app: &AppHandle) -> RecordingRetentionPeriod {
    let settings = get_settings(app);
    settings.recording_retention_period
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn default_settings_disable_auto_submit() {
        let settings = get_default_settings();
        assert!(!settings.auto_submit);
        assert_eq!(settings.auto_submit_key, AutoSubmitKey::Enter);
    }

    #[test]
    fn legacy_push_to_talk_settings_still_deserialize_into_activation_mode() {
        let hold_settings = json!({
            "push_to_talk": true
        });
        let toggle_settings = json!({
            "push_to_talk": false
        });

        let hold = serde_json::from_value::<AppSettings>(hold_settings).expect("hold settings");
        let toggle =
            serde_json::from_value::<AppSettings>(toggle_settings).expect("toggle settings");

        assert_eq!(hold.activation_mode, ActivationMode::Hold);
        assert_eq!(toggle.activation_mode, ActivationMode::Toggle);
    }

    #[test]
    fn recover_settings_preserves_user_data_when_provider_catalog_is_invalid() {
        let invalid_settings = json!({
            "bindings": {
                "transcribe": {
                    "id": "transcribe",
                    "name": "Transcribe",
                    "description": "Converts your speech into text.",
                    "default_binding": "fn",
                    "current_binding": "fn",
                    "post_process_prompt_id": "default_restructure"
                }
            },
            "stt_provider_id": "deepgram",
            "stt_api_keys": {
                "deepgram": "dg-key"
            },
            "post_process_provider_id": "groq",
            "post_process_api_keys": {
                "groq": "groq-key"
            },
            "post_process_providers": [
                {
                    "id": "custom",
                    "label": "Custom",
                    "base_url": "http://localhost:8080/v1",
                    "allow_base_url_edit": true,
                    "models_endpoint": "/models",
                    "supports_structured_output": false
                }
            ],
            "dictionary_terms": ["handless"],
            "stt_providers": [
                {
                    "id": "future",
                    "label": "Future STT",
                    "provider_type": "satellite",
                    "base_url": "https://example.com",
                    "default_model": "future-1"
                }
            ]
        });

        assert!(serde_json::from_value::<AppSettings>(invalid_settings.clone()).is_err());

        let recovered = recover_settings_from_value(invalid_settings);

        assert_eq!(
            recovered.stt_api_keys.get("deepgram").map(String::as_str),
            Some("dg-key")
        );
        assert_eq!(
            recovered
                .post_process_api_keys
                .get("groq")
                .map(String::as_str),
            Some("groq-key")
        );
        assert_eq!(recovered.stt_provider_id, "deepgram");
        assert_eq!(recovered.post_process_provider_id, "groq");
        assert_eq!(recovered.dictionary_terms, vec!["handless"]);
        assert!(recovered
            .stt_providers
            .iter()
            .any(|provider| provider.id == "deepgram"));
        assert!(recovered
            .stt_providers
            .iter()
            .all(|provider| provider.id != "future"));
        assert_eq!(
            recovered
                .post_process_provider("custom")
                .map(|provider| provider.base_url.as_str()),
            Some("http://localhost:8080/v1")
        );
    }

    #[test]
    fn persisted_settings_omit_provider_catalogs_and_keep_custom_base_url() {
        let mut settings = get_default_settings();
        settings
            .post_process_provider_mut("custom")
            .expect("custom provider should exist")
            .base_url = "http://localhost:8080/v1".to_string();

        let serialized = persisted_settings_value(&settings);
        let settings = serialized.as_object().unwrap();

        assert!(!settings.contains_key("stt_providers"));
        assert!(!settings.contains_key("post_process_providers"));
        assert_eq!(
            settings
                .get("post_process_custom_base_url")
                .and_then(|value| value.as_str()),
            Some("http://localhost:8080/v1")
        );
    }

    #[test]
    fn slim_persisted_settings_restore_custom_base_url_after_deserialize() {
        let settings_value = json!({
            "post_process_provider_id": "custom",
            "post_process_models": {
                "custom": "llama3"
            },
            "post_process_api_keys": {
                "custom": ""
            },
            "post_process_custom_base_url": "http://localhost:8080/v1"
        });

        let mut settings =
            serde_json::from_value::<AppSettings>(settings_value.clone()).expect("settings parse");

        let object = settings_value
            .as_object()
            .expect("settings should be an object");
        let changed = apply_custom_post_process_base_url(
            &mut settings,
            configured_custom_post_process_base_url(object),
        );

        assert!(changed);
        assert_eq!(
            settings
                .post_process_provider("custom")
                .map(|provider| provider.base_url.as_str()),
            Some("http://localhost:8080/v1")
        );
    }

    #[test]
    fn persisted_settings_keep_only_custom_prompts_and_binding_overrides() {
        let mut settings = get_default_settings();
        let default_transcribe_binding = settings
            .bindings
            .get("transcribe")
            .expect("transcribe binding should exist")
            .current_binding
            .clone();
        settings
            .bindings
            .get_mut("transcribe")
            .unwrap()
            .current_binding = if default_transcribe_binding == "ctrl+space" {
            "alt+space".to_string()
        } else {
            "ctrl+space".to_string()
        };
        settings
            .bindings
            .get_mut("transcribe_with_post_process")
            .unwrap()
            .post_process_prompt_id = Some("default_restructure".to_string());
        settings.bindings.insert(
            "transcribe_custom_1".to_string(),
            ShortcutBinding {
                id: "transcribe_custom_1".to_string(),
                name: "Custom Transcription".to_string(),
                description: "Custom transcription shortcut.".to_string(),
                default_binding: "ctrl+alt+space".to_string(),
                current_binding: "ctrl+alt+space".to_string(),
                post_process_prompt_id: Some("prompt_1".to_string()),
            },
        );
        settings.post_process_prompts.push(LLMPrompt {
            id: "prompt_1".to_string(),
            name: "Custom".to_string(),
            prompt: "Rewrite this.".to_string(),
        });

        let serialized = persisted_settings_value(&settings);
        let settings = serialized.as_object().unwrap();
        let bindings = settings
            .get("bindings")
            .and_then(JsonValue::as_object)
            .expect("bindings should be serialized");
        let prompts = settings
            .get("post_process_prompts")
            .and_then(JsonValue::as_array)
            .expect("prompts should be serialized");

        assert_eq!(bindings.len(), 3);
        assert_eq!(
            bindings
                .get("transcribe")
                .and_then(JsonValue::as_object)
                .and_then(|binding| binding.get("name")),
            None
        );
        assert_eq!(
            bindings
                .get("transcribe_with_post_process")
                .and_then(JsonValue::as_object)
                .and_then(|binding| binding.get("default_binding")),
            None
        );
        assert_eq!(
            bindings
                .get("transcribe_custom_1")
                .and_then(JsonValue::as_object)
                .and_then(|binding| binding.get("name"))
                .and_then(JsonValue::as_str),
            Some("Custom Transcription")
        );
        assert_eq!(prompts.len(), 1);
        assert_eq!(
            prompts[0]
                .as_object()
                .and_then(|prompt| prompt.get("id"))
                .and_then(JsonValue::as_str),
            Some("prompt_1")
        );
    }

    #[test]
    fn normalize_settings_definitions_rebuilds_builtin_bindings_and_prompts() {
        let mut settings = get_default_settings();
        settings.bindings = HashMap::from([
            (
                "transcribe".to_string(),
                ShortcutBinding {
                    id: "transcribe".to_string(),
                    name: String::new(),
                    description: String::new(),
                    default_binding: String::new(),
                    current_binding: "ctrl+space".to_string(),
                    post_process_prompt_id: None,
                },
            ),
            (
                "transcribe_custom_1".to_string(),
                ShortcutBinding {
                    id: "transcribe_custom_1".to_string(),
                    name: "Custom Transcription".to_string(),
                    description: "Custom transcription shortcut.".to_string(),
                    default_binding: "ctrl+alt+space".to_string(),
                    current_binding: "ctrl+alt+space".to_string(),
                    post_process_prompt_id: Some("prompt_1".to_string()),
                },
            ),
        ]);
        settings.post_process_prompts = vec![LLMPrompt {
            id: "prompt_1".to_string(),
            name: "Custom".to_string(),
            prompt: "Rewrite this.".to_string(),
        }];

        assert!(normalize_settings_definitions(&mut settings));
        assert_eq!(
            settings
                .bindings
                .get("transcribe")
                .map(|binding| binding.name.as_str()),
            Some("Transcribe")
        );
        assert!(settings.bindings.contains_key("cancel"));
        assert_eq!(
            settings
                .post_process_prompts
                .iter()
                .filter(|prompt| crate::post_process::is_builtin_prompt(&prompt.id))
                .count(),
            crate::post_process::prompts::default_prompts().len()
        );
        assert!(settings
            .post_process_prompts
            .iter()
            .any(|prompt| prompt.id == "prompt_1"));
    }
}
