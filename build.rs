use std::path::Path;

fn main() {
    // Indiquer à cargo de recompiler si le fichier de données change
    println!("cargo:rerun-if-changed=data/game_data.json");

    let json_path = Path::new("data/game_data.json");
    if json_path.exists() {
        let content = std::fs::read_to_string(json_path)
            .expect("Impossible de lire le fichier data/game_data.json");

        // Tenter de parser le fichier JSON pour s'assurer qu'il est 100% valide syntaxiquement
        let _val: serde_json::Value = serde_json::from_str(&content)
            .expect("Le fichier data/game_data.json contient des erreurs de syntaxe JSON");
        
        println!("cargo:warning=✅ Validation de game_data.json réussie au build.");
    } else {
        println!("cargo:warning=⚠️ Fichier data/game_data.json manquant lors de la compilation. Le bot utilisera le fallback embarqué.");
    }
}
