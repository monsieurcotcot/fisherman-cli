# -*- coding: utf-8 -*-
import json
import re

def validate_db(path, lang):
    with open(path, "r", encoding="utf-8") as f:
        data = json.load(f)
    
    items = []
    for r, items_list in data.items():
        for item in items_list:
            items.append(item)
            
    items = sorted(items, key=lambda x: x["id"])
    
    print(f"\n--- Validating {lang} ({path}) ---")
    print(f"Total items: {len(items)}")
    
    # Check gaps
    ids = [x["id"] for x in items]
    gaps = [i for i in range(1, max(ids)+1) if i not in ids]
    if gaps:
        print(f"ERROR: Gaps found: {gaps}")
    else:
        print("OK: No ID gaps.")
        
    # Check items 143-242
    errors = 0
    warnings = 0
    all_descs = []
    
    template_phrases = [
        "complètement déchiqueté",
        "ne est plus qu'un débris",
        "débris informe, à moitié",
        "presque entier, bien que",
        "a perdu de sa superbe",
        "usagé qui dérive au gré",
        "completely shredded",
        "is nothing but",
        "shreds of this",
        "cracked and deformed",
        "shows clear signs of",
        "almost intact, although",
        "lost its lustre",
        "worn out, gently drift"
    ]
    
    for item in items:
        idx = item["id"]
        name = item["name"]
        
        # Check fun fact length
        ff = item.get("fun_fact", "")
        if idx >= 143:
            if len(ff) < 500:
                print(f"ERROR [ID {idx} - {name}]: Fun fact too short ({len(ff)} chars). Min 500.")
                errors += 1
                
        # Check descriptions
        descs = item.get("descriptions", {})
        for state in ["badly damaged", "damaged", "worn"]:
            state_list = descs.get(state, [])
            if len(state_list) != 3:
                print(f"ERROR [ID {idx} - {name}]: State '{state}' has {len(state_list)} descriptions, expected 3.")
                errors += 1
            for desc in state_list:
                all_descs.append((idx, name, state, desc))
                # Check for template match
                if idx >= 143:
                    for phrase in template_phrases:
                        if phrase in desc:
                            print(f"WARNING [ID {idx} - {name}]: Template phrase '{phrase}' found in description: '{desc}'")
                            warnings += 1
                            break
                            
    # Check duplicate descriptions across the 143-242 items
    from collections import Counter
    desc_texts = [d[3] for d in all_descs if d[0] >= 143]
    counts = Counter(desc_texts)
    duplicates = {d: c for d, c in counts.items() if c > 1}
    if duplicates:
        print(f"ERROR: Found {len(duplicates)} duplicate descriptions:")
        errors += len(duplicates)
        for d, c in list(duplicates.items())[:5]:
            print(f"  - '{d}' occurs {c} times.")
    else:
        print("OK: No duplicate descriptions in items 143-242.")
        
    print(f"Validation finished: {errors} Errors, {warnings} Warnings.")
    return errors, warnings

if __name__ == "__main__":
    err_fr, warn_fr = validate_db("data/junk_data.json", "FR")
    err_en, warn_en = validate_db("data/junk_data_en.json", "EN")
    total_errors = err_fr + err_en
    print(f"\nTOTAL: {total_errors} errors, {warn_fr + warn_en} warnings.")
