fn cribbage_eval(hand: &[u32; 5]) -> u32 {
    let fifteens_eval = 2 * subset_sum_count(15, hand);
    let sequence_eval = 0; // todo
    let pair_eval = 0; // todo
    let flush_eval = 0; // todo
    let jacks_eval = 0; // todo
    fifteens_eval + sequence_eval + pair_eval + flush_eval + jacks_eval
}

fn subset_sum_count(sum: u32, hand: &[u32]) -> u32 {
    if sum == 0 {
        return 1;
    }
    if hand.len() == 0 {
        return 0;
    }
    let with_elem_count = if sum >= hand[0] {
        subset_sum_count(sum - hand[0], &hand[1..])
    } else {
        0
    };
    let wout_elem_count = subset_sum_count(sum, &hand[1..]);
    with_elem_count + wout_elem_count
}

fn main() {
    let xs = [10, 9, 8, 7, 6];
    println!("cribbage_eval = {}", cribbage_eval(&xs));
}
