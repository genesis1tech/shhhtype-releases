import { NextRequest, NextResponse } from "next/server";

async function refreshZohoToken(
  service: string,
  refreshToken: string
): Promise<{ ok: boolean; error?: string }> {
  const clientId = process.env.ZOHO_CLIENT_ID;
  const clientSecret = process.env.ZOHO_CLIENT_SECRET;

  if (!clientId || !clientSecret || !refreshToken) {
    return { ok: false, error: `Missing credentials for ${service}` };
  }

  const params = new URLSearchParams({
    grant_type: "refresh_token",
    client_id: clientId,
    client_secret: clientSecret,
    refresh_token: refreshToken,
  });

  const resp = await fetch(
    `https://accounts.zoho.com/oauth/v2/token?${params}`,
    { method: "POST" }
  );
  const data = await resp.json();

  if (data.access_token) {
    return { ok: true };
  }

  return { ok: false, error: data.error || "Unknown error" };
}

export async function GET(request: NextRequest) {
  // Verify the request is from Vercel Cron (not a random caller)
  const authHeader = request.headers.get("authorization");
  if (authHeader !== `Bearer ${process.env.CRON_SECRET}`) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }

  const results: Record<string, { ok: boolean; error?: string }> = {};

  const crmToken = process.env.ZOHO_REFRESH_TOKEN_CRM;
  if (crmToken) {
    results.crm = await refreshZohoToken("crm", crmToken);
  }

  const campaignsToken = process.env.ZOHO_REFRESH_TOKEN_CAMPAIGNS;
  if (campaignsToken) {
    results.campaigns = await refreshZohoToken("campaigns", campaignsToken);
  }

  const allOk = Object.values(results).every((r) => r.ok);

  if (!allOk) {
    // Send alert email if any token refresh failed
    const resendKey = process.env.RESEND_API_KEY;
    if (resendKey) {
      await fetch("https://api.resend.com/emails", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${resendKey}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          from: "ShhhType Alerts <onboarding@resend.dev>",
          to: "mwade@genesis1.tech",
          subject: "ShhhType: Zoho token refresh failed",
          html: `<h2>Zoho Token Keepalive Failed</h2><pre>${JSON.stringify(results, null, 2)}</pre><p>Check Zoho API Console and regenerate refresh tokens if needed.</p>`,
        }),
      });
    }

    console.error("[zoho-keepalive] Token refresh failed:", results);
  } else {
    console.log("[zoho-keepalive] All tokens refreshed successfully");
  }

  return NextResponse.json({
    status: allOk ? "ok" : "error",
    results,
    timestamp: new Date().toISOString(),
  });
}
