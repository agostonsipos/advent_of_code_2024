<?php

$s = file_get_contents("7example");
//$s = file_get_contents("7input");

$t = explode("\n", $s);

$s1 = 0;
$s2 = 0;
foreach ($t as $l) {
    $eq = explode(": ", $l);

    $nop = substr_count($eq[1], " ");
    $nconf = pow(2, $nop);

    $rhs = explode(" ", $eq[1]);
    for ($i = 0; $i < $nconf; ++$i) {
        $j = $i;
        $s = $rhs[0];
        for ($ind = 1; $ind < sizeof($rhs); ++$ind) {
            if ($j % 2 == 1) {
                $s += $rhs[$ind];
            } else {
                $s *= $rhs[$ind];
            }
            $j = intdiv($j, 2);
        }
        if ($eq[0] == $s) {
            $s1 += $s;
            break;
        }
    }
    $nconf = pow(3, $nop);
    $rhs = explode(" ", $eq[1]);
    for ($i = 0; $i < $nconf; ++$i) {
        $j = $i;
        $s = $rhs[0];
        for ($ind = 1; $ind < sizeof($rhs); ++$ind) {
            if ($j % 3 == 0) {
                $s += $rhs[$ind];
            } else if ($j % 3 == 1) {
                $s *= $rhs[$ind];
            } else {
                $s .= $rhs[$ind];
            }
            $j = intdiv($j, 3);
        }
        if ($eq[0] == $s) {
            $s2 += $s;
            break;
        }
    }
}

echo("$s1\n");
echo("$s2\n");