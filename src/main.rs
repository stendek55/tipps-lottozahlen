use rand::RngExt;
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

fn zufallszahl(min: u8, max: u8) -> u8 {
    if min > max {
        panic!("Achtung -> Parameter MIN ist größer als MAX!");
    }

    rand::rng().random_range(min..max + 1)
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

    //#########################################################################
    // tests für die zufallsfunktionen
    // verzicht auf tests, welche negative bereiche/grenzen/ergebnisse prüfen -> da nicht benötigt
    #[test]
    fn test_bereich_klein_von_1_bis_6() {
        // da ja schlecht zufällig genau ein wert getestet werden kann ;)
        // werden hundert zufälle generiert die alle in einem kleinen bereich liegen müssen
        for _ in 0..100 {
            let z = zufallszahl(1, 6);
            assert!(z >= 1 && z <= 6, "Wert {z} liegt außerhalb von 1..=6");
        }
    }
    #[test]
    fn test_wenn_min_gleich_max_dann_wert_gleich_grenzen_5() {
        assert_eq!(zufallszahl(5, 5), 5, "Wert liegt außerhalb der grenzen 5")
    }
    #[test]
    #[should_panic(expected = "Achtung -> Parameter MIN ist größer als MAX!")]
    // der test wird bestanden wenn mit panic abgebrochen wird
    // WICHTIG -> expected sollte gesetzt werden, da sonst jeder panic den test bestehen lässt
    fn test_min_groesser_als_max_muss_panikken() {
        // sollte abbrechen, da min grösser max als ungültig betrachtet
        zufallszahl(4, 2);
    }

    #[test]
    fn test_untere_und_obere_grenze_ist_vorhanden_2_bis_8() {
        let mut unterer_wert = 5;
        let mut oberer_wert = 5;
        for _ in 0..55 {
            let z = zufallszahl(2, 8);
            if z <= unterer_wert {
                unterer_wert = z;
            } else if z >= oberer_wert {
                oberer_wert = z;
            }
            if unterer_wert == 2 && oberer_wert == 8 {
                break;
            }
        }
        assert_eq!(unterer_wert, 2, "Untere Grenze wurde nicht erreicht");
        assert_eq!(oberer_wert, 8, "Obere Grenze wurde nicht erreicht");
    }
}
