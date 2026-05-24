#!/bin/bash

OUTPUT_FILE="fisherman_context.txt"

> "$OUTPUT_FILE"

find . -type f \
  -not -path "./.git/*" \
  -not -path "./target/*" \
  -not -name "*.db" \
  -not -name "*.db-shm" \
  -not -name "*.db-wal" \
  -not -name "*tokens.json" \
  -not -name ".env" \
  -not -name "Cargo.lock" \
  -not -name "$OUTPUT_FILE" \
  | sort | while read -r file; do
    echo "========================================" >> "$OUTPUT_FILE"
    echo "Fichier : $file" >> "$OUTPUT_FILE"
    echo "========================================" >> "$OUTPUT_FILE"
    
    # Lecture du fichier et anonymisation à la volée via sed
    cat "$file" | sed -E \
      -e 's/(\?|&)token=[a-zA-Z0-9]+/\1token=***REDACTED_TOKEN***/g' \
      -e 's/(CLIENT_ID\s*=\s*")[a-zA-Z0-9]+(")/\1***REDACTED_CLIENT_ID***\2/g' \
      -e 's/(TWITCH_CLIENT_ID|TWITCH_CLIENT_SECRET|ADMIN_TOKEN|TWITCH_OAUTH_TOKEN)=.*/\1=***REDACTED***/g' \
      >> "$OUTPUT_FILE"
      
    echo -e "\n" >> "$OUTPUT_FILE"
done