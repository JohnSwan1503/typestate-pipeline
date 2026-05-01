use typestate_pipeline::{Pipelined, Resolved};

use super::{MyTag, Started, Tagged};

pub fn main() {
    fn assert<'a, T>()
    where
        T: Pipelined<'a, Tag = MyTag>,
    {
    }
    assert::<Tagged<'_, Started, Resolved>>();
}
