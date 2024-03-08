
fn main() {

    let mut data = [45, 78, 0, 55, 63, -52, 32, 98, 25, 7, 9, 3, 10, 11];
    let n = 14;



    for i in 1..n {
        let to_insert = data[i];
        let mut j  = i;
        while  data[j-1]>to_insert && j>0  {
            data[j]=data[j-1]; 
            j=j-1

        }
        data[j]=to_insert;
    }

}

