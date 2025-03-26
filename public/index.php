<?php
include_once "../holidays.php";

$time = time();
$utc = intval($_GET["utc"] ?? "");

if (-12 <= $utc && $utc <= 12) {
    $time += 60 * 60 * $utc;
} else {
    $utc = 0;
}

$month = $_GET["month"] ?? date("n", $time);
$day = $_GET["day"] ?? date("d", $time);

$holidays = [];

foreach (HOLIDAYS[0] as $holiday) {
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
    <title><?php echo $holiday_count ?> holidays on <?php echo "$day.$month" ?>
        <?php echo $utc != 0 ? ("(UTC" . ($utc >= 0 ? "+" : "") . "$utc)") : "" ?>
    </title>
</head>

<body>
    <h1><?php echo $holiday_count ?> holidays on <?php echo "$day.$month" ?>
        <?php echo $utc != 0 ? ("(UTC" . ($utc >= 0 ? "+" : "") . "$utc)") : "" ?>
    </h1>
    <ul>
        <?php
        foreach ($holidays as $holiday) {
            echo '<li>' . $holiday["name"] . '</li>';
        }
        ?>
    </ul>
</body>

</html>