fn forloop() {
    for elem in 1..5 {
        println!("{}", elem)
    }

    
    for elem in 1..=5 {
        println!("{}", elem)
    }
}

fn nestedLoop() {
    let map = [[5,4,3],[3,2,1]];
    let trg = 3;
    let mut num_of_scanned_elems = 0;

    'outer: for i in 0..2 {
        for j in 0..3 {
            num_of_scanned_elems += 1;
            if map[i][j] == trg {
                break 'outer;
            }
        }
    }

    println!("{}", num_of_scanned_elems)
}

fn main() {
    nestedLoop()
}
