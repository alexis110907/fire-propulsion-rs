use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::io;

// Shared State variables
struct SharedState 
{
    updated_fire_time: bool,
    fired_propulsion: bool,
    fire_time_sec: i32,
    commands: Vec<i32>,
}

fn fire_propulsion_manager(shared_state: Arc<Mutex<SharedState>>) 
{
    let mut start_time = Instant::now();
    loop 
    {
        let mut state = shared_state.lock().expect("Failed to lock the mutex to check updated_fire_time or fired_propulsion.");
        
        if state.updated_fire_time || state.fired_propulsion 
        {
            if let Some(&fire_time) = state.commands.last() 
            {
                state.fire_time_sec = fire_time;
                start_time = Instant::now();
                
                if fire_time == -1 
                {
                    state.commands.clear();
                }
                state.updated_fire_time = false;
                state.fired_propulsion = false;
            }
        }
        drop(state);
        
        // Sleep for 0.1 seconds until it's time to fire propulsion
        if start_time.elapsed() < Duration::from_secs(shared_state.lock().unwrap().fire_time_sec as u64)
         {
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        
        let mut state = shared_state.lock().expect("Failed to lock the mutex that updates shared states after firing propulsion");
        if state.fire_time_sec == -1 
        {
            continue;
        }
        
        println!("firing now!");
        state.fired_propulsion = true;
        state.fire_time_sec = -1;
        state.commands.pop();
    }
}

fn main() 
{
    let shared_state = Arc::new(Mutex::new(SharedState {
        updated_fire_time: false,
        fired_propulsion: false,
        fire_time_sec: -1,
        commands: Vec::new(),
    }));
    
    let state_clone = Arc::clone(&shared_state);
    let manager_thread = thread::spawn(move || fire_propulsion_manager(state_clone));
    
    let stdin = io::stdin();
    let mut input = String::new();
    
    while stdin.read_line(&mut input).is_ok() 
    {
        let trimmed = input.trim();
        if let Ok(num) = trimmed.parse::<i32>() {
            let mut state = shared_state.lock().unwrap();
            
            if num == -1 {
                state.commands.clear();
            }
            state.commands.push(num);
            state.updated_fire_time = true;
        }
        input.clear();
    }
    
    manager_thread.join().expect("Failed to join manager_thread");
}
