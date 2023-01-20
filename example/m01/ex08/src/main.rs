fn main() {
    dbg!(std::mem::size_of::<i32>());
    dbg!(std::mem::size_of::<&i32>());
    dbg!(std::mem::size_of::<[i32; 6]>());
    dbg!(std::mem::size_of::<&[i32; 6]>());
    dbg!(std::mem::size_of::<&[i32]>());
}
