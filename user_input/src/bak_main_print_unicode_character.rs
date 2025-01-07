fn main() {
    println!("\u{231A} \u{231B}");
    println!("{:#06X}", '행' as u32); // Cast char as u32 to get the hexadecimal value
    println!("{:#06X}", 'H' as u32);
    println!("{:#06X}", '居' as u32);
    println!("{:#06X}", 'い' as u32);
    println!("😂: {:#06X}", '😂' as u32);

    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}"); // Try printing them with unicode escape \u
    println!("\u{1f602}");
}
