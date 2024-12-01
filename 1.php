<?php

$s = file_get_contents("1example");
//$s = file_get_contents("1input");

$a1 = [];
$a2 = [];

foreach (explode("\n", $s) as $l) {
	if ($l != "") {
		$x = explode("   ", $l);
		$a1[] = $x[0];
		$a2[] = $x[1];
	}
}

sort($a1);
sort($a2);

$x = 0;

for ($i = 0; $i < sizeof($a1); ++$i) {
	$x += abs($a1[$i] - $a2[$i]);
}

echo($x);

echo("<br/>"); // such pretty, such html, wow...

$y = 0;

for ($i = 0; $i < sizeof($a1); ++$i) {
	$v = $a1[$i];
	for ($j = 0; $j < sizeof($a2); ++$j) {
		if ($v == $a2[$j]) {
			$y += $v;
		}
	}
}

echo($y);

?>

