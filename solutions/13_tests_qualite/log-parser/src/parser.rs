use crate::errors::ParseError;
use crate::models::LogEntry;

// Format Combined Log: IP - USER [DATE] "METHOD PATH PROTO" STATUS SIZE
pub fn parser_ligne(ligne: &str) -> Result<LogEntry, ParseError> {
    // Découper sur les espaces en gérant les tokens entre guillemets/crochets
    // On utilise une approche manuelle pour robustesse
    let invalide = || ParseError::FormatInvalide(ligne.to_string());

    // ip
    let mut iter = ligne.splitn(2, ' ');
    let ip = iter.next().ok_or_else(invalide)?.to_string();
    let reste = iter.next().ok_or_else(invalide)?;

    // - (identd ignoré)
    let mut iter = reste.splitn(2, ' ');
    iter.next().ok_or_else(invalide)?;
    let reste = iter.next().ok_or_else(invalide)?;

    // user
    let mut iter = reste.splitn(2, ' ');
    let user_token = iter.next().ok_or_else(invalide)?;
    let utilisateur = if user_token == "-" { None } else { Some(user_token.to_string()) };
    let reste = iter.next().ok_or_else(invalide)?;

    // [date] — sauter jusqu'au ']'
    let end_bracket = reste.find(']').ok_or_else(invalide)?;
    let reste = reste[end_bracket + 1..].trim_start();

    // "METHOD PATH PROTO"
    if !reste.starts_with('"') { return Err(invalide()); }
    let end_quote = reste[1..].find('"').ok_or_else(invalide)? + 1;
    let requete = &reste[1..end_quote];
    let reste = reste[end_quote + 1..].trim_start();

    let mut req_iter = requete.splitn(3, ' ');
    let methode = req_iter.next().ok_or_else(invalide)?.to_string();
    let chemin = req_iter.next().ok_or_else(invalide)?.to_string();

    // STATUS SIZE
    let mut parties = reste.split_whitespace();
    let statut_str = parties.next().ok_or_else(invalide)?;
    let statut = statut_str.parse::<u16>()
        .map_err(|_| ParseError::StatutInvalide(statut_str.to_string()))?;

    let taille_str = parties.next().unwrap_or("0");
    let taille = if taille_str == "-" {
        0
    } else {
        taille_str.trim_end_matches('"')
            .parse::<u64>()
            .map_err(|_| ParseError::TailleInvalide(taille_str.to_string()))?
    };

    Ok(LogEntry { ip, utilisateur, statut, taille, methode, chemin })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ligne_valide() {
        let ligne = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /index.html HTTP/1.0" 200 2326"#;
        let e = parser_ligne(ligne).unwrap();
        assert_eq!(e.ip, "127.0.0.1");
        assert_eq!(e.utilisateur, Some("frank".to_string()));
        assert_eq!(e.statut, 200);
        assert_eq!(e.taille, 2326);
        assert_eq!(e.methode, "GET");
        assert_eq!(e.chemin, "/index.html");
    }

    #[test]
    fn test_utilisateur_tiret() {
        let ligne = r#"10.0.0.1 - - [01/Jan/2024:00:00:01 +0000] "POST /api HTTP/1.1" 201 256"#;
        let e = parser_ligne(ligne).unwrap();
        assert_eq!(e.utilisateur, None);
    }

    #[test]
    fn test_ligne_invalide() {
        assert!(parser_ligne("pas un log").is_err());
    }
}
