use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", about = "Gestionnaire de tâches CLI", version)]
pub struct Cli {
    #[command(subcommand)]
    pub commande: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Ajouter une nouvelle tâche
    Ajouter {
        /// Description de la tâche
        description: String,
    },
    /// Lister toutes les tâches
    Lister,
    /// Marquer une tâche comme faite
    Terminer {
        /// ID de la tâche
        id: u32,
    },
    /// Supprimer une tâche
    Supprimer {
        /// ID de la tâche
        id: u32,
    },
    /// Supprimer toutes les tâches terminées
    Vider,
}
