#!/bin/bash

OUTPUT_FILE="fisherman_context.txt"

# Initialisation/vidage du fichier de sortie
> "$OUTPUT_FILE"

echo "Génération du contexte IA dans $OUTPUT_FILE..."

# Utilisation de -prune pour ignorer complètement les répertoires inutiles ou volumineux
find . \
  \( \
    -path "./.git" -o \
    -path "./target" -o \
    -path "./target_micka" -o \
    -path "./target_user" -o \
    -path "./data" -o \
    -path "./data_old" -o \
    -path "./.antigravitycli" \
  \) -prune \
  -o -type f \
  -not -name "*.db" \
  -not -name "*.db-shm" \
  -not -name "*.db-wal" \
  -not -name "*tokens.json" \
  -not -name ".env" \
  -not -name "Cargo.lock" \
  -not -name "$OUTPUT_FILE" \
  -print | sort | while read -r file; do

    # Détermination de la syntaxe de coloration pour le bloc de code Markdown
    ext="${file##*.}"
    if [ "$ext" = "rs" ]; then
        lang="rust"
    elif [ "$ext" = "py" ]; then
        lang="python"
    elif [ "$ext" = "toml" ]; then
        lang="toml"
    elif [ "$ext" = "sh" ]; then
        lang="bash"
    elif [ "$ext" = "yml" ] || [ "$ext" = "yaml" ]; then
        lang="yaml"
    elif [ "$ext" = "md" ]; then
        lang="markdown"
    else
        lang=""
    fi

    # Structure de séparation lisible par les LLM
    echo "---" >> "$OUTPUT_FILE"
    echo "FILE: $file" >> "$OUTPUT_FILE"
    echo "---" >> "$OUTPUT_FILE"
    echo "\`\`\`$lang" >> "$OUTPUT_FILE"

    # Lecture, anonymisation à la volée et injection
    sed -E \
      -e 's/(\?|&)token=[a-zA-Z0-9]+/\1token=***REDACTED_TOKEN***/g' \
      -e 's/(CLIENT_ID\s*=\s*")[a-zA-Z0-9]+(")/\1***REDACTED_CLIENT_ID***\2/g' \
      -e 's/(TWITCH_CLIENT_ID|TWITCH_CLIENT_SECRET|ADMIN_TOKEN|TWITCH_OAUTH_TOKEN)=.*/\1=***REDACTED***/g' \
      "$file" >> "$OUTPUT_FILE"

    echo "" >> "$OUTPUT_FILE"
    echo "\`\`\`" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
done

echo "Extraction terminée avec succès."