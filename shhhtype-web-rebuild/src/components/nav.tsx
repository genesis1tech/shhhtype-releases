"use client";

import { useState, useCallback } from "react";
import { ArrowRight, Menu, X } from "lucide-react";
import Image from "next/image";

const CHECKOUT_URL =
  "https://shhhtype.lemonsqueezy.com/checkout/buy/c74816d2-4704-4248-abcb-fe565d518935";

export function Nav() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  const openMenu = useCallback(() => {
    setMobileMenuOpen(true);
    document.body.style.overflow = "hidden";
  }, []);

  const closeMenu = useCallback(() => {
    setMobileMenuOpen(false);
    document.body.style.overflow = "";
  }, []);

  return (
    <>
      {/* Navbar */}
      <div className="fixed top-4 sm:top-6 left-0 right-0 z-50 flex justify-center px-4 animate-fade-up">
        <nav className="glass-nav border border-gray-200 rounded-full pl-3 pr-2 py-2 flex items-center gap-3 sm:gap-8 shadow-sm hover:shadow-lg hover:shadow-rose-500/5 transition-all duration-300">
          <a
            href="#"
            className="group flex items-center gap-2 sm:gap-2.5 text-sm text-gray-900 hover:text-rose-600 transition-colors"
          >
            <Image
              src="/images/shhh_logo_thick.png"
              alt="ShhhType"
              width={36}
              height={36}
              className="w-8 h-8 sm:w-9 sm:h-9"
            />
            <span className="font-logo-mono text-lg sm:text-xl tracking-tight text-gray-900">
              ShhhType
            </span>
          </a>
          <div className="hidden md:flex items-center gap-6 text-sm font-montserrat font-medium text-gray-500">
            <a
              href="#features"
              className="hover:text-rose-600 transition-colors"
            >
              Features
            </a>
            <a
              href="#how-it-works"
              className="hover:text-rose-600 transition-colors"
            >
              How It Works
            </a>
            <a
              href="#pricing"
              className="hover:text-rose-600 transition-colors"
            >
              Pricing
            </a>
          </div>
          <div className="h-4 w-px bg-gray-200 hidden md:block"></div>
          <a
            href={CHECKOUT_URL}
            className="hidden sm:flex group bg-gray-900 text-white text-sm px-5 py-2.5 rounded-full hover:bg-rose-600 hover:shadow-lg hover:shadow-rose-600/30 transition-all duration-300 items-center gap-2 font-montserrat font-medium"
          >
            Try Free
            <ArrowRight className="w-4 h-4 transition-transform group-hover:translate-x-1" strokeWidth={1.5} />
          </a>
          {/* Mobile hamburger button */}
          <button
            onClick={openMenu}
            className="md:hidden flex items-center justify-center w-10 h-10 rounded-full hover:bg-gray-100 transition-colors"
            aria-label="Open menu"
          >
            <Menu className="w-5 h-5" />
          </button>
        </nav>
      </div>

      {/* Mobile menu overlay */}
      <div
        className={`mobile-menu-overlay fixed inset-0 z-[60] md:hidden${mobileMenuOpen ? " active" : ""}`}
      >
        <div
          className="absolute inset-0 bg-black/40 backdrop-blur-sm"
          onClick={closeMenu}
        ></div>
        <div className="mobile-menu-panel absolute top-0 left-0 right-0 bg-white rounded-b-3xl shadow-2xl p-6 pt-8">
          <div className="flex items-center justify-between mb-8">
            <a
              href="#"
              className="flex items-center gap-2"
              onClick={closeMenu}
            >
              <Image
                src="/images/shhh_logo_thick.png"
                alt="ShhhType"
                width={32}
                height={32}
                className="w-8 h-8"
              />
              <span className="font-logo-mono text-lg tracking-tight text-gray-900">
                ShhhType
              </span>
            </a>
            <button
              onClick={closeMenu}
              className="w-10 h-10 rounded-full bg-gray-100 flex items-center justify-center"
              aria-label="Close menu"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
          <div className="flex flex-col gap-2">
            <a
              href="#features"
              onClick={closeMenu}
              className="text-lg font-montserrat font-medium text-gray-700 hover:text-rose-600 px-4 py-3 rounded-xl hover:bg-rose-50 transition-all"
            >
              Features
            </a>
            <a
              href="#how-it-works"
              onClick={closeMenu}
              className="text-lg font-montserrat font-medium text-gray-700 hover:text-rose-600 px-4 py-3 rounded-xl hover:bg-rose-50 transition-all"
            >
              How It Works
            </a>
            <a
              href="#pricing"
              onClick={closeMenu}
              className="text-lg font-montserrat font-medium text-gray-700 hover:text-rose-600 px-4 py-3 rounded-xl hover:bg-rose-50 transition-all"
            >
              Pricing
            </a>
          </div>
          <div className="mt-6 pt-6 border-t border-gray-100">
            <a
              href={CHECKOUT_URL}
              className="w-full bg-gray-900 text-white py-3.5 rounded-full text-base font-montserrat font-medium flex items-center justify-center gap-2 hover:bg-rose-600 transition-colors"
            >
              Start Free Trial
              <ArrowRight className="w-4 h-4" strokeWidth={1.5} />
            </a>
          </div>
        </div>
      </div>
    </>
  );
}
