pub struct Lorenz {
    x: f64,
    y: f64,
    z: f64,
    dt: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl Lorenz {
    pub fn new() -> Self {
        Lorenz {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            dt: 0.005,
            a: 10.0,
            b: 28.0,
            c: 2.6667,
        }
    }

    pub fn set_dt(&mut self, dt: f64) {
        self.dt = dt;
    }

    pub fn step(&mut self) {
        let (x, y, z) = (self.x, self.y, self.z);

        self.x = x + self.dt * (self.a * (y - x));
        self.y = y + self.dt * (x * (self.b - z) - y);
        self.z = z + self.dt * (x * y - self.c * z);
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
}
