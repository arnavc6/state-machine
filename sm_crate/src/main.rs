extern crate sm;
use sm::sm;

sm! {
    Lock {
        InitialStates { Locked, Unlocked }

        TurnKey {
            Locked => Unlocked
            Unlocked => Locked
        }

        Break {
            Locked, Unlocked => Broken
        }
    }
}

fn main() {
    use Lock::*;
    let lock = Machine::new(Locked);
    let lock = lock.transition(TurnKey);

    assert_eq!(lock.state(), Unlocked);
    assert_eq!(lock.trigger().unwrap(), TurnKey);
}