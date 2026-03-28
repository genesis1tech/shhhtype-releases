import { Infinity, Sparkles, Globe, Mic } from "lucide-react";

const CHECKOUT_URL =
  "https://shhhtype.lemonsqueezy.com/checkout/buy/1ea919ae-5f44-4ea9-bc4d-95e64cb41a87";

export function ProVersion() {
  return (
    <div className="bg-gray-900 rounded-2xl sm:rounded-[2rem] md:rounded-[3rem] p-6 sm:p-10 md:p-16 lg:p-20 relative overflow-hidden shadow-2xl mt-8 sm:mt-12">
      {/* Background Effects */}
      <div className="absolute top-0 left-0 w-[300px] sm:w-[500px] h-[300px] sm:h-[500px] bg-orange-500/10 rounded-full blur-[120px] pointer-events-none"></div>
      <div className="absolute bottom-0 right-0 w-[250px] sm:w-[400px] h-[250px] sm:h-[400px] bg-rose-500/10 rounded-full blur-[100px] pointer-events-none"></div>

      <div className="relative z-10">
        <div className="inline-flex items-center gap-2 rounded-full border border-gray-700 bg-gray-800/50 px-4 py-1 text-xs font-montserrat font-semibold text-rose-400 mb-8">
          <span className="inline-flex h-1.5 w-1.5 rounded-full bg-rose-500 animate-pulse"></span>
          PRO VERSION
        </div>

        <div className="grid lg:grid-cols-2 gap-8 lg:gap-16 items-start">
          <div>
            <h2 className="text-2xl sm:text-4xl md:text-5xl text-white mb-4 sm:mb-6 tracking-tight font-medium">
              Post every day without<br />
              <span className="text-gray-400 italic">it consuming your morning.</span>
            </h2>
            <p className="text-base sm:text-lg text-gray-400 mb-8 sm:mb-10 leading-relaxed font-montserrat">
              Ghostwriters cost $500+/month and still don&apos;t sound like you. ChatGPT is generic. ShhhType uses YOUR voice as input — $15/month for content that actually sounds like you wrote it.
            </p>

            <div className="space-y-6">
              <div className="flex gap-4">
                <div className="bg-gray-800 p-3 rounded-xl h-fit">
                  <Mic className="w-5 h-5 text-rose-400" />
                </div>
                <div>
                  <h4 className="text-white text-lg font-semibold font-montserrat">All Voice Skills</h4>
                  <p className="text-gray-500 text-sm mt-1 font-montserrat"><span className="text-green-400 font-mono">/linkedin</span>, <span className="text-green-400 font-mono">/dm</span>, <span className="text-green-400 font-mono">/connection</span> — plus create your own with simple .md files.</p>
                </div>
              </div>
              <div className="flex gap-4">
                <div className="bg-gray-800 p-3 rounded-xl h-fit">
                  <Sparkles className="w-5 h-5 text-rose-400" />
                </div>
                <div>
                  <h4 className="text-white text-lg font-semibold font-montserrat">AI Rewrite Included</h4>
                  <p className="text-gray-500 text-sm mt-1 font-montserrat">All 4 styles, composition buffer, powered by Qwen3 32B.</p>
                </div>
              </div>
              <div className="flex gap-4">
                <div className="bg-gray-800 p-3 rounded-xl h-fit">
                  <Globe className="w-5 h-5 text-rose-400" />
                </div>
                <div>
                  <h4 className="text-white text-lg font-semibold font-montserrat">All 9 Languages</h4>
                  <p className="text-gray-500 text-sm mt-1 font-montserrat">Plus auto-detect. Create LinkedIn content in any supported language.</p>
                </div>
              </div>
              <div className="flex gap-4">
                <div className="bg-gray-800 p-3 rounded-xl h-fit">
                  <Infinity className="w-5 h-5 text-rose-400" />
                </div>
                <div>
                  <h4 className="text-white text-lg font-semibold font-montserrat">Cloud Transcription</h4>
                  <p className="text-gray-500 text-sm mt-1 font-montserrat">Powered by Groq&apos;s generous free tier. Bring your own free API key.</p>
                </div>
              </div>
            </div>

            <div className="mt-8 sm:mt-12 flex flex-col sm:flex-row gap-3 sm:gap-4">
              <a
                href={CHECKOUT_URL}
                className="bg-white text-gray-900 px-6 py-3.5 rounded-full font-semibold hover:bg-rose-50 transition-colors font-montserrat text-center min-h-[44px] flex items-center justify-center"
              >
                Start Free Trial
              </a>
              <a
                href="#pricing"
                className="border border-gray-700 text-white px-6 py-3.5 rounded-full font-semibold hover:bg-gray-800 transition-colors font-montserrat text-center min-h-[44px] flex items-center justify-center"
              >
                Learn More
              </a>
            </div>
          </div>

          {/* Terminal-style license card */}
          <div className="bg-gray-800/50 backdrop-blur border border-gray-700 rounded-2xl sm:rounded-3xl p-4 sm:p-8 font-mono text-xs sm:text-sm text-gray-300 overflow-hidden min-w-0">
            <div className="flex gap-2 mb-6">
              <div className="w-3 h-3 rounded-full bg-red-500"></div>
              <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
              <div className="w-3 h-3 rounded-full bg-green-500"></div>
            </div>
            <p className="text-gray-500 mb-4">{"// ShhhType License"}</p>
            <div className="space-y-1 overflow-hidden">
              <p className="text-gray-500 mb-3">{"// Cost comparison:"}</p>
              <p className="break-all"><span className="text-purple-400">ghostwriter</span>: <span className="text-red-400 line-through">&quot;$500+/mo&quot;</span></p>
              <p className="break-all"><span className="text-purple-400">taplio</span>: <span className="text-red-400 line-through">&quot;$49/mo&quot;</span></p>
              <p className="break-all"><span className="text-purple-400">chatgpt_pro</span>: <span className="text-red-400 line-through">&quot;$20/mo&quot;</span> <span className="text-gray-500">// generic output</span></p>
              <p className="break-all mt-3"><span className="text-purple-400">shhhtype</span>: <span className="text-green-400">&quot;$15/mo&quot;</span> <span className="text-gray-500">// sounds like YOU</span></p>
              <p className="break-all"><span className="text-purple-400">trial</span>: <span className="text-green-400">&quot;7 days free&quot;</span></p>
              <p className="break-all"><span className="text-purple-400">voice_skills</span>: <span className="text-orange-300">linkedin, dm, connect, hormozi</span></p>
              <p className="break-all"><span className="text-purple-400">custom_skills</span>: <span className="text-green-400">&quot;unlimited&quot;</span></p>
            </div>

            <div className="mt-8 pt-6 border-t border-gray-700">
              <div className="flex justify-between items-center gap-2 mb-2">
                <span className="text-gray-400 text-xs truncate">Subscription Status</span>
                <span className="text-green-400 text-xs font-semibold flex-shrink-0">Active</span>
              </div>
              <div className="w-full bg-gray-700 h-2 rounded-full">
                <div className="bg-green-500 h-2 rounded-full w-full"></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
