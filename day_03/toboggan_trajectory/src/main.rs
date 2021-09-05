mod input;

struct Trajectory {
    right: usize,
    down: usize,
}

impl Trajectory {
    fn new(right: usize, down: usize) -> Trajectory {
        Trajectory {
            right,
            down
        }
    }
}

fn main() {
    let data = input::get_input();
    let trajectory = Trajectory {
        right: 3,
        down: 1
    };

    let num_trees_hit = calculate_trajectory(&generate_pattern(&data), &trajectory);

    println!("Number of trees hit for first trajectory: {}", num_trees_hit);

    let trajectories = [
        Trajectory::new(1, 1),
        Trajectory::new(3, 1),
        Trajectory::new(5, 1),
        Trajectory::new(7, 1),
        Trajectory::new(1, 2),
    ];

    let num_trees_hit_product = calculate_trajectory_product(&data, &trajectories);

    println!("Sum of number of trees hit by each trajectory: {}", num_trees_hit_product);
}

fn calculate_trajectory_product(data: &Vec<&str>, trajectories: &[Trajectory]) -> u64 {
    let pattern = generate_pattern(data);
    let mut hit_count: u64 = 1;
    
    for trajectory in trajectories {
        let count = calculate_trajectory(&pattern, trajectory);
        println!("Count: {}", count);
        hit_count *= count;
    }

    return hit_count;
}

fn calculate_trajectory(pattern: &Vec<Vec<u8>>, trajectory: &Trajectory) -> u64 {
    let mut count = 0;
    let mut pos = 0;

    for i in ((trajectory.down)..(pattern.len())).step_by(trajectory.down as usize) {
        pos += trajectory.right;
        if pattern[i][(pos) % pattern[i].len()] == "#".as_bytes()[0] {
            count += 1;
        }
    }

    return count;
}

fn generate_pattern(data: &Vec<&str>) -> Vec<Vec<u8>> {
    let mut pattern = Vec::<Vec<u8>>::new();

    for row in data {
        pattern.push(row.as_bytes().to_vec());
    }

    return pattern;
}