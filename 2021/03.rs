use std::io;
use std::io::Stdin;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

const SLABINFO_HEADER: &str = "slabinfo - version: 2.1\n";

#[derive(Debug, PartialEq)]
struct Slabinfo {
  name: String,
  active_objs: i32,
}

impl Slabinfo {
  fn new(a: &Vec<&str>) -> Result<Slabinfo, ParseIntError> {
    Ok(Slabinfo {
      name: a[0].to_string(),
      active_objs: i32::from_str(a[1])?
    })
  }

  fn minus(&self, s: &Self) -> Option<Self> {
    if self.name == s.name {
      Some(
        Slabinfo {
          name: self.name.clone(),
          active_objs: self.active_objs - s.active_objs
        }
      )
    } else {
      None
    }
  }
}

#[derive(PartialEq)]
enum InfoState {
  Start, HeaderFound, End
}

struct Info {
  stdin: Stdin,
  state: InfoState
}

impl Info {
  fn new() -> Self {
    Info { stdin: io::stdin(), state: InfoState::Start }
  }
}

impl Iterator for Info {
  type Item = HashMap::<String, Slabinfo>;
  fn next(&mut self) -> Option<Self::Item> {

    if self.state == InfoState::End {
      return None;
    }

    let mut line = String::new();
    let mut h = Self::Item::new();

    loop {
      line.clear();
      let s = match self.stdin.read_line(&mut line) {
        Ok(s)  => s,
        Err(_) => break
      };

      if s == 0 {
        self.state = InfoState::End;
        break;
      }

      match self.state {
        InfoState::Start => {
            if line == SLABINFO_HEADER {
              self.state = InfoState::HeaderFound;
            }
          },
        InfoState::HeaderFound => {
            if line == SLABINFO_HEADER {
              break
            }

            let a = line
              .split_whitespace()
              .collect::<Vec<_>>();

            if a[0] == "#" {
              continue;
            }

            if a.len() != 16 {
              self.state = InfoState::Start;
              break;
            }

            if let Ok(s) = Slabinfo::new(&a) {
              h.insert(s.name.clone(), s);
            }
          },
        _ => panic!("internal error")
      }
    }

    Some(h)
  }
}

fn option_diff(x: &Option<&Slabinfo>, y: &Option<&Slabinfo>) -> Option<Slabinfo> {
  if let Some(s1) = x {
    if let Some(s2) = y {
      return s1.minus(s2);
    }
  }
  None
}

fn main() {
  let v = Info::new().collect::<Vec<_>>();

  if v.len() == 0 {
    return;
  } 

  let keys = &v[0]
    .keys()
    .collect::<Vec<_>>();

  for (current, prev) in v[1..].iter().zip(v.iter()) {
    let mut tmp = keys
      .iter()
      .map(|k| (k, option_diff(&current.get(*k), &prev.get(*k))))
      .filter(|x| x.1 != None)
      .map(|x| (x.0, x.1.unwrap()))
      .filter(|x| x.1.active_objs != 0)
      .collect::<Vec<_>>();
    
    tmp.sort_by(|a, b| (&b).1.active_objs
                  .partial_cmp(&a.1.active_objs).unwrap());

    println!("-------------------");
    for e in tmp {
      println!("{:20} {}", e.0, e.1.active_objs);
    }
  }
}
