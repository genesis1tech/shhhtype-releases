import { ArrowRight } from "lucide-react";
import Image from "next/image";

const SIGNUP_URL = "/signup";

export function FinalCta() {
  return (
    <div className="bg-white rounded-2xl sm:rounded-[2rem] md:rounded-[3rem] p-6 sm:p-10 md:p-16 lg:p-20 shadow-sm border border-gray-100 text-center">
      <Image
        src="/images/shhh_logo_thick.png"
        alt="ShhhType"
        width={80}
        height={80}
        className="w-16 sm:w-20 h-16 sm:h-20 mx-auto mb-6 sm:mb-8"
      />
      <h2 className="text-2xl sm:text-4xl md:text-5xl lg:text-6xl tracking-tight text-gray-900 font-medium mb-4 sm:mb-6">
        Stop editing.
        <span className="italic text-gray-400"> Start publishing.</span>
      </h2>
      <p className="text-base sm:text-lg md:text-xl text-gray-500 font-montserrat font-medium max-w-2xl mx-auto mb-8 sm:mb-10 leading-relaxed">
        The thinking takes 30 seconds. The formatting used to take 30 minutes. ShhhType turns your voice into ready-to-publish LinkedIn posts, DMs, and connection notes — in one spoken command.
      </p>
      <a
        href={SIGNUP_URL}
        className="inline-flex items-center gap-2 sm:gap-3 bg-gradient-to-r from-rose-500 to-orange-500 text-white pl-6 sm:pl-8 pr-5 sm:pr-6 py-3.5 sm:py-4 rounded-full text-base sm:text-lg hover:shadow-lg hover:shadow-rose-500/30 transition-all duration-300 font-montserrat font-semibold group/btn min-h-[44px]"
      >
        Start Free Trial
        <div className="bg-white/20 rounded-full p-1.5 group-hover/btn:bg-white/30 transition-colors">
          <ArrowRight className="w-4 h-4 group-hover/btn:translate-x-0.5 transition-transform" />
        </div>
      </a>
      <p className="text-sm text-gray-400 font-montserrat font-medium mt-6">macOS &amp; Windows</p>
    </div>
  );
}
