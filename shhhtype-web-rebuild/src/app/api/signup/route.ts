import { NextRequest, NextResponse } from "next/server";

interface TokenCache {
  accessToken: string;
  expiresAt: number;
}

const tokenCache: Record<string, TokenCache> = {};

async function getZohoToken(service: "crm" | "campaigns"): Promise<string> {
  const cached = tokenCache[service];
  if (cached && cached.expiresAt > Date.now()) {
    return cached.accessToken;
  }

  const clientId = process.env.ZOHO_CLIENT_ID;
  const clientSecret = process.env.ZOHO_CLIENT_SECRET;
  const refreshToken =
    service === "crm"
      ? process.env.ZOHO_REFRESH_TOKEN_CRM
      : process.env.ZOHO_REFRESH_TOKEN_CAMPAIGNS;

  if (!clientId || !clientSecret || !refreshToken) {
    throw new Error(`Missing Zoho ${service} credentials`);
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

  if (!data.access_token) {
    throw new Error(`Zoho token refresh failed for ${service}`);
  }

  tokenCache[service] = {
    accessToken: data.access_token,
    expiresAt: Date.now() + (data.expires_in - 60) * 1000,
  };

  return data.access_token;
}

async function upsertCrmContact(
  firstName: string,
  lastName: string,
  email: string,
  plan: string
) {
  const token = await getZohoToken("crm");

  const contact = {
    Email: email,
    First_Name: firstName,
    Last_Name: lastName,
    Lead_Source: "Website Signup",
    Description: `ShhhType — ${plan === "yearly" ? "Yearly" : "Monthly"} plan interest`,
  };

  // Search for existing contact by email
  const searchResp = await fetch(
    `https://www.zohoapis.com/crm/v2/Contacts/search?email=${encodeURIComponent(email)}`,
    { headers: { Authorization: `Zoho-oauthtoken ${token}` } }
  );

  if (searchResp.ok) {
    const searchData = await searchResp.json();
    if (searchData.data && searchData.data.length > 0) {
      // Update existing contact
      const contactId = searchData.data[0].id;
      await fetch(`https://www.zohoapis.com/crm/v2/Contacts/${contactId}`, {
        method: "PUT",
        headers: {
          Authorization: `Zoho-oauthtoken ${token}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ data: [contact] }),
      });
      return;
    }
  }

  // Create new contact
  await fetch("https://www.zohoapis.com/crm/v2/Contacts", {
    method: "POST",
    headers: {
      Authorization: `Zoho-oauthtoken ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ data: [contact] }),
  });
}

async function addToCampaignsList(
  email: string,
  firstName: string,
  listKey: string
) {
  const token = await getZohoToken("campaigns");

  const params = new URLSearchParams({
    resfmt: "JSON",
    listkey: listKey,
    contactinfo: JSON.stringify({
      "Contact Email": email,
      "First Name": firstName,
    }),
  });

  await fetch(
    `https://campaigns.zoho.com/api/v1.1/json/listsubscribe?${params}`,
    {
      method: "POST",
      headers: { Authorization: `Zoho-oauthtoken ${token}` },
    }
  );
}

export async function POST(request: NextRequest) {
  try {
    const { firstName, lastName, email, plan } = await request.json();

    if (!firstName || !lastName || !email) {
      return NextResponse.json(
        { error: "All fields are required" },
        { status: 400 }
      );
    }

    // Push to Zoho CRM
    try {
      await upsertCrmContact(firstName, lastName, email, plan || "monthly");
    } catch (err) {
      console.error("[signup] CRM error:", err);
    }

    // Push to Zoho Campaigns trial list
    const trialListKey = process.env.ZOHO_LIST_TRIAL;
    if (trialListKey) {
      try {
        await addToCampaignsList(email, firstName, trialListKey);
      } catch (err) {
        console.error("[signup] Campaigns error:", err);
      }
    }

    return NextResponse.json({ success: true });
  } catch {
    return NextResponse.json(
      { error: "Something went wrong" },
      { status: 500 }
    );
  }
}
