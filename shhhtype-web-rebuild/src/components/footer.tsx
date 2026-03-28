import Image from "next/image";

export function Footer() {
  return (
    <footer className="bg-gray-100 mt-auto">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 py-6 sm:py-8 flex flex-col md:flex-row items-center justify-between gap-4">
        <div className="flex items-center gap-2">
          <Image
            src="/images/shhh_logo_thick.png"
            alt="ShhhType"
            width={20}
            height={20}
            className="w-5 h-5"
          />
          <p className="text-sm text-gray-500 font-montserrat font-medium">&copy; 2026 ShhhType. All rights reserved.</p>
        </div>
        <div className="flex items-center gap-6 text-sm text-gray-500 font-montserrat font-medium">
          <a href="/terms" className="hover:text-rose-600 transition-colors">Terms</a>
          <a href="/privacy" className="hover:text-rose-600 transition-colors">Privacy</a>
          <a href="mailto:support@shhhtype.com" className="hover:text-rose-600 transition-colors">support@shhhtype.com</a>
        </div>
      </div>
    </footer>
  );
}
