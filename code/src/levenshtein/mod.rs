pub fn recursive(u: &str, v: &str) -> u32 {
    if u.is_empty() { return v.len() as u32 }
    if v.is_empty() { return u.len() as u32 }

    let uc = u[(u.len() - 1) .. u.len()].chars().next();
    let vc = v[(v.len() - 1) .. v.len()].chars().next();
    let edit_cost = if uc == vc {0} else {1};

    let u_accent = &u[0..(u.len() - 1)];
    let v_accent = &v[0..(v.len() - 1)];

    minimum(
        recursive(&u_accent, &v) + 1,
        recursive(&u, &v_accent) + 1,
        recursive(&u_accent, &v_accent) + edit_cost
    )
}

fn minimum(a: u32, b: u32, c: u32) -> u32 {
    if a < b {
        if a < c {
            a
        } else {
            c
        }
    } else {
        if b < c {
            b
        } else {
            c
        }
    }
}

pub fn matrix(u : &str, v: &str) -> u32 {
    let m = u.len() + 1;
    let n = v.len() + 1;
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; m]; n];

    for column in 0..m {
        matrix[0][column] = column as u32;
    }

    for row in 1..n {
        matrix[row][0] = row as u32;
        for column in 1..m {
            let uc = u[(column - 1) .. column].chars().next();
            let vc = v[(row - 1) .. row].chars().next();
            let edit_cost = if uc == vc {0} else {1};

            matrix[row][column] = minimum(
                matrix[row - 1][column    ] + 1,
                matrix[row    ][column - 1] + 1,
                matrix[row - 1][column - 1] + edit_cost
            );
        }
    }

    matrix[n - 1][m - 1]
}

#[cfg(test)]
mod tests {
    mod recursive {
        use super::super::*;

        #[test]
        fn levenshtein_of_empty_strings_is_zero() {
            assert_eq!(0, recursive("", ""));
        }

        #[test]
        fn levenshtein_of_empty_strings_and_non_empty_string_is_length_of_non_empty_string() {
            let string: &str = "Hello";

            assert_eq!(string.len() as u32, recursive(string, ""));
            assert_eq!(string.len() as u32, recursive("", string));
        }

        #[test]
        fn levenshtein_distace_of_one_is_correctly_calculated() {
            assert_eq!(1, recursive("a", "ab"));
            assert_eq!(1, recursive("ba", "b"));
            assert_eq!(1, recursive("a", "b"));
        }

        #[test]
        fn levenshtein_distance_of_kangaroo_koala() {
            assert_eq!(6, recursive("kangaroo", "koala"));
        }
    }

    mod matrix {
        use super::super::*;

        #[test]
        fn levenshtein_of_empty_strings_is_zero() {
            assert_eq!(0, matrix("", ""));
        }

        #[test]
        fn levenshtein_of_empty_strings_and_non_empty_string_is_length_of_non_empty_string() {
            let string: &str = "Hello";

            assert_eq!(string.len() as u32, matrix(string, ""));
            assert_eq!(string.len() as u32, matrix("", string));
        }

        #[test]
        fn levenshtein_distace_of_one_is_correctly_calculated() {
            assert_eq!(1, matrix("a", "ab"));
            assert_eq!(1, matrix("ba", "b"));
            assert_eq!(1, matrix("a", "b"));
        }

        #[test]
        fn levenshtein_distance_of_kangaroo_koala() {
            assert_eq!(6, matrix("kangaroo", "koala"));
        }
    }
}
