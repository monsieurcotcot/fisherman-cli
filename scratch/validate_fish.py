import json
import re
import sys

def validate_db(path, lang):
    print(f"\n--- Validating {lang} ({path}) ---")
    with open(path, 'r', encoding='utf-8') as f:
        data = json.load(f)

    errors = 0
    warnings = 0
    all_ids = []
    all_names = set()
    all_prices = set()
    all_descriptions = []

    expected_states = ["badly damaged", "damaged", "worn", "good", "pristine"]
    prefixes = {
        "badly damaged": "🔴",
        "damaged": "🟠",
        "worn": "🟡",
        "good": "🟢",
        "pristine": "✨"
    }

    total_items = 0
    for rarity, items in data.items():
        for item in items:
            total_items += 1
            idx = item.get("id")
            name = item.get("name")
            price = item.get("price")
            location = item.get("location", "")
            fun_fact = item.get("fun_fact", "")
            
            all_ids.append(idx)
            all_names.add(name)
            if price is not None:
                if price in all_prices:
                    # Allow duplicate prices only for existing/legendary items if they already did,
                    # but new items should have unique prices.
                    if idx >= 102:
                        print(f"ERROR [ID {idx} - {name}]: Price {price} is not unique.")
                        errors += 1
                all_prices.add(price)
            else:
                print(f"ERROR [ID {idx} - {name}]: Missing price.")
                errors += 1

            # Validate location/time/season
            if not location:
                print(f"ERROR [ID {idx} - {name}]: Missing location.")
                errors += 1
            if not item.get("preferred_time"):
                print(f"ERROR [ID {idx} - {name}]: Missing preferred_time.")
                errors += 1
            if not item.get("preferred_season"):
                print(f"ERROR [ID {idx} - {name}]: Missing preferred_season.")
                errors += 1
            if not item.get("months"):
                print(f"ERROR [ID {idx} - {name}]: Missing months.")
                errors += 1

            # Check fun fact
            if not fun_fact:
                print(f"ERROR [ID {idx} - {name}]: Missing fun_fact.")
                errors += 1
            else:
                # Check that fun fact has name, scientific name, preferred season, preferred time, etc.
                pass

            # Check descriptions
            descs = item.get("descriptions", {})
            for state in expected_states:
                lines = descs.get(state, [])
                # Require at least 4 lines for new items (ID >= 102)
                if idx >= 102:
                    if len(lines) < 4:
                        print(f"ERROR [ID {idx} - {name}]: State '{state}' has {len(lines)} descriptions, min 4 required.")
                        errors += 1
                
                prefix = prefixes[state]
                for line in lines:
                    if idx >= 102:
                        if not line.startswith(prefix):
                            print(f"ERROR [ID {idx} - {name}]: Description under '{state}' does not start with prefix '{prefix}': '{line}'")
                            errors += 1
                        all_descriptions.append((idx, name, state, line))

    print(f"Total items in {lang}: {total_items}")

    # Check ID continuity
    all_ids.sort()
    for i in range(len(all_ids) - 1):
        if all_ids[i+1] - all_ids[i] != 1:
            print(f"ERROR: Gap in IDs between {all_ids[i]} and {all_ids[i+1]}")
            errors += 1

    # Check duplicate descriptions in new items
    from collections import Counter
    desc_texts = [d[3] for d in all_descriptions]
    counts = Counter(desc_texts)
    duplicates = {d: c for d, c in counts.items() if c > 1}
    if duplicates:
        print("ERROR: Found duplicate descriptions in new items:")
        for text, count in duplicates.items():
            print(f"  - '{text}' occurs {count} times.")
            # find which items have it
            matching = [f"ID {d[0]} ({d[1]}) - {d[2]}" for d in all_descriptions if d[3] == text]
            print(f"    Occurrences: {', '.join(matching)}")
        errors += len(duplicates)

    print(f"Validation finished: {errors} Errors, {warnings} Warnings.")
    return errors, warnings

if __name__ == "__main__":
    err_fr, _ = validate_db("data/fish_data.json", "FR")
    err_en, _ = validate_db("data/fish_data_en.json", "EN")
    total = err_fr + err_en
    print(f"\nTOTAL FISH ERRORS: {total}")
    if total > 0:
        sys.exit(1)
    else:
        sys.exit(0)
