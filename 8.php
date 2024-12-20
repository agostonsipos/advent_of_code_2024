<?php

$s = file_get_contents("8example");
//$s = file_get_contents("8input");

$t = explode("\n", $s);

$N = sizeof($t);
$M = strlen($t[0]);
$towers = [];
for ($i = 0; $i < $N; ++$i) {
    for ($j = 0; $j < $M; ++$j) {
        $c = $t[$i][$j];
        if ($c != ".") {
            if (array_key_exists($c, $towers)) {
                $towers[$c][] = [$i, $j];
            } else {
                $towers[$c] = [[$i, $j]];
            }
        }
    }
}

function inside($N, $M, $x, $y) {
    return 0 <= $x && $x < $N && 0 <= $y && $y < $M;
}

$places = [];
foreach ($towers as $frequency) {
    for ($i = 0; $i < sizeof($frequency); ++$i) {
        for ($j = $i + 1; $j < sizeof($frequency); ++$j) {
            $t1 = $frequency[$i];
            $t2 = $frequency[$j];
            $x1 = $t1[0] * 2 - $t2[0];
            $x2 = $t2[0] * 2 - $t1[0];
            $y1 = $t1[1] * 2 - $t2[1];
            $y2 = $t2[1] * 2 - $t1[1];
            if (inside($N, $M, $x1, $y1)) {
                $places[$x1*$N+$y1] = 1;
            }
            if (inside($N, $M, $x2, $y2)) {
                $places[$x2*$N+$y2] = 1;
            }
        }
    }
}

$ans = sizeof($places);
echo("$ans\n");


$places = [];
foreach ($towers as $frequency) {
    for ($i = 0; $i < sizeof($frequency); ++$i) {
        for ($j = $i + 1; $j < sizeof($frequency); ++$j) {
            $t1 = $frequency[$i];
            $t2 = $frequency[$j];
            $x1 = $t1[0];
            $y1 = $t1[1];
            while (inside($N, $M, $x1, $y1)) {
                $places[$x1*$N+$y1] = 1;
                $x1 += $t1[0] - $t2[0];
                $y1 += $t1[1] - $t2[1];
            }
            $x2 = $t2[0];
            $y2 = $t2[1];
            while (inside($N, $M, $x2, $y2)) {
                $places[$x2*$N+$y2] = 1;
                $x2 += $t2[0] - $t1[0];
                $y2 += $t2[1] - $t1[1];
            }
        }
    }
}


$ans = sizeof($places);
echo("$ans\n");
