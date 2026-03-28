import type { Metadata } from "next";
import { WaitingList } from "@/components/waiting-list";

export const metadata: Metadata = {
  title: "ShhhType — Join the Waiting List",
  description:
    "Join the ShhhType waiting list. Be first to know when voice-to-text for macOS and Windows is available.",
};

export default function WaitingPage() {
  return (
    <main className="min-h-screen bg-[#EBEBEB]">
      <WaitingList />
    </main>
  );
}
