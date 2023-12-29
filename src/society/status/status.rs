use crate::society::culture::culture::Culture;
use crate::dice::DiceExt;

use super::nobility::Nobility;

pub trait Status {
    fn wealth(&self)-> f64;
    fn solmod(&self)-> i32;
    fn nobility(&self)-> Option<Nobility>;
    fn survival_bonus(&self)-> i32;
}

pub struct StatusFactory;
impl StatusFactory {
    pub fn new(culture:&dyn Culture)-> Box<dyn Status> {
        let mut c = culture.cumod();
        let mut t = 0;
        let mut n: Option<Nobility> = None;
        loop {
            let r = 1.d(100) + c + t;
            if r <= 12 {
                return Destitute::new(n);
            }
            else if r <= 40 {
                return Poor::new(n);
            }
            else if r <= 84 {
                return Average::new(n);
            }
            else if r == 85 {
                c = 0;
                continue;
            }
            else if r <= 94 {
                return Comfortable::new(n);
            }
            else if r <= 98 {
                return Wealthy::new(n);
            }
            else {
                n = Some(Nobility::new(culture.level()));
                t = n.as_ref().unwrap().timod()
            }
        }
    }
}

fn soldmod_with_potential_nobility(v:i32, n:Option<Nobility>)-> i32  {
    let mut m = v;
    if let Some(_) = n {m += 5}
    m
}

fn survival_with_potential_nobility(v:i32, n:Option<Nobility>)-> i32 {
    let mut m = v;
    if let Some(_) = n {m -= 1}
    m
}

struct Destitute {
    nob: Option<Nobility>,
    surv: i32,
}

impl Status for Destitute {
    fn wealth(&self)-> f64 {0.25}
    fn solmod(&self)-> i32 {soldmod_with_potential_nobility(-3, self.nobility())}
    fn nobility(&self)-> Option<Nobility> {self.nob.clone()}
    fn survival_bonus(&self)-> i32 {survival_with_potential_nobility(self.surv, self.nobility())}
}

impl Destitute {
    pub fn new(nobility:Option<Nobility>)-> Box<dyn Status> {
        Box::new(Destitute{nob: nobility, surv: 1.d(2)})
    }
}

struct Poor {
    nob: Option<Nobility>,
}

impl Status for Poor {
    fn wealth(&self)-> f64 {0.5}
    fn solmod(&self)-> i32 {soldmod_with_potential_nobility(-1, self.nobility())}
    fn nobility(&self)-> Option<Nobility> {self.nob.clone()}
    fn survival_bonus(&self)-> i32 {survival_with_potential_nobility(1, self.nobility())}
}

impl Poor {
    pub fn new(nobility:Option<Nobility>)-> Box<dyn Status> {
        Box::new(Poor{nob: nobility})
    }
}

struct Average {
    nob: Option<Nobility>,
}

impl Status for Average {
    fn wealth(&self)-> f64 {1.0}
    fn solmod(&self)-> i32 {soldmod_with_potential_nobility(0, self.nobility())}
    fn nobility(&self)-> Option<Nobility> {self.nob.clone()}
    fn survival_bonus(&self)-> i32 {survival_with_potential_nobility(0, self.nobility())}
}

impl Average {
    pub fn new(nobility:Option<Nobility>)-> Box<dyn Status> {
        Box::new(Average{nob: nobility})
    }
}

struct Comfortable {
    nob: Option<Nobility>,
}

impl Status for Comfortable {
    fn wealth(&self)-> f64 {2.0}
    fn solmod(&self)-> i32 {soldmod_with_potential_nobility(2, self.nobility())}
    fn nobility(&self)-> Option<Nobility> {self.nob.clone()}
    fn survival_bonus(&self)-> i32 {survival_with_potential_nobility(-1, self.nobility())}
}

impl Comfortable {
    pub fn new(nobility:Option<Nobility>)-> Box<dyn Status> {
        Box::new(Comfortable{nob: nobility})
    }
}

struct Wealthy {
    nob: Option<Nobility>,
    xw: bool,
    surv: i32,
}

impl Status for Wealthy {
    fn wealth(&self)-> f64 {if self.xw {20.0} else {5.0}}
    fn solmod(&self)-> i32 {soldmod_with_potential_nobility(4, self.nobility())}
    fn nobility(&self)-> Option<Nobility> {self.nob.clone()}
    fn survival_bonus(&self)-> i32 {survival_with_potential_nobility(self.surv, self.nobility())}
}

impl Wealthy {
    pub fn new(nobility:Option<Nobility>)-> Box<dyn Status> {
        let xw = match &nobility {
            Some(x) => 1.d(100) <= x.timod()+1,
            _ => 1.d(100) <= 1
        };
        Box::new(Wealthy{nob: nobility, xw, surv: -(1.d(2))})
    }
}
