fn main() {
    //////////////////////// Declare variables ////////////////////////
    const VERSES_AND_NUMBERS_LIST_SIZE: usize = 12;

    let numbers_in_words_list: [&str; VERSES_AND_NUMBERS_LIST_SIZE] = ["first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
    ];

    let verses_list: [&str; VERSES_AND_NUMBERS_LIST_SIZE] = [
    "A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-layin",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
    ];

    let sentence_beginning: &str = "\nOn the ";
    let sentence_end: &str = " day of Christmas, my true love sent to me";

    let mut counter: usize = 0;


    //////////////////////// Run the code ////////////////////////


    while counter < VERSES_AND_NUMBERS_LIST_SIZE {
        println!("{}{}{}", sentence_beginning, numbers_in_words_list[counter], sentence_end);
        counter += 1;
        for verse_index in 0..=counter - 1 {
            println!("{}", verses_list[verse_index]);
        }
    }
}
