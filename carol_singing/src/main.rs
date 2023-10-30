fn main() {
    let lines = vec![
        "A partrige in a peartree!",
        "two turtle doves,",
        "three french hens,",
        "four calling birds,",
        "five gold rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    let numbers = vec![
        "first",
        "second",
        "third",
        "fourth",
        "fith",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelvth",
    ];

    for day_of_christmas in 0..12 {
        println!("On the {} day of Chrismas,",numbers[day_of_christmas]);
        println!("My true love gave to me,");

        for gift in (0..=day_of_christmas).rev() {
            println!("{}",lines[gift]);
        };
        println!("");
    }
}
