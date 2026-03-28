import type { Metadata } from "next";
import { BetaGated } from "@/components/beta-gated";

export const metadata: Metadata = {
  title: "ShhhType Beta — Download for macOS & Windows",
  description:
    "Download the ShhhType beta. Voice-to-text for macOS and Windows.",
};

export default function BetaPage() {
  return (
    <main className="min-h-screen bg-[#EBEBEB]">
      <BetaGated />
    </main>
  );
}
