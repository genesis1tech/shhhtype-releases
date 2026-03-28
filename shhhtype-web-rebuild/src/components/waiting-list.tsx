import Image from "next/image";

export function WaitingList() {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen px-4 sm:px-6 py-16">
      <div className="text-center max-w-2xl mx-auto w-full">
        <div className="flex items-center justify-center gap-3 mb-6 sm:mb-8">
          <Image
            src="/images/shhh_logo_thick.png"
            alt="ShhhType"
            width={80}
            height={80}
            className="w-16 sm:w-20 h-16 sm:h-20"
          />
          <span className="font-logo-mono text-3xl sm:text-4xl tracking-tight text-gray-900">ShhhType</span>
        </div>

        <div className="inline-flex items-center gap-2 rounded-full border border-gray-300 bg-white/80 backdrop-blur px-4 py-1.5 text-xs font-montserrat font-semibold text-rose-500 mb-6 sm:mb-8 shadow-sm">
          <span className="inline-flex h-1.5 w-1.5 rounded-full bg-rose-500 animate-pulse"></span>
          COMING SOON
        </div>

        <h1 className="text-3xl sm:text-5xl md:text-6xl tracking-tight text-gray-900 font-medium mb-4 sm:mb-6">
          Join the
          <span className="italic text-gray-400"> waiting list</span>
        </h1>

        <p className="text-base sm:text-lg text-gray-500 font-montserrat font-medium max-w-lg mx-auto mb-10 sm:mb-14 leading-relaxed">
          Be the first to know when ShhhType is available. Voice to text for macOS &amp; Windows.
        </p>

        {/* Zoho Form Embed */}
        <div className="w-full max-w-xl mx-auto bg-white rounded-2xl sm:rounded-[2rem] border border-gray-100 shadow-sm overflow-hidden">
          <iframe
            aria-label="Waiting List"
            frameBorder="0"
            style={{ height: "500px", width: "100%", border: "none" }}
            src="https://forms.zohopublic.com/genesis1technologiesllc1/form/WaitingList/formperma/WDd5CCw3vTI7kGr7z03BhWhMuykimbuYgmKrrwfVqdQ"
          />
        </div>

        <p className="text-xs text-gray-400 font-montserrat font-medium mt-8">
          macOS &amp; Windows &middot; No spam, ever.
        </p>
      </div>
    </div>
  );
}
