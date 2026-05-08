use crate::managers::model::EngineType;
use log::debug;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CloudProviderOption {
    pub key: String,
    pub label: String,
    pub option_type: CloudOptionType,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<CloudOptionDefault>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "type")]
pub enum CloudOptionType {
    Text,
    Number { min: f64, max: f64, step: f64 },
    Boolean,
    Language,
    LanguageMulti,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(untagged)]
pub enum CloudOptionDefault {
    Bool(bool),
    Text(String),
    Number(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "type")]
pub enum ProviderBackend {
    Local {
        engine_type: EngineType,
        filename: String,
        url: Option<String>,
        size_mb: u64,
        is_downloaded: bool,
        is_downloading: bool,
        partial_size: u64,
        is_directory: bool,
        accuracy_score: f32,
        speed_score: f32,
        is_custom: bool,
    },
    Cloud {
        base_url: String,
        default_model: String,
        console_url: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct SttProviderInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub supported_languages: Vec<String>,
    pub supports_translation: bool,
    pub supports_realtime: bool,
    pub is_recommended: bool,
    pub backend: ProviderBackend,
    #[serde(default)]
    pub available_options: Vec<CloudProviderOption>,
    #[serde(default)]
    pub supports_dictionary_terms: bool,
    #[serde(default)]
    pub supports_dictionary_context: bool,
}

pub fn cloud_provider_registry() -> Vec<SttProviderInfo> {
    vec![
        SttProviderInfo {
            id: "openai_stt".to_string(),
            name: "OpenAI".to_string(),
            description: "onboarding.cloud.openai_stt.description".to_string(),
            supported_languages: vec![
                "af", "ar", "hy", "az", "be", "bs", "bg", "ca", "zh-Hans", "zh-Hant", "hr",
                "cs", "da", "nl", "en", "et", "fi", "fr", "gl", "de", "el",
                "he", "hi", "hu", "is", "id", "it", "ja", "kn", "kk", "ko",
                "lv", "lt", "mk", "ms", "mr", "mi", "ne", "no", "fa", "pl",
                "pt", "ro", "ru", "sr", "sk", "sl", "es", "sw", "sv", "tl",
                "ta", "th", "tr", "uk", "ur", "vi", "cy",
            ].into_iter().map(String::from).collect(),
            supports_translation: true,
            supports_realtime: true,
            is_recommended: false,
            backend: ProviderBackend::Cloud {
                base_url: "https://api.openai.com/v1".to_string(),
                default_model: crate::cloud_stt::OPENAI_REALTIME_TRANSCRIPTION_MODEL.to_string(),
                console_url: Some("https://platform.openai.com/api-keys".to_string()),
            },
            available_options: vec![
                CloudProviderOption {
                    key: "language".to_string(),
                    label: "settings.models.cloudProviders.options.language".to_string(),
                    option_type: CloudOptionType::Language,
                    description: String::new(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "prompt".to_string(),
                    label: "settings.models.cloudProviders.options.prompt".to_string(),
                    option_type: CloudOptionType::Text,
                    description: "settings.models.cloudProviders.options.promptDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "temperature".to_string(),
                    label: "settings.models.cloudProviders.options.temperature".to_string(),
                    option_type: CloudOptionType::Number { min: 0.0, max: 1.0, step: 0.1 },
                    description: "settings.models.cloudProviders.options.temperatureDescription".to_string(),
                    default_value: None,
                },
            ],
            supports_dictionary_terms: true,
            supports_dictionary_context: true,
        },
        SttProviderInfo {
            id: "cartesia".to_string(),
            name: "Cartesia".to_string(),
            description: "onboarding.cloud.cartesia.description".to_string(),
            supported_languages: vec![
                "af", "am", "ar", "az", "ba", "be", "bg", "bn", "bo", "br",
                "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et",
                "eu", "fa", "fi", "fo", "fr", "gl", "gu", "ha", "haw", "he",
                "hi", "hr", "ht", "hu", "hy", "id", "is", "it", "ja", "jw",
                "ka", "kk", "km", "kn", "ko", "la", "lb", "ln", "lo", "lt",
                "lv", "mg", "mi", "mk", "ml", "mn", "mr", "ms", "mt", "my",
                "ne", "nl", "nn", "no", "oc", "pa", "pl", "ps", "pt", "ro",
                "ru", "sa", "sd", "si", "sk", "sl", "sn", "so", "sq", "sr",
                "su", "sv", "sw", "ta", "te", "tg", "th", "tk", "tl", "tr",
                "tt", "uk", "ur", "uz", "vi", "yi", "yo", "yue", "zh-Hans",
            ].into_iter().map(String::from).collect(),
            supports_translation: false,
            supports_realtime: false,
            is_recommended: false,
            backend: ProviderBackend::Cloud {
                base_url: "https://api.cartesia.ai".to_string(),
                default_model: "ink-whisper".to_string(),
                console_url: Some("https://play.cartesia.ai/keys".to_string()),
            },
            available_options: vec![
                CloudProviderOption {
                    key: "language".to_string(),
                    label: "settings.models.cloudProviders.options.language".to_string(),
                    option_type: CloudOptionType::Language,
                    description: String::new(),
                    default_value: None,
                },
            ],
            supports_dictionary_terms: false,
            supports_dictionary_context: false,
        },
        SttProviderInfo {
            id: "elevenlabs".to_string(),
            name: "ElevenLabs".to_string(),
            description: "onboarding.cloud.elevenlabs.description".to_string(),
            supported_languages: vec![
                "af", "am", "ar", "az", "be", "bg", "bn", "bs", "ca", "ceb",
                "cs", "cy", "da", "de", "el", "en", "es", "et", "fa", "ff",
                "fi", "fil", "fr", "ga", "gl", "gu", "ha", "he", "hi", "hr",
                "hu", "hy", "id", "ig", "is", "it", "ja", "jv", "ka", "kk",
                "km", "kn", "ko", "ku", "ky", "lb", "ln", "lo", "lt", "lv",
                "mi", "mk", "ml", "mn", "mr", "ms", "mt", "my", "ne", "nl",
                "no", "oc", "pa", "pl", "ps", "pt", "ro", "ru", "sd", "sk",
                "sl", "sn", "so", "sr", "sv", "sw", "ta", "te", "tg", "th",
                "tl", "tr", "uk", "ur", "uz", "vi", "yo", "yue", "zh-Hans",
            ].into_iter().map(String::from).collect(),
            supports_translation: false,
            supports_realtime: true,
            is_recommended: false,
            backend: ProviderBackend::Cloud {
                base_url: "https://api.elevenlabs.io/v1".to_string(),
                default_model: "scribe_v2".to_string(),
                console_url: Some("https://elevenlabs.io/app/developers/api-keys".to_string()),
            },
            available_options: vec![
                CloudProviderOption {
                    key: "language".to_string(),
                    label: "settings.models.cloudProviders.options.language".to_string(),
                    option_type: CloudOptionType::Language,
                    description: String::new(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "enable_speaker_diarization".to_string(),
                    label: "settings.models.cloudProviders.options.enableSpeakerDiarization".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.enableSpeakerDiarizationDescription".to_string(),
                    default_value: None,
                },
            ],
            supports_dictionary_terms: true,
            supports_dictionary_context: false,
        },
        SttProviderInfo {
            id: "groq".to_string(),
            name: "Groq".to_string(),
            description: "onboarding.cloud.groq.description".to_string(),
            supported_languages: vec![
                "af", "ar", "hy", "az", "be", "bs", "bg", "ca", "zh-Hans", "zh-Hant", "hr",
                "cs", "da", "nl", "en", "et", "fi", "fr", "gl", "de", "el",
                "he", "hi", "hu", "is", "id", "it", "ja", "kn", "kk", "ko",
                "lv", "lt", "mk", "ms", "mr", "mi", "ne", "no", "fa", "pl",
                "pt", "ro", "ru", "sr", "sk", "sl", "es", "sw", "sv", "tl",
                "ta", "th", "tr", "uk", "ur", "vi", "cy",
            ].into_iter().map(String::from).collect(),
            supports_translation: false,
            supports_realtime: false,
            is_recommended: false,
            backend: ProviderBackend::Cloud {
                base_url: "https://api.groq.com/openai/v1".to_string(),
                default_model: "whisper-large-v3-turbo".to_string(),
                console_url: Some("https://console.groq.com/keys".to_string()),
            },
            available_options: vec![
                CloudProviderOption {
                    key: "language".to_string(),
                    label: "settings.models.cloudProviders.options.language".to_string(),
                    option_type: CloudOptionType::Language,
                    description: String::new(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "prompt".to_string(),
                    label: "settings.models.cloudProviders.options.prompt".to_string(),
                    option_type: CloudOptionType::Text,
                    description: "settings.models.cloudProviders.options.promptDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "temperature".to_string(),
                    label: "settings.models.cloudProviders.options.temperature".to_string(),
                    option_type: CloudOptionType::Number { min: 0.0, max: 1.0, step: 0.1 },
                    description: "settings.models.cloudProviders.options.temperatureDescription".to_string(),
                    default_value: None,
                },
            ],
            supports_dictionary_terms: true,
            supports_dictionary_context: true,
        },
        SttProviderInfo {
            id: "mistral".to_string(),
            name: "Mistral AI".to_string(),
            description: "onboarding.cloud.mistral.description".to_string(),
            supported_languages: vec![
                "en", "zh-Hans", "hi", "es", "ar", "fr", "pt", "ru", "de", "ja", "ko", "it", "nl",
            ].into_iter().map(String::from).collect(),
            supports_translation: false,
            supports_realtime: true,
            is_recommended: false,
            backend: ProviderBackend::Cloud {
                base_url: "https://api.mistral.ai".to_string(),
                default_model: "voxtral-mini-latest".to_string(),
                console_url: Some("https://console.mistral.ai".to_string()),
            },
            available_options: vec![
                CloudProviderOption {
                    key: "language".to_string(),
                    label: "settings.models.cloudProviders.options.language".to_string(),
                    option_type: CloudOptionType::Language,
                    description: String::new(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "temperature".to_string(),
                    label: "settings.models.cloudProviders.options.temperature".to_string(),
                    option_type: CloudOptionType::Number { min: 0.0, max: 1.0, step: 0.1 },
                    description: "settings.models.cloudProviders.options.temperatureDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "diarize".to_string(),
                    label: "settings.models.cloudProviders.options.enableSpeakerDiarization".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.enableSpeakerDiarizationDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "context_bias".to_string(),
                    label: "settings.models.cloudProviders.options.contextBias".to_string(),
                    option_type: CloudOptionType::Text,
                    description: "settings.models.cloudProviders.options.contextBiasDescription".to_string(),
                    default_value: None,
                },
            ],
            supports_dictionary_terms: true,
            supports_dictionary_context: false,
        },
        SttProviderInfo {
            id: "soniox".to_string(),
            name: "Soniox".to_string(),
            description: "onboarding.cloud.soniox.description".to_string(),
            supported_languages: vec![
                "af", "sq", "ar", "az", "eu", "be", "bn", "bs", "bg", "ca",
                "zh-Hans", "zh-Hant", "hr", "cs", "da", "nl", "en", "et", "fi", "fr",
                "gl", "de", "el", "gu", "he", "hi", "hu", "id", "it", "ja",
                "kn", "kk", "ko", "lv", "lt", "mk", "ms", "ml", "mr", "no",
                "fa", "pl", "pt", "pa", "ro", "ru", "sr", "sk", "sl", "es",
                "sw", "sv", "tl", "ta", "te", "th", "tr", "uk", "ur", "vi", "cy",
            ].into_iter().map(String::from).collect(),
            supports_translation: true,
            supports_realtime: true,
            is_recommended: false,
            backend: ProviderBackend::Cloud {
                base_url: "https://api.soniox.com/v1".to_string(),
                default_model: "stt-rt-v4".to_string(),
                console_url: Some("https://console.soniox.com".to_string()),
            },
            available_options: vec![
                CloudProviderOption {
                    key: "language_hints".to_string(),
                    label: "settings.models.cloudProviders.options.languageHints".to_string(),
                    option_type: CloudOptionType::LanguageMulti,
                    description: String::new(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "language_hints_strict".to_string(),
                    label: "settings.models.cloudProviders.options.languageHintsStrict".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.languageHintsStrictDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "context_terms".to_string(),
                    label: "settings.models.cloudProviders.options.contextTerms".to_string(),
                    option_type: CloudOptionType::Text,
                    description: "settings.models.cloudProviders.options.contextTermsDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "context_description".to_string(),
                    label: "settings.models.cloudProviders.options.contextDescription".to_string(),
                    option_type: CloudOptionType::Text,
                    description: "settings.models.cloudProviders.options.contextDescriptionDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "enable_speaker_diarization".to_string(),
                    label: "settings.models.cloudProviders.options.enableSpeakerDiarization".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.enableSpeakerDiarizationDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "enable_language_identification".to_string(),
                    label: "settings.models.cloudProviders.options.enableLanguageIdentification".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.enableLanguageIdentificationDescription".to_string(),
                    default_value: None,
                },
            ],
            supports_dictionary_terms: true,
            supports_dictionary_context: true,
        },
        SttProviderInfo {
            id: "deepgram".to_string(),
            name: "Deepgram".to_string(),
            description: "onboarding.cloud.deepgram.description".to_string(),
            supported_languages: vec![
                "ar", "be", "bn", "bs", "bg", "ca", "hr", "cs", "da", "nl",
                "en", "et", "fa", "fi", "fr", "de", "el", "he", "hi", "hu",
                "id", "it", "ja", "kn", "ko", "lv", "lt", "mk", "ms", "mr",
                "no", "pl", "pt", "ro", "ru", "sr", "sk", "sl", "es", "sv",
                "tl", "ta", "te", "tr", "uk", "ur", "vi",
                "zh-Hans", "zh-Hant",
            ].into_iter().map(String::from).collect(),
            supports_translation: false,
            supports_realtime: true,
            is_recommended: false,
            backend: ProviderBackend::Cloud {
                base_url: "https://api.deepgram.com/v1".to_string(),
                default_model: "nova-3".to_string(),
                console_url: Some("https://console.deepgram.com/api-keys".to_string()),
            },
            available_options: vec![
                CloudProviderOption {
                    key: "language".to_string(),
                    label: "settings.models.cloudProviders.options.language".to_string(),
                    option_type: CloudOptionType::Language,
                    description: String::new(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "smart_format".to_string(),
                    label: "settings.models.cloudProviders.options.smartFormat".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.smartFormatDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "punctuate".to_string(),
                    label: "settings.models.cloudProviders.options.punctuate".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.punctuateDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "diarize".to_string(),
                    label: "settings.models.cloudProviders.options.enableSpeakerDiarization".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.enableSpeakerDiarizationDescription".to_string(),
                    default_value: None,
                },
            ],
            supports_dictionary_terms: true,
            supports_dictionary_context: false,
        },
        SttProviderInfo {
            id: "assemblyai".to_string(),
            name: "AssemblyAI".to_string(),
            description: "onboarding.cloud.assemblyai.description".to_string(),
            supported_languages: vec![
                "af", "am", "ar", "az", "be", "bg", "bn", "bs", "ca", "cs",
                "cy", "da", "de", "el", "en", "es", "et", "eu", "fa", "fi",
                "fr", "gl", "gu", "ha", "he", "hi", "hr", "hu", "hy", "id",
                "is", "it", "ja", "jv", "ka", "kk", "km", "kn", "ko", "ku",
                "ky", "la", "lb", "lo", "lt", "lv", "mi", "mk", "ml", "mn",
                "mr", "ms", "mt", "my", "ne", "nl", "no", "pa", "pl", "ps",
                "pt", "ro", "ru", "sa", "sd", "si", "sk", "sl", "sn", "so",
                "sr", "sv", "sw", "ta", "te", "tg", "th", "tk", "tl", "tr",
                "uk", "ur", "uz", "vi", "yo", "zh",
            ].into_iter().map(String::from).collect(),
            supports_translation: false,
            supports_realtime: true,
            is_recommended: false,
            backend: ProviderBackend::Cloud {
                base_url: "https://api.assemblyai.com".to_string(),
                default_model: "universal-3-pro".to_string(),
                console_url: Some("https://www.assemblyai.com/dashboard".to_string()),
            },
            available_options: vec![
                CloudProviderOption {
                    key: "language_code".to_string(),
                    label: "settings.models.cloudProviders.options.language".to_string(),
                    option_type: CloudOptionType::Language,
                    description: String::new(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "speaker_labels".to_string(),
                    label: "settings.models.cloudProviders.options.enableSpeakerDiarization".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.enableSpeakerDiarizationDescription".to_string(),
                    default_value: None,
                },
            ],
            supports_dictionary_terms: true,
            supports_dictionary_context: false,
        },
        SttProviderInfo {
            id: "fireworks".to_string(),
            name: "Fireworks AI".to_string(),
            description: "onboarding.cloud.fireworks.description".to_string(),
            supported_languages: vec![
                "af", "ar", "hy", "az", "be", "bs", "bg", "ca", "zh-Hans", "zh-Hant", "hr",
                "cs", "da", "nl", "en", "et", "fi", "fr", "gl", "de", "el",
                "he", "hi", "hu", "is", "id", "it", "ja", "kn", "kk", "ko",
                "lv", "lt", "mk", "ms", "mr", "mi", "ne", "no", "fa", "pl",
                "pt", "ro", "ru", "sr", "sk", "sl", "es", "sw", "sv", "tl",
                "ta", "th", "tr", "uk", "ur", "vi", "cy",
            ].into_iter().map(String::from).collect(),
            supports_translation: true,
            supports_realtime: true,
            is_recommended: false,
            backend: ProviderBackend::Cloud {
                base_url: "https://audio-prod.api.fireworks.ai/v1".to_string(),
                default_model: "whisper-v3".to_string(),
                console_url: Some("https://fireworks.ai/api-keys".to_string()),
            },
            available_options: vec![
                CloudProviderOption {
                    key: "language".to_string(),
                    label: "settings.models.cloudProviders.options.language".to_string(),
                    option_type: CloudOptionType::Language,
                    description: String::new(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "prompt".to_string(),
                    label: "settings.models.cloudProviders.options.prompt".to_string(),
                    option_type: CloudOptionType::Text,
                    description: "settings.models.cloudProviders.options.promptDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "temperature".to_string(),
                    label: "settings.models.cloudProviders.options.temperature".to_string(),
                    option_type: CloudOptionType::Number { min: 0.0, max: 1.0, step: 0.1 },
                    description: "settings.models.cloudProviders.options.temperatureDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "diarize".to_string(),
                    label: "settings.models.cloudProviders.options.enableSpeakerDiarization".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.enableSpeakerDiarizationDescription".to_string(),
                    default_value: None,
                },
            ],
            supports_dictionary_terms: true,
            supports_dictionary_context: true,
        },
        SttProviderInfo {
            id: "doubao".to_string(),
            name: "Doubao".to_string(),
            description: "onboarding.cloud.doubao.description".to_string(),
            supported_languages: vec![
                "zh-Hans", "zh-Hant", "yue", "en", "ja", "ko", "es", "fr", "de", "ru",
                "pt", "it", "ar", "th", "vi", "id", "ms", "bn", "el", "nl",
                "tr", "pl", "ro", "ne", "uk",
            ].into_iter().map(String::from).collect(),
            supports_translation: false,
            supports_realtime: true,
            is_recommended: false,
            backend: ProviderBackend::Cloud {
                base_url: "wss://openspeech.bytedance.com/api/v3/sauc/bigmodel_async".to_string(),
                default_model: "bigmodel".to_string(),
                console_url: Some("https://console.volcengine.com/speech/app".to_string()),
            },
            available_options: vec![
                CloudProviderOption {
                    key: "app_key".to_string(),
                    label: "settings.models.cloudProviders.doubao.appKey".to_string(),
                    option_type: CloudOptionType::Text,
                    description: "settings.models.cloudProviders.doubao.appKeyDescription".to_string(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "resource_id".to_string(),
                    label: "settings.models.cloudProviders.doubao.resourceId".to_string(),
                    option_type: CloudOptionType::Text,
                    description: "settings.models.cloudProviders.doubao.resourceIdDescription".to_string(),
                    default_value: Some(CloudOptionDefault::Text("volc.seedasr.sauc.duration".to_string())),
                },
                CloudProviderOption {
                    key: "language".to_string(),
                    label: "settings.models.cloudProviders.options.language".to_string(),
                    option_type: CloudOptionType::Language,
                    description: String::new(),
                    default_value: None,
                },
                CloudProviderOption {
                    key: "enable_itn".to_string(),
                    label: "settings.models.cloudProviders.options.enableItn".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.enableItnDescription".to_string(),
                    default_value: Some(CloudOptionDefault::Bool(true)),
                },
                CloudProviderOption {
                    key: "enable_punc".to_string(),
                    label: "settings.models.cloudProviders.options.punctuate".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.punctuateDescription".to_string(),
                    default_value: Some(CloudOptionDefault::Bool(true)),
                },
                CloudProviderOption {
                    key: "enable_ddc".to_string(),
                    label: "settings.models.cloudProviders.options.enableDdc".to_string(),
                    option_type: CloudOptionType::Boolean,
                    description: "settings.models.cloudProviders.options.enableDdcDescription".to_string(),
                    default_value: Some(CloudOptionDefault::Bool(true)),
                },
            ],
            supports_dictionary_terms: true,
            supports_dictionary_context: true,
        },
    ]
}

/// Merge dictionary terms and context into the provider-specific cloud options.
///
/// For prompt-based providers (OpenAI, Groq, Fireworks): terms are prepended as
/// `"Glossary: term1, term2. "` to the `prompt` field, and context is prepended
/// after the glossary. The user's own prompt text is preserved after.
///
/// For Deepgram: terms are merged into the `keyterm` field (comma-separated).
///
/// For AssemblyAI: terms are merged into the `keyterms_prompt` array.
///
/// For Mistral: terms are merged into the `context_bias` field (comma-separated).
///
/// For Soniox: terms are prepended to the `context_terms` field (comma-separated),
/// and context is prepended to the `context_description` field.
pub fn inject_dictionary(
    provider_id: &str,
    options: Option<serde_json::Value>,
    dictionary_terms: &[String],
    dictionary_context: &str,
) -> Option<serde_json::Value> {
    if dictionary_terms.is_empty() && dictionary_context.is_empty() {
        return options;
    }

    let mut opts = options.unwrap_or_else(|| serde_json::json!({}));

    match provider_id {
        "openai_stt" | "groq" | "fireworks" => {
            // Build the dictionary prefix for the prompt field
            let mut prefix_parts = Vec::new();
            if !dictionary_terms.is_empty() {
                prefix_parts.push(format!("Glossary: {}.", dictionary_terms.join(", ")));
            }
            if !dictionary_context.is_empty() {
                prefix_parts.push(dictionary_context.to_string());
            }
            let prefix = prefix_parts.join(" ");

            let existing_prompt = opts
                .get("prompt")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let merged = if existing_prompt.is_empty() {
                prefix
            } else {
                format!("{} {}", prefix, existing_prompt)
            };
            opts["prompt"] = serde_json::json!(merged);
            debug!(
                "Injected dictionary into {} prompt ({} terms, {} chars context)",
                provider_id,
                dictionary_terms.len(),
                dictionary_context.len()
            );
        }
        "deepgram" => {
            // Merge terms into keyterm (comma-separated)
            if !dictionary_terms.is_empty() {
                let dict_keyterm = dictionary_terms.join(", ");
                let existing_keyterm = opts
                    .get("keyterm")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let merged = if existing_keyterm.is_empty() {
                    dict_keyterm
                } else {
                    format!("{}, {}", dict_keyterm, existing_keyterm)
                };
                opts["keyterm"] = serde_json::json!(merged);
            }
            debug!(
                "Injected dictionary into Deepgram options ({} terms)",
                dictionary_terms.len(),
            );
        }
        "assemblyai" => {
            // Merge terms into keyterms_prompt (array of strings)
            if !dictionary_terms.is_empty() {
                let existing: Vec<String> = opts
                    .get("keyterms_prompt")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();

                let mut merged = dictionary_terms.to_vec();
                merged.extend(existing);
                opts["keyterms_prompt"] = serde_json::json!(merged);
            }
            debug!(
                "Injected dictionary into AssemblyAI options ({} terms)",
                dictionary_terms.len(),
            );
        }
        "mistral" => {
            // Merge terms into context_bias (comma-separated)
            if !dictionary_terms.is_empty() {
                let dict_terms_str = dictionary_terms.join(",");
                let existing_bias = opts
                    .get("context_bias")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let merged = if existing_bias.is_empty() {
                    dict_terms_str
                } else {
                    format!("{},{}", dict_terms_str, existing_bias)
                };
                opts["context_bias"] = serde_json::json!(merged);
            }
            debug!(
                "Injected dictionary into Mistral context_bias ({} terms)",
                dictionary_terms.len(),
            );
        }
        "elevenlabs" => {
            // Merge terms into keyterms (JSON array of strings)
            if !dictionary_terms.is_empty() {
                let existing: Vec<String> = opts
                    .get("keyterms")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let mut merged = dictionary_terms.to_vec();
                merged.extend(existing);
                opts["keyterms"] = serde_json::json!(merged);
            }
            debug!(
                "Injected dictionary into ElevenLabs options ({} terms)",
                dictionary_terms.len(),
            );
        }
        "doubao" => {
            // Merge terms into hotwords (comma-separated string parsed by the Doubao module)
            if !dictionary_terms.is_empty() {
                let dict_hotwords = dictionary_terms.join(", ");
                let existing_hotwords = opts
                    .get("hotwords")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let merged = if existing_hotwords.is_empty() {
                    dict_hotwords
                } else {
                    format!("{}, {}", dict_hotwords, existing_hotwords)
                };
                opts["hotwords"] = serde_json::json!(merged);
            }
            // Merge context description into dialog_context
            if !dictionary_context.is_empty() {
                opts["dialog_context"] = serde_json::json!(dictionary_context);
            }
            debug!(
                "Injected dictionary into Doubao options ({} terms, {} chars context)",
                dictionary_terms.len(),
                dictionary_context.len(),
            );
        }
        "soniox" => {
            // Merge terms into context_terms (comma-separated)
            if !dictionary_terms.is_empty() {
                let dict_terms_str = dictionary_terms.join(", ");
                let existing_terms = opts
                    .get("context_terms")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let merged = if existing_terms.is_empty() {
                    dict_terms_str
                } else {
                    format!("{}, {}", dict_terms_str, existing_terms)
                };
                opts["context_terms"] = serde_json::json!(merged);
            }

            // Merge context into context_description
            if !dictionary_context.is_empty() {
                let existing_desc = opts
                    .get("context_description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let merged = if existing_desc.is_empty() {
                    dictionary_context.to_string()
                } else {
                    format!("{} {}", dictionary_context, existing_desc)
                };
                opts["context_description"] = serde_json::json!(merged);
            }
            debug!(
                "Injected dictionary into Soniox options ({} terms, {} chars context)",
                dictionary_terms.len(),
                dictionary_context.len()
            );
        }
        _ => {
            // Unknown provider — no injection
            debug!(
                "Dictionary injection skipped for unknown provider: {}",
                provider_id
            );
        }
    }

    Some(opts)
}
