use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("tâche #{0} introuvable")]
    NonTrouvee(u32),

    #[error("erreur I/O : {0}")]
    Io(#[from] std::io::Error),

    #[error("erreur JSON : {0}")]
    Json(#[from] serde_json::Error),
}
