use rand::{thread_rng, Rng};

#[derive(Debug, Default, Clone)]
pub(super) struct Option {
    pub name: String,
    pub weight: i32,
}

pub(super) fn make_decision(options: &[Option]) -> Option {
    let tournament_size = options.len() / 2;
    let mut tournament = Vec::with_capacity(tournament_size);
    let mut rng = rand::thread_rng();

    for _ in 0..tournament_size {
        let index = rng.gen_range(0..options.len());
        tournament.push(options[index].to_owned());
    }

    tournament.iter().fold(tournament[0].to_owned(), |acc, x| {
        if acc.weight > x.weight {
            acc
        } else {
            x.to_owned()
        }
    })
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_make_decision() {
//         let options = vec![
//             Option {
//                 name: String::from("Option1"),
//                 weight: 3,
//             },
//             Option {
//                 name: String::from("Option2"),
//                 weight: 2,
//             },
//             Option {
//                 name: String::from("Option3"),
//                 weight: 4,
//             },
//             Option {
//                 name: String::from("Option4"),
//                 weight: 1,
//             },
//         ];
//         let mut i = 0;
//         let mut sum = (0, 0, 0, 0);
//         for _ in 0..1000 {
//             let decision = make_decision(&options);
//             if decision.weight == 1 {
//                 sum.0 += 1;
//             } else if decision.weight == 2 {
//                 sum.1 += 1;
//             } else if decision.weight == 3 {
//                 sum.2 += 1;
//             } else if decision.weight == 4 {
//                 sum.3 += 1;
//             }

//             // i += 1;
//         }
//         println!("weight:{:?}", sum);
//         // assert_eq!(decision.name, String::from("Option3"));
//         // assert_eq!(decision.weight, 40);
//         assert!(true);
//     }
// }
