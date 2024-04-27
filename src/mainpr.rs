use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct TeamStat {
    game: String,
}

#[derive(Debug)]
struct DefenseStatsWeight {
    Feature: String,
    Average_Weight: f64,
}

#[derive(Debug)]
struct PlayerDefenseStats {
    player: String,
    tkl: u32,
    tklw: u32,
    def3rd: u32,
    mid3rd: u32,
    att3rd: u32,
    att: u32,
    lost: u32,
    blocks: u32,
    sh: u32,
    pass: u32,
}

#[derive(Debug)]
struct PlayerAttackingStats {
    player: String,
    gls: i64,
    ast: i64,
    pk: i64,
    pkatt: i64,
    sh: i64,
    sot: i64,
    crdy: i64,
    crdr: i64,
    touches: i64,
    tkl: i64,
    int_: i64,
    blocks: i64,
    xg: f64,
    npxg: f64,
    xag: f64,
    sca: i64,
    gca: i64,
    cmp: i64,
    att: i64,
    prgp: i64,
    carries: i64,
    prgc: i64,
    succ: i64,
}

#[derive(Debug)]
struct PlayerPassingStats {
    player: String,
    cmp: i64,
    att: i64,
    totdist: i64,
    prgdist: i64,
}

impl PlayerDefenseStats {
    // 计算球员加权综合得分
    fn calculate_score(&self, weights: &HashMap<String, f64>) -> f64 {
        let mut score = 0.0;
        // 使用循环遍历所有属性，并根据权重计算加权得分
        for (key, value) in weights.iter() {
            match key.as_str() {
                "tkl" => score += (self.tkl as f64) * value,
                "tklw" => score += (self.tklw as f64) * value,
                "def3rd" => score += (self.def3rd as f64) * value,
                "mid3rd" => score += (self.mid3rd as f64) * value,
                "att3rd" => score += (self.att3rd as f64) * value,
                "att" => score += (self.att as f64) * value,
                "lost" => score += (self.lost as f64) * value,
                "blocks" => score += (self.blocks as f64) * value,
                "sh" => score += (self.sh as f64) * value,
                "pass" => score += (self.pass as f64) * value,
                _ => (),
            }
        }
        score
    }
}


impl PlayerAttackingStats {
    // 计算球员加权综合得分
    fn calculate_score(&self, weights: &HashMap<String, f64>) -> f64 {
        let mut score = 0.0;
        // 使用循环遍历所有属性，并根据权重计算加权得分
        for (key, value) in weights.iter() {
            match key.as_str() {
                "gls" => score += (self.gls as f64) * value,
                "ast" => score += (self.ast as f64) * value,
                "pk" => score += (self.pk as f64) * value,
                "pkatt" => score += (self.pkatt as f64) * value,
                "sh" => score += (self.sh as f64) * value,
                "sot" => score += (self.sot as f64) * value,
                "crdy" => score += (self.crdy as f64) * value,
                "crdr" => score += (self.crdr as f64) * value,
                "touches" => score += (self.touches as f64) * value,
                "tkl" => score += (self.tkl as f64) * value,
                "int_" => score += (self.int_ as f64) * value,
                "blocks" => score += (self.blocks as f64) * value,
                "xg" => score += self.xg * value,
                "npxg" => score += self.npxg * value,
                "xag" => score += self.xag * value,
                "sca" => score += (self.sca as f64) * value,
                "gca" => score += (self.gca as f64) * value,
                "cmp" => score += (self.cmp as f64) * value,
                "att" => score += (self.att as f64) * value,
                "prgp" => score += (self.prgp as f64) * value,
                "carries" => score += (self.carries as f64) * value,
                "prgc" => score += (self.prgc as f64) * value,
                "succ" => score += (self.succ as f64) * value,
                _ => (),
            }
        }
        score
    }
}


impl PlayerPassingStats {
    // 计算球员加权综合得分
    fn calculate_score(&self, weights: &HashMap<String, f64>) -> f64 {
        let mut score = 0.0;
        // 使用循环遍历所有属性，并根据权重计算加权得分
        for (key, value) in weights.iter() {
            match key.as_str() {
                "cmp" => score += (self.cmp as f64) * value,
                "att" => score += (self.att as f64) * value,
                "totdist" => score += (self.totdist as f64) * value,
                "prgdist" => score += (self.prgdist as f64) * value,
                _ => (),
            }
        }
        score
    }
}



fn read_player_defense_stats(file_path: &str) -> Result<Vec<PlayerDefenseStats>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut player_stats = Vec::new();

    // 跳过标题行
    let mut lines = reader.lines();
    if let Some(Ok(_)) = lines.next() {
        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();

            // 解析每行数据
            let player = parts[1].to_string();
            let tkl: u32 = parts[2].parse()?;
            let tklw: u32 = parts[3].parse()?;
            let def3rd: u32 = parts[4].parse()?;
            let mid3rd: u32 = parts[5].parse()?;
            let att3rd: u32 = parts[6].parse()?;
            let att: u32 = parts[7].parse()?;
            let lost: u32 = parts[8].parse()?;
            let blocks: u32 = parts[9].parse()?;
            let sh: u32 = parts[10].parse()?;
            let pass: u32 = parts[11].parse()?;

            let player_stat = PlayerDefenseStats {
                player,
                tkl,
                tklw,
                def3rd,
                mid3rd,
                att3rd,
                att,
                lost,
                blocks,
                sh,
                pass,
            };
            player_stats.push(player_stat);
        }
    }

    Ok(player_stats)
}


fn read_player_attack_stats(file_path: &str) -> Result<Vec<PlayerAttackingStats>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut player_stats = Vec::new();

    // 跳过标题行
    let mut lines = reader.lines();
    if let Some(Ok(_)) = lines.next() {
        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();

            // 解析每行数据
            let player = parts[1].to_string();
            let gls: i64 = parts[2].parse()?;
            let ast: i64 = parts[3].parse()?;
            let pk: i64 = parts[4].parse()?;
            let pkatt: i64 = parts[5].parse()?;
            let sh: i64 = parts[6].parse()?;
            let sot: i64 = parts[7].parse()?;
            let crdy: i64 = parts[8].parse()?;
            let crdr: i64 = parts[9].parse()?;
            let touches: i64 = parts[10].parse()?;
            let tkl: i64 = parts[11].parse()?;
            let int_: i64 = parts[12].parse()?;
            let blocks: i64 = parts[13].parse()?;
            let xg: f64 = parts[14].parse()?;
            let npxg: f64 = parts[15].parse()?;
            let xag: f64 = parts[16].parse()?;
            let sca: i64 = parts[17].parse()?;
            let gca: i64 = parts[18].parse()?;
            let cmp: i64 = parts[19].parse()?;
            let att: i64 = parts[20].parse()?;
            let prgp: i64 = parts[21].parse()?;
            let carries: i64 = parts[22].parse()?;
            let prgc: i64 = parts[23].parse()?;
            let succ: i64 = parts[24].parse()?;

            let player_stat = PlayerAttackingStats {
                player,
                gls,
                ast,
                pk,
                pkatt,
                sh,
                sot,
                crdy,
                crdr,
                touches,
                tkl,
                int_,
                blocks,
                xg,
                npxg,
                xag,
                sca,
                gca,
                cmp,
                att,
                prgp,
                carries,
                prgc,
                succ,
            };
            player_stats.push(player_stat);
        }
    }

    Ok(player_stats)
}

fn read_player_passing_stats(file_path: &str) -> Result<Vec<PlayerPassingStats>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut player_stats = Vec::new();

    // 跳过标题行
    let mut lines = reader.lines();
    if let Some(Ok(_)) = lines.next() {
        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();

            // 解析每行数据
            let player = parts[1].to_string();
            let cmp: i64 = parts[2].parse()?;
            let att: i64 = parts[3].parse()?;
            let totdist: i64 = parts[4].parse()?;
            let prgdist: i64 = parts[5].parse()?;

            let player_stat = PlayerPassingStats {
                player,
                cmp,
                att,
                totdist,
                prgdist,
            };
            player_stats.push(player_stat);
        }
    }

    Ok(player_stats)
}



// 读取权重文件并解析为 HashMap
fn read_defense_weights(file_path: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut weights = HashMap::new();

    // 跳过标题行
    let mut lines = reader.lines();
    if let Some(Ok(_)) = lines.next() {
        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();

            // 解析每行数据
            let feature = parts[0].to_string();
            let weight: f64 = parts[1].parse()?;

            weights.insert(feature, weight);
        }
    }

    Ok(weights)
}

fn read_attack_weights(file_path: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut weights = HashMap::new();

    // 跳过标题行
    let mut lines = reader.lines();
    if let Some(Ok(_)) = lines.next() {
        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();

            // 解析每行数据
            let feature = parts[0].to_string();
            let weight: f64 = parts[1].parse()?;

            weights.insert(feature, weight);
        }
    }

    Ok(weights)
}

fn read_passing_weights(file_path: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut weights = HashMap::new();

    // 跳过标题行
    let mut lines = reader.lines();
    if let Some(Ok(_)) = lines.next() {
        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();

            // 解析每行数据
            let feature = parts[0].to_string();
            let weight: f64 = parts[1].parse()?;

            weights.insert(feature, weight);
        }
    }

    Ok(weights)
}


fn read_team_stats(file_path: &str) -> Result<Vec<TeamStat>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut team_stats = Vec::new();

    // 跳过标题行
    let mut lines = reader.lines();
    if let Some(Ok(_)) = lines.next() {
        for line in lines {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();

            // 解析每行数据
            let game = parts[3].to_string();

            let team_stat = TeamStat { game };
            team_stats.push(team_stat);
        }
    }

    Ok(team_stats)
}



fn main() {
    let team_stats_file_path = "/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/team_stat.csv"; 
    let defense_weights_file_path = "/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/defense_statsweight.csv";
    let attack_weights_file_path = "/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/attacking_statsweight.csv"; 
    let pass_weights_file_path = "/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/passing_statsweight.csv"; 
    

    // 读取比赛统计信息
    let team_stats = match read_team_stats(team_stats_file_path) {
        Ok(stats) => stats,
        Err(err) => {
            eprintln!("Error reading team stats: {}", err);
            return;
        }
    };

    // 读取权重信息
    let defens_weights = match read_defense_weights(defense_weights_file_path) {
        Ok(defens_weights) => defens_weights,
        Err(err) => {
            eprintln!("Error reading defens_weights: {}", err);
            return;
        }
    };

    // 使用哈希表记录每个球员的得分总和和出现次数
    let mut defense_player_scores: HashMap<String, (f64, usize)> = HashMap::new();

    // 遍历每场比赛的球员数据
    for team_stat in &team_stats {
        let defense_stats_file_path = format!("/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/data/{}/defense_stats.csv", team_stat.game);
        match read_player_defense_stats(&defense_stats_file_path) {
            Ok(player_stats) => {
                for player_stat in &player_stats {
                    let score = player_stat.calculate_score(&defens_weights);
                    let (total_score, count) = defense_player_scores.entry(player_stat.player.clone()).or_insert((0.0, 0));
                    *total_score += score;
                    *count += 1;
                }
            }
            Err(err) => eprintln!("Error reading defense stats for game {}: {}", team_stat.game, err),
        }
    }


    let attack_weights = match read_attack_weights(attack_weights_file_path) {
        Ok(attack_weights) => attack_weights,
        Err(err) => {
            eprintln!("Error reading attack_weights: {}", err);
            return;
        }
    };

    // 使用哈希表记录每个球员的得分总和和出现次数
    let mut attack_player_scores: HashMap<String, (f64, usize)> = HashMap::new();

    // 遍历每场比赛的球员数据
    for team_stat in &team_stats {
        let attack_stats_file_path = format!("/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/data/{}/attacking_stats.csv", team_stat.game);
        match read_player_attack_stats(&attack_stats_file_path) {
            Ok(player_stats) => {
                for player_stat in &player_stats {
                    let score = player_stat.calculate_score(&attack_weights);
                    let (total_score, count) = attack_player_scores.entry(player_stat.player.clone()).or_insert((0.0, 0));
                    *total_score += score;
                    *count += 1;
                }
            }
            Err(err) => eprintln!("Error reading attack stats for game {}: {}", team_stat.game, err),
        }
    }


    // 读取权重信息
    let pass_weights = match read_passing_weights(pass_weights_file_path) {
        Ok(pass_weights) => pass_weights,
        Err(err) => {
            eprintln!("Error reading pass_weights: {}", err);
            return;
        }
    };

    // 使用哈希表记录每个球员的得分总和和出现次数
    let mut pass_player_scores: HashMap<String, (f64, usize)> = HashMap::new();

    // 遍历每场比赛的球员数据
    for team_stat in &team_stats {
        let pass_stats_file_path = format!("/Users/andyfei/Desktop/学习/BU/DS 210/finalproject/data/{}/passing_stats.csv", team_stat.game);
        match read_player_passing_stats(&pass_stats_file_path) {
            Ok(player_stats) => {
                for player_stat in &player_stats {
                    let score = player_stat.calculate_score(&pass_weights);
                    let (total_score, count) = pass_player_scores.entry(player_stat.player.clone()).or_insert((0.0, 0));
                    *total_score += score;
                    *count += 1;
                }
            }
            Err(err) => eprintln!("Error reading passs stats for game {}: {}", team_stat.game, err),
        }
    }

    let mut sorted_defense_scores: Vec<_> = defense_player_scores.iter().collect();
    sorted_defense_scores.sort_by(|a, b| b.1 .0.partial_cmp(&a.1 .0).unwrap());
    let top_defense_scores: Vec<_> = sorted_defense_scores.iter().take(4).collect();

    let mut sorted_pass_scores: Vec<_> = pass_player_scores.iter().collect();
    sorted_pass_scores.sort_by(|a, b| b.1 .0.partial_cmp(&a.1 .0).unwrap());
    let top_pass_scores: Vec<_> = sorted_pass_scores.iter().take(3).collect();

    let mut sorted_attack_scores: Vec<_> = attack_player_scores.iter().collect();
    sorted_attack_scores.sort_by(|a, b| b.1 .0.partial_cmp(&a.1 .0).unwrap());
    let top_attack_scores: Vec<_> = sorted_attack_scores.iter().take(3).collect();

    
    print!("def:");
    for (player, (score, _)) in &top_defense_scores {
        print!("{}, ", player);
    }
    println!("");

    print!("pass:");
    for (player, (score, _)) in &top_pass_scores {
        print!("{}, ", player);
    }
    println!("");

    print!("att:");
    for (player, (score, _)) in &top_attack_scores {
        print!("{}, ", player);
    }
    println!("");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_score() {
        let weights: HashMap<String, f64> = [("tkl".to_string(), 1.0), ("att".to_string(), 2.0)]
            .iter().cloned().collect();
        
        let player_stat = PlayerDefenseStats {
            player: "Test Player".to_string(),
            tkl: 10,
            tklw: 0,
            def3rd: 0,
            mid3rd: 0,
            att3rd: 0,
            att: 20,
            lost: 0,
            blocks: 0,
            sh: 0,
            pass: 0,
        };

        let score = player_stat.calculate_score(&weights);

        assert_eq!(score, 50.0);
    }
}
