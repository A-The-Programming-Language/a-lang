#!/bin/bash
# Remove comentários desnecessários mantendo docstrings importantes

find src -name "*.rs" -type f | while read file; do
    # Criar backup
    cp "$file" "$file.bak"
    
    # Remover comentários óbvios mantendo docstrings (//!)
    sed -i '/^[[:space:]]*\/\/ TODO:/d' "$file"
    sed -i '/^[[:space:]]*\/\/ FIXME:/d' "$file"
    sed -i '/^[[:space:]]*\/\/ NOTE:/d' "$file"
    sed -i '/^[[:space:]]*\/\/ Helper/d' "$file"
    sed -i '/^[[:space:]]*\/\/ Create/d' "$file"
    sed -i '/^[[:space:]]*\/\/ Set/d' "$file"
    sed -i '/^[[:space:]]*\/\/ Get/d' "$file"
    sed -i '/^[[:space:]]*\/\/ Update/d' "$file"
    sed -i '/^[[:space:]]*\/\/ Check/d' "$file"
done

echo "✓ Comentários limpos"
