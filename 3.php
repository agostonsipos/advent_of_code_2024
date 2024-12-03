<?php

$inp = file_get_contents("3example");
//$inp = file_get_contents("3input");

preg_match_all('/mul\(([0-9]+)\,([0-9]+)\)/', $inp, $matches);

$s = 0;
for ($i = 0; $i < sizeof($matches[1]); ++$i) {
	$s += $matches[1][$i] * $matches[2][$i];
}
echo($s);
echo("<br/>");

preg_match_all('/mul\(([0-9]+)\,([0-9]+)\)|do\(\)|don\'t\(\)/', $inp, $matches);

$s = 0;
$on = true;
for ($i = 0; $i < sizeof($matches[0]); ++$i) {
	if ($matches[0][$i] == "don't()") {
		$on = false;
	} else if ($matches[0][$i] == "do()") {
		$on = true;
	} else if ($on) {
		$s += $matches[1][$i] * $matches[2][$i];
	}
}

echo($s);
