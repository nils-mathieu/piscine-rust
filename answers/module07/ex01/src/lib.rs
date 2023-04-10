pub type GoldNugget = u16;

pub type Iron = u32;
pub type Mercure = u64;

pub struct PhilosopherStone;

impl PhilosopherStone {
    pub fn transmute_iron(self, iron: Iron) -> [GoldNugget; 2] {
        // SAFETY:
        //  We know that `u16` and `u64` both allow every bit pattern.
        unsafe { std::mem::transmute(iron) }
    }

    pub fn transmute_mercure(self, mercure: Mercure) -> [GoldNugget; 4] {
        // SAFETY:
        //  We know that `u16` and `u64` both allow every bit pattern.
        unsafe { std::mem::transmute(mercure) }
    }
}

#[test]
fn transmute_iron_and_mercure() {
    let iron = 0x12345678;
    assert_eq!(PhilosopherStone.transmute_iron(iron), [0x5678, 0x1234]);
    let mercure = 0x0123456789ABCDEF;
    assert_eq!(
        PhilosopherStone.transmute_mercure(mercure),
        [0xCDEF, 0x89AB, 0x4567, 0x0123],
    );
}

pub type Gold = [GoldNugget];

/// # Safety
///
/// Implementing this trait guarantees that the transmutation from `Self` to a bunch of
/// `GoldNugget`s is valid.
pub unsafe trait Metal {}

impl PhilosopherStone {
    pub fn transmute_metal<M: Metal>(self, metal: &M) -> &Gold {
        let len = std::mem::size_of_val(metal) / std::mem::size_of::<GoldNugget>();

        // SAFETY:
        //  The `Metal` trait ensures that this transmutation is valid.
        unsafe { std::slice::from_raw_parts(metal as *const M as *const GoldNugget, len) }
    }
}

// SAFETY:
//  `GoldNugger` allows every bit pattern and can be transmuted into another `u16`.
unsafe impl Metal for GoldNugget {}

// SAFETY:
//  `Iron` allows every bit pattern and can be transmuted into two `u16`s.
unsafe impl Metal for Iron {}

// SAFETY:
//  `Mercure` allows every bit pattern and can be transmuted into four `u16`s.
unsafe impl Metal for Mercure {}

#[test]
fn test_transmute_metal() {
    let nugget: GoldNugget = 0x1234;
    assert_eq!(PhilosopherStone.transmute_metal(&nugget), &[0x1234]);

    let iron: Iron = 0x12345678;
    assert_eq!(PhilosopherStone.transmute_metal(&iron), &[0x5678, 0x1234]);
    let mercure: Mercure = 0x0123456789ABCDEF;
    assert_eq!(
        PhilosopherStone.transmute_metal(&mercure),
        &[0xCDEF, 0x89AB, 0x4567, 0x0123],
    );
}
