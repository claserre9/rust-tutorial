mod cli;
mod errors;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Cmd};
use errors::TodoError;
use models::Todo;

fn main() {
    if let Err(e) = run() {
        eprintln!("Erreur : {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), TodoError> {
    let cli = Cli::parse();
    let mut todos = storage::charger()?;

    match cli.commande {
        Cmd::Ajouter { description } => {
            let id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            todos.push(Todo::nouveau(id, description));
            storage::sauvegarder(&todos)?;
            println!("✓ Tâche #{id} ajoutée.");
        }

        Cmd::Lister => {
            if todos.is_empty() {
                println!("Aucune tâche.");
            } else {
                println!("{} tâche(s) :", todos.len());
                for t in &todos {
                    let etat = if t.fait { "✓" } else { "○" };
                    println!("  [{etat}] #{}: {}", t.id, t.description);
                }
            }
        }

        Cmd::Terminer { id } => {
            let t = todos.iter_mut()
                .find(|t| t.id == id)
                .ok_or(TodoError::NonTrouvee(id))?;
            t.fait = true;
            storage::sauvegarder(&todos)?;
            println!("✓ Tâche #{id} marquée comme faite.");
        }

        Cmd::Supprimer { id } => {
            let avant = todos.len();
            todos.retain(|t| t.id != id);
            if todos.len() == avant {
                return Err(TodoError::NonTrouvee(id));
            }
            storage::sauvegarder(&todos)?;
            println!("✓ Tâche #{id} supprimée.");
        }

        Cmd::Vider => {
            let avant = todos.len();
            todos.retain(|t| !t.fait);
            let supprimées = avant - todos.len();
            storage::sauvegarder(&todos)?;
            println!("✓ {supprimées} tâche(s) terminée(s) supprimée(s).");
        }
    }

    Ok(())
}
