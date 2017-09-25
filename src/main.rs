extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use std::io;
use std::convert::AsRef;

enum Direction {
	Up, Right, Down, Left
}

fn main()
{
    // Grid 4*4
    let mut grid  = [[0i8; 4];4];
    let mut key;
    let mut valid = true;

    // Initialization
    init_game(&mut grid);
    
    // Main game-loop
    loop
    {
        println!("Your turn (2: down, 4: left, 6: right, 8: top) :");
        key = input_key();
        match key.as_ref()
        {
            "2\n" => move_grid(Direction::Up, &mut grid),
            "4\n" => move_grid(Direction::Left, &mut grid),
            "6\n" => move_grid(Direction::Right, &mut grid),
            "8\n" => move_grid(Direction::Down, &mut grid),
            _ => valid = false,
        }
        
        if valid
        {
            add_tile(&mut grid);
            display_grid(grid);
        }
        else
        {
            valid = true;
            println!("Are you dummy ?");
        }
    }
}

fn init_game(mut grid: &mut [[i8; 4]; 4])
{
    println!("-- 2048 --");
    add_tile(&mut grid);
    display_grid(*grid);
}

fn move_grid(d: Direction, mut grid: &mut [[i8; 4]; 4])
{
	let mut x_delta:i8=0;
	let mut y_delta:i8=0;

	match d {
		Direction::Up => 	x_delta = -1,
		Direction::Down => 	x_delta = 1,
		Direction::Right => y_delta = -1,
		Direction::Left => 	y_delta = 1
	}
	for _ in 0..4
	{
		for x in (if x_delta == 1 {1} else {0})..(if x_delta == -1 {grid.len() - 1} else {grid.len()})
		{
			for y in (if y_delta == 1 {1} else {0})..(if y_delta == -1 {grid.len() - 1} else {grid.len()})
			{
				println!("x: {}, x_delta: {}, y: {}, y_delta: {}", x, x_delta, y, y_delta);
				if grid[((x as i8) - x_delta) as usize][((y as i8) - y_delta) as usize] == 0
				{
					grid[((x as i8) - x_delta) as usize][((y as i8) - y_delta) as usize] = grid[x][y];
					grid[x][y] = 0;
				}
				else if grid[((x as i8) - x_delta) as usize][((y as i8) - y_delta) as usize] == grid[x][y]
				{
					grid[((x as i8) - x_delta) as usize][((y as i8) - y_delta) as usize] += grid[x][y];
					grid[x][y] = 0;
				}
			}
		} 
	}
}

fn input_key() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input;
}


fn add_tile(mut grid: &mut [[i8; 4]; 4])
{
    let free_tiles = get_free_tiles(*grid);
    let range = Range::new(0, free_tiles.len());
    let mut rand = rand::thread_rng();
    let rand_tile;
    let values = [2, 2, 2, 4]; // Tile value 2 or 4
    let mut rand_value = rand::thread_rng();
    let value;
    let index;

    
    rand_tile = range.ind_sample(&mut rand);
    value = match rand_value.choose(&values)
    {
        Some(i) => *i,
        None => 0,
    };
    index = free_tiles[rand_tile];
    grid[index.0][index.1] = value;
}

fn get_free_tiles(grid: [[i8; 4]; 4]) -> Vec<(usize, usize)>
{  
    let mut free_tiles = Vec::new();
    let mut x_index = 0;
    let mut y_index = 0;

    for x in grid.iter()
    {
        for y in x.iter()
        {
            if *y == 0
            {
                free_tiles.push((x_index, y_index));
            }
            y_index += 1;
        }
        x_index += 1;
        y_index = 0;
    }

    return free_tiles;
}

/** Diplay functions **/

fn display_grid(grid: [[i8; 4]; 4])
{
    for x in grid.iter()
    {
        print!("\t");
        print_line('-');;
        print_blocks();
        print!("\t|");
        for y in x.iter()
        {
            match *y
            {
                y if y == 0 => print!("         |"),
                y if y < 10 => print!("    {}    |", y),
                y if y >= 10 => print!("   {}    |", y),
                y if y >= 100 => print!("  {}    |", y),  
                _ => print!("  {}    ", y),
            }
            
        }
        print!("\n");
        print_blocks();  
    }
    print!("\t");
    print_line('-');
}

fn print_line(ln_char: char)
{
    for _ in 0..41
    {
        print!("{}", ln_char);
    }
    print!("\n");
}

fn print_blocks()
{
    print!("\t");
    for _ in 0..4
    {
        print!("|         ")
    }
    print!("|\n");
}
