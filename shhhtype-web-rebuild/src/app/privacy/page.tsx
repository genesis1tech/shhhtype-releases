import type { Metadata } from "next";
import { Nav } from "@/components/nav";
import { Footer } from "@/components/footer";
import { PrivacyContent } from "@/components/legal/privacy-content";

export const metadata: Metadata = {
  title: "ShhhType — Privacy Policy",
  description: "Privacy Policy for ShhhType voice-to-text application.",
};

export default function PrivacyPage() {
  return (
    <>
      <Nav />
      <main className="max-w-4xl mx-auto pt-24 sm:pt-32 px-4 sm:px-6 pb-12 sm:pb-20 flex-grow">
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-6 sm:p-10 md:p-14 border border-gray-100 shadow-sm">
          <PrivacyContent />
        </div>
      </main>
      <Footer />
    </>
  );
}
