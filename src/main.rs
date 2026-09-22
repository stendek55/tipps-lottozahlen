//! # Lotto-Simulator 6 aus 49
//! dieses modul stellt verschiedene algorithmen zur generierung von zufallszahlen bereit.
//!
//! es dient als plattform, um unterschiedliche herangehensweisen (ablehnung, standardbereich,
//! gleichverteilung) zu testen, zu vergleichen und per tdd abzusichern.
//! Es simuliert Lottotipps. Der Benutzer kann über die Konsole
//! eingeben, wie viele Ziehungen generiert werden sollen. Die Ziehungen werden
//! anschließend über verschiedene Zufallsalgorithmen berechnet.
use rand::RngExt;
use rand::SeedableRng;
use rand::distr::{Distribution, Uniform};
use rand::prelude::IndexedRandom;
use rand::rngs::StdRng;
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
/// Validiert und konvertiert eine Benutzereingabe (Text) in eine Ganzzahl vom Typ `u8`.
///
/// Die Funktion entfernt führende sowie nachfolgende Leer- und Steuerzeichen und prüft
/// die Eingabe auf spezifische Fehlerkriterien, bevor sie in eine Zahl umgewandelt wird.
///
/// # Parameter
///
/// * `eingabe` - Ein String-Slice (`&str`), der die rohe Eingabe des Benutzers enthält.
///
/// # Rückgabewert
///
/// * `Ok(u8)` - Die erfolgreich validierte und konvertierte Zahl (größer als 0).
/// * `Err(EingabeFehler)` - Ein spezifischer Fehler, falls die Eingabe ungültig ist:
///   * `EingabeFehler::Leer` - Wenn die Eingabe leer ist oder nur aus Leerzeichen bestand.
///   * `EingabeFehler::NegativerWertNichtErlaubt` - Wenn die Eingabe mit einem `-` beginnt.
///   * `EingabeFehler::KeineGueltigeZahl` - Wenn die Eingabe keine Zahl ist, den Wert `0` hat oder den Wertebereich von `u8` (255) überschreitet.
///
/// # Examples
///
/// ```
/// // Gültige Konvertierung
/// assert_eq!(wandle_eingabe(" 42 \n"), Ok(42));
///
/// // Fehlerfälle abfangen
/// assert!(wandle_eingabe("").is_err());
/// assert!(wandle_eingabe("-5").is_err());
/// assert!(wandle_eingabe("0").is_err());
/// assert!(wandle_eingabe("abc").is_err());
/// ```
fn wandle_eingabe(eingabe: &str) -> Result<u8, EingabeFehler> {
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
/// prüft, ob die untere grenze größer als die obere grenze ist.
///
/// # panics
///
/// die funktion bricht kontrolliert ab (panic), wenn `min` größer als `max` ist.
fn pruefe_grenzen_min_max_vertauscht(min: u8, max: u8) {
    if min > max {
        panic!("Achtung -> Parameter MIN ist größer als MAX!");
    }
}

/// generiert eine zufallszahl durch wiederholtes würfeln des gesamten u8-bereichs.
///
/// das ablehnungsverfahren (rejection sampling) wiederholt den vorgang so lange,
/// bis eine generierte zahl innerhalb der gewünschten grenzen liegt.
///
/// # beispiele
///
/// ```
/// let zahl = zufallszahl_durch_ablehnung(1, 6);
/// assert!((1..=6).contains(&zahl));
/// ```
///
/// # panics
///
/// bricht ab, wenn der parameter `min` größer als `max` ist.
fn zufallszahl_durch_ablehnung(min: u8, max: u8) -> u8 {
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

/// generiert eine zufallszahl über die standard-bereichsmethode von rand.
///
/// diese methode nutzt die standard-implementierung der rand-bibliothek,
/// um einen wert im übergebenen bereich zu erzeugen.
///
/// # beispiele
///
/// ```
/// let zahl = zufallszahl_durch_standardbereich(10, 20);
/// assert!((10..=20).contains(&zahl));
/// ```
///
/// # panics
///
/// bricht ab, wenn der parameter `min` größer als `max` ist.
fn zufallszahl_durch_standard(min: u8, max: u8) -> u8 {
    pruefe_grenzen_min_max_vertauscht(min, max);

    rand::rng().random_range(min..max + 1)
}

/// generiert eine zufallszahl über eine explizit erstellte gleichverteilung.
///
/// nutzt das `uniform`-konstrukt für gleichbleibende chancen aller werte
/// im gewählten bereich.
///
/// # beispiele
///
/// ```
/// let zahl = zufallszahl_durch_gleichverteilung(5, 5);
/// assert_eq!(zahl, 5);
/// ```
///
/// # panics
///
/// bricht ab, wenn der parameter `min` größer als `max` ist.
fn zufallszahl_durch_gleichverteilung(min: u8, max: u8) -> u8 {
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

/// generiert eine zufallszahl, indem zuerst ein array mit allen zahlen von `min`
/// bis inklusive `max` befüllt und daraus ein zufälliges element ausgewählt wird.
///
/// # beispiele
///
/// ```
/// let zahl = zufallszahl_durch_array_auswahl(1, 5);
/// assert!((1..=5).contains(&zahl));
/// ```
///
/// # panics
///
/// bricht ab, wenn der parameter `min` größer als `max` ist.
fn zufallszahl_durch_array_auswahl(min: u8, max: u8) -> u8 {
    pruefe_grenzen_min_max_vertauscht(min, max);

    // vektor wird mit dem bereich von min bis inklusive max befüllt
    let zahlen_liste: Vec<u8> = (min..=max).collect();

    // der zufallsgenerator wird geholt
    let mut rng = rand::rng();

    // .choose() wählt eine zufällige referenz aus dem slice aus
    // unwrap ist sicher, da die liste durch den vorherigen test niemals leer ist
    // das führende sternchen * kopiert den u8-wert aus der referenz heraus
    *zahlen_liste.choose(&mut rng).unwrap()
}

/// generiert eine kryptografisch hochsichere zufallszahl durch direkte abfrage
/// der systementropie des betriebssystems.
///
/// dieses verfahren holt echte entropie direkt aus dem betriebssystem-kernel
/// und initialisiert den krypto-generator jedes mal frisch.
///
/// # beispiele
///
/// ```
/// let geheimnis = zufallszahl_durch_systementropie(1, 100);
/// assert!((1..=100).contains(&geheimnis));
/// ```
///
/// # panics
///
/// bricht ab, wenn der parameter `min` größer als `max` ist.
fn zufallszahl_durch_systementropie(min: u8, max: u8) -> u8 {
    pruefe_grenzen_min_max_vertauscht(min, max);

    // erzeugt einen frischen krypto-generator, der direkt über die
    // ungepufferte system-entropie (sysrng) befüttert und gestartet wird
    let mut rng = StdRng::try_from_rng(&mut getrandom::SysRng).unwrap();

    let verteilung = Uniform::new(min, max + 1).unwrap();

    // sample zieht die zahl direkt aus der krypto-quelle.
    verteilung.sample(&mut rng)
}

/// Erzeugt eine verschachtelte Liste von Lottoschein-Ziehungen (6 aus 49).
///
/// Für jede Ziehung werden 6 eindeutige Zufallszahlen im Bereich von 1 bis 49 generiert.
/// Die Zahlen werden dabei dynamisch und zufällig über verschiedene mathematische
/// und systemnahe Zufallsalgorithmen ermittelt.
///
/// # Parameter
///
/// * `anzahl` - Die Anzahl der zu generierenden Tippscheine / Ziehungen (Spiele).
///
/// # Rückgabewert
///
/// Gibt einen Vektor von Vektoren (`Vec<Vec<u8>>`) zurück. Jede innere Liste repräsentiert
/// eine Ziehung und enthält genau 6 eindeutige, ungeordnete Gewinnzahlen.
///
/// # Algorithmus und Besonderheiten
///
/// Die Funktion nutzt intern ein Array aus fünf verschiedenen Zufallsfunktionen. Für jede
/// einzelne Gewinnzahl wird per Zufall entschieden, welcher Algorithmus (z. B. Ablehnung,
/// Systementropie, Gleichverteilung) die Zahl generiert. Duplikate innerhalb einer Ziehung
/// werden strikt gefiltert.
///
/// # Examples
///
/// ```
/// let anzahl_tipps = 3;
/// let lotto_tipps = erzeuge_zufallszahlen(anzahl_tipps);
///
/// assert_eq!(lotto_tipps.len(), 3);
/// assert_eq!(lotto_tipps[0].len(), 6);
/// ```
fn erzeuge_zufallszahlen(anzahl: u8) -> Vec<Vec<u8>> {
    //grenzen für zahlenbereich LOTTO 6 aus 49
    let start = 1_u8;
    let ende = 49_u8;
    let zahlen = 6;
    //liste für neue ziehung
    let mut gewinnzahlen = Vec::new();
    //liste eigener zufallsfunktionen
    let zufallsfunktionen = [
        zufallszahl_durch_ablehnung,
        zufallszahl_durch_standard,
        zufallszahl_durch_gleichverteilung,
        zufallszahl_durch_array_auswahl,
        zufallszahl_durch_systementropie,
    ];
    //zufallsgenerator starten
    let mut rng = rand::rng();

    //durcjgänge für geforderte anzahl an ziehungen
    for _ in 0..anzahl {
        let mut ziehung = Vec::new();

        //durchgänge für die sechs gewinnzahlen
        for _ in 0..zahlen {
            //eine zufallfunktion zufällig auswählen
            if let Some(gewinn_funktion) = zufallsfunktionen.choose(&mut rng) {
                //loop um dopplungen zu vermeiden
                loop {
                    let glueckszahl = gewinn_funktion(start, ende);
                    //prüfen das die zahl noch nicht in der liste ist
                    if !ziehung.contains(&glueckszahl) {
                        ziehung.push(glueckszahl);
                        break;
                    }
                }
            }
        }
        gewinnzahlen.push(ziehung);
    }

    gewinnzahlen
}

//########################################################################
//###################-----HAUPTfunktion-----##############################
//########################################################################
fn main() {
    println!("Hello, world!");
    println!("hier wird mir RUST die nächsten korrekten lottozahlen zufällig generieren");
    println!("💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵💵");
    println!("💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰💰");
    println!();
    println!("bitte gebe deine gewünschte anzahl an ziehungen an: ");

    let anzahl = loop {
        let mut eingabe = String::new();
        std::io::stdin().read_line(&mut eingabe).unwrap();

        match wandle_eingabe(&eingabe) {
            Ok(zahl) => {
                break zahl;
            }
            Err(EingabeFehler::Leer) => {
                println!("bitte gebe eine zahl ein!");
            }
            Err(EingabeFehler::NegativerWertNichtErlaubt) => {
                println!("bitte gebe eine positive zahl ein!");
            }
            Err(EingabeFehler::KeineGueltigeZahl) => {
                println!("bitte gebe eine gültige ganzzahl ein!");
            }
        }
    };

    let volltreffer = erzeuge_zufallszahlen(anzahl);
    println!("--------> DEINE GLÜCKSZAHLEN <-----");
    for ziehung in volltreffer.iter() {
        println!();
        println!("💵💵💰💰💰 {:?} 💵💵💵💵💰💰", ziehung);
        println!();
    }
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
    generiere_tests_fuer_zufallszahlen_funktionen!(variante_standard, zufallszahl_durch_standard);
    generiere_tests_fuer_zufallszahlen_funktionen!(variante_ablehnung, zufallszahl_durch_ablehnung);
    generiere_tests_fuer_zufallszahlen_funktionen!(
        variante_gleich,
        zufallszahl_durch_gleichverteilung
    );
    generiere_tests_fuer_zufallszahlen_funktionen!(variante_array, zufallszahl_durch_array_auswahl);
    generiere_tests_fuer_zufallszahlen_funktionen!(
        variante_sicher,
        zufallszahl_durch_systementropie
    );

    #[test]
    fn test_werden_geforderte_anzahl_von_zahlen_gegeben() {
        let anzahl = 8;
        let zahlen = erzeuge_zufallszahlen(anzahl);

        assert_eq!(
            zahlen.len(),
            anzahl.into(),
            "Es wurde nicht die geforderte Anzahl an Zahlen geliefert"
        );
    }

    #[test]
    fn test_keine_doppelten_zahlen_liefern() {
        let anzahl = 1;
        let zahlen = erzeuge_zufallszahlen(anzahl);
        let mut sortiert = zahlen[0].clone();
        sortiert.sort();
        dbg!(&zahlen);
        dbg!(&sortiert);

        // windows(2) -> prüft jedes aufeinanderfolgende paar
        // wichtiger hinweis -> any bricht bei erstem treffer ab
        // also nicht versuchen auf ungleichheit zu prüfen
        // und dann eine true-assert zu erwarten ;)
        let sind_doppelte = sortiert.windows(2).any(|r| r[0] == r[1]);

        assert!(!sind_doppelte, "Darin sind doppelte Werte {:?}", zahlen);
    }
}
