fn main() {
    
    let mut num = 0;
    let mut count = 0;
    let range = 10..30;
    let fruits = vec!["Apple","Mangoe","Strawberry","Grapes"];
    
    loop {

        num += 1;

        println!("The Number is {num}");
        
        if num > 10 {
            break;
        }
    }


    while count >= -10 {

        if count % 5 == 0 {
            println!("Count = {count}");
        }

        count -= 1;
    }

    for el in range {
        println!("Element = {el}");
    }

    for (index,fruit) in fruits.iter().enumerate() {
        println!("{index},{fruit}");
    }
}
