mod e0001;
mod e0002;
mod e0003;
mod e0004;
mod e0005;
mod e0006;
mod e0007;
mod e0008;
mod e0009;
mod e0010;
mod e0011;
mod e0012;
mod e0013;
mod e0014;
mod e0015;

use crate::euler::e0001::euler_0001;
use crate::euler::e0002::euler_0002_fast;
use crate::euler::e0003::euler_0003;
use crate::euler::e0004::euler_0004;
use crate::euler::e0005::euler_0005;
use crate::euler::e0006::euler_0006;
use crate::euler::e0007::euler_0007;
use crate::euler::e0008::euler_0008;
use crate::euler::e0009::euler_0009;
use crate::euler::e0010::euler_0010;
use crate::euler::e0011::euler_0011;
use crate::euler::e0012::euler_0012;
use crate::euler::e0013::euler_0013;
use crate::euler::e0014::euler_0014;
use crate::euler::e0015::euler_0015;

pub fn get_method(problem_number: usize) -> impl Fn() -> String {
    match problem_number {
        1 => euler_0001,
        2 => euler_0002_fast,
        3 => euler_0003,
        4 => euler_0004,
        5 => euler_0005,
        6 => euler_0006,
        7 => euler_0007,
        8 => euler_0008,
        9 => euler_0009,
        10 => euler_0010,
        11 => euler_0011,
        12 => euler_0012,
        13 => euler_0013,
        14 => euler_0014,
        15 => euler_0015,
        _ => || "Not implemented".to_string(),
    }
}