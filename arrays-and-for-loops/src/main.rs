fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            transposed[j][i] = matrix[i][j];
        }
    }

    transposed
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{:?}", row);
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    println!("transposed:");
    pretty_print(&transpose(matrix));
}
