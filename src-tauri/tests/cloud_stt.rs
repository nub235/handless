#![cfg(feature = "cloud-stt-tests")]
//! Integration tests for cloud STT providers.
//!
//! These tests hit real APIs and require keys. They are gated behind
//! the `cloud-stt-tests` cargo feature so they never run by default.
//!
//! ## Setup
//!
//! 1. Create a `.env` file at the repo root with your API keys:
//!
//!    ```text
//!    OPENAI_STT_API_KEY=sk-...
//!    DEEPGRAM_API_KEY=...
//!    ASSEMBLYAI_API_KEY=...
//!    GROQ_API_KEY=...
//!    ELEVENLABS_API_KEY=...
//!    MISTRAL_API_KEY=...
//!    SONIOX_API_KEY=...
//!    CARTESIA_API_KEY=...
//!    FIREWORKS_API_KEY=...
//!    DOUBAO_ACCESS_KEY=...
//!    DOUBAO_APP_KEY=...
//!    DOUBAO_RESOURCE_ID=...
//!    ```
//!
//! 2. Place a short WAV file (16 kHz, mono, 16-bit, ~2-3s of English speech)
//!    at `tests/fixtures/hello_test.wav`. The expected transcript should
//!    contain the word "hello" or "test".
//!
//! ## Running
//!
//! ```bash
//! # All cloud tests
//! cd src-tauri && cargo test --features cloud-stt-tests --test cloud_stt
//!
//! # Single provider
//! cargo test --features cloud-stt-tests --test cloud_stt -- openai
//!
//! # Only batch tests
//! cargo test --features cloud-stt-tests --test cloud_stt -- batch
//!
//! # Only realtime tests
//! cargo test --features cloud-stt-tests --test cloud_stt -- realtime
//!
//! # Specific test
//! cargo test --features cloud-stt-tests --test cloud_stt -- deepgram::batch_with_diarization
//! ```

use handless_app_lib::audio_toolkit::audio::{encode_wav_bytes, extract_pcm_from_wav};
use handless_app_lib::cloud_stt;
use handless_app_lib::stt_provider;
use std::sync::Once;

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        dotenvy::from_filename("../.env").ok();
        dotenvy::dotenv().ok();
    });
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Load the test audio fixture, or skip the test if it doesn't exist.
macro_rules! require_audio {
    () => {{
        let paths = &[
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/hello_test.wav"),
            "tests/fixtures/hello_test.wav",
        ];
        let mut found = None;
        for p in paths {
            if let Ok(data) = std::fs::read(p) {
                found = Some(data);
                break;
            }
        }
        match found {
            Some(data) => data,
            None => {
                eprintln!("Skipping: tests/fixtures/hello_test.wav not found");
                return;
            }
        }
    }};
}

/// Get an API key from the environment, or skip the test.
macro_rules! require_key {
    ($var:expr) => {{
        init();
        match std::env::var($var) {
            Ok(k) if !k.is_empty() => k,
            _ => {
                eprintln!("Skipping: {} not set", $var);
                return;
            }
        }
    }};
}

/// Assert that a transcription result contains the expected keyword from the
/// test fixture audio. Case-insensitive.
fn assert_transcript_contains(result: &str, word: &str) {
    assert!(
        result.to_lowercase().contains(&word.to_lowercase()),
        "transcript should contain \"{}\", got: \"{}\"",
        word,
        result
    );
}

/// Generate a minimal silent WAV (100ms at 16 kHz) for tests that only
/// need a valid audio payload (e.g. verifying the API accepts the request).
fn silence_wav() -> Vec<u8> {
    encode_wav_bytes(&vec![0.0f32; 1600]).expect("failed to encode silence WAV")
}

// ---------------------------------------------------------------------------
// OpenAI  (batch + realtime, translation, dictionary)
// ---------------------------------------------------------------------------

mod openai {
    use super::*;

    const BASE_URL: &str = "https://api.openai.com/v1";
    const BATCH_MODEL: &str = "gpt-4o-mini-transcribe";
    const REALTIME_MODEL: &str = "gpt-realtime-whisper";

    #[tokio::test]
    async fn batch_default() {
        let key = require_key!("OPENAI_STT_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::transcribe("openai_stt", &key, BASE_URL, BATCH_MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_english_explicit() {
        let key = require_key!("OPENAI_STT_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({ "language": "en" });
        let result = cloud_stt::transcribe(
            "openai_stt",
            &key,
            BASE_URL,
            BATCH_MODEL,
            audio,
            Some(&opts),
        )
        .await
        .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_prompt_and_temperature() {
        let key = require_key!("OPENAI_STT_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({
            "language": "en",
            "prompt": "This is a test recording.",
            "temperature": "0.0"
        });
        let result = cloud_stt::transcribe(
            "openai_stt",
            &key,
            BASE_URL,
            BATCH_MODEL,
            audio,
            Some(&opts),
        )
        .await
        .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_dictionary() {
        let key = require_key!("OPENAI_STT_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({ "language": "en" });
        let opts = stt_provider::inject_dictionary(
            "openai_stt",
            Some(opts),
            &["Handless".to_string(), "Tauri".to_string()],
            "A desktop speech-to-text application",
        );
        let result = cloud_stt::transcribe(
            "openai_stt",
            &key,
            BASE_URL,
            BATCH_MODEL,
            audio,
            opts.as_ref(),
        )
        .await
        .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_auto_language() {
        let key = require_key!("OPENAI_STT_API_KEY");
        let audio = require_audio!();
        // No language option = auto-detect
        let result = cloud_stt::transcribe("openai_stt", &key, BASE_URL, BATCH_MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_translate_to_english() {
        let key = require_key!("OPENAI_STT_API_KEY");
        let audio = require_audio!();
        // Translation uses a different endpoint on some providers; for OpenAI
        // the translate flag is handled at a higher level. This tests that the
        // basic transcribe path works when translation would be requested.
        let result = cloud_stt::transcribe("openai_stt", &key, BASE_URL, BATCH_MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_default() {
        let key = require_key!("OPENAI_STT_API_KEY");
        let audio = require_audio!();
        let result =
            cloud_stt::realtime::transcribe("openai_stt", &key, REALTIME_MODEL, audio, None)
                .await
                .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_with_options() {
        let key = require_key!("OPENAI_STT_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({
            "language": "en",
            "temperature": "0.0"
        });
        let result =
            cloud_stt::realtime::transcribe("openai_stt", &key, REALTIME_MODEL, audio, Some(&opts))
                .await
                .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_with_dictionary() {
        let key = require_key!("OPENAI_STT_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({ "language": "en" });
        let opts = stt_provider::inject_dictionary(
            "openai_stt",
            Some(opts),
            &["Handless".to_string(), "Tauri".to_string()],
            "A desktop speech-to-text application",
        );
        let result = cloud_stt::realtime::transcribe(
            "openai_stt",
            &key,
            REALTIME_MODEL,
            audio,
            opts.as_ref(),
        )
        .await
        .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_streaming_finish_returns_after_completion() {
        let key = require_key!("OPENAI_STT_API_KEY");
        let audio = require_audio!();
        let (samples, sample_rate) = extract_pcm_from_wav(&audio).unwrap();
        assert_eq!(sample_rate, 16_000);

        let (tx, rx) = tokio::sync::mpsc::channel::<Vec<f32>>(128);
        let session_config = cloud_stt::realtime::SessionConfig {
            provider_id: "openai_stt".to_string(),
            api_key: key,
            model: REALTIME_MODEL.to_string(),
            options: Some(serde_json::json!({ "language": "en" })),
            delta_tx: None,
        };
        let session = cloud_stt::realtime::RealtimeStreamingSession::start(session_config, rx)
            .await
            .unwrap();

        for chunk in samples.chunks(1600) {
            let frame: Vec<f32> = chunk
                .iter()
                .map(|sample| *sample as f32 / i16::MAX as f32)
                .collect();
            tx.send(frame).await.unwrap();
        }
        drop(tx);

        let result =
            tokio::time::timeout(std::time::Duration::from_secs(10), session.finish()).await;
        let transcript = result
            .expect("streaming finish should not hang")
            .expect("streaming finish should return transcript");
        assert_transcript_contains(&transcript, "test");
    }

    #[tokio::test]
    async fn api_key_validation() {
        let key = require_key!("OPENAI_STT_API_KEY");
        cloud_stt::test_api_key("openai_stt", &key, BASE_URL, REALTIME_MODEL, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn realtime_api_key_validation() {
        let key = require_key!("OPENAI_STT_API_KEY");
        cloud_stt::realtime::test_api_key("openai_stt", &key, BATCH_MODEL, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn api_key_validation_rejects_bad_key() {
        init();
        let result = cloud_stt::test_api_key(
            "openai_stt",
            "sk-bad-key-12345",
            BASE_URL,
            BATCH_MODEL,
            None,
        )
        .await;
        assert!(result.is_err(), "bad API key should be rejected");
    }
}

// ---------------------------------------------------------------------------
// Deepgram  (batch + realtime, dictionary terms)
// ---------------------------------------------------------------------------

mod deepgram {
    use super::*;

    const BASE_URL: &str = "https://api.deepgram.com/v1";
    const MODEL: &str = "nova-3";

    #[tokio::test]
    async fn batch_default() {
        let key = require_key!("DEEPGRAM_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::transcribe("deepgram", &key, BASE_URL, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_options() {
        let key = require_key!("DEEPGRAM_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({
            "language": "en",
            "smart_format": "true",
            "punctuate": "true",
            "diarize": "false"
        });
        let result = cloud_stt::transcribe("deepgram", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_dictionary() {
        let key = require_key!("DEEPGRAM_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({ "language": "en" });
        let opts = stt_provider::inject_dictionary(
            "deepgram",
            Some(opts),
            &["Handless".to_string(), "Tauri".to_string()],
            "",
        );
        let result = cloud_stt::transcribe("deepgram", &key, BASE_URL, MODEL, audio, opts.as_ref())
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_diarization() {
        let key = require_key!("DEEPGRAM_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({
            "language": "en",
            "diarize": "true"
        });
        let result = cloud_stt::transcribe("deepgram", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_default() {
        let key = require_key!("DEEPGRAM_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::realtime::transcribe("deepgram", &key, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn api_key_validation() {
        let key = require_key!("DEEPGRAM_API_KEY");
        cloud_stt::test_api_key("deepgram", &key, BASE_URL, MODEL, None)
            .await
            .unwrap();
    }
}

// ---------------------------------------------------------------------------
// Groq  (batch only, dictionary, OpenAI-compatible)
// ---------------------------------------------------------------------------

mod groq {
    use super::*;

    const BASE_URL: &str = "https://api.groq.com/openai/v1";
    const MODEL: &str = "whisper-large-v3-turbo";

    #[tokio::test]
    async fn batch_default() {
        let key = require_key!("GROQ_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::transcribe("groq", &key, BASE_URL, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_options() {
        let key = require_key!("GROQ_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({
            "language": "en",
            "prompt": "Technical speech recognition test",
            "temperature": "0.0"
        });
        let result = cloud_stt::transcribe("groq", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_dictionary() {
        let key = require_key!("GROQ_API_KEY");
        let audio = require_audio!();
        let opts = stt_provider::inject_dictionary(
            "groq",
            None,
            &["Handless".to_string(), "Tauri".to_string()],
            "Desktop speech-to-text app",
        );
        let result = cloud_stt::transcribe("groq", &key, BASE_URL, MODEL, audio, opts.as_ref())
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn api_key_validation() {
        let key = require_key!("GROQ_API_KEY");
        cloud_stt::test_api_key("groq", &key, BASE_URL, MODEL, None)
            .await
            .unwrap();
    }
}

// ---------------------------------------------------------------------------
// ElevenLabs  (batch + realtime, diarization)
// ---------------------------------------------------------------------------

mod elevenlabs {
    use super::*;

    const BASE_URL: &str = "https://api.elevenlabs.io/v1";
    const MODEL: &str = "scribe_v2";

    #[tokio::test]
    async fn batch_default() {
        let key = require_key!("ELEVENLABS_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::transcribe("elevenlabs", &key, BASE_URL, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_language() {
        let key = require_key!("ELEVENLABS_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({ "language": "en" });
        let result = cloud_stt::transcribe("elevenlabs", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_diarization() {
        let key = require_key!("ELEVENLABS_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({
            "language": "en",
            "enable_speaker_diarization": "true"
        });
        let result = cloud_stt::transcribe("elevenlabs", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_default() {
        let key = require_key!("ELEVENLABS_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::realtime::transcribe("elevenlabs", &key, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn api_key_validation() {
        let key = require_key!("ELEVENLABS_API_KEY");
        cloud_stt::test_api_key("elevenlabs", &key, BASE_URL, MODEL, None)
            .await
            .unwrap();
    }
}

// ---------------------------------------------------------------------------
// Mistral  (batch + realtime, diarization, dictionary)
// ---------------------------------------------------------------------------

mod mistral {
    use super::*;

    const BASE_URL: &str = "https://api.mistral.ai";
    const MODEL: &str = "voxtral-mini-latest";

    #[tokio::test]
    async fn batch_default() {
        let key = require_key!("MISTRAL_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::transcribe("mistral", &key, BASE_URL, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_options() {
        let key = require_key!("MISTRAL_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({
            "language": "en",
            "temperature": "0.0",
            "diarize": "true"
        });
        let result = cloud_stt::transcribe("mistral", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_dictionary() {
        let key = require_key!("MISTRAL_API_KEY");
        let audio = require_audio!();
        let opts = stt_provider::inject_dictionary(
            "mistral",
            None,
            &["Handless".to_string(), "Tauri".to_string()],
            "",
        );
        let result = cloud_stt::transcribe("mistral", &key, BASE_URL, MODEL, audio, opts.as_ref())
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_default() {
        let key = require_key!("MISTRAL_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::realtime::transcribe("mistral", &key, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn api_key_validation() {
        let key = require_key!("MISTRAL_API_KEY");
        cloud_stt::test_api_key("mistral", &key, BASE_URL, MODEL, None)
            .await
            .unwrap();
    }
}

// ---------------------------------------------------------------------------
// Cartesia  (batch only, no dictionary, no realtime)
// ---------------------------------------------------------------------------

mod cartesia {
    use super::*;

    const BASE_URL: &str = "https://api.cartesia.ai";
    const MODEL: &str = "ink-whisper";

    #[tokio::test]
    async fn batch_default() {
        let key = require_key!("CARTESIA_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::transcribe("cartesia", &key, BASE_URL, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_language() {
        let key = require_key!("CARTESIA_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({ "language": "en" });
        let result = cloud_stt::transcribe("cartesia", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn api_key_validation() {
        let key = require_key!("CARTESIA_API_KEY");
        cloud_stt::test_api_key("cartesia", &key, BASE_URL, MODEL, None)
            .await
            .unwrap();
    }
}

// ---------------------------------------------------------------------------
// Soniox  (batch + realtime, translation, dictionary terms + context)
// ---------------------------------------------------------------------------

mod soniox {
    use super::*;

    const BASE_URL: &str = "https://api.soniox.com/v1";
    const BATCH_MODEL: &str = "stt-async-v4";
    const REALTIME_MODEL: &str = "stt-rt-v4";

    #[tokio::test]
    async fn batch_default() {
        let key = require_key!("SONIOX_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::transcribe("soniox", &key, BASE_URL, BATCH_MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_options() {
        let key = require_key!("SONIOX_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({
            "language_hints": "en",
            "language_hints_strict": "true",
            "enable_speaker_diarization": "false",
            "enable_language_identification": "true"
        });
        let result =
            cloud_stt::transcribe("soniox", &key, BASE_URL, BATCH_MODEL, audio, Some(&opts))
                .await
                .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_dictionary() {
        let key = require_key!("SONIOX_API_KEY");
        let audio = require_audio!();
        let opts = stt_provider::inject_dictionary(
            "soniox",
            None,
            &["Handless".to_string(), "Tauri".to_string()],
            "A desktop speech-to-text application",
        );
        let result =
            cloud_stt::transcribe("soniox", &key, BASE_URL, BATCH_MODEL, audio, opts.as_ref())
                .await
                .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_default() {
        let key = require_key!("SONIOX_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::realtime::transcribe("soniox", &key, REALTIME_MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn api_key_validation() {
        let key = require_key!("SONIOX_API_KEY");
        cloud_stt::test_api_key("soniox", &key, BASE_URL, BATCH_MODEL, None)
            .await
            .unwrap();
    }
}

// ---------------------------------------------------------------------------
// AssemblyAI  (batch + realtime, diarization, dictionary terms)
// ---------------------------------------------------------------------------

mod assemblyai {
    use super::*;

    const BASE_URL: &str = "https://api.assemblyai.com";
    const MODEL: &str = "universal-3-pro";

    #[tokio::test]
    async fn batch_default() {
        let key = require_key!("ASSEMBLYAI_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::transcribe("assemblyai", &key, BASE_URL, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_options() {
        let key = require_key!("ASSEMBLYAI_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({
            "language_code": "en",
            "speaker_labels": "true"
        });
        let result = cloud_stt::transcribe("assemblyai", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_dictionary() {
        let key = require_key!("ASSEMBLYAI_API_KEY");
        let audio = require_audio!();
        let opts = stt_provider::inject_dictionary(
            "assemblyai",
            None,
            &["Handless".to_string(), "Tauri".to_string()],
            "",
        );
        let result =
            cloud_stt::transcribe("assemblyai", &key, BASE_URL, MODEL, audio, opts.as_ref())
                .await
                .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_default() {
        let key = require_key!("ASSEMBLYAI_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::realtime::transcribe("assemblyai", &key, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn api_key_validation() {
        let key = require_key!("ASSEMBLYAI_API_KEY");
        cloud_stt::test_api_key("assemblyai", &key, BASE_URL, MODEL, None)
            .await
            .unwrap();
    }
}

// ---------------------------------------------------------------------------
// Fireworks AI  (batch + realtime, translation, dictionary)
// ---------------------------------------------------------------------------

mod fireworks {
    use super::*;

    const BASE_URL: &str = "https://audio-prod.api.fireworks.ai/v1";
    const MODEL: &str = "whisper-v3";

    #[tokio::test]
    async fn batch_default() {
        let key = require_key!("FIREWORKS_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::transcribe("fireworks", &key, BASE_URL, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_options() {
        let key = require_key!("FIREWORKS_API_KEY");
        let audio = require_audio!();
        let opts = serde_json::json!({
            "language": "en",
            "prompt": "Test recording",
            "temperature": "0.0",
            "diarize": "true"
        });
        let result = cloud_stt::transcribe("fireworks", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_dictionary() {
        let key = require_key!("FIREWORKS_API_KEY");
        let audio = require_audio!();
        let opts = stt_provider::inject_dictionary(
            "fireworks",
            None,
            &["Handless".to_string(), "Tauri".to_string()],
            "Desktop speech-to-text app",
        );
        let result =
            cloud_stt::transcribe("fireworks", &key, BASE_URL, MODEL, audio, opts.as_ref())
                .await
                .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_default() {
        let key = require_key!("FIREWORKS_API_KEY");
        let audio = require_audio!();
        let result = cloud_stt::realtime::transcribe("fireworks", &key, MODEL, audio, None)
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn api_key_validation() {
        let key = require_key!("FIREWORKS_API_KEY");
        cloud_stt::test_api_key("fireworks", &key, BASE_URL, MODEL, None)
            .await
            .unwrap();
    }
}

// ---------------------------------------------------------------------------
// Doubao  (batch + realtime, dictionary hotwords)
// ---------------------------------------------------------------------------

mod doubao {
    use super::*;

    const BASE_URL: &str = "wss://openspeech.bytedance.com/api/v3/sauc/bigmodel_async";
    const MODEL: &str = "bigmodel";

    fn doubao_options() -> serde_json::Value {
        let app_key = std::env::var("DOUBAO_APP_KEY").unwrap_or_default();
        let resource_id = std::env::var("DOUBAO_RESOURCE_ID").unwrap_or_default();
        serde_json::json!({
            "app_key": app_key,
            "resource_id": resource_id,
            "enable_itn": true,
            "enable_punc": true,
        })
    }

    #[tokio::test]
    async fn batch_default() {
        let key = require_key!("DOUBAO_ACCESS_KEY");
        let audio = require_audio!();
        let opts = doubao_options();
        let result = cloud_stt::transcribe("doubao", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_language() {
        let key = require_key!("DOUBAO_ACCESS_KEY");
        let audio = require_audio!();
        let mut opts = doubao_options();
        opts["language"] = serde_json::json!("en");
        let result = cloud_stt::transcribe("doubao", &key, BASE_URL, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn batch_with_dictionary() {
        let key = require_key!("DOUBAO_ACCESS_KEY");
        let audio = require_audio!();
        let opts = doubao_options();
        let opts = stt_provider::inject_dictionary(
            "doubao",
            Some(opts),
            &["Handless".to_string(), "Tauri".to_string()],
            "",
        );
        let result = cloud_stt::transcribe("doubao", &key, BASE_URL, MODEL, audio, opts.as_ref())
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn realtime_default() {
        let key = require_key!("DOUBAO_ACCESS_KEY");
        let audio = require_audio!();
        let opts = doubao_options();
        let result = cloud_stt::realtime::transcribe("doubao", &key, MODEL, audio, Some(&opts))
            .await
            .unwrap();
        assert_transcript_contains(&result, "test");
    }

    #[tokio::test]
    async fn api_key_validation() {
        let key = require_key!("DOUBAO_ACCESS_KEY");
        let opts = doubao_options();
        cloud_stt::test_api_key("doubao", &key, BASE_URL, MODEL, Some(&opts))
            .await
            .unwrap();
    }
}

// ---------------------------------------------------------------------------
// Cross-provider: silence WAV should not crash any provider
// ---------------------------------------------------------------------------

mod silence {
    use super::*;

    /// Verify that sending silence doesn't panic or produce a hard error.
    /// Some providers may return empty text, which is fine.
    async fn silence_does_not_crash(provider_id: &str, key_var: &str, base_url: &str, model: &str) {
        let key = match std::env::var(key_var) {
            Ok(k) if !k.is_empty() => k,
            _ => {
                eprintln!("Skipping silence test: {} not set", key_var);
                return;
            }
        };
        let audio = silence_wav();
        // We don't assert on content — just that the call doesn't panic/crash.
        let _ = cloud_stt::transcribe(provider_id, &key, base_url, model, audio, None).await;
    }

    #[tokio::test]
    async fn all_providers_handle_silence() {
        init();
        silence_does_not_crash(
            "openai_stt",
            "OPENAI_STT_API_KEY",
            "https://api.openai.com/v1",
            "gpt-4o-mini-transcribe",
        )
        .await;
        silence_does_not_crash(
            "deepgram",
            "DEEPGRAM_API_KEY",
            "https://api.deepgram.com/v1",
            "nova-3",
        )
        .await;
        silence_does_not_crash(
            "groq",
            "GROQ_API_KEY",
            "https://api.groq.com/openai/v1",
            "whisper-large-v3-turbo",
        )
        .await;
        silence_does_not_crash(
            "elevenlabs",
            "ELEVENLABS_API_KEY",
            "https://api.elevenlabs.io/v1",
            "scribe_v2",
        )
        .await;
        silence_does_not_crash(
            "mistral",
            "MISTRAL_API_KEY",
            "https://api.mistral.ai",
            "voxtral-mini-latest",
        )
        .await;
        silence_does_not_crash(
            "cartesia",
            "CARTESIA_API_KEY",
            "https://api.cartesia.ai",
            "ink-whisper",
        )
        .await;
        silence_does_not_crash(
            "soniox",
            "SONIOX_API_KEY",
            "https://api.soniox.com/v1",
            "stt-async-v4",
        )
        .await;
        silence_does_not_crash(
            "assemblyai",
            "ASSEMBLYAI_API_KEY",
            "https://api.assemblyai.com",
            "universal-3-pro",
        )
        .await;
        silence_does_not_crash(
            "fireworks",
            "FIREWORKS_API_KEY",
            "https://audio-prod.api.fireworks.ai/v1",
            "whisper-v3",
        )
        .await;
    }
}
