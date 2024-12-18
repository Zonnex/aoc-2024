// pub struct Grid<T> {
//     pub grid: Vec<T>,
//     pub width: usize,
//     pub height: usize,
// }

// impl<T> Grid<T> {
//     pub fn parse(input: &str) -> Self {
//         let grid = input
//             .lines()
//             .rev()
//             .map(|l| l.as_bytes())
//             .collect::<Vec<_>>();

//         let (width, height) = (grid[0].len(), grid.len());

//         Self {
//             grid,
//             width,
//             height,
//         }
//     }
// }