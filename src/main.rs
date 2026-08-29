//########################################################################
//######################----eigene FEHLERenums-----#######################
//########################################################################
// #[derive(Debug, PartialEq)] ist nötig um das enum in tests mit assertions vergleichen können
#[derive(Debug, PartialEq)]
pub enum EingabeFehler {
    Leer,
    KeineGueltigeZahl,
    NegativerWertNichtErlaubt,
}

fn wandle_eingabe(eingabe: &str) -> Result<u8, EingabeFehler> {
    //todo!()

    // whitespaces und steuerzeichen und zeilenumbruch entfernen
    let getrimmt = eingabe.trim();

    // prüfen obs leer ist
    if getrimmt.is_empty() {
        return Err(EingabeFehler::Leer);
    }

    // prüfen ob zahl negativ
    // wird hier an dieser stelle direkt über das vorzeichen ermittelt
    // weil u8 später erst garnicht geparst wird wenn negativ und somit könnte der
    // spezifische error auf negativ auch nicht mehr geworfen werden
    // andere möglichkeit wäre
    //          -> erst in typ wandeln der negativ sein kann
    //          -> dann prüfen ob kleiner 0 und fehler werfen
    //          -> danach erst zu u8 wandeln
    if getrimmt.starts_with('-') {
        return Err(EingabeFehler::NegativerWertNichtErlaubt);
    }

    // in u8 wandeln
    let zahl = match getrimmt.parse::<u8>() {
        Ok(z) => {
            if z == 0 {
                return Err(EingabeFehler::KeineGueltigeZahl);
            } else {
                z
            }
        }
        Err(_) => return Err(EingabeFehler::KeineGueltigeZahl),
    };

    Ok(zahl)
}

//########################################################################
//###################-----HAUPTfunktion-----##############################
//########################################################################
fn main() {
    println!("Hello, world!");
    println!("hier wird mir RUST die nächsten korrekten lottozahlen zufällig generieren");
    println!("💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵");
    println!("💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰");
}

//#########################################################################
//#######################-----TDDbereich-----##############################
//#########################################################################
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eingabe_string_erfolgreich_in_zahl_wandeln() {
        assert_eq!(wandle_eingabe("23"), Ok(23));
        assert_eq!(wandle_eingabe("    42  "), Ok(42));
        assert_eq!(wandle_eingabe("    55\n"), Ok(55));
    }

    #[test]
    fn test_fehler_bei_leerer_eingabe() {
        assert_eq!(wandle_eingabe(""), Err(EingabeFehler::Leer));
        assert_eq!(wandle_eingabe("   "), Err(EingabeFehler::Leer));
        assert_eq!(wandle_eingabe("   \n"), Err(EingabeFehler::Leer));
    }

    #[test]
    fn test_fehler_wenn_keine_zahl() {
        assert_eq!(wandle_eingabe("xyz"), Err(EingabeFehler::KeineGueltigeZahl));
    }

    #[test]
    fn test_fehler_wenn_keine_volle_zahl_oder_0() {
        assert_eq!(
            wandle_eingabe("42.23"),
            Err(EingabeFehler::KeineGueltigeZahl)
        );
        assert_eq!(wandle_eingabe("0"), Err(EingabeFehler::KeineGueltigeZahl));
    }

    #[test]
    fn test_fehler_wenn_eingabe_negativ() {
        assert_eq!(
            wandle_eingabe("-55"),
            Err(EingabeFehler::NegativerWertNichtErlaubt)
        )
    }
}
