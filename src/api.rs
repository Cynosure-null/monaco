mod apiclient {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Debug, Serialize, Deserialize)]
    struct MatchInfo {
        key: String,
        comp_level: String,
        set_number: i32,
        match_number: i32,
        alliances: Alliances,
        winning_alliance: String,
        event_key: String,
        time: i32,
        actual_time: i32,
        predicted_time: i32,
        post_result_time: i32,
        score_breakdown: ScoreBreakdown, // This can be a custom struct if you know the structure
        videos: Vec<Video>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Alliances {
        red: Alliance,
        blue: Alliance,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Alliance {
        score: i32,
        team_keys: Vec<String>,
        surrogate_team_keys: Vec<String>,
        dq_team_keys: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Video {
        #[serde(rename = "type")]
        video_type: String,
        key: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct ScoreBreakdown {
        blue: AllianceScore,
        red: AllianceScore,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct AllianceScore {
        activationBonusAchieved: bool,
        autoBridgeState: String,
        autoChargeStationRobot1: String,
        autoChargeStationRobot2: String,
        autoChargeStationRobot3: String,
        autoCommunity: HashMap<String, Vec<String>>,
        autoGamePieceCount: i32,
        autoGamePiecePoints: i32,
        autoMobilityPoints: i32,
        autoPoints: i32,
        coopertitionCriteriaMet: bool,
        endGameBridgeState: String,
        endGameChargeStationRobot1: String,
        endGameChargeStationRobot2: String,
        endGameChargeStationRobot3: String,
        extraGamePieceCount: i32,
        foulCount: i32,
        foulPoints: i32,
        linkPoints: i32,
        links: Vec<Link>,
        mobilityRobot1: String,
        mobilityRobot2: String,
        mobilityRobot3: String,
        rp: i32,
        sustainabilityBonusAchieved: bool,
        techFoulCount: i32,
        teleopCommunity: HashMap<String, Vec<String>>,
        teleopGamePieceCount: i32,
        teleopGamePiecePoints: i32,
        teleopPoints: i32,
        totalPoints: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Link {
        nodes: Vec<i32>,
        row: String,
    }

    pub async fn get_next_match(team: String) -> Result<i32, reqwest::Error> {
        //GET https://www.thebluealliance.com/api/v3/team/frc2240/event/2023cokc/matches
        //Then map data to a vector of match structs
        // Then, give the largest match number (qm_x) where red score or blue score is -1

        let req = reqwest::get(format!(
            "https://www.thebluealliance.com/api/v3/team/frc254/event/{}/matches",
            team
        )).await;

        match req {
            Ok(v) => {
                println!("Got match data");
                match v.text().await {
                    Ok(v) => {},
                    Err(e) => {}
                }
            },
            Err(e) => {
                println!("Error: could not get match data, {}",e);
            },
        }

        return Result::Ok(10);
    }

}
