import { Palette, Layers, Zap } from "lucide-react";

const CHECKOUT_URL =
  "https://shhhtype.lemonsqueezy.com/checkout/buy/c74816d2-4704-4248-abcb-fe565d518935";

export function AiRewrite() {
  return (
    <div className="bg-gray-900 rounded-2xl sm:rounded-[2rem] md:rounded-[3rem] p-6 sm:p-10 md:p-16 lg:p-20 relative overflow-hidden shadow-2xl">
      {/* Background Effects */}
      <div className="absolute top-0 right-0 w-[300px] sm:w-[500px] h-[300px] sm:h-[500px] bg-rose-500/10 rounded-full blur-[120px] pointer-events-none"></div>
      <div className="absolute bottom-0 left-0 w-[250px] sm:w-[400px] h-[250px] sm:h-[400px] bg-blue-500/10 rounded-full blur-[100px] pointer-events-none"></div>

      <div className="relative z-10">
        <div className="inline-flex items-center gap-2 rounded-full border border-gray-700 bg-gray-800/50 px-4 py-1 text-xs font-montserrat font-semibold text-rose-400 mb-8">
          <span className="inline-flex h-1.5 w-1.5 rounded-full bg-rose-500 animate-pulse"></span>
          AI REWRITE
        </div>

        <div className="grid lg:grid-cols-2 gap-8 lg:gap-16 items-start">
          <div>
            <h2 className="text-2xl sm:text-4xl md:text-5xl text-white mb-4 sm:mb-6 tracking-tight font-medium">
              Your voice. Your ideas.<br />
              <span className="text-gray-400 italic">Just structured better.</span>
            </h2>
            <p className="text-base sm:text-lg text-gray-400 mb-8 sm:mb-10 leading-relaxed font-montserrat">
              ShhhType rewrites YOUR words, not a generic prompt. Your stories, your examples, your voice — structured for LinkedIn so it sounds like you, not ChatGPT.
            </p>

            <div className="space-y-6">
              <div className="flex gap-4">
                <div className="bg-gray-800 p-3 rounded-xl h-fit">
                  <Palette className="w-5 h-5 text-rose-400" />
                </div>
                <div>
                  <h4 className="text-white text-lg font-semibold font-montserrat">Voice-Triggered Skills</h4>
                  <p className="text-gray-500 text-sm mt-1 font-montserrat">Say <span className="text-green-400 font-mono">/linkedin</span>, <span className="text-green-400 font-mono">/dm</span>, or <span className="text-green-400 font-mono">/connect</span> — or build your own with a simple .md file.</p>
                </div>
              </div>
              <div className="flex gap-4">
                <div className="bg-gray-800 p-3 rounded-xl h-fit">
                  <Layers className="w-5 h-5 text-rose-400" />
                </div>
                <div>
                  <h4 className="text-white text-lg font-semibold font-montserrat">Think Out Loud</h4>
                  <p className="text-gray-500 text-sm mt-1 font-montserrat">Record multiple segments over 10 minutes, then rewrite them all as one cohesive piece.</p>
                </div>
              </div>
              <div className="flex gap-4">
                <div className="bg-gray-800 p-3 rounded-xl h-fit">
                  <Zap className="w-5 h-5 text-rose-400" />
                </div>
                <div>
                  <h4 className="text-white text-lg font-semibold font-montserrat">Works In Any App</h4>
                  <p className="text-gray-500 text-sm mt-1 font-montserrat">Text appears wherever your cursor is. Chrome, Slack, LinkedIn, Notes — anywhere on your Mac or Windows.</p>
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
                See Pricing
              </a>
            </div>
          </div>

          {/* Terminal-style card */}
          <div className="bg-gray-800/50 backdrop-blur border border-gray-700 rounded-2xl sm:rounded-3xl p-4 sm:p-8 font-mono text-xs sm:text-sm text-gray-300 overflow-hidden min-w-0">
            <div className="flex gap-2 mb-4 sm:mb-6">
              <div className="w-3 h-3 rounded-full bg-red-500"></div>
              <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
              <div className="w-3 h-3 rounded-full bg-green-500"></div>
            </div>
            <p className="text-gray-500 mb-2">{"// Voice input:"}</p>
            <p className="text-orange-300 mb-4 sm:mb-6 leading-relaxed break-words">&quot;/linkedin I just realized the biggest mistake founders make is trying to build a perfect product before talking to a single customer ship fast get feedback iterate&quot;</p>
            <p className="text-gray-500 mb-2">{"// → LinkedIn Post:"}</p>
            <p className="text-green-400 leading-relaxed break-words">&quot;**The biggest mistake founders make?**<br/><br/>Building the &ldquo;perfect&rdquo; product before talking to a single customer.<br/><br/>Here&apos;s what works instead:<br/><br/>1. Ship fast<br/>2. Get feedback<br/>3. Iterate<br/><br/>Your first version will never be perfect. But it will be real.<br/><br/>#founders #startups #buildinpublic&quot;</p>

            <div className="mt-6 sm:mt-8 pt-4 sm:pt-6 border-t border-gray-700">
              <div className="flex flex-wrap gap-2 mb-4 sm:mb-6">
                <span className="px-3 py-1.5 rounded-lg bg-rose-500/20 border border-rose-500/30 text-xs font-semibold text-rose-400">LinkedIn Post</span>
                <span className="px-3 py-1.5 rounded-lg bg-gray-700/50 border border-gray-600/30 text-xs font-medium text-gray-400">Professional</span>
                <span className="px-3 py-1.5 rounded-lg bg-gray-700/50 border border-gray-600/30 text-xs font-medium text-gray-400">Concise</span>
                <span className="px-3 py-1.5 rounded-lg bg-gray-700/50 border border-gray-600/30 text-xs font-medium text-gray-400">Friendly</span>
              </div>
              <div className="flex justify-between items-center gap-2 min-w-0">
                <span className="text-gray-500 text-xs truncate">Powered by Qwen3 32B</span>
                <span className="text-green-400 text-xs font-semibold flex-shrink-0">Ready</span>
              </div>
              <div className="w-full bg-gray-700 h-1.5 rounded-full mt-2">
                <div className="bg-green-500 h-1.5 rounded-full w-full"></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
