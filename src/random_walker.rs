use core::panic;

use std::collections::*;
use rand_pcg::Pcg64;
use rand::prelude::*;


/// Ein "Struct" - quasi eine Sammlung von Variablen
/// Hier von 2 ganzen Zahlen die als x und y koordinate dienen
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position{
    // x koordinate - eine Ganze Zahl
    pub x: i32,
    // y koordinate - eine Ganze Zahl
    pub y: i32,
}

#[derive(Debug, Clone, Default)]
pub struct History{
    pub vec: Vec<Position>,
    pub hash: HashSet<Position>
}

impl History{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn len(&self) -> usize 
    {
        self.vec.len()
    }

    pub fn push(&mut self, pos: Position)
    {
        self.vec.push(pos.clone());
        self.hash.insert(pos);
    }
}

/// Dies ist der "Random Walker"
/// 
/// Er enthält die aktuelle x und y Koordinate,
/// Sowie einen anfangs leeren Vektor (quasi eine Liste)
/// der die vergangenen x und y Koordinaten enthalten soll.
/// 
/// Außerdem befindet sich hier noch ein Random Number Generator (RNG)
/// der zur erzeugung von Zufallszahlen verwendet werden kann
#[derive(Debug, Clone)]
pub struct RandomWalker{
    pub ort: Position,
    pub history: History,
    pub rng: Pcg64
}


impl RandomWalker
{
    /// Diese funktion dient dazu einen neuen Random walker zu erzeugen.
    /// 
    /// 
    /// Dieser befindet sich am Anfang am Ursprung, also auf position 0,0 und 
    /// hat noch keine "history", da er ja gerade erst erschaffen wurde.
    /// 
    /// Der "seed" wird verwendet um den Random Number Generator zu "seeden".
    /// Computer sind schlecht im erzeugen von tatsächlichen Zufallszahlen,
    /// stattdessen werden sogenannte Pseudo Zufallszahlen generiert,
    /// wobei der Computer die nächste Zufallszahl mittels einer komplizierten Formel 
    /// aus der alten berechnet. 
    /// Die Resultierende Zahlenfolge ist im besten Fall nicht vom Zufall zu unterscheiden.
    /// 
    /// Jedoch muss man ja bei irgendeiner Zahl anfangen - dafür wird der seed verwendet.
    /// Das bedeutet jedoch auch, dass man die selbe Zahlenfolge erhält wenn man mit dem gleichen 
    /// seed beginnt
    /// 
    /// Sie Können diese funktion z.B. mit RandomWalker::new(1231) aufrufen,
    /// wobei 1231 dann als seed verwendet wird
    pub fn new(seed: u64) -> Self {
        Self { 
            ort: Position { x: 0, y: 0 },
            history: History::new(), 
            rng: Pcg64::seed_from_u64(seed)
        }
    }

    /// mit dieser Funktion können Sie eine zahl erzeugen die gleichverteilt
    /// zufällig im Interval [0,1] liegt.
    /// Beispiel:
    /// ```
    /// use GirlsDay::random_walker::*;
    /// let mut walker = RandomWalker::new(123);
    /// 
    /// let number = walker.get_random_number();
    /// ```
    pub fn get_random_number(&mut self) -> f64 {
        self.rng.gen()
    }

    /// diese funktion soll einen zufälligen schritt ausführen,
    /// also mit gleicher wahrscheinlichkeit nach entweder rechts,
    /// links, unten oder oben laufen
    pub fn random_step(&mut self)
    {
        let probability = self.get_random_number();
        let alter_ort = self.ort.clone();

        if probability <= 0.25 {
            self.ort.x += 1;
        } else if probability <= 0.5 {
            self.ort.x -= 1;
        } else if probability <= 0.75{
            self.ort.y += 1;
        } else {
            self.ort.y -= 1;
        }
        self.history.push(alter_ort);
    }
}