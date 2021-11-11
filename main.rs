
use std::f64::consts::PI;
#[derive(Debug)]
pub struct Circle<T>{
    pub r:T
}

#[derive(Debug)]
pub struct Triangle<T,U,V>{
    pub a:T,
    pub b:U,
    pub c:V,
}
#[derive(Debug)]
pub struct Square<T>{
    pub a:T
}

pub trait AreaCalculate{
    fn area(&self)->f64;
}

impl<T> AreaCalculate for Circle<T>
where T:Copy+Into<f64>
{
    fn area(&self)->f64{
        let r:f64 = self.r.into();
        PI*r*r

    }
}

impl <T,U,V> AreaCalculate for Triangle <T,U,V>
where T:Copy+Into<f64>,
    U:Copy+Into<f64>,
    V:Copy+Into<f64>
{
    fn area(&self)->f64{

        let a:f64 = self.a.into();
        let b:f64 = self.b.into();
        let c:f64 = self.c.into();

        (a*b*c)/2.0

    }

}

impl <T> AreaCalculate for Square<T>
where T:Copy + Into<f64>{
    fn area(&self)->f64{
        let a= self.a.into();

        a*a

    }

}

fn area<T:AreaCalculate>(shape: &T)->f64{
    shape.area()

}
#[derive(Debug)]
enum TrafficLight{
    RED,
    GREEN,
    YELLOW,
}
trait Delay {
    fn delay_time(&self) ->u32;
}

impl Delay for TrafficLight{
    fn delay_time(&self)->u32{
        match self {
            TrafficLight::RED=>22,
            TrafficLight::GREEN => 20,
            TrafficLight::YELLOW => 2,
        }
    }
}

fn try_sum(a: &[i32])->Option<i32>{
    let mut it = a .iter();
    let sum = it.try_fold(0i32, |acc, &x| acc.checked_add(x));
    sum
}

fn main() {
    let a = [10,20,30, 50, 100];

    println!("Hello, world! {:#?}", try_sum(&a));

    let c = Circle{
        r:10
    };
    // let d=c.copy();
    print!("circle area is {}, {:#?}",area(&c),c );



    // let light_r = TrafficLight::RED;
    // let light_g = TrafficLight::GREEN;
    // let light_y = TrafficLight::YELLOW;

    // println!("{:#?}, {:#?}", light_r,light_r.delay_time());
    // println!("{:#?}, {:#?}", light_g,light_g.delay_time());
    // println!("{:#?}, {:#?}", light_y,light_y.delay_time());

}
