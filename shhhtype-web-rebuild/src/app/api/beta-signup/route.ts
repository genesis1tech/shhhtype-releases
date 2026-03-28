import { NextRequest, NextResponse } from "next/server";

export async function POST(request: NextRequest) {
  try {
    const { firstName, lastName, email } = await request.json();

    if (!firstName || !lastName || !email) {
      return NextResponse.json(
        { error: "All fields are required" },
        { status: 400 }
      );
    }

    // Send notification email via Resend (free tier: 100 emails/day)
    // If RESEND_API_KEY is not set, fall back to logging
    const resendKey = process.env.RESEND_API_KEY;

    if (resendKey) {
      const res = await fetch("https://api.resend.com/emails", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${resendKey}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          from: "ShhhType Beta <onboarding@resend.dev>",
          to: "mwade@genesis1.tech",
          subject: `New Beta Tester: ${firstName} ${lastName}`,
          html: `
            <h2>New ShhhType Beta Signup</h2>
            <p><strong>Name:</strong> ${firstName} ${lastName}</p>
            <p><strong>Email:</strong> ${email}</p>
            <p><strong>Signed up:</strong> ${new Date().toISOString()}</p>
          `,
        }),
      });

      if (!res.ok) {
        console.error("Resend error:", await res.text());
      }
    } else {
      // Log to Vercel logs when no email service configured
      console.log("BETA SIGNUP:", { firstName, lastName, email, timestamp: new Date().toISOString() });
    }

    return NextResponse.json({ success: true });
  } catch {
    return NextResponse.json(
      { error: "Something went wrong" },
      { status: 500 }
    );
  }
}
