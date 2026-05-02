use std::path::Path;
use crate::errors::TodoError;
use crate::models::Todo;

const FICHIER: &str = "todos.json";

pub fn charger() -> Result<Vec<Todo>, TodoError> {
    charger_depuis(FICHIER)
}

pub fn sauvegarder(todos: &[Todo]) -> Result<(), TodoError> {
    sauvegarder_vers(FICHIER, todos)
}

pub fn charger_depuis(path: &str) -> Result<Vec<Todo>, TodoError> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }
    let contenu = std::fs::read_to_string(path)?;
    let todos = serde_json::from_str(&contenu)?;
    Ok(todos)
}

pub fn sauvegarder_vers(path: &str, todos: &[Todo]) -> Result<(), TodoError> {
    let json = serde_json::to_string_pretty(todos)?;
    std::fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Todo;

    #[test]
    fn test_sauvegarder_et_charger() {
        let path = std::env::temp_dir()
            .join("todo_test.json")
            .to_string_lossy()
            .to_string();

        let todos = vec![
            Todo::nouveau(1, String::from("Apprendre Rust")),
            Todo { id: 2, description: String::from("Lire le livre"), fait: true },
        ];

        sauvegarder_vers(&path, &todos).unwrap();
        let chargés = charger_depuis(&path).unwrap();

        assert_eq!(chargés.len(), 2);
        assert_eq!(chargés[0], todos[0]);
        assert_eq!(chargés[1], todos[1]);

        // Nettoyage
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_charger_fichier_absent() {
        let result = charger_depuis("/chemin/qui/nexiste/pas.json");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
