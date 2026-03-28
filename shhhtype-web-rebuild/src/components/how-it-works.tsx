import { Keyboard, Mic, Zap, Sparkles, ArrowRight } from "lucide-react";

export function HowItWorks() {
  return (
    <div id="how-it-works" className="py-16 sm:py-24 overflow-hidden">
      <h2 className="text-3xl sm:text-4xl md:text-5xl text-center mb-4 tracking-tight text-gray-900 font-montserrat font-semibold">From idea to LinkedIn post in 60 seconds</h2>
      <p className="text-base sm:text-lg text-gray-500 font-montserrat font-medium text-center mb-10 sm:mb-16 max-w-2xl mx-auto">You speak at 150 words per minute. You type at 40. Start with your voice.</p>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-8">
        {/* Step 1: Trigger */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col min-w-0 overflow-hidden">
          <div className="w-10 h-10 sm:w-12 sm:h-12 rounded-full bg-gradient-to-br from-rose-500 to-orange-500 flex items-center justify-center text-white text-base sm:text-lg font-montserrat font-bold mb-4 sm:mb-6 flex-shrink-0">1</div>
          <div className="h-16 sm:h-20 w-full bg-[#FFE4D6] rounded-xl sm:rounded-2xl flex items-center justify-center mb-4 sm:mb-6 text-orange-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <Keyboard className="w-8 h-8 sm:w-10 sm:h-10" />
          </div>
          <h3 className="text-base sm:text-xl font-montserrat font-semibold text-gray-900 mb-2 sm:mb-3 tracking-tight">Trigger</h3>
          <p className="text-xs sm:text-sm text-gray-500 font-montserrat font-medium leading-relaxed mb-4">Press <code className="text-xs bg-gray-100 px-1.5 py-0.5 rounded">Option+V</code> from any app.</p>
          <div className="mt-auto flex flex-wrap gap-1.5">
            <span className="inline-flex items-center justify-center bg-gray-100 text-gray-700 text-xs font-mono font-bold px-2.5 py-1.5 rounded-lg border border-gray-200 shadow-sm">&#8997;</span>
            <span className="inline-flex items-center justify-center bg-gray-100 text-gray-700 text-xs font-mono font-bold px-2.5 py-1.5 rounded-lg border border-gray-200 shadow-sm">V</span>
          </div>
        </div>

        {/* Step 2: Speak */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col min-w-0 overflow-hidden">
          <div className="w-10 h-10 sm:w-12 sm:h-12 rounded-full bg-gradient-to-br from-rose-500 to-orange-500 flex items-center justify-center text-white text-base sm:text-lg font-montserrat font-bold mb-4 sm:mb-6 flex-shrink-0">2</div>
          <div className="h-16 sm:h-20 w-full bg-[#E0E7FF] rounded-xl sm:rounded-2xl flex items-center justify-center mb-4 sm:mb-6 text-indigo-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <Mic className="w-8 h-8 sm:w-10 sm:h-10" />
          </div>
          <h3 className="text-base sm:text-xl font-montserrat font-semibold text-gray-900 mb-2 sm:mb-3 tracking-tight">Speak Your Idea</h3>
          <p className="text-xs sm:text-sm text-gray-500 font-montserrat font-medium leading-relaxed mb-4">Talk like you&apos;re explaining it to a friend. Say &quot;/linkedin&quot; to activate a skill.</p>
          <div className="mt-auto flex items-end gap-1 h-8">
            <div className="wave-bar opacity-40" style={{ animationDelay: "0s" }}></div>
            <div className="wave-bar opacity-60" style={{ animationDelay: "0.15s" }}></div>
            <div className="wave-bar opacity-80" style={{ animationDelay: "0.3s" }}></div>
            <div className="wave-bar" style={{ animationDelay: "0.45s" }}></div>
            <div className="wave-bar opacity-70" style={{ animationDelay: "0.6s" }}></div>
            <div className="wave-bar opacity-50" style={{ animationDelay: "0.75s" }}></div>
            <div className="wave-bar opacity-90" style={{ animationDelay: "0.9s" }}></div>
            <div className="wave-bar opacity-30" style={{ animationDelay: "1.05s" }}></div>
          </div>
        </div>

        {/* Step 3: AI Polishes */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col min-w-0 overflow-hidden">
          <div className="w-10 h-10 sm:w-12 sm:h-12 rounded-full bg-gradient-to-br from-rose-500 to-orange-500 flex items-center justify-center text-white text-base sm:text-lg font-montserrat font-bold mb-4 sm:mb-6 flex-shrink-0">3</div>
          <div className="h-16 sm:h-20 w-full bg-[#FFE4D6] rounded-xl sm:rounded-2xl flex items-center justify-center mb-4 sm:mb-6 text-orange-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <Zap className="w-8 h-8 sm:w-10 sm:h-10" />
          </div>
          <h3 className="text-base sm:text-xl font-montserrat font-semibold text-gray-900 mb-2 sm:mb-3 tracking-tight">AI Structures It</h3>
          <p className="text-xs sm:text-sm text-gray-500 font-montserrat font-medium leading-relaxed mb-4">Hook, bold text, line breaks, hashtags — your words, structured for LinkedIn.</p>
          <div className="mt-auto bg-gray-900 rounded-xl px-3 py-2.5 overflow-hidden">
            <p className="text-xs font-mono text-green-400 break-words">&rarr; &quot;**3 things I learned** about building in public...&quot;</p>
          </div>
        </div>

        {/* Step 4: Post */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col min-w-0 overflow-hidden">
          <div className="w-10 h-10 sm:w-12 sm:h-12 rounded-full bg-gradient-to-br from-rose-500 to-orange-500 flex items-center justify-center text-white text-base sm:text-lg font-montserrat font-bold mb-4 sm:mb-6 flex-shrink-0">4</div>
          <div className="h-16 sm:h-20 w-full bg-[#E0E7FF] rounded-xl sm:rounded-2xl flex items-center justify-center mb-4 sm:mb-6 text-indigo-500 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <Sparkles className="w-8 h-8 sm:w-10 sm:h-10" />
          </div>
          <h3 className="text-base sm:text-xl font-montserrat font-semibold text-gray-900 mb-2 sm:mb-3 tracking-tight">Paste and Publish</h3>
          <p className="text-xs sm:text-sm text-gray-500 font-montserrat font-medium leading-relaxed mb-4">Ready-to-publish post. Tweak one line if you want to. Done in 60 seconds.</p>
          <div className="mt-auto flex items-center gap-2">
            <span className="text-xs font-montserrat font-medium text-gray-400">voice note</span>
            <ArrowRight className="w-3.5 h-3.5 text-rose-500 flex-shrink-0" />
            <span className="text-xs font-montserrat font-semibold text-rose-600 bg-rose-50 px-2.5 py-1 rounded-full">LinkedIn Post</span>
          </div>
        </div>
      </div>
    </div>
  );
}
