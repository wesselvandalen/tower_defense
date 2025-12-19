use crate::towers::Tower;

struct Map {
    size    : (usize, usize),
    path    : Vec<(usize, usize)>,
    towers  : Vec<(usize, usize, Tower)>
}