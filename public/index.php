<?php
include_once "../holidays.php";

$time = time();
$utc = intval($_GET["utc"] ?? "");

if (-12 <= $utc && $utc <= 12) {
    $time += 60 * 60 * $utc;
} else {
    $utc = 0;
}

$month = intval($_GET["month"] ?? date("n", $time));
$day = intval($_GET["day"] ?? date("d", $time));

$search = $_GET["search"] ?? "";
$search_regex = "/(?i)" . preg_quote($search, "/") . "/";

$search = htmlspecialchars($search);

$holidays = [];

foreach (HOLIDAYS[0] as $holiday) {
    if ($search != "") {
        if (preg_match($search_regex, $holiday["name"])) {
            array_push($holidays, $holiday);
        }
        continue;
    }

    if ($holiday["date"][0] == $month && $holiday["date"][1] == $day) {
        array_push($holidays, $holiday);
    }
}

if (CLIENT_REQUIRES_JSON) {
    header("Content-Type: application/json");
    echo json_encode($holidays);
    exit;
}

$holiday_count = count($holidays);
?>

<html>

<head>
    <title><?php
    if ($search == "") {
        echo "$holiday_count holidays on $day.$month" . ($utc != 0 ? (" (UTC" . ($utc >= 0 ? "+" : "") . "$utc)") : "");
    } else {
        echo count($holidays) . " holidays found on query \"$search\"";
    }
    ?></title>
</head>

<body>
    <h1><?php
    if ($search == "") {
        echo "$holiday_count holidays on $day.$month" . ($utc != 0 ? (" (UTC" . ($utc >= 0 ? "+" : "") . "$utc)") : "");
    } else {
        echo count($holidays) . " holidays found on query \"$search\"";
    }
    ?></h1>
    <ul>
        <?php
        foreach ($holidays as $holiday) {
            if ($search == "") {
                echo '<li>' . $holiday["name"] . '</li>';
            } else {
                echo '<li>' . $holiday["name"] . ' (' . $holiday["date"][1] . '.' . $holiday["date"][0] . ')</li>';
            }
        }
        ?>
    </ul>
</body>

</html>