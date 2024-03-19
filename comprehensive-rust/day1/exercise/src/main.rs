fn traspose_matrix(matrix: [[i32;3];3]) -> [[i32;3];3] {
    
    let mut transposed_matrix: [[i32;3];3] = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ];

    for i in 0..matrix.len(){
        for j in 0..matrix[0].len() {
            transposed_matrix[j][i] = matrix[i][j]
        }
    };

    return  transposed_matrix;
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);

    let trasposed = traspose_matrix(matrix);
    
    println!("matrix: {:#?}", trasposed);

}
