
use egui::plot::PlotPoint;
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
    pub distance_from_origin: Vec<f64>
}

impl History{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self
    {
        Self{
            vec: Vec::with_capacity(capacity),
            distance_from_origin: Vec::with_capacity(capacity)
        }
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
        let distance_from_origin = ((pos.x * pos.x + pos.y*pos.y) as f64).sqrt();
        self.vec.push(pos);
        self.distance_from_origin.push(distance_from_origin);
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
    pub rng: Pcg64,
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
        let rng = Pcg64::seed_from_u64(seed);
        Self::with_capacity_and_rng(rng, 100000)
    }

    pub fn with_capacity_and_rng(rng: Pcg64, capacity: usize) -> Self{
        Self { 
            ort: Position { x: 0, y: 0 },
            history: History::with_capacity(capacity), 
            rng
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

    pub fn random_step_biased_away(&mut self, strength_of_bias: f64)
    {
        let probability = self.get_random_number();
        let alter_ort = self.ort.clone();

        if probability > 0.5{
           #[allow(clippy::comparison_chain)]
            let prob_p = if self.ort.x == 0{
                0.5
            }else if self.ort.x > 0 {
                0.5+strength_of_bias
            } else {
                0.5-strength_of_bias
            };
            let prob = self.get_random_number();
            if prob < prob_p {
                self.ort.x += 1;
            } else {
                self.ort.x -= 1;
            }

        } else {
            #[allow(clippy::comparison_chain)]
            let prob_p = if self.ort.y == 0{
                0.5
            }else if self.ort.y > 0 {
                0.5 + strength_of_bias
            } else {
                0.5 - strength_of_bias
            };
            let prob = self.get_random_number();
            if prob < prob_p {
                self.ort.y += 1;
            } else {
                self.ort.y -= 1;
            }
        }
        self.history.push(alter_ort);
    }

    pub fn random_step_biased_to_origin(&mut self, strength_of_bias: f64)
    {
        let probability = self.get_random_number();
        let alter_ort = self.ort.clone();

        if probability > 0.5{
           #[allow(clippy::comparison_chain)]
            let prob_p = if self.ort.x == 0{
                0.5
            }else if self.ort.x > 0 {
                0.5 - strength_of_bias
            } else {
                0.5 + strength_of_bias
            };
            let prob = self.get_random_number();
            if prob < prob_p {
                self.ort.x += 1;
            } else {
                self.ort.x -= 1;
            }

        } else {
            #[allow(clippy::comparison_chain)]
            let prob_p = if self.ort.y == 0{
                0.5
            }else if self.ort.y > 0 {
                0.5 - strength_of_bias
            } else {
                0.5 + strength_of_bias
            };
            let prob = self.get_random_number();
            if prob < prob_p {
                self.ort.y += 1;
            } else {
                self.ort.y -= 1;
            }
        }
        self.history.push(alter_ort);
    }

    pub fn your_step_function(&mut self, _strength_of_bias: f64)
    {
        let probability = self.get_random_number();
        let alter_ort = self.ort.clone();
        if probability <= 0.125 {
            self.ort.x += 1;
            self.ort.y += 2;
        } else if probability <= 0.25{
            
            self.ort.x += 1;
            self.ort.y -= 2;
            
        } else if probability <= 0.375 {
            self.ort.x -= 1;
            self.ort.y -= 2;
        } else if probability <= 0.5{
            self.ort.x -= 1;
            self.ort.y += 2;
        } else if probability <= 0.625{
            self.ort.y += 1;
            self.ort.x += 2;
        }else if probability <= 0.75{
            self.ort.y += 1;
            self.ort.x -= 2;
        } else if probability <= 0.875 {
            self.ort.y -= 1;
            self.ort.x -= 2;
        }else {
            self.ort.y -= 1;
            self.ort.x += 2;
        }
        self.history.push(alter_ort);
    }
}

#[derive(Debug, Default, Clone)]
pub struct AverageDistance{
    pub average_distance_plot_data: Vec<PlotPoint>
}

impl AverageDistance{

    pub fn push_averages(&mut self, averages: &[f64]){
        let start = self.average_distance_plot_data.len();
        self.average_distance_plot_data
            .extend(
                averages
                    .iter()
                    .zip(start..)
                    .map(|(y, x)| PlotPoint{x: x as f64, y: *y})
            );
    }

    pub fn update_on_step_of_walkers(&mut self, number_of_steps: usize, walkers: &[RandomWalker])
    {
        let mut sums = vec![0.0; number_of_steps];
        let idx_start = walkers[0].history.len() - number_of_steps;
        let num_of_walkers = walkers.len();

        for walker in walkers{
            let new_additions_slice = &walker.history.distance_from_origin[idx_start..];
            for i in 0..sums.len()
            {
                sums[i] += new_additions_slice[i];
            }
        }
        
        sums.iter_mut()
            .for_each(|val| *val /= num_of_walkers as f64);
        self.push_averages(&sums);
    }

    pub fn cloned_average(&self) -> Vec<PlotPoint>
    {
        self.average_distance_plot_data.clone()
    }

    pub fn get_approximation(&self) -> Vec<PlotPoint>
    {
        self.average_distance_plot_data
            .iter()
            .step_by(100)
            .copied()
            .collect()
    }
}