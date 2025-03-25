<?php
include_once "../holidays.php";

$month = $_GET["month"] ?? date("n");
$day = $_GET["day"] ?? date("d");

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
    <title><?php echo $holiday_count ?> holidays on <?php echo "$day.$month" ?></title>
</head>

<body>
    <h1><?php echo $holiday_count ?> holidays on <?php echo "$day.$month" ?></h1>
    <ul>
        <?php
        foreach ($holidays as $holiday) {
            echo '<li>' . $holiday["name"] . '</li>';
        }
        ?>
    </ul>
</body>

</html>