import { Check, Info } from "lucide-react";

export function LocalVsCloud() {
  return (
    <div className="py-16 sm:py-24">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-10 sm:mb-16">
          <h2 className="text-3xl sm:text-4xl md:text-5xl tracking-tight text-gray-900 font-serif mb-6">
            Two Modes, <span className="italic text-gray-400">One App</span>
          </h2>
          <p className="text-sm sm:text-lg text-gray-500 font-montserrat font-medium max-w-2xl mx-auto">
            Cloud is the default for speed and accuracy. Local mode is there when you need full privacy.
          </p>
        </div>

        <div className="bg-white rounded-2xl sm:rounded-[2.5rem] border border-gray-200 shadow-xl shadow-gray-200/40 overflow-hidden">
          <div className="grid md:grid-cols-2">
            {/* Cloud (Groq) */}
            <div className="p-6 sm:p-10 md:p-12 border-b md:border-b-0 md:border-r border-gray-100">
              <div className="flex items-center gap-3 mb-4">
                <div className="w-10 h-10 rounded-full bg-green-50 flex items-center justify-center text-green-600">
                  <Check className="w-5 h-5" />
                </div>
                <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900">Cloud (Groq)</h3>
              </div>
              <span className="inline-block mb-8 text-xs font-montserrat font-bold text-rose-600 bg-rose-50 px-3 py-1 rounded-full border border-rose-100">RECOMMENDED</span>
              <div className="space-y-6">
                <div>
                  <h4 className="text-base font-bold text-gray-900 font-montserrat mb-1">Whisper Large V3 Turbo</h4>
                  <p className="text-sm text-gray-500 font-medium font-montserrat leading-relaxed">State-of-the-art speech recognition model.</p>
                </div>
                <div>
                  <h4 className="text-base font-bold text-gray-900 font-montserrat mb-1">Sub-second Transcription</h4>
                  <p className="text-sm text-gray-500 font-medium font-montserrat leading-relaxed">Results appear almost instantly after you stop speaking.</p>
                </div>
                <div>
                  <h4 className="text-base font-bold text-gray-900 font-montserrat mb-1">No Model Download</h4>
                  <p className="text-sm text-gray-500 font-medium font-montserrat leading-relaxed">Start transcribing immediately — nothing to install.</p>
                </div>
                <div>
                  <h4 className="text-base font-bold text-gray-900 font-montserrat mb-1">AI Rewrite Enabled</h4>
                  <p className="text-sm text-gray-500 font-medium font-montserrat leading-relaxed">Polish your text with Llama 3.3 70B in one hotkey.</p>
                </div>
                <div>
                  <h4 className="text-base font-bold text-gray-900 font-montserrat mb-1">Free Groq API Key Required</h4>
                  <p className="text-sm text-gray-500 font-medium font-montserrat leading-relaxed">Bring your own free Groq API key (takes 30 seconds to get). The generous Groq free tier is more than enough for normal dictation use.</p>
                </div>
              </div>
            </div>

            {/* Local (Whisper) */}
            <div className="p-6 sm:p-10 md:p-12 bg-gray-50/50">
              <div className="flex items-center gap-3 mb-8">
                <div className="w-10 h-10 rounded-full bg-gray-200/50 flex items-center justify-center text-gray-500">
                  <Info className="w-5 h-5" />
                </div>
                <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900">Local (Whisper)</h3>
              </div>
              <div className="space-y-6">
                <div>
                  <h4 className="text-base font-bold text-gray-900 font-montserrat mb-1">On-device via whisper.cpp</h4>
                  <p className="text-sm text-gray-500 font-medium font-montserrat leading-relaxed">Everything runs locally on your machine.</p>
                </div>
                <div>
                  <h4 className="text-base font-bold text-gray-900 font-montserrat mb-1">Metal GPU Acceleration</h4>
                  <p className="text-sm text-gray-500 font-medium font-montserrat leading-relaxed">GPU accelerated on macOS (Metal) and Windows (DirectML).</p>
                </div>
                <div>
                  <h4 className="text-base font-bold text-gray-900 font-montserrat mb-1">Multiple Model Sizes</h4>
                  <p className="text-sm text-gray-500 font-medium font-montserrat leading-relaxed">From Tiny (75MB) to Large V3 (3.1GB) — pick your balance.</p>
                </div>
                <div>
                  <h4 className="text-base font-bold text-gray-900 font-montserrat mb-1">Fully Offline</h4>
                  <p className="text-sm text-gray-500 font-medium font-montserrat leading-relaxed">Zero data leaves your machine. Complete privacy.</p>
                </div>
                <div>
                  <h4 className="text-base font-bold text-gray-900 font-montserrat mb-1">Slower on Larger Models</h4>
                  <p className="text-sm text-gray-500 font-medium font-montserrat leading-relaxed">Transcription time depends on model size and hardware.</p>
                </div>
              </div>
            </div>
          </div>

          {/* Footer */}
          <div className="bg-[#1A2626] p-6 sm:p-8 md:p-10 text-center border-t border-gray-800/30">
            <p className="text-gray-300 font-montserrat font-medium text-sm md:text-base leading-relaxed max-w-4xl mx-auto">
              We recommend Cloud mode for the best experience — Groq&apos;s free tier handles normal dictation easily. Just bring your own free API key. Switch to Local anytime in Settings for full offline privacy.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
