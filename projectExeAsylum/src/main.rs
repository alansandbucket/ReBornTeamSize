use std::io::{Stdout, Stdin, self, Write};
use std::process::Command;
use std::time::Duration;
use sha256::digest;
use std::fs::{File, self, OpenOptions, remove_file};
use std::io::Read;
use std::path::Path;
use dll_syringe::{Syringe, process::OwnedProcess};
use std::collections::HashMap;
use std::process;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
	// lol
	unsafe{
		let mut stdout: Stdout = io::stdout();
		let stdin: Stdin = io::stdin();
		let mut playPVP: String = "no".to_string();
		let mut numBotsA: String = "".to_string();
		let mut numBotsB: String = "".to_string();
	
		println!("Welcome to the ReBorn CLI launcher! Now with Multiplayer Maps!");
		if(locate_battleborn()){
			println!("Type the number of the PvE mission you want to play:");
			
			println!("1) Prologue - Story Mission 0");
			println!("2) The Algorithm - Story Mission 1");
			println!("3) Void's Edge - Story Mission 2");
			println!("4) The Renegade - Story Mission 3");
			println!("5) The Archive - Story Mission 4");
			println!("6) Sentinel - Story Mission 5");
			println!("7) The Experiment - Story Mission 6");
			println!("8) The Saboteur - Story Mission 7");
			println!("9) Heliophage - Story Mission 8");
			println!("10) Attikus and the Thrall Rebellion - Operation 1");
			println!("11) Toby's Friendship Raid - Operation 2");
			println!("12) Oscar Mike vs the Battleschool - Operation 3");
			println!("13) Montana and the Demon Bear - Operation 4");
			println!("14) Phoebe and the Heart of Ekkunar - Operation 5");
			println!();
			
			println!("Or enter the number of the PvP map you'd like to play vs Bots");
			println!("15) Aerie - Rocket Brawl");
			println!("16) Coldsnap - Meltdown");			
			println!("17) Echelon - Incursion");
			println!("18) Horizon - Supercharge");
			println!("19) Monuments - Incursion");
			println!("20) Permafrost - Supercharge");
			println!("21) Outback - Capture");
			println!("22) Overgrowth - Incursion");
			println!("23) Smelter - Tank Yankers");
			println!("24) Snowblind - Capture");
			println!("25) Snowblind - Face-Off");
			println!("26) Temples - Capture");
			println!("27) Temples - Face-Off");
			println!("28) Ziggurat - Supercharge");
			println!();
			
			println!("Or enter the number of one of the following training modes:");
			println!("29) Versus Training");
			println!("30) Dojo");
			

	
			stdout.flush();
			let mut map_selection: String = "".to_string();
			stdin.read_line(&mut map_selection);

			if (map_selection.trim().parse::<i32>().unwrap() >= 15 && map_selection.trim().parse::<i32>().unwrap() <= 28)
			{
				playPVP = "yes".to_string();
				println!("Type the number of bots you want on your team (0-4):");
				stdout.flush();
				stdin.read_line(&mut numBotsA);
				println!("Type the number of bots you want on your opponents team (0-5):");
				stdout.flush();
				stdin.read_line(&mut numBotsB);
				println!("A = {} B = {}", numBotsA.as_str(), numBotsB.as_str());
			}
			
			let mut character_selection: String = "".to_string();
			if (map_selection.trim().parse::<i32>().unwrap() < 15 || map_selection.trim().parse::<i32>().unwrap() > 29)
			{	
				println!("Type the number of the character you want to play:");
				
				println!("1) Alani");
				println!("2) Ambra");
				println!("3) Attikus");
				println!("4) Beatrix");
				println!("5) Benedict");
				println!("6) Boldur");
				println!("7) Caldarius");
				println!("8) Deande");
				println!("9) El Dragon");
				println!("10) Ernest");
				println!("11) Galilea");
				println!("12) Ghalt");
				println!("13) ISIC");
				println!("14) Kelvin");
				println!("15) Kid Ultra");
				println!("16) Kleese");
				println!("17) Marquis");
				println!("18) Mellka");
				println!("19) Miko");
				println!("20) Montana");
				println!("21) Orendi");
				println!("22) Oscar Mike");
				println!("23) Pendles");
				println!("24) Phoebe");
				println!("25) Rath");
				println!("26) Reyna");
				println!("27) Shayne & Aurox");
				println!("28) Thorn");
				println!("29) Toby");
				println!("30) Whiskey Foxtrot");
		
				stdout.flush();
				
				stdin.read_line(&mut character_selection);
			}
			else
			{
					character_selection = "22".to_string();
			}
			
			write_config_file("110.0".to_string(), "60.0".to_string(), "60.0".to_string(), "false".to_string(), mapMapSelectionToMap(map_selection.trim().to_string()), mapCharSelectionToChar(character_selection.trim().to_string()), numBotsA.trim().to_string(), numBotsB.trim().to_string(), playPVP);
	
			println!("Launching BattleBorn...");
	
			launch_battleborn();
	
			println!("Waiting for the game to launch...");
	
			std::thread::sleep(Duration::from_secs(7));
	
			println!("Injecting ReBorn...");
	
			inject_reborn();
		}
		else{
			println!("Failed to find your installation of Battleborn: try placing this exe into the root of your Battleborn directory.");
			stdout.flush();
			let mut useless_string = "".to_string();
			stdin.read_line(&mut useless_string);
			return;
		}
    }
}

fn mapMapSelectionToMap(map: String) -> String
{
	let map_ipc_dict: HashMap<&str, &str> = HashMap::from([
        ("1", "PvE_Prologue_P"),
        ("2", "Caverns_P"),
		("3", "Portal_P"),
		("4", "Captains_P"),
		("5", "Evacuation_P"),
		("6", "Ruins_P"),
		("7", "Observatory_p"),
		("8", "Refinery_P"),
		("9", "Cathedral_P"),
		("10", "Slums_P"),
		("11", "Toby_Raid_P"),
		("12", "CullingFacility_P"),
		("13", "TallTales_P"),
		("14", "Heart_Ekkunar_P"),
		("15", "BENEDICT_Void_P"),
		("16", "IceScort_P"),
		("17", "Viaduct_P"),
		("18", "Ripple_P"),
		("19", "Snowdrift_P"),
		("20", "Cascade_P"),
		("21", "Ravine_P"),
		("22", "Inc_Stronghold2_P"),
		("23", "Ghalt_ChainChainChain_P"),
		("24", "Snowblind_P"),
		("25", "Snowblind_Headhunter_P"),
		("26", "BlissRuins_P"),
		("27", "BlissRuins_Headhunter_P"),
		("28", "Wishbone_P"),
		("29", "IncTut_Freeze_P"),
		("30", "Dojo_P"),
    ]);
	
	return map_ipc_dict[map.as_str()].to_string();
}

fn mapCharSelectionToChar(map: String) -> String
{
	// This is assuredly overdesigned but nothing in this code is to be taken seriously, judge me not
	let character_ipc_dict: HashMap<&str, &str> = HashMap::from([
        ("1", "WaterMonk"),
        ("2", "SunPriestess"),
		("3", "SoulCollector"),
		("4", "PlagueBringer"),
		("5", "RocketHawk"),
		("6", "DwarvenWarrior"),
		("7", "AssaultJump"),
		("8", "DarkAssassin"),
		("9", "LeapingLuchador"),
		("10", "Bombirdier"),
		("11", "Blackguard"),
		("12", "PapaShotgun"),
		("13", "SpiritMech"),
		("14", "IceGolem"),
		("15", "SideKick"),
		("16", "TacticalBuilder"),
		("17", "GentSniper"),
		("18", "MutantFist"),
		("19", "TribalHealer"),
		("20", "MachineGunner"),
		("21", "ChaosMage"),
		("22", "ModernSoldier"),
		("23", "CornerSneaker"),
		("24", "MageBlade"),
		("25", "DeathBlade"),
		("26", "RogueCommander"),
		("27", "BoyAndDjinn"),
		("28", "DarkElf"),
		("29", "PenguinMech"),
		("30", "RogueSoldier"),
    ]);
	
	return character_ipc_dict[map.as_str()].to_string();
}

fn inject_reborn(){
    let battleborn = OwnedProcess::find_first_by_name("Battleborn.exe").unwrap();
    let syringe = Syringe::for_process(battleborn);

    let payload = syringe.inject("ReBorn.dll").unwrap();
}

fn launch_battleborn(){
    if(Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe").exists()){
        Command::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe").spawn();
    }
    else if(Path::new("Binaries\\Win64\\Battleborn.exe").exists()){
        Command::new("Binaries\\Win64\\Battleborn.exe").spawn();
    }
	else if(Path::new("Battleborn.exe").exists()){
        Command::new("Battleborn.exe").spawn();
    }
}

#[derive(serde::Serialize, Clone)]
struct Config{
    FOV: String,
    MouseSensitivityX: String,
    MouseSensitivityY: String,
    subtitles: String,
    mapToLoad: String,
    characterToLoad: String,
	teamSizeA: String,
	teamSizeB: String,
	inPVP: String
}

fn write_config_file(fov: String, mouseX: String, mouseY: String, subs: String, map: String, character: String, teamSizeA: String, teamSizeB: String, inPVP: String){
	
	let config = Config {
		FOV: fov,
		MouseSensitivityX: mouseX,
		MouseSensitivityY: mouseY,
		subtitles: subs,
		mapToLoad: map,
		characterToLoad: character,
		teamSizeA: teamSizeA,
		teamSizeB: teamSizeB,
		inPVP: inPVP
	};
	
    if(Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe").exists()){
        if(Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\config.json").exists()){
            remove_file("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\config.json");
        }

        let mut config_file: File = File::create("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\config.json").unwrap();
        config_file.write(serde_json::to_string(&config).unwrap().as_bytes());
    }
    else if(Path::new("Binaries\\Win64\\Battleborn.exe").exists()){
        if(Path::new("Binaries\\Win64\\config.json").exists()){
            remove_file("Binaries\\Win64\\config.json");
        }

        let mut config_file: File = File::create("Binaries\\Win64\\config.json").unwrap();
        config_file.write(serde_json::to_string(&config).unwrap().as_bytes());
    }
	else if(Path::new("Battleborn.exe").exists()){
		if(Path::new("config.json").exists()){
            remove_file("config.json");
        }
		let mut config_file: File = File::create("config.txt").unwrap();
        config_file.write(serde_json::to_string(&config).unwrap().as_bytes());
	}
}

unsafe fn locate_battleborn() -> bool{
    if(Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Battleborn\\Binaries\\Win64\\Battleborn.exe").exists()){
        return true;
    }
    else if(Path::new("Binaries\\Win64\\Battleborn.exe").exists()){
        return true;
    }
	else if(Path::new("Battleborn.exe").exists()){
        return true;
    }
	return false;
}