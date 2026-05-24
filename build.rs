use std::path::Path;

fn main() {
    let files = vec![
        "data/fish_data.json",
        "data/junk_data.json",
        "data/fail_messages.json",
    ];

    for file in &files {
        // Indiquer à cargo de recompiler si l'un de ces fichiers change
        println!("cargo:rerun-if-changed={}", file);

        let json_path = Path::new(file);
        if json_path.exists() {
            let content = std::fs::read_to_string(json_path)
                .unwrap_or_else(|_| panic!("Impossible de lire le fichier {}", file));

            // Tenter de parser le fichier JSON pour s'assurer qu'il est 100% valide syntaxiquement
            let _val: serde_json::Value = serde_json::from_str(&content)
                .unwrap_or_else(|_| panic!("Le fichier {} contient des erreurs de syntaxe JSON", file));
            
            println!("cargo:warning=✅ Validation de {} réussie au build.", file);
        } else {
            println!("cargo:warning=⚠️ Fichier {} manquant lors de la compilation. Le bot utilisera le fallback embarqué.", file);
        }
    }
}
