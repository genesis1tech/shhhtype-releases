"use client";

import { useState } from "react";
import { Apple, Monitor, ArrowRight, Keyboard, Lock, Zap, Sparkles, Globe, Clock } from "lucide-react";
import Image from "next/image";

const features = [
  {
    icon: Sparkles,
    title: "AI Rewrite",
    description: "Polish transcriptions with 4 styles: Professional, Casual, Concise, Friendly. One hotkey, instant results.",
    color: "bg-[#E0E7FF] text-indigo-500",
    tag: "Llama 3.3 70B \u00B7 Smart",
  },
  {
    icon: Keyboard,
    title: "Global Hotkey",
    description: "Push-to-talk or toggle mode. Works from any app — documents, emails, chat, code editors, browsers.",
    color: "bg-[#FFE4D6] text-orange-500",
    tag: "Configurable \u00B7 Universal",
  },
  {
    icon: Lock,
    title: "Privacy First",
    description: "Local mode keeps everything on-device. Cloud mode sends only audio to Groq — nothing else leaves your machine.",
    color: "bg-[#E0E7FF] text-indigo-500",
    tag: "On-device \u00B7 Secure",
  },
  {
    icon: Zap,
    title: "Blazing Fast",
    description: "Groq cloud transcription returns results in under a second. GPU acceleration powers local mode on both macOS and Windows.",
    color: "bg-[#FFE4D6] text-orange-500",
    tag: "Sub-second \u00B7 Optimized",
  },
  {
    icon: Globe,
    title: "9 Languages + Auto-detect",
    description: "English, Spanish, French, German, Italian, Portuguese, Japanese, Korean, Chinese — plus automatic language detection.",
    color: "bg-[#FFE4D6] text-orange-500",
    tag: "Multilingual \u00B7 Auto-detect",
  },
  {
    icon: Clock,
    title: "Smart History",
    description: "Searchable transcription history with JSON export. Custom dictionary corrects terms automatically.",
    color: "bg-[#E0E7FF] text-indigo-500",
    tag: "Searchable \u00B7 Exportable",
  },
];

export function BetaGated() {
  const [gateOpen, setGateOpen] = useState(false);
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setSubmitting(true);

    try {
      const res = await fetch("/api/beta-signup", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ firstName, lastName, email }),
      });

      if (res.ok) {
        setGateOpen(true);
      } else {
        setError("Something went wrong. Please try again.");
      }
    } catch {
      setError("Something went wrong. Please try again.");
    } finally {
      setSubmitting(false);
    }
  };

  if (!gateOpen) {
    return (
      <div className="flex items-center justify-center min-h-screen px-4 sm:px-6">
        <div className="text-center max-w-md mx-auto w-full">
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
            BETA
          </div>

          <h1 className="text-3xl sm:text-4xl md:text-5xl tracking-tight text-gray-900 font-medium mb-3 sm:mb-4">
            Join the
            <span className="italic text-gray-400"> beta</span>
          </h1>

          <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium mb-8 sm:mb-10">
            Enter your details to access the beta downloads.
          </p>

          <form onSubmit={handleSubmit} className="space-y-3 text-left">
            <div className="grid grid-cols-2 gap-3">
              <input
                type="text"
                required
                value={firstName}
                onChange={(e) => setFirstName(e.target.value)}
                placeholder="First name"
                className="px-4 py-3 rounded-xl bg-white border border-gray-200 text-gray-900 placeholder-gray-400 text-sm font-montserrat focus:outline-none focus:border-rose-500 focus:ring-1 focus:ring-rose-500 transition-colors min-h-[44px]"
              />
              <input
                type="text"
                required
                value={lastName}
                onChange={(e) => setLastName(e.target.value)}
                placeholder="Last name"
                className="px-4 py-3 rounded-xl bg-white border border-gray-200 text-gray-900 placeholder-gray-400 text-sm font-montserrat focus:outline-none focus:border-rose-500 focus:ring-1 focus:ring-rose-500 transition-colors min-h-[44px]"
              />
            </div>
            <input
              type="email"
              required
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="Email address"
              className="w-full px-4 py-3 rounded-xl bg-white border border-gray-200 text-gray-900 placeholder-gray-400 text-sm font-montserrat focus:outline-none focus:border-rose-500 focus:ring-1 focus:ring-rose-500 transition-colors min-h-[44px]"
            />

            {error && (
              <p className="text-xs text-red-500 font-montserrat">{error}</p>
            )}

            <button
              type="submit"
              disabled={submitting || !firstName.trim() || !lastName.trim() || !email.trim()}
              className="w-full bg-gray-900 text-white py-3.5 rounded-full text-sm font-montserrat font-medium hover:bg-rose-600 hover:shadow-lg hover:shadow-rose-600/20 transition-all duration-300 flex items-center justify-center gap-2 group/btn min-h-[44px] disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {submitting ? "Submitting..." : "Continue to Downloads"}
              {!submitting && (
                <ArrowRight className="w-4 h-4 group-hover/btn:translate-x-0.5 transition-transform" />
              )}
            </button>
          </form>

          <p className="text-sm text-gray-500 font-montserrat mt-8 leading-relaxed max-w-sm mx-auto">
            By joining the beta, you agree to share feedback, insights, and suggestions that help us improve ShhhType. Your input directly shapes the product.
          </p>
          <p className="text-xs text-gray-400 font-montserrat mt-4">
            We&apos;ll only use your email for beta updates. No spam.
          </p>
        </div>
      </div>
    );
  }

  // Download page (after gate)
  return (
    <div className="max-w-7xl mx-auto pt-16 sm:pt-24 px-4 sm:px-6 pb-12 sm:pb-20">
      {/* Download section */}
      <div className="text-center max-w-2xl mx-auto mb-16 sm:mb-24">
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
          BETA
        </div>

        <h1 className="text-3xl sm:text-5xl md:text-6xl tracking-tight text-gray-900 font-medium mb-3 sm:mb-4">
          Welcome,
          <span className="italic text-gray-400"> {firstName}.</span>
        </h1>

        <p className="text-base sm:text-lg text-gray-500 font-montserrat font-medium max-w-lg mx-auto mb-10 sm:mb-14 leading-relaxed">
          Download the latest beta build. Voice to text, instantly.
        </p>

        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          <a
            href="https://github.com/genesis1tech/shhhtype-releases/releases/tag/latest-mac"
            target="_blank"
            rel="noopener"
            className="bg-gray-900 text-white pl-6 sm:pl-8 pr-5 sm:pr-6 py-4 sm:py-5 rounded-full text-base sm:text-lg hover:bg-rose-600 hover:shadow-lg hover:shadow-rose-600/20 transition-all duration-300 flex items-center justify-center gap-3 font-montserrat font-medium group/btn min-h-[52px]"
          >
            <Apple className="w-5 h-5 sm:w-6 sm:h-6" />
            Download for Mac
            <div className="bg-white/20 rounded-full p-1 group-hover/btn:bg-white/30 transition-colors">
              <ArrowRight className="w-3.5 h-3.5 group-hover/btn:translate-x-0.5 transition-transform" />
            </div>
          </a>

          <a
            href="https://github.com/genesis1tech/shhhtype-releases/releases/tag/latest-win"
            target="_blank"
            rel="noopener"
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

      {/* Features */}
      <div>
        <h2 className="text-3xl sm:text-4xl md:text-5xl text-center mb-10 sm:mb-16 tracking-tight text-gray-900 font-montserrat font-semibold">Features</h2>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-8">
          {features.map((feature) => (
            <div
              key={feature.title}
              className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col h-full"
            >
              <div className={`h-28 sm:h-44 w-full ${feature.color.split(" ")[0]} rounded-2xl flex items-center justify-center mb-5 sm:mb-8 transition-transform group-hover:scale-[1.02] duration-500`}>
                <feature.icon className={`w-10 sm:w-12 h-10 sm:h-12 ${feature.color.split(" ")[1]}`} />
              </div>
              <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">{feature.title}</h3>
              <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">{feature.description}</p>
              <div className="mt-auto pt-2">
                <p className="text-xs font-semibold text-gray-400 uppercase tracking-wider">{feature.tag}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
