<?php

$s = file_get_contents("6example");
$s = file_get_contents("6input");

$t = explode("\n", $s);

$blocks = [];

$n = sizeof($t);
$m = strlen($t[0]);

for ($i = 0; $i < $n; ++$i) {
    for ($j = 0; $j < $m; ++$j) {
        $c = $t[$i][$j];
        if ($c == "#") {
            $blocks[] = [$i, $j];
        } else if ($c == "^") {
            $pos = [$i, $j];
        }
    }
}

function blocked($blocks, $pos) {
    foreach ($blocks as $block) {
        if ($block[0] == $pos[0] && $block[1] == $pos[1]) {
            return true;
        }
    }
    return false;
}

function visit($pos, $blocks, $n, $m) {

    $dir = [-1, 0];
    $visited = [];
    $states = [];

    while (0 <= $pos[0] && $pos[0] < $n && 0 <= $pos[1] && $pos[1] < $m) {
        if (blocked($blocks, $pos)) {
            $pos[0] -= $dir[0];
            $pos[1] -= $dir[1];
            $newdir = [$dir[1], -$dir[0]];
            $dir = $newdir;
        } else {
            $visited[] = $pos[0] * $m + $pos[1];
            $state = ($pos[0] * $m + $pos[1]) * 5 + ($dir[0]*2+$dir[1]);
            if (array_search($state, $states)) {
                //echo("loop");
                return NULL;
            }
            $states[] = $state;
        }
        $pos[0] += $dir[0];
        $pos[1] += $dir[1];
    }
    sort($visited);
    return array_unique($visited);
}

$visited = visit($pos, $blocks, $n, $m);

echo(sizeof($visited)."\n");

$c = 0;
foreach ($visited as $newblock) {
    $blocksplus = $blocks;
    $blocksplus[] = [intdiv($newblock, $m), $newblock % $m];
    if (visit($pos, $blocksplus, $n, $m) == NULL) {
        $c++;
    }
}

echo($c."\n");
