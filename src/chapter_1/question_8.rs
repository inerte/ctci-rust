// The strategy here is to loop the matrix to get each
// row, then loop each number. If a number is a zero,
// set the row as zero and go back to previous rows
// and set the equivalent column as zero.
// We can't go to forward rows and set to zero otherwise
// the next loop iteration will see a zero and insert a
// row of zeroes.
// So we add the column number to a HashSet, and if the
// number is not zero, but the column is on the HashSet,
// we set the number to zero
use std::collections::HashSet;

fn zero_matrix(matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut zeroed_matrix = matrix.clone();

    let mut columns_to_zero = HashSet::new();

    for (i, row) in matrix.iter().enumerate() {
        let mut found_zero_in_row = false;

        for (j, number) in row.iter().enumerate() {
            if number == &0 {
                for previous_i in 0..i {
                    zeroed_matrix[previous_i][j] = 0;
                }
                found_zero_in_row = true;
                columns_to_zero.insert(j);
            } else if columns_to_zero.contains(&j) {
                zeroed_matrix[i][j] = 0;
            }
        }

        if found_zero_in_row {
            zeroed_matrix[i] = vec![0; row.len()];
        }
    }

    return zeroed_matrix;
}

#[test]
fn zero_matrix_should_zero_column_and_row() {
    let matrix = vec![
        vec![1, 2, 3, 4],
        vec![9, 1, 3, 7],
        vec![1, 2, 0, 4],
    ];

    let zeroed_matrix = vec![
        vec![1, 2, 0, 4],
        vec![9, 1, 0, 7],
        vec![0, 0, 0, 0],
    ];

    assert_eq!(zero_matrix(matrix), zeroed_matrix);
}

#[test]
fn zero_matrix_should_work_with_1x1_matrix() {
    let matrix = vec![
        vec![0],
    ];

    let zeroed_matrix = vec![
        vec![0],
    ];

    assert_eq!(zero_matrix(matrix), zeroed_matrix);
}

#[test]
fn zero_matrix_should_returns_original_matrix_values_if_there_is_no_zero() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![9, 1, 7],
    ];

    let zeroed_matrix = vec![
        vec![1, 2, 3],
        vec![9, 1, 7],
    ];

    assert_eq!(zero_matrix(matrix), zeroed_matrix);
}

#[test]
fn zero_matrix_should_zero_everything_when_entire_row_has_zeroes() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![0, 0, 0],
        vec![9, 1, 5],
    ];

    let zeroed_matrix = vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
    ];

    assert_eq!(zero_matrix(matrix), zeroed_matrix);
}

#[test]
fn zero_matrix_should_zero_everything_when_entire_column_has_zeroes() {
    let matrix = vec![
        vec![1, 0, 3],
        vec![8, 0, 7],
        vec![9, 0, 5],
    ];

    let zeroed_matrix = vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
    ];

    assert_eq!(zero_matrix(matrix), zeroed_matrix);
}
