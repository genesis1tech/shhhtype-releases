import { NextRequest, NextResponse } from "next/server";

export function middleware(request: NextRequest) {
  const hostname = request.headers.get("host") || "";

  if (hostname.startsWith("beta.")) {
    return NextResponse.rewrite(new URL("/beta", request.url));
  }

  if (hostname.startsWith("waiting.")) {
    return NextResponse.rewrite(new URL("/waiting", request.url));
  }

  return NextResponse.next();
}

export const config = {
  matcher: "/",
};
