#!/bin/bash
# Setup a self-signed code signing certificate for vox2txt development.
# Run this once per machine. The certificate persists in your login keychain
# and ensures macOS TCC permissions (Input Monitoring, Accessibility) survive
# across rebuilds.

set -euo pipefail

CERT_NAME="Vox2txt Dev"

# Check if certificate already exists
if security find-identity -v -p codesigning 2>/dev/null | grep -q "$CERT_NAME"; then
    echo "Certificate '$CERT_NAME' already exists in keychain."
    security find-identity -v -p codesigning
    exit 0
fi

echo "Creating self-signed code signing certificate: $CERT_NAME"

TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# 1. Generate key + self-signed cert (valid 10 years)
openssl req -x509 -newkey rsa:2048 \
    -keyout "$TMP_DIR/dev.key" \
    -out "$TMP_DIR/dev.crt" \
    -days 3650 -nodes \
    -subj "/CN=$CERT_NAME" \
    -addext "keyUsage=digitalSignature" \
    -addext "extendedKeyUsage=codeSigning" \
    2>/dev/null

# 2. Package as p12 (legacy format for macOS compatibility)
openssl pkcs12 -export \
    -out "$TMP_DIR/dev.p12" \
    -inkey "$TMP_DIR/dev.key" \
    -in "$TMP_DIR/dev.crt" \
    -passout pass:vox2txt \
    -legacy 2>/dev/null

# 3. Import into login keychain
security import "$TMP_DIR/dev.p12" \
    -k ~/Library/Keychains/login.keychain-db \
    -T /usr/bin/codesign \
    -P "vox2txt"

# 4. Trust the certificate for code signing (user level, no sudo needed)
security add-trusted-cert -r trustRoot \
    -k ~/Library/Keychains/login.keychain-db \
    "$TMP_DIR/dev.crt"

echo ""
echo "Certificate '$CERT_NAME' created successfully!"
echo ""
security find-identity -v -p codesigning
echo ""
echo "Now run 'npm run tauri build' to create a signed app."
echo "macOS permissions will persist across rebuilds."
