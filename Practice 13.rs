fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    if total % n as u32 != 0 {
        return usize::MAX;
    }

    let target = total / n as u32;
    let mut moves: usize = 0;

    for &shipment in shipments {
        if shipment > target {
            moves += (shipment - target) as usize;
        }
    }

    moves
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let total: u32 = (n as u32 * (n as u32 + 1)) / 2;
    let mut shipments = vec![total / n as u32; n];

    for i in 0..total as usize % n {
        shipments[i] += 1;
    }

    shipments
}

fn main() {
    let shipments = vec![4, 3, 5];
    let moves = count_permutation(&shipments);
    
    if moves == usize::MAX {
        println!("Неможливо розподілити вантаж.");
    } else {
        println!("Мінімальна кількість переносу вантажу: {}", moves);
    }

    let generated_shipments = gen_shipments(5);
    println!("Згенеровані вантажі: {:?}", generated_shipments);
}

fn test_data() -> Vec<u32> {
    vec![4, 3, 5]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_permutation() {
        let shipments = test_data();
        let moves = count_permutation(&shipments);
        assert_eq!(moves, 1);
    }

    #[test]
    fn test_gen_shipments() {
        let generated_shipments = gen_shipments(5);
        assert_eq!(generated_shipments.len(), 5);
        assert!(generated_shipments.iter().all(|&x| x >= 1));
    }
}
