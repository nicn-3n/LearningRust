fn main() {

    let mut data = [45, 78, 0, 55, 63, -52, 32, 98, 25, 7, 9, 3, 10, 11];
    let n = 14;

    for i in 0..n {
        for j in 0..n-1-i {
            if data[j]>data[j+1]{
                (data[j], data[j+1])=(data[j+1], data[j] )
            }
        } 
    }

    println!("{:?}",data);


}
