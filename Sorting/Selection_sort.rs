fn main() {

    let mut data = [45, 78, 0, 55, 63, -52, 32, 98, 25, 7, 9, 3, 10, 11];
    let n = 14;

    for i in 0..n-1{
        let mut min = i;
        for j in i+1..n {
            if data[j]<data[min]{
                min=j;
            }
        } 
        (data[min], data[i])=(data[i], data[min] );
    }

    println!("0{:?}",data);


}

