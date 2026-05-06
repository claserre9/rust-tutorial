#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub ip: String,
    pub utilisateur: Option<String>,
    pub statut: u16,
    pub taille: u64,
    pub methode: String,
    pub chemin: String,
}
