import { Check } from "lucide-react";

const CHECKOUT_URL_MONTHLY =
  "https://shhhtype.lemonsqueezy.com/checkout/buy/c74816d2-4704-4248-abcb-fe565d518935";

const CHECKOUT_URL_YEARLY =
  "https://shhhtype.lemonsqueezy.com/checkout/buy/142cb9f5-10e6-4a7d-ab93-ec87f4ee98e6";

const features = [
  "Voice Skills — /linkedin, /dm, /connect, /hormozi",
  "AI Rewrite — 4 styles",
  "Build unlimited custom skills",
  "9 languages + auto-detect",
  "Cloud + local transcription",
  "Searchable history with export",
];

export function Pricing() {
  return (
    <div id="pricing" className="py-16 sm:py-24">
      <h2 className="text-3xl sm:text-4xl md:text-5xl text-center mb-4 tracking-tight text-gray-900 font-montserrat font-semibold">Less than a coffee a week</h2>
      <p className="text-base sm:text-lg text-gray-500 font-montserrat font-medium text-center mb-10 sm:mb-16 max-w-2xl mx-auto">Start with a 7-day free trial. No credit card required. Cancel anytime.</p>

      <div className="grid sm:grid-cols-2 gap-4 sm:gap-8 max-w-3xl mx-auto">
        {/* Monthly */}
        <div className="bg-white rounded-2xl sm:rounded-[2.5rem] border border-gray-200 shadow-sm p-6 sm:p-10 text-center">
          <p className="text-sm font-montserrat font-semibold text-gray-400 uppercase tracking-wider mb-4">Monthly</p>
          <div className="mb-2">
            <span className="text-4xl sm:text-5xl font-montserrat font-bold text-gray-900">$20</span>
            <span className="text-base text-gray-500 font-montserrat font-medium ml-1">/mo</span>
          </div>
          <p className="text-sm text-gray-500 font-montserrat font-medium mb-6 sm:mb-8">Billed monthly</p>

          <div className="text-left space-y-3 mb-8">
            {features.map((feature) => (
              <div key={feature} className="flex items-center gap-3">
                <div className="w-5 h-5 rounded-full bg-green-50 flex items-center justify-center flex-shrink-0">
                  <Check className="w-3 h-3 text-green-600" />
                </div>
                <span className="text-sm text-gray-700 font-montserrat font-medium">{feature}</span>
              </div>
            ))}
          </div>

          <a
            href={CHECKOUT_URL_MONTHLY}
            className="w-full bg-gray-900 text-white h-12 rounded-xl flex items-center justify-center font-montserrat font-bold text-sm hover:bg-rose-600 hover:shadow-lg hover:shadow-rose-600/20 transition-all active:scale-[0.98]"
          >
            Start Free Trial
          </a>
        </div>

        {/* Yearly */}
        <div className="bg-white rounded-2xl sm:rounded-[2.5rem] border-2 border-rose-500 shadow-xl shadow-rose-500/10 p-6 sm:p-10 text-center relative">
          <span className="absolute -top-3 left-1/2 -translate-x-1/2 inline-flex items-center gap-1.5 bg-gradient-to-r from-rose-500 to-orange-500 text-white px-4 py-1 rounded-full text-xs font-montserrat font-bold uppercase tracking-wider">BEST VALUE</span>

          <p className="text-sm font-montserrat font-semibold text-gray-400 uppercase tracking-wider mb-4 mt-2">Yearly</p>
          <div className="mb-2">
            <span className="text-4xl sm:text-5xl font-montserrat font-bold text-gray-900">$200</span>
            <span className="text-base text-gray-500 font-montserrat font-medium ml-1">/yr</span>
          </div>
          <p className="text-sm text-gray-500 font-montserrat font-medium mb-1">Billed annually</p>
          <p className="text-xs text-rose-600 font-montserrat font-semibold mb-6 sm:mb-8">Save $40/year vs monthly</p>

          <div className="text-left space-y-3 mb-8">
            {features.map((feature) => (
              <div key={feature} className="flex items-center gap-3">
                <div className="w-5 h-5 rounded-full bg-green-50 flex items-center justify-center flex-shrink-0">
                  <Check className="w-3 h-3 text-green-600" />
                </div>
                <span className="text-sm text-gray-700 font-montserrat font-medium">{feature}</span>
              </div>
            ))}
          </div>

          <a
            href={CHECKOUT_URL_YEARLY}
            className="w-full bg-gradient-to-r from-rose-500 to-orange-500 text-white h-12 rounded-xl flex items-center justify-center font-montserrat font-bold text-sm hover:shadow-lg hover:shadow-rose-500/30 transition-all active:scale-[0.98]"
          >
            Start Free Trial
          </a>
        </div>
      </div>

      <p className="text-center text-sm text-gray-400 font-montserrat font-medium mt-8">7-day free trial on all plans. No credit card required to start.</p>
    </div>
  );
}
