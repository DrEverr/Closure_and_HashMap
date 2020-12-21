use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  arg: HashMap<u32, u32>,
  value: Option<u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      arg: HashMap::new(),
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.arg.get(&arg) {
      Some(v) => *v,
      None => {
        let v = (self.calculation)(arg);
        self.arg.insert(arg, v);
        self.value = Some(v);
        v
      },
    } 
  }
}

fn test(a1: u32, a2: u32) {
  let mut expensive_computation = Cacher::new(|num| {
    println!("slow calculation ...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if a1 < 25 {
    println!("Do {} pushups!", expensive_computation.value(a1));
    println!("Do {} situps!", expensive_computation.value(a1+1));
    println!("Do {} pushups!", expensive_computation.value(a1));
    println!("Do {} situps!", expensive_computation.value(a1+1));
    println!("Do {} situps!", expensive_computation.value(a1+1));
    println!("Do {} situps!", expensive_computation.value(a1+2));
  } else {
    if a2 == 3 {
      println!("Take a breake today");
    } else {
      println!(
        "Today, run for {} minutes", 
        expensive_computation.value(a1)
      );
    }
  }
}

fn main() {
  test(22, 1);
}
