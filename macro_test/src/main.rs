macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

macro_rules! vec {
    ( $x:ty ) => {
        {
            let temp_vec: Vec<$x> = Vec::new();
            temp_vec
        }
    };
    ( $( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}


fn main() {
    assert_eq!(25, five_times!(2 + 3));
    let x = vec!(0,1,2);
    println!("{:?}", x);
    let y = vec!(i32);
    println!("{:?}", y);

}
