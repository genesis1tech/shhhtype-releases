import { Nav } from "@/components/nav";
import { Hero } from "@/components/hero";
import { HowItWorks } from "@/components/how-it-works";
import { WhoItsFor } from "@/components/who-its-for";
import { Features } from "@/components/features";
import { LocalVsCloud } from "@/components/local-vs-cloud";
import { AiRewrite } from "@/components/ai-rewrite";
import { ProVersion } from "@/components/pro-version";
import { Marquee } from "@/components/marquee";
import { Pricing } from "@/components/pricing";
import { FinalCta } from "@/components/final-cta";
import { Footer } from "@/components/footer";

export default function Home() {
  return (
    <>
      <Nav />
      <main className="max-w-7xl mx-auto pt-24 sm:pt-32 px-4 sm:px-6 pb-12 sm:pb-20 flex-grow overflow-x-hidden w-full box-border">
        <Hero />
        <HowItWorks />
        <WhoItsFor />
        <AiRewrite />
        <Features />
        <LocalVsCloud />
        <ProVersion />
        <Marquee />
        <Pricing />
        <FinalCta />
      </main>
      <Footer />
    </>
  );
}
