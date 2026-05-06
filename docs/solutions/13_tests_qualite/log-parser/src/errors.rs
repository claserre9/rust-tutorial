use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("format invalide : '{0}'")]
    FormatInvalide(String),

    #[error("code statut invalide : '{0}'")]
    StatutInvalide(String),

    #[error("taille invalide : '{0}'")]
    TailleInvalide(String),
}
