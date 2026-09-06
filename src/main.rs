use rand::RngExt;
use rand::distr::{Distribution, Uniform};
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

//########################################################################
//###################-----eigeneFUNKTIONEN-----###########################
//########################################################################
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

fn pruefe_grenzen_min_max_vertauscht(min: u8, max: u8) {
    if min > max {
        panic!("Achtung -> Parameter MIN ist größer als MAX!");
    }
}

fn zufallszahl0(min: u8, max: u8) -> u8 {
    pruefe_grenzen_min_max_vertauscht(min, max);

    // rand::random()
    // -> generiert eine zufallszahl aus dem kompletten wertebereich des typs
    // bool -> [true, false]
    // f64  -> [0.0,  1.0]
    // u8   -> [0,    255]
    // let foo = loop ... break gibt wert an foo heraus
    loop {
        let zuff: u8 = rand::random();
        if zuff >= min && zuff <= max {
            break zuff;
        }
    }
}

fn zufallszahl(min: u8, max: u8) -> u8 {
    pruefe_grenzen_min_max_vertauscht(min, max);

    rand::rng().random_range(min..max + 1)
}

// nutzt eine variante wobei im gewahlten bereich sicher gleiche chancen bestehen
// ausserdem schnell
fn zufallszahl_1(min: u8, max: u8) -> u8 {
    pruefe_grenzen_min_max_vertauscht(min, max);
    // rng holt den zufallsgenerator für den aktuellen thread
    // mut ist nötig, weil der generator seinen zustand beim würfeln ändert
    let mut rng = rand::rng();

    // uniform::new baut die mathematische verteilung auf
    // unwrap entpackt den wert, da new ab v0.9 ein result zurückgibt
    let verteilung = Uniform::new(min, max + 1).unwrap();

    // sample zieht die zufallszahl mithilfe des generators
    // &mut übergibt rng als veränderbare referenz für den nächsten zustand.
    verteilung.sample(&mut rng)
}

//########################################################################
//###################-----HAUPTfunktion-----##############################
//########################################################################
fn main() {
    println!("Hello, world!");
    println!("hier wird mir RUST die nächsten korrekten lottozahlen zufällig generieren");
    println!("💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵");
    println!("💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰");

    let zz = zufallszahl_1(10, 20);
    println!("zufallszahl----->{}", zz)
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
    // erstellen eines makros da verschiedene funktionen immer das selbe verhalten abbilden sollen
    //      - bekommen immer gleiche Parameter
    //      - liefern immer gleichen rückgabetyp
    // makro verhindert da sreduntante aufrufen der tests
    macro_rules! generiere_tests_fuer_zufallszahlen_funktionen {
        ($mod_name:ident, $funktion:expr) => {
            mod $mod_name {

                use super::*;
                #[test]
                fn test_bereich_klein_von_1_bis_6() {
                    // da ja schlecht zufällig genau ein wert getestet werden kann ;)
                    // werden hundert zufälle generiert die alle in einem kleinen bereich liegen müssen
                    for _ in 0..100 {
                        let z = $funktion(1, 6);
                        //assert!(z >= 1 && z <= 6, "Wert {z} liegt außerhalb von 1..=6");
                        assert!((1..=6).contains(&z), "Wert {z} liegt außerhalb von 1..=6");
                    }
                }
                #[test]
                fn test_wenn_min_gleich_max_dann_wert_gleich_grenzen_5() {
                    assert_eq!($funktion(5, 5), 5, "Wert liegt außerhalb der grenzen 5");
                }
                #[test]
                #[should_panic(expected = "Achtung -> Parameter MIN ist größer als MAX!")]
                // der test wird bestanden wenn mit panic abgebrochen wird
                // WICHTIG -> expected sollte gesetzt werden, da sonst jeder panic den test bestehen lässt
                fn test_min_groesser_als_max_muss_panikken() {
                    // sollte abbrechen, da min grösser max als ungültig betrachtet
                    $funktion(4, 2);
                }

                #[test]
                fn test_untere_und_obere_grenze_ist_vorhanden_2_bis_8() {
                    let mut unterer_wert_z = u8::MAX;
                    let mut oberer_wert_z = u8::MIN;
                    for _ in 0..55 {
                        let z = $funktion(2, 8);
                        unterer_wert_z = std::cmp::min(unterer_wert_z, z);
                        oberer_wert_z = std::cmp::max(oberer_wert_z, z);

                        if unterer_wert_z == 2 && oberer_wert_z == 8 {
                            break;
                        }
                    }
                    assert_eq!(unterer_wert_z, 2, "Untere Grenze wurde nicht erreicht");
                    assert_eq!(oberer_wert_z, 8, "Obere Grenze wurde nicht erreicht");
                }
            }
        };
    }
    // hier werden tests vollautomatisch auf verschieden varianten angewendet
    generiere_tests_fuer_zufallszahlen_funktionen!(variante_standard, zufallszahl);
    generiere_tests_fuer_zufallszahlen_funktionen!(variante_null, zufallszahl0);
    generiere_tests_fuer_zufallszahlen_funktionen!(variante_eins, zufallszahl_1);
}
