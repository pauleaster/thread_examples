use std::time::Instant;
use std::ops::{Index, IndexMut};
use std::sync::mpsc;
use std::thread;

const NLOOPS:u32 = 100_000;
struct Coordinate {

    x: f64,
    y: f64,
    z: f64,
    mag: f64,
}

impl Coordinate {

    fn calc_mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn as_vec(&self) -> Vec<f64> {
        vec![self.x,self.y,self.z]
    }

    fn thread_calc_mag(&self) -> f64 {
        
        let (tx, rx) = mpsc::channel();

        let vals =self.as_vec();
        let vals2 =self.as_vec();
        thread::spawn(move || vals);

        for val in vals2 {
            tx.send(val.powi(2)).unwrap();
        } 
        let mut result :f64 = 0.0;
        let mut counter: usize = 0;
        for received in rx {
            result +=  received;
            // println!("Received {}, cumulative sum = {}, counter = {}",received,result,counter);
            counter += 1;
            if counter >= 3 {
                break;
            }
        }
        result.sqrt()
        
    }
}


impl<'a> IntoIterator for &'a Coordinate {
    type Item = f64;
    type IntoIter = CoordinateIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CoordinateIntoIterator {
            coordinate: self,
            index: 0,
        }
    }
}



pub struct CoordinateIntoIterator<'a> {
    coordinate: &'a Coordinate,
    index: usize,
}



impl <'a> Iterator for CoordinateIntoIterator <'a>{
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        let result = match self.index {
            0 => self.coordinate.x,
            1 => self.coordinate.y,
            2 => self.coordinate.z,
            3 => self.coordinate.mag,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}


impl Index<&'_ usize> for Coordinate {
    type Output = f64;
    fn index(&self, i: &usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.mag,
            _ => panic!("Out of range: {}", i),
        }
    }
}

impl IndexMut<&'_ usize> for Coordinate {
    fn index_mut(&mut self, i: &usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.mag,
            _ => panic!("Out of range: {}", i),
        }
    }
}


fn main() {
    
    println!("Hello, world!");
    
    

    let mut x = Coordinate{x:3.,y:4.,z:5.,mag:0.};
    let mut y:f64 = 0.0;

    let start1 = Instant::now();
    for _ in 0..NLOOPS {
        y = y + x.calc_mag();
    }
    let duration1 = start1.elapsed().as_secs_f64();
    println!("Conventional calculation = {}",y);
    x.mag = x.calc_mag();
    
    println!("Elapsed time = {:0.6}",duration1); 
    for (i, component) in x.into_iter().enumerate() {
        println!("{}, {}", i, component);
        
    }
    let mut x = Coordinate{x:3.,y:4.,z:5.,mag:0.};
    let mut y:f64 = 0.0;

    let start1 = Instant::now();
    for _ in 0..NLOOPS {
        y = y + x.thread_calc_mag();
    }
    let duration2 = start1.elapsed().as_secs_f64();
    println!("Threaded calculation = {}",y);
    x.mag = x.thread_calc_mag();
    
    println!("Elapsed time = {:0.6}",duration2); 
    for (i, component) in x.into_iter().enumerate() {
        println!("{}, {}", i, component);
        
    }
    println!("Duration ratio (1/2) = {}",duration1/duration2);

}