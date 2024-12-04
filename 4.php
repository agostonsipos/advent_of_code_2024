<?php

$s = file_get_contents("4example");
//$s = file_get_contents("4input");

$t = [];
foreach (explode("\n", $s) as $l) {
	if ($l != "") {
        $t[] = $l;
    }
}

function letter($t, $i, $j) {
    if ($i < 0 || $i >= sizeof($t) || $j < 0 || $j >= strlen($t[$i])) {
        return ".";
    }
    return $t[$i][$j];
}

$dirs = [[-1,-1], [-1,0], [-1,1], [0,1], [1,1], [1,0], [1,-1], [0,-1]];
$pattern = "XMAS";

$c = 0;
for ($i = 0; $i < sizeof($t); ++$i) {
    for ($j = 0; $j < strlen($t[$i]); ++$j) {
        if (letter($t, $i, $j) == 'X') {
            foreach($dirs as $dir) {
                $a = $i; $b = $j;
                $ok = true;
                for ($k = 1; $k <= 3; ++$k) {
                    $a += $dir[0];
                    $b += $dir[1];
                    if (letter($t, $a, $b) != $pattern[$k]) {
                        $ok = false;
                        break;
                    }
                }
                if ($ok) {
                    ++$c;
                }
            }
        }
    }
}

echo($c."\n");
echo("<br/>\n");

$c = 0;
for ($i = 0; $i < sizeof($t); ++$i) {
    for ($j = 0; $j < strlen($t[$i]); ++$j) {
        if (letter($t, $i, $j) == 'A') {
            if ((letter($t, $i-1, $j-1) == 'M' && letter($t, $i+1, $j+1) == 'S'
                    || letter($t, $i-1, $j-1) == 'S' && letter($t, $i+1, $j+1) == 'M')
                && (letter($t, $i+1, $j-1) == 'M' && letter($t, $i-1, $j+1) == 'S'
                    || letter($t, $i+1, $j-1) == 'S' && letter($t, $i-1, $j+1) == 'M')) {
                        $c++;
                    }

        }
    }
}

echo($c."\n");
