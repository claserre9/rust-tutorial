pub mod analytics;
pub mod models;
pub mod parser;
mod errors;

pub use errors::ParseError;
pub use models::LogEntry;

pub fn parser_fichier(contenu: &str) -> (Vec<LogEntry>, Vec<ParseError>) {
    let mut entrées = Vec::new();
    let mut erreurs = Vec::new();

    for ligne in contenu.lines() {
        let ligne = ligne.trim();
        if ligne.is_empty() { continue; }
        match parser::parser_ligne(ligne) {
            Ok(e) => entrées.push(e),
            Err(e) => erreurs.push(e),
        }
    }

    (entrées, erreurs)
}
