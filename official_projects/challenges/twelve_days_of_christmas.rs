fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", days[i]);

        for j in (0..i+1).rev() {
            if j == 0 && i > 0 {
                print!("and ");
            }
            println!("{}", gifts[j]);
        }

        println!(); // empty line between stanza
    }
}