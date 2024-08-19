
#[derive(Debug)]
struct Degrees(f32);

#[derive(Debug)]
struct Radians(f32);

impl From<Radians> for Degrees{
    fn from(rad: Radians) -> Self {
        Degrees(rad.0 * 180.0 / std::f32::consts::PI)
    }
}

impl From<Degrees> for Radians{
    fn from(deg: Degrees) -> Self {
        Radians(deg.0 * std::f32::consts::PI / 180.0)
    }
}

fn sin(angle: impl Into<Radians>) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}

fn main() {
    let behind_you = Degrees(13.0);
    let behind_you_radians =  Degrees::from(behind_you);
    let behind_you_radians2: Radians =  Degrees(180.0).into();

    println!("{:?}", behind_you_radians);
    println!("{:?}", behind_you_radians2);
    println!("{:?}", sin(behind_you_radians));
}