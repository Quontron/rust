fn main() {

    //Focusing on teaching shadowing concept - essentially every multiple let on the same
    //variable shows shadowing
    let x = 5; //1

    let x = x + 1; //2 shadows 1

    {
        let x = x * 2; //3 shadows 2
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}