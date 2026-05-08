use serde::Deserialize;

pub mod assemblyai;
pub mod cartesia;
pub mod deepgram;
pub mod doubao;
pub mod elevenlabs;
pub mod fireworks;
pub mod groq;
pub mod mistral;
pub mod openai;
pub mod realtime;
pub mod soniox;

pub(crate) const OPENAI_BATCH_TRANSCRIPTION_MODEL: &str = "gpt-4o-mini-transcribe";
pub(crate) const OPENAI_REALTIME_TRANSCRIPTION_MODEL: &str = "gpt-realtime-whisper";

#[derive(Deserialize)]
pub(crate) struct TranscriptionResponse {
    pub text: String,
}

pub(crate) async fn check_response(
    response: reqwest::Response,
    context: &str,
) -> anyhow::Result<reqwest::Response> {
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!("{} ({}): {}", context, status, body));
    }
    Ok(response)
}

/// Generate a minimal silent WAV clip for API key validation.
pub(crate) fn test_silence_wav() -> anyhow::Result<Vec<u8>> {
    crate::audio_toolkit::audio::encode_wav_bytes(&vec![0.0f32; 1600])
}

/// Strip language subtags (e.g. "zh-Hans" -> "zh") for APIs that expect ISO 639-1 codes.
pub(crate) fn strip_lang_subtag(lang: &str) -> &str {
    lang.split('-').next().unwrap_or(lang)
}

pub async fn test_api_key(
    provider_id: &str,
    api_key: &str,
    base_url: &str,
    model: &str,
    options: Option<&serde_json::Value>,
) -> anyhow::Result<()> {
    match provider_id {
        "cartesia" => cartesia::test_api_key(api_key, base_url, model).await,
        "doubao" => doubao::test_api_key(api_key, base_url, model, options).await,
        "mistral" => mistral::test_api_key(api_key, base_url, model).await,
        "openai_stt" => openai::test_api_key(api_key, base_url, model).await,
        "elevenlabs" => elevenlabs::test_api_key(api_key, base_url, model).await,
        "groq" => groq::test_api_key(api_key, base_url, model).await,
        "soniox" => soniox::test_api_key(api_key, base_url, model).await,
        "deepgram" => deepgram::test_api_key(api_key, base_url, model).await,
        "assemblyai" => assemblyai::test_api_key(api_key, base_url, model).await,
        "fireworks" => fireworks::test_api_key(api_key, base_url, model).await,
        _ => Err(anyhow::anyhow!(
            "Unknown cloud STT provider: {}",
            provider_id
        )),
    }
}

pub async fn transcribe(
    provider_id: &str,
    api_key: &str,
    base_url: &str,
    model: &str,
    audio_wav: Vec<u8>,
    options: Option<&serde_json::Value>,
) -> anyhow::Result<String> {
    match provider_id {
        "cartesia" => cartesia::transcribe(api_key, base_url, model, audio_wav, options).await,
        "doubao" => doubao::transcribe(api_key, base_url, model, audio_wav, options).await,
        "mistral" => mistral::transcribe(api_key, base_url, model, audio_wav, options).await,
        "openai_stt" => openai::transcribe(api_key, base_url, model, audio_wav, options).await,
        "elevenlabs" => elevenlabs::transcribe(api_key, base_url, model, audio_wav, options).await,
        "groq" => groq::transcribe(api_key, base_url, model, audio_wav, options).await,
        "soniox" => soniox::transcribe(api_key, base_url, model, audio_wav, options).await,
        "deepgram" => deepgram::transcribe(api_key, base_url, model, audio_wav, options).await,
        "assemblyai" => assemblyai::transcribe(api_key, base_url, model, audio_wav, options).await,
        "fireworks" => fireworks::transcribe(api_key, base_url, model, audio_wav, options).await,
        _ => Err(anyhow::anyhow!(
            "Unknown cloud STT provider: {}",
            provider_id
        )),
    }
}
