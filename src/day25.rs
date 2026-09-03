use crate::prelude::*;
use std::{sync::mpsc, time::Duration};

type Pos = (i32, i32);

fn move_to(pos: Pos, dir: &str) -> Pos {
    match dir.trim() {
        "north" => (pos.0, pos.1 - 1),
        "south" => (pos.0, pos.1 + 1),
        "east"  => (pos.0 + 1, pos.1),
        "west"  => (pos.0 - 1, pos.1),
        _ => pos,
    }
}

fn rev_dir(dir: &str) -> &'static str {
    match dir.trim() {
        "north" => "south",
        "south" => "north",
        "east"  => "west",
        "west"  => "east",
        _ => "",
    }
}

fn step_to_idx(pos: Pos, target: Pos) -> usize {
    match (pos.0 - target.0, pos.1 - target.1) {
        (0, -1) => 0,
        (0, 1)  => 1,
        (1, 0)  => 2,
        (-1, 0) => 3,
        _ => unreachable!(),
    }
}

fn step_to_str(pos: Pos, target: Pos) -> &'static str {
    match (pos.0 - target.0, pos.1 - target.1) {
        (0, -1) => "north",
        (0, 1)  => "south",
        (1, 0)  => "east",
        (-1, 0) => "west",
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    BlacklistItem,
    Checkpoint,
    Empty,
    PressurePlate,
    SafeItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Room {
    pub feature: Feature,
    pub doors: [bool; 4], 
}

impl Room {
    pub fn new(feature: Feature, doors: [bool; 4]) -> Self {
        Self { feature, doors }
    }
}

// Direction indices helper
const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;

const DIR_STRINGS: [&str; 4] = ["north", "south", "east", "west"];

// Communication from the background worker thread back to the main agent thread.
pub enum ThreadMessage {
    OutputReceived(String),  // Worker paused normally for input and returned the screen text output.
    GameCompleted(String),   // Worker hit a hard halt command (Opcode 99 / HCF).
    CrashDetected,           // Worker self-detected a loop crash or soft-lock via `run_safe`.
    WorkerReleased(Machine), // Worker is set free from the agent.
}

// A structured container holding the channels for the active background worker.
pub struct ActiveWorker {
    pub tx_cmd: mpsc::Sender<String>,
    pub rx_msg: mpsc::Receiver<ThreadMessage>,
}

pub fn spawn_worker(input: &Program) -> ActiveWorker {
    let (tx_cmd, rx_cmd) = mpsc::channel::<String>();
    let (tx_msg, rx_msg) = mpsc::channel::<ThreadMessage>();

    let mut local_vm = Machine::new(input);

    // Initial boot up to extract game introduction text
    local_vm.run();
    let boot_text = local_vm.drain_to_string();
    let _ = tx_msg.send(ThreadMessage::OutputReceived(boot_text));

    // Spawn the background thread
    std::thread::spawn(move || {
        loop {
            // Block until a new command is issued by the main mapping agent
            match rx_cmd.recv() {
                Ok(command_str) => {
                    // Check if the master agent wants to take manual control of the VM
                    if command_str == "RELEASE_VM" {
                        let _ = tx_msg.send(ThreadMessage::WorkerReleased(local_vm));
                        break; // Terminate worker thread safely
                    }

                    local_vm.push_manual_input(&command_str);
                    local_vm.resume();

                    // Run in safe mode using ultra-high-performance zero-allocation engine
                    // with limits on max instructions and consecutive input repeats
                    match local_vm.run_safe_mode(200_000, 4) {
                        SafeRunResult::Paused => {
                            let text_output = local_vm.drain_to_string();
                            if tx_msg.send(ThreadMessage::OutputReceived(text_output)).is_err() { break; }
                        }
                        SafeRunResult::Halted => {
                            let final_text = local_vm.drain_to_string();
                            let _ = tx_msg.send(ThreadMessage::GameCompleted(final_text));
                            break; // Graceful worker termination
                        }
                        SafeRunResult::SoftLocked | SafeRunResult::InfiniteLoop => {
                            // Self-detected a break condition! Signal the master loop to execute shutdown.
                            let _ = tx_msg.send(ThreadMessage::CrashDetected);
                            break; // Hard abort this background thread
                        }
                    }
                }
                Err(_) => break, // Channel closed by main thread, terminate gracefully
            }
        }
    });

    ActiveWorker { tx_cmd, rx_msg }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RoomEvent {
    RoomName(String),     // Found a room title (e.g., "Security Checkpoint")
    Door(String),         // Found an available exit path direction
    ItemOnGround(String), // Found an item lying on the ground
    ItemTaken(String),    // Confirmed an item was successfully added to inventory
    AlertTooHeavy,        // Caught weight alert: Droid is too heavy
    AlertTooLight,        // Caught weight alert: Droid is too light
    Password(String),     // Received the password
}

fn analyse_room(text: &String) -> Vec<RoomEvent> {
    // Prompts to look for:
    //
    // == Pressure-Sensitive Floor ==
    // == Security Checkpoint ==
    //
    // A loud, robotic voice says "Alert! Droids on this ship are heavier than the detected value!" and you are ejected back to the checkpoint.
    // A loud, robotic voice says "Alert! Droids on this ship are lighter than the detected value!" and you are ejected back to the checkpoint.
    //
    // Doors here lead:
    // - north
    // - east
    // - south
    // - west
    //
    // Items here:
    // - <item>
    //
    // You take the <item>.
    //
    // A loud voice says: 'Keypad password is: <password>'
    let mut events = Vec::new();

    // Context reading state
    let mut reading_doors = false;
    let mut reading_items = false;

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }

        // Detect room names (e.g., "== Security Checkpoint ==")
        if trimmed.starts_with("==") && trimmed.ends_with("==") {
            let name = trimmed
                .trim_matches('=')
                .trim()
                .to_string();
            events.push(RoomEvent::RoomName(name));
            continue;
        }

        // Context triggers: determine what the bullet points below mean
        if trimmed.starts_with("Doors here lead:") {
            reading_doors = true;
            reading_items = false;
            continue;
        }
        if trimmed.starts_with("Items here:") {
            reading_items = true;
            reading_doors = false;
            continue;
        }

        // Process bullet points based on active context state
        if trimmed.starts_with("- ") {
            let entry = trimmed[2..].trim().to_string();
            if reading_doors {
                events.push(RoomEvent::Door(entry));
            } else if reading_items {
                events.push(RoomEvent::ItemOnGround(entry));
            }
            continue;
        }

        // Reset lists if a line doesn't start with a bullet point or header
        if !trimmed.starts_with("- ") {
            reading_doors = false;
            reading_items = false;
        }

        // Detect item actions (e.g., "You take the spool of cat6.")
        if trimmed.starts_with("You take the ") {
            let item_name = trimmed["You take the ".len()..]
                .trim_end_matches('.')
                .trim()
                .to_string();
            events.push(RoomEvent::ItemTaken(item_name));
            continue;
        }

        // Detect pressure floor alerts
        if trimmed.contains("Alert!") {
            if trimmed.contains("heavier") {
                events.push(RoomEvent::AlertTooHeavy);
            } else if trimmed.contains("lighter") {
                events.push(RoomEvent::AlertTooLight);
            }
            continue;
        }

        // Detect the final passcode sequence
        if trimmed.contains("keypad") || trimmed.contains("password") {
            let password: String = trimmed.chars()
                .filter(|c| c.is_ascii_digit())
                .collect();

            if !password.is_empty() {
                events.push(RoomEvent::Password(password));
                continue;
            }
        }
    }

    events
}

fn solve_puzzle(vm: &mut Machine, plate_dir: &str, items: &Vec<String>) -> String {
    for item in items {
        vm.push_manual_input(&format!("drop {}", item));
        vm.resume();
        vm.clear_output(); 
    }

    for i in 0..256_usize {
        
        // Pick up items matching the active bits of index 'i'
        for j in 0..items.len() {
            if (i & (1 << j)) != 0 {
                vm.push_manual_input(&format!("take {}", items[j]));
                vm.resume();
                vm.clear_output();
            }
        }

        // Step forward onto the scale plate
        vm.push_manual_input(plate_dir);
        vm.resume();

        let text_output = vm.drain_to_string();

        // Pass text to event matrix
        let events = analyse_room(&text_output);
        for event in events {
            if let RoomEvent::Password(password) = event {
                return password;
            }
        }

        // Shed only the items picked up for this turn
        for j in 0..items.len() {
            if (i & (1 << j)) != 0 {
                vm.push_manual_input(&format!("drop {}", items[j]));
                vm.resume();
                vm.clear_output();
            }
        }
    }

    panic!("Cycled all 256 combinations but the airlock remained locked!");
}

pub fn path_to_target(map: &DynaMap<Room>, start: Pos, target: Pos) -> Option<std::collections::vec_deque::IntoIter<&'static str>> {
    if start == target {
        return None;
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == target {
            let mut forward_path = VecDeque::new();
            let mut curr = current;
            while let Some(&(prev, dir_idx)) = parent.get(&curr) {
                forward_path.push_front(DIR_STRINGS[dir_idx]); 
                curr = prev;
            }
            return Some(forward_path.into_iter());
        }

        if let Some(room) = map.get(&current) {
            for i in 0..4 {
                if room.doors[i] {
                    let next_pos = move_to(current, DIR_STRINGS[i]);
                    if !visited.contains(&next_pos) && (map.is_explored(next_pos) || next_pos == target) {
                        visited.insert(next_pos);
                        parent.insert(next_pos, (current, i));
                        queue.push_back(next_pos);
                    }
                }
            }
        }
    }

    None
}

fn compute_safe_harvest_path(map: &DynaMap<Room>, start: Pos, target: Pos) -> Vec<usize> {
    if start == target { return Vec::new(); }

    let mut queue = std::collections::VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == target {
            let mut path = Vec::new();
            let mut curr = current;
            while let Some(&(prev, dir_idx)) = parent.get(&curr) {
                path.push(dir_idx);
                curr = prev;
            }
            path.reverse();
            return path;
        }

        if let Some(node) = map.get(&current) {
            for i in 0..4 {
                if node.doors[i] {
                    let next_pos = move_to(current, DIR_STRINGS[i]);
                    if !visited.contains(&next_pos) {
                        if let Some(neighbor_node) = map.get(&next_pos) {
                            if neighbor_node.feature != Feature::BlacklistItem 
                                && neighbor_node.feature != Feature::PressurePlate 
                            {
                                visited.insert(next_pos);
                                parent.insert(next_pos, (current, i));
                                queue.push_back(next_pos);
                            }
                        }
                    }
                }
            }
        }
    }

    Vec::new()
}

fn find_closest_feature(map: &DynaMap<Room>, start: Pos, target_feature: Feature) -> Pos {
    let mut queue = std::collections::VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if let Some(node) = map.get(&current) {
            if node.feature == target_feature {
                return current;
            }
            
            for i in 0..4 {
                if node.doors[i] {
                    let next_pos = move_to(current, DIR_STRINGS[i]);
                    if !visited.contains(&next_pos) && map.is_explored(next_pos) {
                        visited.insert(next_pos);
                        queue.push_back(next_pos);
                    }
                }
            }
        }
    }
    start // Fallback safeguard
}

fn find_feature_coordinates(map: &DynaMap<Room>, target_feature: Feature) -> Pos {
    map.iter()
        .find(|(_, node)| node.feature == target_feature)
        .map(|(&pos, _)| pos)
        .expect("The requested destination room is missing from the mapped layout database.")
}

fn extract_item_name(room_info: &[RoomEvent]) -> String {
    room_info.iter()
        .find_map(|e| if let RoomEvent::ItemOnGround(name) = e { Some(name.clone()) } else { None })
        .unwrap_or_default()
}

fn extract_room_name(room_info: &[RoomEvent]) -> String {
    room_info.iter()
        .find_map(|e| if let RoomEvent::RoomName(name) = e { Some(name.clone()) } else { None })
        .unwrap_or_default()
}

fn is_adjacent(start: Pos, end: Pos) -> bool {
    (start.0 - end.0).abs() + (start.1 - end.1).abs() == 1    
}

fn execute_travel_sequence<I>(
    worker: &mut ActiveWorker,
    map: &mut DynaMap<Room>,
    dfs_target_stack: &mut Vec<Pos>,
    pos: &mut Pos,
    current_room_text: &mut String,
    plate_dir: &mut &'static str,
    path_iterable: I
) where I: IntoIterator<Item = &'static str> {
    let mut current_step_pos = *pos;
    let mut last_received_text = current_room_text.clone();
    let mut bounced_early = false;

    // Consume the static string slices lazily straight out of the stream payload
    for dir_str in path_iterable {
        let cmd = format!("{}\n", dir_str);
        let target_pos = move_to(current_step_pos, dir_str);

        worker.tx_cmd.send(cmd).unwrap();
        
        last_received_text = match worker.rx_msg.recv().unwrap() {
            ThreadMessage::OutputReceived(txt) => txt,
            _ => panic!("Worker thread collapsed during string iterator navigation execution"),
        };

        // Intercept intermediate Pressure Plate bounces mid-route
        let step_events = analyse_room(&last_received_text);
        if step_events.iter().any(|e| matches!(e, RoomEvent::AlertTooHeavy | RoomEvent::AlertTooLight)) {
            let checkpoint_pos = find_feature_coordinates(map, Feature::Checkpoint);
            
            map.insert(target_pos, Room::new(Feature::PressurePlate, [false; 4]));
            map.set_explored(&target_pos); 
            dfs_target_stack.retain(|&x| x != target_pos);

            *plate_dir = dir_str;
            *pos = checkpoint_pos;
            *current_room_text = last_received_text.clone();
            bounced_early = true;
            break; 
        }

        current_step_pos = target_pos;
    }

    if !bounced_early {
        *pos = current_step_pos;
        *current_room_text = last_received_text;
    }
}

// Play mode 1: fully interactive
pub fn interactive(input: &Program) -> String {
    let mut vm = Machine::new(input);
    vm.run();
    loop {
        print!("{}", vm.drain_to_string());
        vm.take_manual_input();
        vm.resume();
    }
}

// Play mode 2: assisted (automatically solves the final puzzle)
pub fn assisted(input: &Program) -> String {
    let mut vm = Machine::new(input);
    let mut items: Vec<String> = Vec::with_capacity(8);
    let mut current_room_name = String::new();
    let mut plate_dir = "";

    vm.run();

    loop {
        let text = vm.drain_to_string();
        print!("{text}");
        let room_events = analyse_room(&text);

        for event in room_events {
            match event {
                RoomEvent::RoomName(name) => {
                    current_room_name = name;
                }
                RoomEvent::Door(direction) => {
                    if current_room_name == "Pressure-Sensitive Floor" {
                        plate_dir = rev_dir(&direction);
                    }
                }
                RoomEvent::ItemTaken(item) => {
                    items.push(item);
                }
                _ => {}
            }
        }

        if items.len() == 8 && !plate_dir.is_empty() && current_room_name == "Security Checkpoint" {
            break;
        }

        vm.take_manual_input();
        vm.resume();
    }

    // Solve puzzle automagically
    solve_puzzle(&mut vm, plate_dir, &items)
}

// Play mode 3: fully agentic
pub fn agentic(input: &Program) -> String {
    let mut map: DynaMap<Room> = DynaMap::new(); // Master map shared amongst agent spawns
    let mut pos = (0, 0);
    let mut room_events;
    let mut room_item;
    let mut items = Vec::with_capacity(8);
    let mut current_room_name = String::new();
    let mut current_doors;
    let mut dfs_target_stack = Vec::new();
    let mut plate_dir: &'static str = "";

    // Instantiate worker thread
    let mut worker = spawn_worker(input);
    
    // Capture and read the initial room baseline text
    let mut current_room_text = match worker.rx_msg.recv().unwrap() {
        ThreadMessage::OutputReceived(text) => text,
        _ => panic!("Failed to receive initial boot string"),
    };

    // New split loop strategy: 1st loop: explore, 2nd loop: test items
    // Phase 1: Explore and map
    loop {
        current_doors = [false; 4];
        room_item = None;
        room_events = analyse_room(&current_room_text);

        for event in room_events {
            match event {
                RoomEvent::RoomName(name) => current_room_name = name,
                RoomEvent::Door(direction) => match direction.as_str() {
                    "north" => current_doors[NORTH] = true,
                    "south" => current_doors[SOUTH] = true,
                    "east"  => current_doors[EAST] = true,
                    "west"  => current_doors[WEST] = true,
                    _ => {}
                },
                RoomEvent::ItemOnGround(item) => room_item = Some(item),
                _ => {}
            }
        }
        println!("Room name: {}, current doors [N,S,E,W]: {:?}", current_room_name, current_doors);

        // Map the current room features (strictly non-destructive)
        if !map.is_explored(pos) {
            let feature = match current_room_name.as_str() {
                "Security Checkpoint" => Feature::Checkpoint,
                "Pressure-Sensitive Floor" => Feature::PressurePlate,
                _ => if room_item.is_some() { Feature::SafeItem } else { Feature::Empty },
            };
            map.resolve_incomplete(pos, Room::new(feature, current_doors));
        }

        for i in 0..4 {
            if current_doors[i] {
                let neighbor_pos = move_to(pos, DIR_STRINGS[i]);
                println!("Pos: {:?}, Door: {:?}", pos, neighbor_pos);
                if !map.is_explored(neighbor_pos) {
                    if map.new_unexplored(neighbor_pos) {
                        dfs_target_stack.push(neighbor_pos);
                    }
                }
            }
        }

        while let Some(top_target) = dfs_target_stack.last() {
            if map.is_explored(*top_target) {
                dfs_target_stack.pop();
            } else {
                break;
            }
        }

        // Check global termination criteria using the stack status
        if dfs_target_stack.is_empty() {
            println!("All exploration is complete! {:?}", map.iter().collect::<Vec<_>>());
            break;
        }

        // Target the very last added unexplored position
        let current_target = *dfs_target_stack.last().unwrap();

        // Improved stack-allocated combined travel iterator
        if is_adjacent(pos, current_target) {
            // Short-circuit bypass: Create a stack-allocated iterator that fires only once
            let dir_str = step_to_str(pos, current_target);

            execute_travel_sequence(&mut worker, &mut map, &mut dfs_target_stack, &mut pos, &mut current_room_text, &mut plate_dir, std::iter::once(dir_str));
        } else {
            // Fallback heavy route: Compute the multi-step route iterator
            if let Some(path_iter) = path_to_target(&map, pos, current_target) {
                execute_travel_sequence(&mut worker, &mut map, &mut dfs_target_stack, &mut pos, &mut current_room_text, &mut plate_dir, path_iter);
            } else {
                panic!("Exploration stalled: Targeted coordinate {:?} is unreachable from {:?}", current_target, pos);
            }
        }
        println!("DFS queue: {:?}", dfs_target_stack);
    }
    println!("Phase 1 complete!");

    // Phase 2: Harvesting safe items, and finding the passcode
    loop {
        // Evaluate Terminal Automated Solver Hand-off Condition
        if items.len() == 8 && current_room_name == "Security Checkpoint" {
            println!("All items harvested safely. Reclaiming Intcode engine...");
            worker.tx_cmd.send("RELEASE_VM".to_string()).unwrap();
            
            match worker.rx_msg.recv().unwrap() {
                ThreadMessage::WorkerReleased(mut active_vm) => {
                    return solve_puzzle(&mut active_vm, &plate_dir, &items);
                }
                _ => panic!("Expected WorkerReleased signal but pipeline broke."),
            }
        }

        // Determine destination coordinates using current global grid map metrics
        let target_pos = if items.len() == 8 {
            find_feature_coordinates(&map, Feature::Checkpoint)
        } else {
            find_closest_feature(&map, pos, Feature::SafeItem)
        };

        let path = compute_safe_harvest_path(&map, pos, target_pos);

        let next_command = if path.is_empty() {
            // We have physically arrived in the target room
            room_events = analyse_room(&current_room_text);
            let item_name = extract_item_name(&room_events);
            
            items.push(item_name.clone());
            
            // Mark the item as cleared out from this node so we don't return
            if let Some(node) = map.get_mut(&pos) {
                node.feature = Feature::Empty;
            }
            format!("take {}\n", item_name)
        } else {
            // Step along shortest route path
            let next_dir_idx = path[0];
            pos = move_to(pos, DIR_STRINGS[next_dir_idx]);
            format!("{}\n", DIR_STRINGS[next_dir_idx])
        };

        worker.tx_cmd.send(next_command).unwrap();

        // Process telemetry streams using safety window thresholds
        match worker.rx_msg.recv_timeout(Duration::from_millis(100)) {
            Ok(ThreadMessage::OutputReceived(updated_text)) => {
                current_room_text = updated_text;
                // current_room_name = parse_current_room_name(&current_room_text);
            }
            Ok(ThreadMessage::GameCompleted(victory_text)) => {
                return victory_text; 
            }
            Ok(ThreadMessage::CrashDetected) => {
                // Aborted: caught by VM's inner signature tracking
                println!("Worker self-detected internal loop. Shutting down thread...");
                map.insert(pos, Room::new(Feature::BlacklistItem, current_doors));
                map.set_explored(&pos);

                // Reset everything and spin up a new worker thread
                pos = (0, 0);
                items.clear();
                current_room_name.clear();
                worker = spawn_worker(input);
                current_room_text = match worker.rx_msg.recv().unwrap() {
                    ThreadMessage::OutputReceived(txt) => txt,
                    _ => panic!("Thread reboot sequence failed"),
                };
            }
            Ok(ThreadMessage::WorkerReleased(_)) => {
                panic!("WorkerReleased unexpected encounter inside harvest telemetry loop.");
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Timeout: secondary safety valve if the thread somehow escapes a check
                println!("Hardware timeout breached. Killing and reallocating thread...");
                map.insert(pos, Room::new(Feature::BlacklistItem, current_doors));
                map.set_explored(&pos);

                // Reset everything and spin up a new worker thread
                pos = (0, 0);
                items.clear();
                current_room_name.clear();
                worker = spawn_worker(input);
                current_room_text = match worker.rx_msg.recv().unwrap() {
                    ThreadMessage::OutputReceived(txt) => txt,
                    _ => panic!("Thread reboot sequence failed"),
                };
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => panic!("Communication pipe broken."),
        }
    }
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Program {
    Machine::parse(input)
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &Program) -> String {
    // ### Play the game interactively ###
    // interactive(input)

    // ### Play the game with assistance solving the final puzzle ###
    assisted(input)

    // ### Play the game agentically ###
    // agentic(input)
}