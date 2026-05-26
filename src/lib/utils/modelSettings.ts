import type { SttProviderInfo } from "@/bindings";

/**
 * Derive inline-settings props for a ModelCard based on whether
 * the provider is the active local model and what it supports.
 */
export const getModelSettingsProps = (provider: SttProviderInfo) => {
  const engineType =
    provider.backend.type === "Local"
      ? provider.backend.engine_type
      : undefined;
  const supportsAutoLanguage =
    provider.backend.type !== "Local" ||
    engineType === "Whisper" ||
    engineType === "SenseVoice";
  const supportsLanguageSelection =
    engineType === "Whisper" ||
    engineType === "SenseVoice" ||
    engineType === "Canary" ||
    engineType === "Cohere";
  return {
    showSettings:
      supportsLanguageSelection ||
      provider.supports_translation ||
      provider.supports_realtime,
    supportedLanguages: provider.supported_languages,
    supportsTranslation: provider.supports_translation,
    supportsAutoLanguage,
  };
};
