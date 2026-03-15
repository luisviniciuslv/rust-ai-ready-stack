use std::fs;
use std::path::{Path, PathBuf};

fn collect_rust_files(dir: &Path, acc: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("Não foi possível ler diretório {}: {}", dir.display(), e));

    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("Erro ao ler entrada de diretório: {}", e));
        let path = entry.path();

        if path.is_dir() {
            collect_rust_files(&path, acc);
            continue;
        }

        if path.extension().is_some_and(|ext| ext == "rs") {
            acc.push(path);
        }
    }
}

#[test]
fn domain_layer_must_not_depend_on_serialization_or_database_types() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let domain_dir = root.join("src").join("domain");

    let mut rust_files = Vec::new();
    collect_rust_files(&domain_dir, &mut rust_files);

    let forbidden_patterns = [
        "use serde",
        "serde::",
        "#[serde",
        "mongodb::",
        "bson::",
        "ObjectId",
        "Serialize",
        "Deserialize",
    ];

    let mut violations = Vec::new();

    for file in rust_files {
        let content = fs::read_to_string(&file)
            .unwrap_or_else(|e| panic!("Não foi possível ler arquivo {}: {}", file.display(), e));

        for pattern in forbidden_patterns {
            if content.contains(pattern) {
                violations.push(format!(
                    "{} contém padrão proibido: '{}'",
                    file.strip_prefix(root).unwrap_or(file.as_path()).display(),
                    pattern
                ));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Domínio acoplado à infraestrutura. Violações:\n{}",
        violations.join("\n")
    );
}
