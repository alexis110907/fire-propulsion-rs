use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::io;

// Shared State variables
struct SharedState 
{
    updated_fire_time: bool,
    fired_propulsion: bool,
    commands: Vec<i32>,
}

fn fire_propulsion_manager(shared_state: Arc<Mutex<SharedState>>) 
{
    let mut fire_time_sec: i32 = -1;
    let mut start_time = Instant::now();
    loop 
    {
        let mut state = shared_state.lock().expect("Failed to lock the mutex to check updated_fire_time or fired_propulsion."); // Get lock
        
        if state.updated_fire_time || state.fired_propulsion 
        {
            if let Some(&fire_time) = state.commands.last() // If there is a value at the end of command, get <Options> value
            {
                fire_time_sec = fire_time;
                start_time = Instant::now(); // Restart the start time
                //println!("Time changed to: {}", fire_time_sec); // Used for debugging purposes
                
                if fire_time_sec == -1 
                {
                    state.commands.clear();
                }
                state.updated_fire_time = false;
                state.fired_propulsion = false;
            }
        }
        drop(state); // Release the lock
        
        // Sleep for 0.1 seconds until it's time to fire propulsion
        if start_time.elapsed() < Duration::from_secs(fire_time_sec as u64)
         {
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        
        if fire_time_sec == -1 
        {
            continue;
        }
        
        //println!("Fire Time: {}", fire_time_sec); // Used for debugging purposes
        println!("firing now!");
        let mut state = shared_state.lock().expect("Failed to lock the mutex that updates shared states after firing propulsion"); // Get the lock
        state.fired_propulsion = true;
        fire_time_sec = -1;
        state.commands.pop();
    } // Lock is automatically release at end of bounds
}

fn main() 
{
    let shared_state = Arc::new(Mutex::new(SharedState {
        updated_fire_time: false,
        fired_propulsion: false,
        commands: Vec::new(),
    })); // SharedState is wrapped inside a mutex, which is then wrapped inside an Arc (shared_state)
    
    let state_clone = Arc::clone(&shared_state); // Clone reference to Arc (shared_state) so new thread can access the SharedState data
    let manager_thread = thread::spawn(move || fire_propulsion_manager(state_clone)); //Moves fire_propulsion_manager to a new thread
    
    let stdin = io::stdin(); //Gets standard input (keyboard)
    let mut input = String::new(); //To store input
    
    while stdin.read_line(&mut input).is_ok() //While stdin can read a line from the console, if not, while loop will terminate
    {
        let trimmed = input.trim(); // Removes extra spaces or newlines
        if let Ok(num) = trimmed.parse::<i32>() 
        {
            let mut state = shared_state.lock().expect("Failed to lock the mutex that updates the commands vector"); //Get lock         
            if num == -1 
            {
                state.commands.clear();
            }
            state.commands.push(num);
            state.updated_fire_time = true;
        }
        input.clear();
    }
    
    manager_thread.join().expect("Failed to join manager_thread");
}
