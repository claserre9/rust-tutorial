use log_parser::{parser_fichier, analytics::analyser};

const LOG_EXEMPLE: &str = r#"
127.0.0.1 - alice [01/Jan/2024:00:00:01 +0000] "GET / HTTP/1.1" 200 1024
10.0.0.1 - - [01/Jan/2024:00:00:02 +0000] "POST /api HTTP/1.1" 201 256
192.168.1.1 - bob [01/Jan/2024:00:00:03 +0000] "GET /img.png HTTP/1.1" 404 0
127.0.0.1 - - [01/Jan/2024:00:00:04 +0000] "GET /style.css HTTP/1.1" 200 512
LIGNE_INVALIDE
"#;

#[test]
fn test_parser_plusieurs_lignes() {
    let (entrées, erreurs) = parser_fichier(LOG_EXEMPLE);
    assert_eq!(entrées.len(), 4, "4 lignes valides attendues");
    assert_eq!(erreurs.len(), 1, "1 ligne invalide attendue");
}

#[test]
fn test_premiere_entree() {
    let (entrées, _) = parser_fichier(LOG_EXEMPLE);
    let e = &entrées[0];
    assert_eq!(e.ip, "127.0.0.1");
    assert_eq!(e.utilisateur, Some("alice".to_string()));
    assert_eq!(e.statut, 200);
    assert_eq!(e.taille, 1024);
    assert_eq!(e.methode, "GET");
    assert_eq!(e.chemin, "/");
}

#[test]
fn test_statistiques_basiques() {
    let (entrées, _) = parser_fichier(LOG_EXEMPLE);
    let stats = analyser(&entrées);

    assert_eq!(stats.total_requetes, 4);
    assert_eq!(stats.total_octets, 1024 + 256 + 0 + 512);
    assert_eq!(stats.codes_statut[&200], 2);
    assert_eq!(stats.codes_statut[&201], 1);
    assert_eq!(stats.codes_statut[&404], 1);
}

#[test]
fn test_taux_erreurs() {
    let (entrées, _) = parser_fichier(LOG_EXEMPLE);
    let stats = analyser(&entrées);
    // 1 erreur (404) sur 4 = 25%
    assert!((stats.taux_erreurs - 25.0).abs() < 0.01);
}

#[test]
fn test_ip_frequente() {
    let (entrées, _) = parser_fichier(LOG_EXEMPLE);
    let stats = analyser(&entrées);
    // 127.0.0.1 apparaît 2 fois
    assert_eq!(stats.ips_frequentes[0].0, "127.0.0.1");
    assert_eq!(stats.ips_frequentes[0].1, 2);
}

#[test]
fn test_log_vide() {
    let (entrées, erreurs) = parser_fichier("");
    assert!(entrées.is_empty());
    assert!(erreurs.is_empty());
}

#[test]
fn test_ligne_invalide_seule() {
    let (entrées, erreurs) = parser_fichier("pas un log");
    assert!(entrées.is_empty());
    assert_eq!(erreurs.len(), 1);
}
