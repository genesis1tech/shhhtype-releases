export function WhoItsFor() {
  return (
    <div className="py-16 sm:py-24 overflow-hidden">
      <h2 className="text-3xl sm:text-4xl md:text-5xl text-center mb-4 tracking-tight text-gray-900 font-montserrat font-semibold">
        Built for people who know what to say
      </h2>
      <p className="text-base sm:text-lg text-gray-500 font-montserrat font-medium text-center mb-10 sm:mb-16 max-w-2xl mx-auto">
        The bottleneck isn&apos;t ideas — it&apos;s the gap between thinking and publishing. ShhhType collapses that gap.
      </p>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 sm:gap-8">
        {/* Tier 1: Creator-Founders */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col min-w-0 overflow-hidden">
          <div className="h-28 sm:h-36 w-full rounded-2xl overflow-hidden mb-5 sm:mb-8 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <img src="https://images.unsplash.com/photo-1478737270239-2f02b77fc618?w=800&h=600&fit=crop&auto=format&q=80" alt="Content creator with microphone" className="w-full h-full object-cover" loading="lazy" />
          </div>
          <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">
            LinkedIn Creators
          </h3>
          <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">
            You have great ideas but stare at a blank box for 20 minutes. Speak for 30 seconds, say <span className="text-green-600 font-mono font-bold">/linkedin</span>, and get a formatted post with hooks, bold text, and hashtags.
          </p>
          <div className="mt-auto pt-4 border-t border-gray-100">
            <p className="text-xs text-gray-400 font-montserrat font-medium italic">
              &ldquo;I went from posting 2x a week to posting daily. Not because I got more disciplined — because the friction disappeared.&rdquo;
            </p>
          </div>
        </div>

        {/* Tier 2: Sales Professionals */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col min-w-0 overflow-hidden">
          <div className="h-28 sm:h-36 w-full rounded-2xl overflow-hidden mb-5 sm:mb-8 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <img src="https://images.unsplash.com/photo-1552581234-26160f608093?w=800&h=600&fit=crop&auto=format&q=80" alt="Sales professional collaborating" className="w-full h-full object-cover" loading="lazy" />
          </div>
          <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">
            B2B Sales Pros
          </h3>
          <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">
            You send 30 DMs a day and every one needs to feel personal. Speak the context, say <span className="text-green-600 font-mono font-bold">/dm</span>, and get a personalized message under 300 characters. Every time.
          </p>
          <div className="mt-auto pt-4 border-t border-gray-100">
            <p className="text-xs text-gray-400 font-montserrat font-medium italic">
              &ldquo;30 DMs in 25 minutes instead of 2.5 hours. Same personalization. 6x faster.&rdquo;
            </p>
          </div>
        </div>

        {/* Tier 3: Consultants */}
        <div className="bg-white rounded-2xl sm:rounded-[2rem] p-5 sm:p-8 border border-gray-100 shadow-sm hover:shadow-xl hover:shadow-gray-200/40 transition-all duration-300 group flex flex-col min-w-0 overflow-hidden">
          <div className="h-28 sm:h-36 w-full rounded-2xl overflow-hidden mb-5 sm:mb-8 transition-transform group-hover:scale-[1.02] duration-500 flex-shrink-0">
            <img src="https://images.unsplash.com/photo-1498050108023-c5249f4df085?w=800&h=600&fit=crop&auto=format&q=80" alt="Consultant working at laptop" className="w-full h-full object-cover" loading="lazy" />
          </div>
          <h3 className="text-xl sm:text-2xl font-montserrat font-semibold text-gray-900 mb-3 sm:mb-4 tracking-tight">
            Content Consultants
          </h3>
          <p className="text-sm sm:text-base text-gray-500 font-montserrat font-medium leading-relaxed mb-4 sm:mb-6">
            You write 20+ posts a week across multiple clients. Create a custom skill per client — their voice, their hooks, their tone. Switch with a voice command.
          </p>
          <div className="mt-auto pt-4 border-t border-gray-100">
            <p className="text-xs text-gray-400 font-montserrat font-medium italic">
              &ldquo;Skills are just .md files. Drop /client-a or /client-b to switch voice instantly.&rdquo;
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
