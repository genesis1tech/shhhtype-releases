import { Cpu, AudioWaveform, Layers, Cog, Brain } from "lucide-react";

function AppleSiliconIcon({ className }: { className?: string }) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="32"
      height="32"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      strokeLinecap="round"
      strokeLinejoin="round"
      className={className}
    >
      <path d="M12 2a8.4 8.4 0 0 0-2.2.3 7.5 7.5 0 0 0-4.6 3.5A8.3 8.3 0 0 0 4 9.5c0 1.4.3 2.6.8 3.7.5 1 1.2 1.8 2 2.6l.6.5c.3.3.5.7.6 1.1V20a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-2.6c.1-.4.3-.8.6-1.1l.6-.5c.8-.8 1.5-1.6 2-2.6.5-1.1.8-2.3.8-3.7a8.3 8.3 0 0 0-1.2-3.7 7.5 7.5 0 0 0-4.6-3.5A8.4 8.4 0 0 0 12 2z" />
    </svg>
  );
}

function MarqueeStrip() {
  return (
    <div className="flex items-center justify-around w-1/2 gap-6 sm:gap-16 px-4 sm:px-8">
      <div className="flex items-center gap-3 text-gray-400 hover:text-rose-600 transition-colors duration-300">
        <Cpu className="w-6 h-6 sm:w-8 sm:h-8" />
        <span className="text-sm sm:text-lg font-montserrat font-semibold">Groq</span>
      </div>
      <div className="flex items-center gap-3 text-gray-400 hover:text-orange-600 transition-colors duration-300">
        <AudioWaveform className="w-6 h-6 sm:w-8 sm:h-8" />
        <span className="text-sm sm:text-lg font-montserrat font-semibold">Whisper</span>
      </div>
      <div className="flex items-center gap-3 text-gray-400 hover:text-gray-900 transition-colors duration-300">
        <AppleSiliconIcon className="w-6 h-6 sm:w-8 sm:h-8" />
        <span className="text-sm sm:text-lg font-montserrat font-semibold hidden sm:inline">Apple Silicon</span>
        <span className="text-sm font-montserrat font-semibold sm:hidden">Apple</span>
      </div>
      <div className="flex items-center gap-3 text-gray-400 hover:text-blue-600 transition-colors duration-300">
        <Layers className="w-6 h-6 sm:w-8 sm:h-8" />
        <span className="text-sm sm:text-lg font-montserrat font-semibold">Tauri</span>
      </div>
      <div className="flex items-center gap-3 text-gray-400 hover:text-orange-600 transition-colors duration-300">
        <Cog className="w-6 h-6 sm:w-8 sm:h-8" />
        <span className="text-sm sm:text-lg font-montserrat font-semibold">Rust</span>
      </div>
      <div className="flex items-center gap-3 text-gray-400 hover:text-purple-600 transition-colors duration-300">
        <Brain className="w-6 h-6 sm:w-8 sm:h-8" />
        <span className="text-sm sm:text-lg font-montserrat font-semibold">Llama 3.3</span>
      </div>
    </div>
  );
}

export function Marquee() {
  return (
    <div className="w-full py-8 sm:py-12 mt-4 sm:mt-8 overflow-hidden marquee-mask relative group bg-transparent">
      <div className="flex w-[200%] animate-infinite-scroll hover:[animation-play-state:paused]">
        <MarqueeStrip />
        <MarqueeStrip />
      </div>
    </div>
  );
}
