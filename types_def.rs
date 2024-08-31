use ndarray::Array2;

pub type Spin = i8;
pub type Index = (usize, usize);
pub type Grid = Array2<Spin>;
pub type Neighbours = Array2<[Index; 4]>;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum InitMode {
    Random,
    Chess,
    AllUp,
    AllDown,
    RandomDist,
}
