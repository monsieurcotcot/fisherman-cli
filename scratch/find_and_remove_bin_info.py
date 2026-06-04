# -*- coding: utf-8 -*-
import json
import re
import subprocess

JUNK_FR_PATH = "data/junk_data.json"
JUNK_EN_PATH = "data/junk_data_en.json"

kws_fr = [
    'poubelle', 'poubelles', 'décharge', 'décharges', 'déchèterie', 
    'déchèteries', 'déchetterie', 'déchetteries', 'encombrants', 
    'benne', 'bennes', 'bac de tri', 'bacs de tri'
]
# drop weee to avoid removing definitions of WEEE
kws_en = ['bin', 'bins', 'landfill', 'landfills', 'scrap metal']

def restore_clean_baseline():
    print("Reverting any partial cleanup and restoring the clean baseline...")
    
    # Run batch updaters sequentially to restore the baseline for both FR and EN
    for i in range(1, 11):
        script_name = f"scratch/update_batch_{i}.py"
        print(f"Running {script_name}...")
        subprocess.run(["python3", script_name], check=True)
        
    # Run extension script
    print("Running scratch/extend_fun_facts.py...")
    subprocess.run(["python3", "scratch/extend_fun_facts.py"], check=True)
    print("Baseline successfully restored!")

def clean_database(path, lang, kws):
    with open(path, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    cleaned_count = 0
    total_items = 0
    
    for rarity, items in data.items():
        for item in items:
            total_items += 1
            ff = item["fun_fact"]
            # Split by sentence endings (.!? followed by space)
            sentences = re.split(r'(?<=[.!?])\s+', ff)
            
            cleaned_sentences = []
            removed_any = False
            
            for s in sentences:
                matched = False
                if any(re.search(r'\b' + kw + r'\b', s, re.IGNORECASE) for kw in kws):
                    matched = True
                    
                # Prevent false positive for ID 52: "décharges électriques"
                if matched and lang == "FR" and item["id"] == 52 and "électrique" in s:
                    matched = False
                    
                if matched:
                    removed_any = True
                else:
                    cleaned_sentences.append(s)
                    
            if removed_any:
                new_ff = " ".join(cleaned_sentences).strip()
                new_ff = re.sub(r'\s+', ' ', new_ff)
                item["fun_fact"] = new_ff
                cleaned_count += 1
                
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)
        
    print(f"{lang} Database: Cleaned {cleaned_count} / {total_items} items.")

if __name__ == "__main__":
    restore_clean_baseline()
    
    print("\n=== CLEANING FRENCH DATABASE ===")
    clean_database(JUNK_FR_PATH, "FR", kws_fr)
    
    print("\n=== CLEANING ENGLISH DATABASE ===")
    clean_database(JUNK_EN_PATH, "EN", kws_en)
