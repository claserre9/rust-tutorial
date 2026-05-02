use std::collections::HashMap;
use crate::models::LogEntry;

#[derive(Debug)]
pub struct Statistiques {
    pub total_requetes: usize,
    pub total_octets: u64,
    pub codes_statut: HashMap<u16, usize>,
    pub ips_frequentes: Vec<(String, usize)>,
    pub chemins_populaires: Vec<(String, usize)>,
    pub taux_erreurs: f64,
}

pub fn analyser(entrées: &[LogEntry]) -> Statistiques {
    let total_requetes = entrées.len();
    let total_octets: u64 = entrées.iter().map(|e| e.taille).sum();

    let mut codes_statut: HashMap<u16, usize> = HashMap::new();
    let mut ip_counts: HashMap<String, usize> = HashMap::new();
    let mut chemin_counts: HashMap<String, usize> = HashMap::new();

    for e in entrées {
        *codes_statut.entry(e.statut).or_insert(0) += 1;
        *ip_counts.entry(e.ip.clone()).or_insert(0) += 1;
        *chemin_counts.entry(e.chemin.clone()).or_insert(0) += 1;
    }

    let mut ips_frequentes: Vec<(String, usize)> = ip_counts.into_iter().collect();
    ips_frequentes.sort_by(|a, b| b.1.cmp(&a.1));
    ips_frequentes.truncate(10);

    let mut chemins_populaires: Vec<(String, usize)> = chemin_counts.into_iter().collect();
    chemins_populaires.sort_by(|a, b| b.1.cmp(&a.1));
    chemins_populaires.truncate(10);

    let erreurs = entrées.iter().filter(|e| e.statut >= 400).count();
    let taux_erreurs = if total_requetes > 0 {
        erreurs as f64 / total_requetes as f64 * 100.0
    } else {
        0.0
    };

    Statistiques { total_requetes, total_octets, codes_statut, ips_frequentes, chemins_populaires, taux_erreurs }
}
