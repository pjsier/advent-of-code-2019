use std::convert::TryFrom;
use std::fs;

struct SpaceImage {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Vec<Vec<usize>>>,
}

fn count_digits(layers: &Vec<Vec<usize>>, digit: usize) -> usize {
    layers.iter().flatten().filter(|v| **v == digit).count()
}

impl SpaceImage {
    pub fn new(width: usize, height: usize, layers: Vec<Vec<Vec<usize>>>) -> SpaceImage {
        SpaceImage {
            width,
            height,
            layers,
        }
    }

    pub fn from_data(width: usize, height: usize, data: String) -> SpaceImage {
        let layers = data
            .trim()
            .chars()
            .map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap())
            .collect::<Vec<usize>>()
            .chunks(width * height)
            .map(|c| {
                c.to_vec()
                    .chunks(width)
                    .map(|ch| ch.to_vec())
                    .collect::<Vec<Vec<usize>>>()
            })
            .collect();

        SpaceImage {
            width,
            height,
            layers,
        }
    }

    pub fn min_digit_layer(&self, digit: usize) -> &Vec<Vec<usize>> {
        &self
            .layers
            .iter()
            .min_by(|a, b| count_digits(&a, digit).cmp(&count_digits(&b, digit)))
            .unwrap()
    }

    pub fn decode_image(&self) -> String {
        (0..(self.width * self.height))
            .map(|p| {
                self.layers
                    .iter()
                    .map(|lp| lp[p / self.width][p % self.width])
                    .filter(|v| *v != 2)
                    .nth(0)
                    .unwrap_or(2)
            })
            .collect::<Vec<usize>>()
            .chunks(self.width)
            .map(|c| {
                c.iter()
                    .map(|i| match i {
                        1 => "#".to_owned(),
                        _ => " ".to_owned(),
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let space_image = SpaceImage::from_data(25, 6, contents);

    let min_zero_layer = space_image.min_digit_layer(0).clone();
    println!(
        "Part 1: {:?}",
        count_digits(&min_zero_layer, 1) * count_digits(&min_zero_layer, 2)
    );
    println!("Part 2: {}", space_image.decode_image());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_data() {
        let space_image = SpaceImage::from_data(3, 2, "123456789012".to_owned());
        assert_eq!(
            space_image.layers,
            vec![
                vec![vec![1, 2, 3], vec![4, 5, 6]],
                vec![vec![7, 8, 9], vec![0, 1, 2]]
            ]
        );
    }
}
