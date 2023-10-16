function submit() {
  //var submitter = document.getElementById("submitter").value;
  //var team = document.getElementById("team").value;
  // var obj = {
  // submitter: submitter,
  // team: team,
  // };
  var obj = {
    submitter: "Chloe",
    team: 2240,
    match_number: 1,
    auto_data: {
      cubes: {
        low: 1,
        mid: 0,
        high: 0,
      },
      cones: {
        low: 0,
        mid: 0,
        high: 0,
      },
    },
    left_com: true,
    auto_charge_station: "Failed",
    teleop_data: {
      cubes: {
        low: 8,
        mid: 0,
        high: 0,
      },
      cones: {
        low: 1,
        mid: 0,
        high: 0,
      },
    },
    cones_missed: 2,
    cubes_missed: 1,
    teleop_charge_station: "DockedAndBalanced",
    charge_station_time: 15,
    defense: "DidNotAttempt",
    fouls_caused: "N/A",
    fouls_commited: 0,
    notes: "N/A",
  };

  console.log(obj);
  fetch("http://127.0.0.1:8080/v1/form", {
    method: "POST",
    body: obj,
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    },
  });
}

submit();
