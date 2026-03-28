import { Apple, Monitor, ArrowRight } from "lucide-react";
import Image from "next/image";

export function BetaDownload() {
  return (
    <div className="text-center max-w-2xl mx-auto">
      <Image
        src="/images/shhh_logo_thick.png"
        alt="ShhhType"
        width={80}
        height={80}
        className="w-16 sm:w-20 h-16 sm:h-20 mx-auto mb-6 sm:mb-8"
      />

      <div className="inline-flex items-center gap-2 rounded-full border border-gray-300 bg-white/80 backdrop-blur px-4 py-1.5 text-xs font-montserrat font-semibold text-rose-500 mb-6 sm:mb-8 shadow-sm">
        <span className="inline-flex h-1.5 w-1.5 rounded-full bg-rose-500 animate-pulse"></span>
        BETA
      </div>

      <h1 className="text-3xl sm:text-5xl md:text-6xl tracking-tight text-gray-900 font-medium mb-4 sm:mb-6">
        ShhhType
        <span className="italic text-gray-400"> Beta</span>
      </h1>

      <p className="text-base sm:text-lg text-gray-500 font-montserrat font-medium max-w-lg mx-auto mb-10 sm:mb-14 leading-relaxed">
        Download the latest beta build. Voice to text, instantly.
      </p>

      <div className="flex flex-col sm:flex-row gap-4 justify-center">
        <a
          href="#"
          className="bg-gray-900 text-white pl-6 sm:pl-8 pr-5 sm:pr-6 py-4 sm:py-5 rounded-full text-base sm:text-lg hover:bg-rose-600 hover:shadow-lg hover:shadow-rose-600/20 transition-all duration-300 flex items-center justify-center gap-3 font-montserrat font-medium group/btn min-h-[52px]"
        >
          <Apple className="w-5 h-5 sm:w-6 sm:h-6" />
          Download for Mac
          <div className="bg-white/20 rounded-full p-1 group-hover/btn:bg-white/30 transition-colors">
            <ArrowRight className="w-3.5 h-3.5 group-hover/btn:translate-x-0.5 transition-transform" />
          </div>
        </a>

        <a
          href="#"
          className="bg-gray-900 text-white pl-6 sm:pl-8 pr-5 sm:pr-6 py-4 sm:py-5 rounded-full text-base sm:text-lg hover:bg-rose-600 hover:shadow-lg hover:shadow-rose-600/20 transition-all duration-300 flex items-center justify-center gap-3 font-montserrat font-medium group/btn min-h-[52px]"
        >
          <Monitor className="w-5 h-5 sm:w-6 sm:h-6" />
          Download for Windows
          <div className="bg-white/20 rounded-full p-1 group-hover/btn:bg-white/30 transition-colors">
            <ArrowRight className="w-3.5 h-3.5 group-hover/btn:translate-x-0.5 transition-transform" />
          </div>
        </a>
      </div>

      <p className="text-xs text-gray-400 font-montserrat font-medium mt-8">
        macOS 13+ &middot; Windows 10+
      </p>
    </div>
  );
}
