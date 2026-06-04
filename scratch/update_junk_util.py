# -*- coding: utf-8 -*-
import json

JUNK_FR_PATH = "data/junk_data.json"
JUNK_EN_PATH = "data/junk_data_en.json"

def load_db(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)

def save_db(path, data):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=4, ensure_ascii=False)

def update_db_file(path, updates):
    data = load_db(path)
    # create lookup map
    lookup = {}
    for r, items in data.items():
        for item in items:
            lookup[item["id"]] = item
            
    updated_count = 0
    for update in updates:
        idx = update["id"]
        if idx in lookup:
            item = lookup[idx]
            if "descriptions" in update:
                item["descriptions"] = update["descriptions"]
            if "fun_fact" in update:
                item["fun_fact"] = update["fun_fact"]
            updated_count += 1
        else:
            print(f"WARNING: ID {idx} not found in database {path}")
            
    save_db(path, data)
    print(f"Successfully updated {updated_count} items in {path}")

def apply_updates(updates_fr, updates_en):
    update_db_file(JUNK_FR_PATH, updates_fr)
    update_db_file(JUNK_EN_PATH, updates_en)
