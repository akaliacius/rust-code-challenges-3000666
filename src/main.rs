fn median(a: Vec<f32>) -> Option<f32> {
    let length = a.len();
    let half = length/2;
    match length {
        0 => None,
        1 => Some(a[0]),
        _ => {
            let mut copy: Vec<f32> = a.clone();
            copy.sort_by(|a, b| a.partial_cmp(b).unwrap());
            match length % 2 == 0 {
                true => {
                    let h = copy[half];
                    let n = copy[half-1];
                    Some((h+n)/2.0)
                },
                false => Some(copy[half])
            }
        }
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
