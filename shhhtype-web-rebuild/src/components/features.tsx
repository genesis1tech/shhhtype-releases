import { Mic, Lock, Sparkles, Globe, Clock, ArrowRight, Hash, Flame } from "lucide-react";

const SIGNUP_URL = "/signup";

export function Features() {
  return (
    <div id="features" className="py-16 sm:py-24 overflow-hidden">
      <h2 className="text-3xl sm:text-4xl md:text-5xl text-center mb-4 tracking-tight text-gray-900 font-montserrat font-semibold">Everything you need to post daily</h2>
      <p className="text-base sm:text-lg text-gray-500 font-montserrat font-medium text-center mb-10 sm:mb-16 max-w-2xl mx-auto">Voice-triggered skills turn your speech into LinkedIn posts, DMs, and connection notes — formatted, structured, and ready to go.</p>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-8">
        {/* Card 1: Voice Skills */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col h-full min-w-0 overflow-hidden">
          <div className="h-28 sm:h-44 w-full bg-[#FFE4D6] rounded-2xl flex items-center justify-center mb-5 sm:mb-8 text-orange-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <Mic className="w-10 sm:w-12 h-10 sm:h-12" />
          </div>
          <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">Never leave your flow state</h3>
          <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">Say a trigger word and your speech becomes platform-ready content. No menus, no mode selection — just speak and publish. Build your own skills with simple .md files.</p>
          <div className="flex flex-wrap gap-2 mb-4 sm:mb-6">
            <span className="text-xs font-mono font-bold text-green-600 bg-green-50 px-2.5 py-1 rounded-full border border-green-200">/linkedin</span>
            <span className="text-xs font-mono font-bold text-green-600 bg-green-50 px-2.5 py-1 rounded-full border border-green-200">/dm</span>
            <span className="text-xs font-mono font-bold text-green-600 bg-green-50 px-2.5 py-1 rounded-full border border-green-200">/connection</span>
          </div>
          <p className="text-xs text-gray-400 font-montserrat font-medium leading-relaxed mt-auto">Say the trigger at the start or end of your recording.</p>
          <div className="mt-3 pt-2">
            <p className="text-xs font-semibold text-gray-400 uppercase tracking-wider">Voice-triggered &middot; Extensible</p>
          </div>
        </div>

        {/* Card 2: AI Rewrite */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col h-full min-w-0 overflow-hidden">
          <div className="h-28 sm:h-44 w-full bg-[#E0E7FF] rounded-2xl flex items-center justify-center mb-5 sm:mb-8 text-indigo-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <Sparkles className="w-10 sm:w-12 h-10 sm:h-12" />
          </div>
          <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">Your words, just structured better</h3>
          <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">Four rewrite styles — Professional, Casual, Concise, Friendly. AI restructures your spoken words, it doesn&apos;t replace your voice.</p>
          <div className="mt-auto pt-2">
            <p className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-4 sm:mb-6">Qwen3 32B &middot; Smart</p>
          </div>
        </div>

        {/* Card 3: LinkedIn Formatting */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col h-full min-w-0 overflow-hidden">
          <div className="h-28 sm:h-44 w-full bg-[#FFE4D6] rounded-2xl flex items-center justify-center mb-5 sm:mb-8 text-orange-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <Hash className="w-10 sm:w-12 h-10 sm:h-12" />
          </div>
          <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">Posts that look polished instantly</h3>
          <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">Unicode bold, line spacing, scroll-stopping hooks, and hashtags — all applied automatically. No manual formatting, ever.</p>
          <div className="mt-auto pt-2">
            <p className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-4 sm:mb-6">Automatic &middot; Feed-optimized</p>
          </div>
        </div>

        {/* Card 4: Privacy First */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col h-full min-w-0 overflow-hidden">
          <div className="h-28 sm:h-44 w-full bg-[#E0E7FF] rounded-2xl flex items-center justify-center mb-5 sm:mb-8 text-indigo-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <Lock className="w-10 sm:w-12 h-10 sm:h-12" />
          </div>
          <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">Works on a plane or behind a firewall</h3>
          <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">Dual transcription engine — cloud for speed, local Whisper for full privacy. No internet? No problem.</p>
          <div className="mt-auto pt-2">
            <p className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-4 sm:mb-6">On-device &middot; Secure</p>
          </div>
        </div>

        {/* Card 5: 9 Languages */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col h-full min-w-0 overflow-hidden">
          <div className="h-28 sm:h-44 w-full bg-[#FFE4D6] rounded-2xl flex items-center justify-center mb-5 sm:mb-8 text-orange-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <Globe className="w-10 sm:w-12 h-10 sm:h-12" />
          </div>
          <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">Create content in 9 languages</h3>
          <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">English, Spanish, French, German, Italian, Portuguese, Japanese, Korean, Chinese — with automatic detection.</p>
          <div className="mt-auto pt-2">
            <p className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-4 sm:mb-6">Multilingual &middot; Auto-detect</p>
          </div>
        </div>

        {/* Card 6: Smart History */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col h-full min-w-0 overflow-hidden">
          <div className="h-28 sm:h-44 w-full bg-[#E0E7FF] rounded-2xl flex items-center justify-center mb-5 sm:mb-8 text-indigo-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <Clock className="w-10 sm:w-12 h-10 sm:h-12" />
          </div>
          <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">Every transcription, searchable</h3>
          <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">Full history with search and export. Custom dictionary auto-corrects industry terms and names.</p>
          <div className="mt-auto pt-2">
            <p className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-4 sm:mb-6">Searchable &middot; Exportable</p>
          </div>
        </div>
      </div>

      {/* Creator Styles */}
      <div className="mt-10 sm:mt-16">
        <h3 className="text-2xl sm:text-3xl text-center mb-6 sm:mb-10 tracking-tight text-gray-900 font-montserrat font-semibold">Creator Styles</h3>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-8">
          <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col h-full min-w-0 overflow-hidden">
            <div className="h-28 sm:h-44 w-full bg-[#FFE4D6] rounded-2xl flex items-center justify-center mb-5 sm:mb-8 text-orange-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
              <Flame className="w-10 sm:w-12 h-10 sm:h-12" />
            </div>
            <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">Hormozi Style</h3>
            <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">Write in Alex Hormozi&apos;s punchy, framework-driven voice. Bold claims, short sentences, contrarian hooks. Perfect for business content.</p>
            <div className="flex flex-wrap gap-2 mb-4 sm:mb-6">
              <span className="text-xs font-mono font-bold text-green-600 bg-green-50 px-2.5 py-1 rounded-full border border-green-200">/hormozi</span>
            </div>
            <div className="mt-auto pt-2">
              <p className="text-xs font-semibold text-gray-400 uppercase tracking-wider">Creator Voice &middot; Business</p>
            </div>
          </div>

          {/* Placeholder for future styles */}
          <div className="bg-gray-50 rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-dashed border-gray-300 flex flex-col items-center justify-center h-full min-h-[280px]">
            <p className="text-sm font-montserrat font-semibold text-gray-400 mb-2">More coming soon</p>
            <p className="text-xs text-gray-400 font-montserrat text-center">Create your own styles with simple .md files</p>
          </div>
        </div>
      </div>

      {/* Single Buy Now CTA */}
      <div className="text-center mt-10 sm:mt-12">
        <a
          href={SIGNUP_URL}
          className="inline-flex items-center gap-2 sm:gap-3 bg-gray-900 text-white pl-6 sm:pl-8 pr-5 sm:pr-6 py-3.5 sm:py-4 rounded-full text-sm sm:text-base hover:bg-rose-600 hover:shadow-lg hover:shadow-rose-600/20 transition-all duration-300 font-montserrat font-medium group/btn min-h-[44px]"
        >
          Start Free Trial
          <div className="bg-white/20 rounded-full p-1 group-hover/btn:bg-white/30 transition-colors">
            <ArrowRight className="w-3 h-3 group-hover/btn:translate-x-0.5 transition-transform" />
          </div>
        </a>
      </div>
    </div>
  );
}
