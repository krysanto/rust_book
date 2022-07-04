fn main() {
    let verses = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for (i, element) in verses.iter().enumerate() {
        println!("On the {} day of Christmas, my true love sent to me", i + 1);
        
        for (k, element) in verses.iter().enumerate().rev() {
            if k > i{
                continue;
            }
            if (k == 1){
                println!("{}, and", element);
            }else{
                println!("{}", element);
            }
        }
        println!();
    }
}
