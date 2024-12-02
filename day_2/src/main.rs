
fn main() {
    let input = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];

    let mut valid_cnt = 0;

    for report in input {
        if is_report_valid(&report) {
            valid_cnt += 1;
        }
    }

    println!("Valid reports: {}", valid_cnt);
}

/// Check the validity of a report.
///
/// Each report must maintain the following invariants:
///     1. The levels are either all increasing or all
///         decreasing.
///     2. Any two adjacent levels differ by at least one
///         and at most three.
///
fn is_report_valid( report: &Vec<u32>) -> bool {
    let is_increasing = report[0] < report[1];

    for i in 0..(report.len()-1) {
        /* Check invariant 2 */
        let diff = report[i].abs_diff(report[i+1]);
        if diff > 3 || diff < 1 {
            return false
        }

        /* Check invariant 1. */
        if is_increasing {
            if report[i] > report[i+1] {
                return false
            }
        } else {
            if report[i] < report[i+1] {
                return false
            }
        }
    }

    true
}
