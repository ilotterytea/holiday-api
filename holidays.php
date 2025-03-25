<?php
function load_holidays()
{
    $files = glob("../data/*.json");
    $holidays = [];

    foreach ($files as $file) {
        $contents = file_get_contents($file);
        array_push($holidays, json_decode($contents, true));
    }

    return $holidays;
}

define("HOLIDAYS", load_holidays());

// other constants

define("CLIENT_REQUIRES_JSON", isset($_SERVER["HTTP_ACCEPT"]) && $_SERVER["HTTP_ACCEPT"] == "application/json");