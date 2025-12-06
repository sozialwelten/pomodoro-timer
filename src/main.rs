use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🍅 Pomodoro Timer");
    println!("=================\n");

    loop {
        println!("Wähle eine Option:");
        println!("1. Work Session (25 Min)");
        println!("2. Short Break (5 Min)");
        println!("3. Long Break (15 Min)");
        println!("4. Custom Timer");
        println!("5. Beenden");
        print!("\nDeine Wahl: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => start_timer(25, "Work Session"),
            "2" => start_timer(5, "Short Break"),
            "3" => start_timer(15, "Long Break"),
            "4" => {
                print!("Minuten eingeben: ");
                io::stdout().flush().unwrap();
                let mut custom = String::new();
                io::stdin().read_line(&mut custom).unwrap();
                if let Ok(mins) = custom.trim().parse::<u64>() {
                    start_timer(mins, "Custom Timer");
                } else {
                    println!("❌ Ungültige Eingabe!\n");
                }
            }
            "5" => {
                println!("👋 Tschüss!");
                break;
            }
            _ => println!("❌ Ungültige Wahl!\n"),
        }
    }
}

fn start_timer(minutes: u64, session_type: &str) {
    println!("\n⏰ {} startet: {} Minuten", session_type, minutes);
    println!("Drücke Ctrl+C zum Abbrechen\n");

    let total_seconds = minutes * 60;

    for remaining in (1..=total_seconds).rev() {
        let mins = remaining / 60;
        let secs = remaining % 60;

        print!("\r⏱️  {:02}:{:02} verbleibend", mins, secs);
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));
    }

    println!("\n\n✅ {} abgeschlossen!", session_type);
    println!("🔔 Zeit für eine Pause!\n");

    // Einfacher "Alarm" - mehrfach piepen
    for _ in 0..3 {
        print!("\x07"); // Bell character
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(500));
    }

    println!();
}
