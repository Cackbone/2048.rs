extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use std::io;
use std::convert::AsRef;

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
            "2\n" => move_down(&mut grid),
            "4\n" => move_left(&mut grid),
            "6\n" => move_right(&mut grid),
            "8\n" => move_up(&mut grid),
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


fn move_left(mut grid: &mut [[i8; 4]; 4])
{
    for i in 0..4
    {
        for x in 0..grid.len()
        {
            for y in 1..grid[x].len()  
            {
                if grid[x][y-1] == 0
                {
                    grid[x][y-1] = grid[x][y];
                    grid[x][y] = 0;                    
                }
                else if grid[x][y-1] == grid[x][y]
                {
                    grid[x][y-1] += grid[x][y];
                    grid[x][y] = 0;
                }
            }
        }
    }
}

fn move_right(mut grid: &mut [[i8; 4]; 4])
{
    for i in 0..4
    {
        for x in 0..grid.len()
        {
            for y in 0..grid[x].len()-1  
            {
                if grid[x][y+1] == 0
                {
                    grid[x][y+1] = grid[x][y];
                    grid[x][y] = 0;
                }
                else if grid[x][y+1] == grid[x][y]
                {
                    grid[x][y+1] += grid[x][y];
                    grid[x][y] = 0;
                }
            }
        }
    }
}

fn move_down(mut grid: &mut [[i8; 4]; 4])
{
    for i in 0..4
    {
        for x in 0..grid.len()-1
        {
            for y in 0..grid[x].len()  
            {
                if grid[x+1][y] == 0
                {
                    grid[x+1][y] = grid[x][y];
                    grid[x][y] = 0;
                }
                else if grid[x+1][y] == grid[x][y]
                {
                    grid[x+1][y] += grid[x][y];
                    grid[x][y] = 0;
                }
            }
        }
    }
}

fn move_up(mut grid: &mut [[i8; 4]; 4])
{
    for i in 0..4
    {
        for x in 1..grid.len()
        {
            for y in 0..grid[x].len()  
            {
                if grid[x-1][y] == 0
                {
                    grid[x-1][y] = grid[x][y];
                    grid[x][y] = 0;
                }
                else if grid[x-1][y] == grid[x][y]
                {
                    grid[x-1][y] += grid[x][y];
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
