"use client";

import { useState } from "react";
import { useSearchParams } from "next/navigation";
import { ArrowRight, Loader2 } from "lucide-react";
import Image from "next/image";
import { Suspense } from "react";

const CHECKOUT_URL_MONTHLY =
  "https://shhhtype.lemonsqueezy.com/checkout/buy/c74816d2-4704-4248-abcb-fe565d518935";

const CHECKOUT_URL_YEARLY =
  "https://shhhtype.lemonsqueezy.com/checkout/buy/142cb9f5-10e6-4a7d-ab93-ec87f4ee98e6";

function SignupFormInner() {
  const searchParams = useSearchParams();
  const plan = searchParams.get("plan") || "monthly";

  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState("");

  const checkoutUrl =
    plan === "yearly" ? CHECKOUT_URL_YEARLY : CHECKOUT_URL_MONTHLY;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setSubmitting(true);

    try {
      const res = await fetch("/api/signup", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ firstName, lastName, email, plan }),
      });

      if (!res.ok) {
        setError("Something went wrong. Please try again.");
        setSubmitting(false);
        return;
      }

      // Redirect to LemonSqueezy checkout with email prefilled
      const checkoutWithEmail = `${checkoutUrl}?checkout[email]=${encodeURIComponent(email)}&checkout[name]=${encodeURIComponent(`${firstName} ${lastName}`)}`;
      window.location.href = checkoutWithEmail;
    } catch {
      setError("Something went wrong. Please try again.");
      setSubmitting(false);
    }
  };

  return (
    <div className="flex items-center justify-center min-h-screen px-4 sm:px-6">
      <div className="text-center max-w-md mx-auto w-full">
        <a href="/" className="inline-flex items-center justify-center gap-3 mb-6 sm:mb-8">
          <Image
            src="/images/shhh_logo_thick.png"
            alt="ShhhType"
            width={80}
            height={80}
            className="w-16 sm:w-20 h-16 sm:h-20"
          />
          <span className="font-logo-mono text-3xl sm:text-4xl tracking-tight text-gray-900">
            ShhhType
          </span>
        </a>

        <h1 className="text-3xl sm:text-4xl md:text-5xl tracking-tight text-gray-900 font-medium mb-3 sm:mb-4">
          Start your
          <span className="italic text-gray-400"> free trial</span>
        </h1>

        <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium mb-8 sm:mb-10">
          Create your account to begin your 7-day free trial.
          {plan === "yearly" ? " Yearly plan — $200/yr." : " Monthly plan — $20/mo."}
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
            disabled={
              submitting ||
              !firstName.trim() ||
              !lastName.trim() ||
              !email.trim()
            }
            className="w-full bg-gray-900 text-white py-3.5 rounded-full text-sm font-montserrat font-medium hover:bg-rose-600 hover:shadow-lg hover:shadow-rose-600/20 transition-all duration-300 flex items-center justify-center gap-2 group/btn min-h-[44px] disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {submitting ? (
              <>
                <Loader2 className="w-4 h-4 animate-spin" />
                Creating account...
              </>
            ) : (
              <>
                Continue to Checkout
                <ArrowRight className="w-4 h-4 group-hover/btn:translate-x-0.5 transition-transform" />
              </>
            )}
          </button>
        </form>

        <p className="text-xs text-gray-400 font-montserrat mt-6">
          7-day free trial. No charge until the trial ends. Cancel anytime.
        </p>
      </div>
    </div>
  );
}

export function SignupForm() {
  return (
    <Suspense
      fallback={
        <div className="flex items-center justify-center min-h-screen">
          <Loader2 className="w-8 h-8 animate-spin text-gray-400" />
        </div>
      }
    >
      <SignupFormInner />
    </Suspense>
  );
}
