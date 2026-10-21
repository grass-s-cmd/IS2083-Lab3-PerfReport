// ============================================================================
//  BOILERPLATE : PerfReport (Rust)  |  IS2083 Lab 3
// ============================================================================
//  This boilerplate already compiles and runs. It prints the banner, computes
//  a score from stub numbers, and prints it. Your job is to replace each TODO
//  with a real reading from the sysinfo crate, following the handout, one at a
//  time. Run `cargo run` after each change to be sure it still builds.
//
//  The scoring logic and helpers are done for you. You mostly add println! and
//  set the mem_load and disk_used_pct variables from real data.
// ============================================================================

use sysinfo::{Disks, System};

const GB: f64 = 1024.0 * 1024.0 * 1024.0;

// --- Given: findings + scoring (do not change) ------------------------------
struct Findings {
    mem_load: u64,
    disk_used_pct: f64,
}

fn compute_score(f: &Findings) -> i32 {
    let mut score: i32 = 100;
    if f.mem_load > 85 {
        score -= 25;
    } else if f.mem_load > 70 {
        score -= 10;
    }
    if f.disk_used_pct > 90.0 {
        score -= 25;
    } else if f.disk_used_pct > 75.0 {
        score -= 10;
    }
    if score < 0 {
        score = 0;
    }
    score
}

fn rating(score: i32) -> &'static str {
    if score >= 85 {
        "GOOD"
    } else if score >= 60 {
        "FAIR"
    } else {
        "POOR"
    }
}

fn main() {
    println!("==== PerfReport (Rust) ====");

    // This line makes the sysinfo tools available. You will read from `sys`.
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpus = sys.cpus().len();
    println!("Logical processors : {}", cpus);

    let total = sys.total_memory();
    let used = sys.used_memory();
    let mem_load = if total > 0 {
        (used as f64 / total as f64 * 100.0) as u64
    } else {
        0
    };
    println!("Memory total (GB) : {:.2}", total as f64 / GB);
    println!("Memory free (GB)  : {:.2}", total.saturating_sub(used) as f64 / GB);
    println!("Memory in use (%) : {}", mem_load);

    println!("Uptime (minutes)  : {}", System::uptime() / 60);

    let disks = Disks::new_with_refreshed_list();
    let mut disk_used_pct: f64 = 0.0;
    if let Some(disk) = disks.list().first() {
        let dtotal = disk.total_space();
        let dfree = disk.available_space();
        if dtotal > 0 {
            disk_used_pct = 100.0 * (dtotal.saturating_sub(dfree)) as f64 / dtotal as f64;
        }
        println!("Disk total (GB)   : {:.2}", dtotal as f64 / GB);
        println!("Disk free (GB)    : {:.2}", dfree as f64 / GB);
        println!("Disk used (%)     : {:.2}", disk_used_pct);
    }

    // Section 5: score (provided scoring rules)
    let findings = Findings {
        mem_load,
        disk_used_pct,
    };
    let score = compute_score(&findings);
    println!(
        "Efficiency score   : {} / 100 ({})",
        score,
        rating(score)
    );
    println!("Done.");
}

#[cfg(test)]
mod tests {
    use super::{compute_score, rating, Findings};

    #[test]
    fn healthy_system_is_good() {
        assert_eq!(compute_score(&Findings { mem_load: 50, disk_used_pct: 40.0 }), 100);
        assert_eq!(rating(100), "GOOD");
    }

    #[test]
    fn high_usage_lowers_score() {
        assert_eq!(compute_score(&Findings { mem_load: 80, disk_used_pct: 80.0 }), 80);
        assert_eq!(rating(80), "FAIR");
    }

    #[test]
    fn very_high_usage_is_poor() {
        assert_eq!(compute_score(&Findings { mem_load: 95, disk_used_pct: 95.0 }), 50);
        assert_eq!(rating(50), "POOR");
    }
}
