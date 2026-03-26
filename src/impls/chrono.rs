use crate::Atom;
use chrono::TimeDelta;

pub struct Micros<T>(pub T);

impl Atom for Option<Micros<TimeDelta>> {
    type Repr = i64;

    fn pack(self) -> Self::Repr {
        self.map(|d| d.0.num_microseconds().unwrap())
            .unwrap_or(i64::MIN)
    }

    fn unpack(src: Self::Repr) -> Self {
        (src != i64::MIN).then(|| Micros(TimeDelta::microseconds(src)))
    }
}

impl Atom for Micros<TimeDelta> {
    type Repr = i64;

    fn pack(self) -> Self::Repr {
        self.0.num_microseconds().unwrap()
    }

    fn unpack(src: Self::Repr) -> Self {
        Micros(TimeDelta::microseconds(src))
    }
}
