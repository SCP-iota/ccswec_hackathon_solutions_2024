pub fn climbingLeaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    player.iter().map(|score| find_place(ranked, *score)).collect()
}

fn find_place(ranked: &[i32], score: i32) -> i32 {
    let mut place = 1;
    let mut last_score = ranked[0];

    for i in 0..ranked.len() {
        if ranked[i] != last_score {
            place += 1;
            last_score = ranked[i];
        }

        if ranked[i] <= score {
            return place;
        }
    }

    if score < last_score {
        place + 1
    } else {
        place
    }
}