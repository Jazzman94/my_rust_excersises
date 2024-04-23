fn main() {
    let lyrics= ["And a partridge in a pear tree.\n",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five gold rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,"];
    let days = ["first","second","third","fourth","fifth","sixth","seventh","eighth","nineth","tenth","eleventh","twelveth"];
    println!("\nThe Twelve Days of Christmas\n");

    for (index, day) in days.iter().enumerate() {
        println!("On the {day} day of Christmas my true love sent to me");
        if index == 0 {
            println!("A partridge in a pear tree.\n");
        } else {
            for i in (0..index+1).rev() {
                println!("{}",lyrics[i]);
            }
        }
    }

}
