fn main() {

    let mut data = [45, 78, 0, 55, 63, -52, 32, 98, 25, 7, 9, 3, 10, 11, 98];
    let n = 14;

    println!( "Initial data : {:?}",data);

    for i in 1..n {
        let to_insert = data[i];
        let mut j  = i;
        while j>0 && data[j-1]>to_insert{
            data[j]=data[j-1]; 
            j=j-1

        }
        data[j]=to_insert;
    }

    println!("Sorted data : {:?}",data);
}

