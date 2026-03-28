"use client";

import { useState } from "react";
import { Sparkles, ArrowRight, FlaskConical, Bug, MessageSquare, Zap } from "lucide-react";
import Image from "next/image";

const benefits = [
  {
    icon: Zap,
    title: "Early Access",
    description: "Be the first to try new features before they ship to everyone.",
    color: "bg-[#FFE4D6] text-orange-500",
  },
  {
    icon: Bug,
    title: "Bug Bounty",
    description: "Find bugs, report them, and help us build a rock-solid product.",
    color: "bg-[#E0E7FF] text-indigo-500",
  },
  {
    icon: MessageSquare,
    title: "Direct Feedback",
    description: "Your voice shapes the roadmap. Tell us what to build next.",
    color: "bg-[#FFE4D6] text-orange-500",
  },
  {
    icon: FlaskConical,
    title: "Experimental Features",
    description: "Test cutting-edge capabilities that push the limits of voice-to-text.",
    color: "bg-[#E0E7FF] text-indigo-500",
  },
];

export function BetaSignup() {
  const [email, setEmail] = useState("");
  const [submitted, setSubmitted] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (email) {
      setSubmitted(true);
    }
  };

  return (
    <div className="space-y-8 sm:space-y-12">
      {/* Hero Card */}
      <div className="bg-gray-900 rounded-2xl sm:rounded-[2rem] md:rounded-[3rem] p-6 sm:p-10 md:p-16 lg:p-20 relative overflow-hidden shadow-2xl">
        {/* Background Effects */}
        <div className="absolute top-0 right-0 w-[300px] sm:w-[500px] h-[300px] sm:h-[500px] bg-rose-500/10 rounded-full blur-[120px] pointer-events-none"></div>
        <div className="absolute bottom-0 left-0 w-[250px] sm:w-[400px] h-[250px] sm:h-[400px] bg-indigo-500/10 rounded-full blur-[100px] pointer-events-none"></div>

        <div className="relative z-10 max-w-3xl mx-auto text-center">
          <div className="inline-flex items-center gap-2 rounded-full border border-gray-700 bg-gray-800/50 px-4 py-1 text-xs font-montserrat font-semibold text-rose-400 mb-8">
            <span className="inline-flex h-1.5 w-1.5 rounded-full bg-rose-500 animate-pulse"></span>
            BETA PROGRAM
          </div>

          <Image
            src="/images/shhh_logo_thick.png"
            alt="ShhhType"
            width={64}
            height={64}
            className="w-14 sm:w-16 h-14 sm:h-16 mx-auto mb-6 sm:mb-8"
          />

          <h1 className="text-3xl sm:text-5xl md:text-6xl lg:text-7xl tracking-tight text-white font-medium mb-4 sm:mb-6">
            Shape the
            <span className="italic text-gray-400"> future</span>
            <br />
            of ShhhType.
          </h1>

          <p className="text-base sm:text-lg md:text-xl text-gray-400 font-montserrat font-medium max-w-2xl mx-auto mb-8 sm:mb-12 leading-relaxed">
            Join the beta program to get early access to new features, test experimental capabilities, and help us make ShhhType even better.
          </p>

          {/* Sign Up Form */}
          {!submitted ? (
            <form onSubmit={handleSubmit} className="flex flex-col sm:flex-row gap-3 max-w-lg mx-auto">
              <input
                type="email"
                required
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                placeholder="Enter your email"
                className="flex-1 px-6 py-3.5 rounded-full bg-gray-800 border border-gray-700 text-white placeholder-gray-500 text-sm font-montserrat focus:outline-none focus:border-rose-500 focus:ring-1 focus:ring-rose-500 transition-colors min-h-[44px]"
              />
              <button
                type="submit"
                className="bg-gradient-to-r from-rose-500 to-orange-500 text-white px-8 py-3.5 rounded-full text-sm font-montserrat font-semibold hover:shadow-lg hover:shadow-rose-500/30 transition-all flex items-center justify-center gap-2 group/btn min-h-[44px]"
              >
                Join Beta
                <ArrowRight className="w-4 h-4 group-hover/btn:translate-x-0.5 transition-transform" />
              </button>
            </form>
          ) : (
            <div className="animate-fade-up bg-gray-800/50 border border-gray-700 rounded-2xl p-6 max-w-lg mx-auto">
              <div className="w-12 h-12 rounded-full bg-green-500/20 flex items-center justify-center mx-auto mb-4">
                <Sparkles className="w-6 h-6 text-green-400" />
              </div>
              <h3 className="text-white text-lg font-montserrat font-semibold mb-2">You&apos;re on the list!</h3>
              <p className="text-gray-400 text-sm font-montserrat">
                We&apos;ll send you an invite when the next beta build is ready. Keep an eye on your inbox.
              </p>
            </div>
          )}

          <p className="text-xs text-gray-500 font-montserrat mt-6">
            Available on macOS &amp; Windows. No spam, ever.
          </p>
        </div>
      </div>

      {/* Benefits Grid */}
      <div>
        <h2 className="text-3xl sm:text-4xl md:text-5xl text-center mb-10 sm:mb-16 tracking-tight text-gray-900 font-montserrat font-semibold">
          Why join the beta?
        </h2>
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-8">
          {benefits.map((benefit) => (
            <div
              key={benefit.title}
              className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col"
            >
              <div className={`h-16 sm:h-20 w-full ${benefit.color.split(" ")[0]} rounded-xl sm:rounded-2xl flex items-center justify-center mb-4 sm:mb-6 transition-transform group-hover:scale-[1.02] duration-500`}>
                <benefit.icon className={`w-8 h-8 sm:w-10 sm:h-10 ${benefit.color.split(" ")[1]}`} />
              </div>
              <h3 className="text-base sm:text-xl font-montserrat font-semibold text-gray-900 mb-2 sm:mb-3 tracking-tight">
                {benefit.title}
              </h3>
              <p className="text-xs sm:text-sm text-gray-500 font-montserrat font-medium leading-relaxed">
                {benefit.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
